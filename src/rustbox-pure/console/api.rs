extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate ole32;

pub use self::winapi::{
    /* TYPES */
    HANDLE, HWND, HMODULE, HWINEVENTHOOK, MSG, LPMSG, LPARAM, WPARAM, POINT, CHAR, SHORT, WORD,
    INT, UINT, LONG, DWORD, BOOL, LPCSTR, LPCWSTR, LPDWORD, LPVOID, CONSOLE_SCREEN_BUFFER_INFO,
    PCONSOLE_SCREEN_BUFFER_INFO, CONSOLE_CURSOR_INFO, PCONSOLE_CURSOR_INFO, COORD, SMALL_RECT,
    INPUT_RECORD, PINPUT_RECORD, SECURITY_ATTRIBUTES, MOUSE_EVENT_RECORD, KEY_EVENT_RECORD,
    WINDOW_BUFFER_SIZE_RECORD, KEY_EVENT, MOUSE_EVENT, WINDOW_BUFFER_SIZE_EVENT,

    /* VALUES */
    STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, ENABLE_MOUSE_INPUT, ENABLE_PROCESSED_INPUT, SB_BOTH,
    COINIT_APARTMENTTHREADED, FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE,
	MOUSE_MOVED, MOUSE_WHEELED, FROM_LEFT_1ST_BUTTON_PRESSED, RIGHTMOST_BUTTON_PRESSED,
	LEFT_CTRL_PRESSED, RIGHT_CTRL_PRESSED, VK_TAB, VK_RETURN, VK_ESCAPE, VK_BACK, VK_RIGHT,
    VK_UP, VK_LEFT, VK_DOWN, VK_DELETE, VK_HOME, VK_END, VK_PRIOR, VK_NEXT, VK_F1, VK_F24,
	FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, FOREGROUND_INTENSITY,
	BACKGROUND_RED, BACKGROUND_GREEN, BACKGROUND_BLUE, BACKGROUND_INTENSITY
};

pub use self::kernel32::{
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
    CreateConsoleScreenBuffer
};

pub use self::user32::{
    GetWindowThreadProcessId,
    GetMessageW,
    EnableScrollBar
};

pub use self::ole32::{
    CoInitializeEx,
    CoUninitialize
};

extern "system" {

    pub fn SetWinEventHook(
        eventmin: UINT, eventMax: UINT, hmodWinEventProc: HMODULE, lpfnWinEventProc: WINEVENTPROC,
        idProcess: DWORD, idThread: DWORD, dwflags: UINT
    ) -> HWINEVENTHOOK;

    pub fn UnhookWinEvent(hWinEventHook: HWINEVENTHOOK) -> BOOL;
}

pub type WINEVENTPROC = extern fn(HWINEVENTHOOK, DWORD, HWND, LONG, LONG, DWORD, DWORD);

pub const WINEVENT_OUTOFCONTEXT: DWORD = 0x0000_0000;
pub const EVENT_SYSTEM_MOVESIZESTART: DWORD = 0x0000_000A;
pub const EVENT_SYSTEM_MOVESIZEEND: DWORD = 0x0000_000B;
pub const ESB_DISABLE_BOTH: DWORD = 0x0000_0003;
pub const ESB_ENABLE_BOTH: DWORD = 0x0000_0000;
pub const CONSOLE_TEXTMODE_BUFFER: DWORD = 0x0000_0001;
