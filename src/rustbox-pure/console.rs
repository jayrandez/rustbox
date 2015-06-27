/* This windows console API encapsulates all of the win32 data types and unsafe FFI calls,
exposing the console in a more rustic manner */

use super::winapi::{
    DWORD,
    LPDWORD,
    HANDLE,
    BOOL,
    LPCSTR,
    LPCWSTR,
    CONSOLE_SCREEN_BUFFER_INFO,
    PCONSOLE_SCREEN_BUFFER_INFO,
    COORD,
    SMALL_RECT,
    CONSOLE_CURSOR_INFO
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

/* Full Declarations for Imported FFI Functions

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
