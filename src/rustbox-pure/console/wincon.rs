/* This windows console API encapsulates all of the win32 data types and unsafe FFI calls,
exposing the useful console functions in a more rustic manner */

use super::winapi::{
    HANDLE,
    CHAR, SHORT, WORD, DWORD, BOOL,
    LPCSTR, LPCWSTR, LPDWORD,
    CONSOLE_SCREEN_BUFFER_INFO, PCONSOLE_SCREEN_BUFFER_INFO,
    CONSOLE_CURSOR_INFO, PCONSOLE_CURSOR_INFO,
    COORD, SMALL_RECT,
    STD_OUTPUT_HANDLE
};

use super::kernel32::{
    GetStdHandle,
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
    SetConsoleCursorPosition
};

#[derive(Clone, Copy)]
pub struct Handle
{
    ptr: HANDLE
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
    let result = unsafe {
        GetStdHandle(STD_OUTPUT_HANDLE)
    };

    if result as isize <= 0 {
        None
    }
    else {
        Some(Handle { ptr: result })
    }
}

/* NOTE: The following ffi calls provide bool return value, and some of them provide other
result values (such as number of characters written), both of which are ignored even though
they maybe shouldn't be. */

fn screen_buffer_info(handle: Handle) -> CONSOLE_SCREEN_BUFFER_INFO
{
    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: COORD {X: 0, Y: 0},
        dwCursorPosition: COORD {X: 0, Y: 0},
        wAttributes: 0 as WORD,
        srWindow: SMALL_RECT {Top: 0, Bottom: 0, Left: 0, Right: 0},
        dwMaximumWindowSize: COORD {X: 0, Y: 0},
    };

    unsafe {
        GetConsoleScreenBufferInfo(handle.ptr, &mut csbi as PCONSOLE_SCREEN_BUFFER_INFO);
    }

    return csbi;
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
            handle.ptr,
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
            handle.ptr,
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
            handle.ptr,
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
            handle.ptr,
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

    unsafe {
        GetConsoleCursorInfo(handle.ptr, &mut cci as PCONSOLE_CURSOR_INFO);
    }

    return cci.bVisible != 0;
}

pub fn set_cursor_visible(handle: Handle, visible: bool)
{
    let cci = CONSOLE_CURSOR_INFO {
        dwSize: 25 as DWORD,
        bVisible: visible as BOOL
    };

    unsafe {
        SetConsoleCursorInfo(handle.ptr, &cci as *const CONSOLE_CURSOR_INFO);
    }
}

pub fn cursor_location(handle: Handle) -> Location
{
    let coord = screen_buffer_info(handle).dwCursorPosition;

    Location {x: coord.X as usize, y: coord.Y as usize}
}

pub fn set_cursor_location(handle: Handle, location: Location)
{
    let coord = COORD {X: location.x as SHORT, Y: location.y as SHORT};

    unsafe {
        SetConsoleCursorPosition(handle.ptr, coord);
    }
}

/* Full Declaration Reference for Imported FFI Functions

pub fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
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
pub fn SetConsoleCursorPosition(hConsoleOutput: HANDLE, dwCursorPosition: COORD) -> BOOL; */
