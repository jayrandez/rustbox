pub mod event;
pub mod keyboard;
pub mod mouse;
pub mod cell;
pub mod style;

mod running;
mod console;

pub use self::event::{Event, EventResult};
pub use self::keyboard::Key;
pub use self::mouse::Mouse;
pub use self::cell::{Cell, CellBuffer};
pub use self::style::{Color, Style, RB_BOLD, RB_UNDERLINE, RB_REVERSE, RB_NORMAL};

use self::running::running;
use self::console::{Handle, Size, Location};

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
    cell_buffer: CellBuffer,
    default_attr: u16,
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

        /* This function will eventually return a DisplayInfo struct encapsulating (in addition
        to visible_size and display_line) the original state to be restored when finished */
        let (visible_size, display_line) = console::beginDisplay(handle);
        let Size {width: width, height: height} = visible_size;

        let default_attr = console::attr_translate(Color::Default, Color::Black, style::RB_NORMAL);
        let cell_buffer = CellBuffer::new(width, height, b' ', default_attr);

        // Create the RustBox.
        let mut rb = RustBox {
            handle: handle,
            display_line: display_line,
            cell_buffer: cell_buffer,
            default_attr: default_attr,
            _running: running
        };

        match opts.input_mode {
            InputMode::Current => (),
            _ => rb.set_input_mode(opts.input_mode),
        };

        Ok(rb)
    }

    pub fn width(&self) -> usize {
        // TODO: !! This should report the internal buffer size, for consistency with termbox.
        console::visible_size(self.handle).width
    }

    pub fn height(&self) -> usize {
        // TODO: !! This should report the internal buffer size for consistency with termbox.
        console::visible_size(self.handle).height
    }

    pub fn clear(&self) {

    }

    pub fn present(&self) {

    }

    pub fn set_cursor(&self, x: isize, y: isize) {
        if x == -1 && y == -1 {
            console::set_cursor_visible(self.handle, false);
        }
        else {
            let (x, y) = (x as usize, y as usize);
            let visible_size = console::visible_size(self.handle);

            if x < visible_size.width && y < visible_size.height {
                let location = Location {x: x, y: self.display_line + y};
                console::set_cursor_location(self.handle, location);
                console::set_cursor_visible(self.handle, true);
            }
        }
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

        /* See sibling comment for console::startDisplay(). Will receive inst of DisplayInfo */
        console::finishDisplay(self.handle, self.display_line);
    }
}
