extern crate winapi;
extern crate kernel32;

mod running;
mod console;
pub mod event;
pub mod style;
pub mod keyboard;
pub mod mouse;

pub use self::event::{Event, EventResult};
pub use self::keyboard::Key;
pub use self::mouse::Mouse;
pub use self::running::running;
pub use self::style::{Color, Style, RB_BOLD, RB_UNDERLINE, RB_REVERSE, RB_NORMAL};
pub use self::console::{Handle, Size, Location};

use std::default::Default;
use std::error::Error;
use std::{fmt, io, char};
use num::FromPrimitive;
use libc::c_int;
use time::Duration;

#[derive(Clone, Copy, Debug)]
pub enum InputMode {
    Current = 0x00,
    /// When ESC sequence is in the buffer and it doesn't match any known
    /// ESC sequence => ESC means TB_KEY_ESC
    Esc = 0x01,
    /// When ESC sequence is in the buffer and it doesn't match any known
    /// sequence => ESC enables TB_MOD_ALT modifier for the next keyboard event.
    Alt = 0x02,
    /// Same as `Esc` but enables mouse events
    EscMouse = 0x05,
    /// Same as `Alt` but enables mouse events
    AltMouse = 0x06
}

#[derive(Debug)]
pub enum InitError {
    BufferStderrFailed(io::Error),
    AlreadyOpen,
    UnsupportedTerminal,
    FailedToOpenTTy,
    PipeTrapError,
    Unknown(isize),
}

impl fmt::Display for InitError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description())
    }
}

impl Error for InitError {
    fn description(&self) -> &str {
        match *self {
            InitError::BufferStderrFailed(_) => "Could not redirect stderr",
            InitError::AlreadyOpen => "RustBox is already open",
            InitError::UnsupportedTerminal => "Unsupported terminal",
            InitError::FailedToOpenTTy => "Failed to open TTY",
            InitError::PipeTrapError => "Pipe trap error",
            InitError::Unknown(_) => "Unknown error from Termbox",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            InitError::BufferStderrFailed(ref e) => Some(e),
            _ => None
        }
    }
}

impl FromPrimitive for InitError {
   fn from_i64(n: i64) -> Option<InitError> {
      match n {
         -1 => Some(InitError::UnsupportedTerminal),
         -2 => Some(InitError::FailedToOpenTTy),
         -3 => Some(InitError::PipeTrapError),
         n => Some(InitError::Unknown(n as isize)),
      }
   }

   fn from_u64(n: u64) -> Option<InitError> {
      Some(InitError::Unknown(n as isize))
   }
}

#[derive(Clone, Copy,Debug)]
pub struct InitOptions {
    /// Use this option to initialize with a specific input mode.
    /// See InputMode enum for details on the variants.

    pub input_mode: InputMode,

    /// NOTE: buffer_stderr remains for API consistency, but is not supported on Windows.
    /// Functionality will eventually converge between Linux/OSX/Windows.

    pub buffer_stderr: bool,
}

impl Default for InitOptions {
    fn default() -> Self {
        InitOptions {
            input_mode: InputMode::Current,
            buffer_stderr: false,
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct RustBox {
    handle: Handle,
    display_line: usize,
    /* Note that running *MUST* be the last field in the destructor, since destructors run in
    top-down order. Otherwise it will not properly protect the above fields. */
    _running: running::RunningGuard
}

// Termbox is not thread safe
impl !Send for RustBox {}

impl RustBox {
    /// Initialize Rustbox.
    ///
    /// For the default options, you can use:
    /// ```
    /// use rustbox::RustBox;
    /// use std::default::Default;
    /// let rb = RustBox::init(Default::default());
    /// ```
    ///
    /// Otherwise, you can specify:
    /// ```
    /// use rustbox::{RustBox, InitOptions};
    /// use std::default::Default;
    /// let rb = RustBox::init(InitOptions {
    ///     input_mode: rustbox::InputMode::Esc,
    ///    buffer_stderr: false
    /// });
    /// ```
    ///
    /// Again, buffer_stderr is unimplemented on windows.

    pub fn init(opts: InitOptions) -> Result<RustBox, InitError> {
        let running = match running::run() {
            Some(r) => r,
            None => return Err(InitError::AlreadyOpen),
        };

        let handle = match console::handle() {
            Some(val) => val,
            None => return Err(InitError::UnsupportedTerminal)
        };

        // Create the RustBox.
        let mut rb = RustBox {
            handle: handle,
            display_line: 0,
            _running: running
        };

        rb.beginDisplay();

        match opts.input_mode {
            InputMode::Current => (),
            _ => rb.set_input_mode(opts.input_mode),
        }

        Ok(rb)
    }

    fn beginDisplay(&mut self)
    {
        /* Begin display should set up the console with the necessary properties (buffer capacity,
        window size, font), and display a blank region for rustbox to use, while preserving the
        original console contents above this region. */

        // TODO: Set up window parameters buffer size, window size, font
        // ...

        // Scroll console to clear a region at least the height of the visible window. Kludgy
        let visible_size = console::visible_size(self.handle);
        for i in 0..visible_size.height {
            println!("");
        }

        // Set display_line for the display to the y origin of the region just cleared
        let cursor_line = console::cursor_location(self.handle).y;
        self.display_line = cursor_line - visible_size.height + 1;

        // By default, hide the cursor
        console::set_cursor_visible(self.handle, false);
    }

    fn finishDisplay(&mut self)
    {
        /* Finish display should restore the original console properties (buffer capacity, window
        size, font), clear the display area used by rustbox, and place the cursor one line below
        its original location before the rustbox display began. */

        // Redisplay cursor
        console::set_cursor_visible(self.handle, true);

        // Place cursor at origin of rustbox display region
        console::set_cursor_location(self.handle, Location {x: 0, y: self.display_line});

        // Scroll console until cursor is just below the rustbox display region. Kludgy
        let visible_size = console::visible_size(self.handle);
        for i in 0..visible_size.height {
            println!("");
        }

        // TODO: Restore original window parameters buffer size, window size, font
        // ...
    }

    pub fn width(&self) -> usize {
        0
    }

    pub fn height(&self) -> usize {
        0
    }

    pub fn clear(&self) {

    }

    pub fn present(&self) {

    }

    pub fn set_cursor(&self, x: isize, y: isize) {

    }

    pub unsafe fn change_cell(&self, x: usize, y: usize, ch: u32, fg: u16, bg: u16) {

    }

    pub fn print(&self, x: usize, y: usize, sty: Style, fg: Color, bg: Color, s: &str) {

    }

    pub fn print_char(&self, x: usize, y: usize, sty: Style, fg: Color, bg: Color, ch: char) {

    }

    pub fn poll_event(&self, raw: bool) -> EventResult {
        Ok(Event::NoEvent)
    }

    pub fn peek_event(&self, timeout: Duration, raw: bool) -> EventResult {
        Ok(Event::NoEvent)
    }

    pub fn set_input_mode(&self, mode: InputMode) {

    }
}

impl Drop for RustBox {
    fn drop(&mut self) {
        /* Since only one instance of the RustBox is ever accessible, we should not
        need to do this atomically.
        NOTE: we should definitely have RUSTBOX_RUNNING = true here.*/

        self.finishDisplay();
    }
}
