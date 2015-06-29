/* This windows console API encapsulates all of the win32 data types and unsafe FFI calls,
exposing the useful console functions in a more rustic manner */

use super::winapi::{
    HANDLE, HWND,
    CHAR, SHORT, WORD, DWORD, BOOL,
    LPCSTR, LPCWSTR, LPDWORD,
    CONSOLE_SCREEN_BUFFER_INFO, PCONSOLE_SCREEN_BUFFER_INFO,
    CONSOLE_CURSOR_INFO, PCONSOLE_CURSOR_INFO,
    COORD, SMALL_RECT,
    STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
    ENABLE_MOUSE_INPUT, ENABLE_PROCESSED_INPUT,
    INPUT_RECORD, PINPUT_RECORD, MOUSE_EVENT_RECORD
};

use super::kernel32::{
    GetStdHandle,
    GetConsoleWindow,
    SetConsoleMode,
    GetConsoleScreenBufferInfo,
    SetConsoleScreenBufferSize,
    WriteConsoleOutputCharacterA,
    WriteConsoleOutputCharacterW,
    WriteConsoleOutputAttribute,
    FillConsoleOutputCharacterA,
    FillConsoleOutputCharacterW,
    FillConsoleOutputAttribute,
    GetConsoleCursorInfo,
    SetConsoleCursorInfo,
    SetConsoleCursorPosition,
    ReadConsoleInputA,
    ReadConsoleInputW,
};

#[derive(Clone, Copy)]
pub struct Handle
{
    input: HANDLE,
    output: HANDLE
}

#[derive(Clone, Copy)]
pub struct RawEvent
{
    pub record: INPUT_RECORD
}

#[derive(Clone, Copy)]
pub struct Size
{
    pub width: usize,
    pub height: usize
}

#[derive(Clone, Copy)]
pub struct Location
{
    pub x: usize,
    pub y: usize
}

pub fn handle() -> Option<Handle>
{
    let (in_handle, out_handle) = unsafe {
        (GetStdHandle(STD_INPUT_HANDLE), GetStdHandle(STD_OUTPUT_HANDLE))
    };

    if (in_handle as isize <= 0) || (out_handle as isize <= 0) {
        None
    }
    else {
        Some(Handle { input: in_handle, output: out_handle })
    }
}

pub fn window_handle() -> HWND
{
    unsafe { GetConsoleWindow() }
}

/* NOTE: The following ffi calls provide bool return value, and some of them provide other
result values (such as number of characters written), both of which are ignored even though
they maybe shouldn't be. */

pub fn set_mode(handle: Handle, enable_mouse: bool, enable_ctrlc: bool)
{
    let mut mode: DWORD = 0;
    if enable_mouse { mode = mode | ENABLE_MOUSE_INPUT; }
    if enable_ctrlc { mode = mode | ENABLE_PROCESSED_INPUT; }

    unsafe { SetConsoleMode(handle.input, mode); }
}

fn screen_buffer_info(handle: Handle) -> CONSOLE_SCREEN_BUFFER_INFO
{
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

pub fn buffer_size(handle: Handle) -> Size
{
    let csbi = screen_buffer_info(handle);

    Size {width: csbi.dwSize.X as usize, height: csbi.dwSize.Y as usize}
}

pub fn set_buffer_size(handle: Handle, size: Size)
{
    panic!("Set Buffer Size Unimplemented");
}

pub fn visible_size(handle: Handle) -> Size
{
    let csbi = screen_buffer_info(handle);

    let width = csbi.srWindow.Right - csbi.srWindow.Left + 1;
    let height = csbi.srWindow.Bottom - csbi.srWindow.Top + 1;

    Size {width: width as usize, height: height as usize}
}

pub fn set_visible_size(handle: Handle, size: Size)
{
    panic!("Set Visible Size Unimplemented");
}

pub fn visible_origin(handle: Handle) -> Location
{
    panic!("Visible Origin unimplemented.");
}

pub fn set_visible_origin(handle: Handle, location: Location)
{
    panic!("Set Visbile Origin unimplemented");
}

pub fn write_characters(handle: Handle, characters: &[u8], location: Location)
{
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

pub fn write_attributes(handle: Handle, attributes: &[u16], location: Location)
{
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

pub fn fill_character(handle: Handle, character: u8, length: usize, location: Location)
{
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

pub fn fill_attribute(handle: Handle, attribute: u16, length: usize, location: Location)
{
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

pub fn cursor_visible(handle: Handle) -> bool
{
    let mut cci = CONSOLE_CURSOR_INFO {
        dwSize: 0 as DWORD,
        bVisible: false as BOOL
    };

    unsafe { GetConsoleCursorInfo(handle.output, &mut cci as PCONSOLE_CURSOR_INFO); }

    cci.bVisible != 0
}

pub fn set_cursor_visible(handle: Handle, visible: bool)
{
    let cci = CONSOLE_CURSOR_INFO {
        dwSize: 25 as DWORD,
        bVisible: visible as BOOL
    };

    unsafe { SetConsoleCursorInfo(handle.output, &cci as *const CONSOLE_CURSOR_INFO); }
}

pub fn cursor_location(handle: Handle) -> Location
{
    let coord = screen_buffer_info(handle).dwCursorPosition;

    Location {x: coord.X as usize, y: coord.Y as usize}
}

pub fn set_cursor_location(handle: Handle, location: Location)
{
    let coord = COORD {X: location.x as SHORT, Y: location.y as SHORT};

    unsafe { SetConsoleCursorPosition(handle.output, coord); }
}

pub fn read_input(handle: Handle) -> RawEvent
{
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

/* Full Declaration Reference for Imported FFI Functions

pub fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
pub fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL
pub fn GetConsoleScreenBufferInfo(hConsoleOutput: HANDLE, lpConsoleScreenBufferInfo: PCONSOLE_SCREEN_BUFFER_INFO) -> BOOL;
pub fn SetConsoleScreenBufferSize(hConsoleOutput: HANDLE, dwSize: COORD) -> BOOL;
pub fn WriteConsoleOutputCharacterA(hConsoleOutput: HANDLE, lpCharacter: LPCSTR, nLength: DWORD, dwWriteCoord: COORD, lpNumberOfCharsWritten: LPDWORD) -> BOOL;
pub fn WriteConsoleOutputCharacterW(hConsoleOutput: HANDLE, lpCharacter: LPCWSTR, nLength: DWORD, dwWriteCoord: COORD, lpNumberOfCharsWritten: LPDWORD) -> BOOL;
pub fn WriteConsoleOutputAttribute(hConsoleOutput: HANDLE, lpAttribute: *const WORD, nLength: DWORD, dwWriteCoord: COORD, lpNumberOfAttrsWritten: LPDWORD) -> BOOL;
pub fn FillConsoleOutputCharacterA(hConsoleOutput: HANDLE, cCharacter: CHAR, nLength: DWORD, dwWriteCoord: COORD, lpNumberOfCharsWritten: LPDWORD) -> BOOL;
pub fn FillConsoleOutputCharacterW(hConsoleOutput: HANDLE, cCharacter: WCHAR, nLength: DWORD, dwWriteCoord: COORD, lpNumberOfCharsWritten: LPDWORD) -> BOOL;
pub fn FillConsoleOutputAttribute(hConsoleOutput: HANDLE, wAttribute: WORD, nLength: DWORD, dwWriteCoord: COORD, lpNumberOfAttrsWritten: LPDWORD) -> BOOL;
pub fn GetConsoleCursorInfo(hConsoleOutput: HANDLE, lpConsoleCursorInfo: PCONSOLE_CURSOR_INFO) -> BOOL;
pub fn SetConsoleCursorInfo(hConsoleOutput: HANDLE, lpConsoleCursorInfo: *const CONSOLE_CURSOR_INFO) -> BOOL;
pub fn SetConsoleCursorPosition(hConsoleOutput: HANDLE, dwCursorPosition: COORD) -> BOOL;
pub fn ReadConsoleInputA(hConsoleInput: HANDLE, lpBuffer: PINPUT_RECORD, nLength: DWORD, lpNumberOfEventsRead: LPDWORD) -> BOOL;
pub fn ReadConsoleInputW(hConsoleInput: HANDLE, lpBuffer: PINPUT_RECORD, nLength: DWORD, lpNumberOfEventsRead: LPDWORD) -> BOOL;
*/
