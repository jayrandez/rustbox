pub mod event;
pub mod keyboard;
pub mod cell;
pub mod style;

mod running;
mod console;

pub use self::event::{Event, Mouse, Key, EventResult};
pub use self::cell::{Cell, CellBuffer};
pub use self::style::{Color, Style, RB_BOLD, RB_UNDERLINE, RB_REVERSE, RB_NORMAL};

use self::running::running;
use self::console::{DisplayInfo, Handle, RawEvent, Size, Location};

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

        /* This function will eventually return a DisplayInfo struct encapsulating (in addition
        to visible_size and display_line) the original state to be restored when finished */
        let DisplayInfo {
            handle: handle,
            visible_size: visible_size,
            display_line: display_line
        } = console::begin_display();

        // For now enable mouse input, ctrl-c by default
        console::set_mode(handle, true, true);

        let Size {width: width, height: height} = visible_size;

        let default_attr = console::translate_attr(Color::Default, Color::Black, style::RB_NORMAL);
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

    pub fn shutdown(self) {}

    pub fn width(&self) -> usize {
        self.cell_buffer.width
    }

    pub fn height(&self) -> usize {
        self.cell_buffer.height
    }

    pub fn clear(&mut self) {
        let Size {width: width, height: height} = console::visible_size(self.handle);

        // Resize backbuffer if its size doesn't match the visible size.
        if width != self.cell_buffer.width || height != self.cell_buffer.height {
            self.cell_buffer.resize_blindly(width, height);
        }

        let char_slice = self.cell_buffer.char_buffer.as_mut_slice();
        let attr_slice = self.cell_buffer.attr_buffer.as_mut_slice();

        for i in 0..char_slice.len() {
            char_slice[i] = b' ';
            attr_slice[i] = self.default_attr;
        }
    }

    pub fn set_clear_attributes(&mut self, foreground: Color, background: Color, style: Style) {
        self.default_attr = console::translate_attr(foreground, background, style);
    }

    pub fn present(&mut self) {
        let Size {width: width, height: height} = console::visible_size(self.handle);

        // Resize backbuffer if its size doesn't match the visible size.
        if width != self.cell_buffer.width || height != self.cell_buffer.height {
            self.cell_buffer.resize(width, height, b' ', self.default_attr);
        }

        let char_slice = self.cell_buffer.char_buffer.as_slice();
        let attr_slice = self.cell_buffer.attr_buffer.as_slice();

        // Copy line-by-line, since buffer width is not equal to visible width.
        for line in 0..height {
            let index = line * width;

            let char_subslice = &char_slice[index..(index + width)];
            let attr_subslice = &attr_slice[index..(index + width)];

            let origin = Location {x: 0, y: line + self.display_line};
            console::write_characters(self.handle, char_subslice, origin);
            console::write_attributes(self.handle, attr_subslice, origin);
        }
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

    pub fn change_cell(&mut self, x: usize, y: usize, ch: u32, fg: Color, bg: Color, sty: Style) {
        let width = self.cell_buffer.width;
        let height = self.cell_buffer.height;

        if x < width && y < height {
            let attr = console::translate_attr(fg, bg, sty);

            let char_slice = self.cell_buffer.char_buffer.as_mut_slice();
            let attr_slice = self.cell_buffer.attr_buffer.as_mut_slice();

            let index = (y * width) + x;
            char_slice[index] = ch as u8;
            attr_slice[index] = attr;
        }
    }

    pub fn put_cell(&mut self, x: usize, y: usize, cell: Cell)
    {
        self.change_cell(x, y, cell.ch as u32, cell.fg, cell.bg, cell.sty);
    }

    pub fn print(&self, x: usize, y: usize, sty: Style, fg: Color, bg: Color, s: &str) {

    }

    pub fn print_char(&self, x: usize, y: usize, sty: Style, fg: Color, bg: Color, ch: char) {

    }

    pub fn poll_event(&self, raw: bool) -> EventResult {
        /* Don't like the way this is implemented. I think Event::NoEvent is un-rustic,
        should be indicated by None instead.

        Also there is currently no error-handling in wincon.rs, so Err result is not used. */

        let raw_event = console::read_input(self.handle);

        if let Some(event) = console::translate_event(raw_event) { Ok(event) }
        else { Ok(Event::NoEvent)  }
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
        console::finish_display(self.handle, self.display_line);
    }
}
