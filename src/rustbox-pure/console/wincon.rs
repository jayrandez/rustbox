use rustbox::console::{Handle, Size, Location, RawEvent};
use rustbox::console::api::*;

pub fn new_buffer() -> HANDLE {
    let handle = unsafe {
        CreateConsoleScreenBuffer(
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            0 as *const SECURITY_ATTRIBUTES,
            CONSOLE_TEXTMODE_BUFFER,
            0 as LPVOID
        )
    };
    println!("New buffer is {}", handle as u32);
    handle
}

pub fn window_handle() -> HWND {
    unsafe { GetConsoleWindow() }
}

pub fn stdin_buffer() -> HANDLE {
    unsafe { GetStdHandle(STD_INPUT_HANDLE) }
}

pub fn stdout_buffer() -> HANDLE {
    unsafe { GetStdHandle(STD_OUTPUT_HANDLE) }
}

pub fn set_mode(handle: Handle, enable_mouse: bool, enable_ctrlc: bool) {
    let mut mode: DWORD = 0;
    if enable_mouse { mode = mode | ENABLE_MOUSE_INPUT; }
    if enable_ctrlc { mode = mode | ENABLE_PROCESSED_INPUT; }

    unsafe { SetConsoleMode(handle.input, mode); }
}

fn screen_buffer_info(handle: Handle) -> CONSOLE_SCREEN_BUFFER_INFO {
    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: COORD {X: 0, Y: 0},
        dwCursorPosition: COORD {X: 0, Y: 0},
        wAttributes: 0 as WORD,
        srWindow: SMALL_RECT {Top: 0, Bottom: 0, Left: 0, Right: 0},
        dwMaximumWindowSize: COORD {X: 0, Y: 0},
    };

    unsafe { GetConsoleScreenBufferInfo(handle.output, &mut csbi as PCONSOLE_SCREEN_BUFFER_INFO); }

    csbi
}

pub fn buffer_size(handle: Handle) -> Size {
    let csbi = screen_buffer_info(handle);

    Size {width: csbi.dwSize.X as usize, height: csbi.dwSize.Y as usize}
}

pub fn set_buffer_size(handle: Handle, size: Size) {
    unsafe {
        SetConsoleScreenBufferSize(handle.output, COORD {X: size.width as SHORT, Y: size.height as SHORT});
    }
}

pub fn visible_size(handle: Handle) -> Size {
    let csbi = screen_buffer_info(handle);

    let width = csbi.srWindow.Right - csbi.srWindow.Left + 1;
    let height = csbi.srWindow.Bottom - csbi.srWindow.Top + 1;

    Size {width: width as usize, height: height as usize}
}

pub fn write_characters(handle: Handle, characters: &[u8], location: Location) {
    let mut _written: DWORD = 0;

    unsafe {
        WriteConsoleOutputCharacterA(
            handle.output,
            characters.as_ptr() as LPCSTR,
            characters.len() as DWORD,
            COORD {X: location.x as SHORT, Y: location.y as SHORT},
            &mut _written as LPDWORD
        );
    }
}

pub fn write_attributes(handle: Handle, attributes: &[u16], location: Location) {
    let mut _written: DWORD = 0;

    unsafe {
        WriteConsoleOutputAttribute(
            handle.output,
            attributes.as_ptr() as *const WORD,
            attributes.len() as DWORD,
            COORD {X: location.x as SHORT, Y: location.y as SHORT},
            &mut _written as LPDWORD
        );
    }
}

pub fn fill_character(handle: Handle, character: u8, length: usize, location: Location) {
    let mut _written: DWORD = 0;

    unsafe {
        FillConsoleOutputCharacterA(
            handle.output,
            character as CHAR,
            length as DWORD,
            COORD {X: location.x as SHORT, Y: location.y as SHORT},
            &mut _written as LPDWORD
        );
    }
}

pub fn fill_attribute(handle: Handle, attribute: u16, length: usize, location: Location) {
    let mut _written: DWORD = 0;

    unsafe {
        FillConsoleOutputAttribute(
            handle.output,
            attribute as WORD,
            length as DWORD,
            COORD {X: location.x as SHORT, Y: location.y as SHORT},
            &mut _written as LPDWORD
        );
    }
}

pub fn cursor_visible(handle: Handle) -> bool {
    let mut cci = CONSOLE_CURSOR_INFO {
        dwSize: 0 as DWORD,
        bVisible: false as BOOL
    };

    unsafe { GetConsoleCursorInfo(handle.output, &mut cci as PCONSOLE_CURSOR_INFO); }

    cci.bVisible != 0
}

pub fn set_cursor_visible(handle: Handle, visible: bool) {
    let cci = CONSOLE_CURSOR_INFO {
        dwSize: 25 as DWORD,
        bVisible: visible as BOOL
    };

    unsafe { SetConsoleCursorInfo(handle.output, &cci as *const CONSOLE_CURSOR_INFO); }
}

pub fn cursor_location(handle: Handle) -> Location {
    let coord = screen_buffer_info(handle).dwCursorPosition;

    Location {x: coord.X as usize, y: coord.Y as usize}
}

pub fn set_cursor_location(handle: Handle, location: Location) {
    let coord = COORD {X: location.x as SHORT, Y: location.y as SHORT};

    unsafe { SetConsoleCursorPosition(handle.output, coord); }
}

pub fn read_input(handle: Handle) -> RawEvent {
    let mut _read: DWORD = 0;

    /* NOTE: Based on comments in winapi->wincon.rs, this structure is subject to change, instead
    using enum (i.e. tagged union) of MOUSE_EVENT_RECORD, KEY_EVENT_RECORD, etc. */
    let mut record = INPUT_RECORD {
        EventType: 0 as WORD,
        Event: MOUSE_EVENT_RECORD {
            dwMousePosition: COORD {X: 0, Y: 0},
            dwButtonState: 0 as DWORD,
            dwControlKeyState: 0 as DWORD,
            dwEventFlags: 0 as DWORD,
        }
    };

    unsafe {
        ReadConsoleInputA(
            handle.input,
            &mut record as PINPUT_RECORD,
            1 as DWORD,
            &mut _read as LPDWORD
        );
    }

    RawEvent { record: record }
}

pub fn set_scroll_enable(handle: Handle, enable: bool) {
    unsafe {
        EnableScrollBar(
            handle.window,
            SB_BOTH as UINT,
            if enable {ESB_ENABLE_BOTH} else {ESB_DISABLE_BOTH}
        );
    }
}
