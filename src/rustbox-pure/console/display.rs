use super::wincon::*;
use std::thread;
use std::thread::JoinHandle;

/* --------------------------------------------------------------
   THESE TYPES SHOULD BE DEFINED BY CRATES "winapi", "user32-sys"
   -------------------------------------------------------------- */

type WINEVENTPROC = extern fn(HWINEVENTHOOK, DWORD, HWND, LONG, LONG, DWORD, DWORD);

const WINEVENT_OUTOFCONTEXT: DWORD = 0x0000_0000;
const EVENT_SYSTEM_MOVESIZESTART: DWORD = 0x0000_000A;
const EVENT_SYSTEM_MOVESIZEEND: DWORD = 0x0000_000B;

extern "system" {
    pub fn SetWinEventHook(
        eventmin: UINT,
        eventMax: UINT,
        hmodWinEventProc: HMODULE,
        lpfnWinEventProc: WINEVENTPROC,
        idProcess: DWORD,
        idThread: DWORD,
        dwflags: UINT
    ) -> HWINEVENTHOOK;

    pub fn UnhookWinEvent(hWinEventHook: HWINEVENTHOOK) -> BOOL;
}

/* -------------------------------------------------------------
   END
   ------------------------------------------------------------- */

use super::winapi::{
    BOOL, INT, WORD, DWORD, LPDWORD, LPVOID, UINT, LONG,
    HWND, MSG, LPMSG, POINT, LPARAM, WPARAM,
    HWINEVENTHOOK, HMODULE, COINIT_APARTMENTTHREADED
};

use super::user32::{GetWindowThreadProcessId, GetMessageW};
use super::ole32::{CoInitializeEx, CoUninitialize};

static mut console_window: HWND = 0 as HWND;

pub fn co_initialize() {
    let result = unsafe { CoInitializeEx(0 as LPVOID, COINIT_APARTMENTTHREADED) };
}

pub fn co_uninitialize() {
    unsafe { CoUninitialize(); }
}

pub fn event_hook() -> HWINEVENTHOOK {
    unsafe {
        SetWinEventHook(
            EVENT_SYSTEM_MOVESIZESTART as UINT,
            EVENT_SYSTEM_MOVESIZEEND as UINT,
            0 as HMODULE,
            callback as WINEVENTPROC,
            0,
            0,
            WINEVENT_OUTOFCONTEXT
        )
    }
}

pub fn event_unhook(hook: HWINEVENTHOOK) {
    unsafe { UnhookWinEvent(hook); }
}

pub fn get_message() -> Option<MSG> {

    let mut message = MSG {
        hwnd: 0 as HWND,
        message: 0 as UINT,
        wParam: 0 as WPARAM,
        lParam: 0 as LPARAM,
        time: 0 as DWORD,
        pt: POINT {x: 0 as LONG, y: 0 as LONG}
    };

    let result: BOOL = unsafe {
        GetMessageW(&mut message as LPMSG, 0 as HWND, 0 as UINT, 0 as UINT)
    };

    if result != 0 { Some(message) } else { None }
}

pub fn hook_monitor() -> JoinHandle<()>
{
    thread::spawn(move || {
        store_hwnd();
        co_initialize();

        let hook = event_hook();
        while let Some(msg) = get_message() {}

        event_unhook(hook);
        co_uninitialize();
    })
}

pub fn store_hwnd() {
    unsafe { console_window = window_handle() };
}

extern fn callback(hWinEventHook: HWINEVENTHOOK, event: DWORD, hwnd: HWND, idObject: LONG, idChild: LONG, dwEventThread: DWORD, dwmsEventTime: DWORD) -> () {
    let for_console = unsafe { hwnd == console_window };

    if(for_console) {
        match(event) {
            EVENT_SYSTEM_MOVESIZESTART => println!("Resize start."),
            EVENT_SYSTEM_MOVESIZEEND => println!("Resize stop."),
            _ => {}
        }
    }
}

pub fn begin_display(handle: Handle) -> (Size, usize) {
    /* Begin display should set up the console with the necessary properties (buffer capacity,
    window size, font), and display a blank region for rustbox to use, while preserving the
    original console contents above this region. */

    // Scroll console to clear a region at least the height of the visible window. Kludgy
    let visible_size = visible_size(handle);
    for i in 0..visible_size.height {
        println!("");
    }

    // Set display_line for the display to the y origin of the region just cleared
    let cursor_line = cursor_location(handle).y;
    let display_line = cursor_line - visible_size.height + 1;

    // By default, hide the cursor
    set_cursor_visible(handle, false);

    return (visible_size, display_line);
}

pub fn finish_display(handle: Handle, display_line: usize) {
    /* Finish display should restore the original console properties (buffer capacity, window
    size, font), clear the display area used by rustbox, and place the cursor one line below
    the rustbox display. NOTE: May clear display area to be consistent with ncurses. */

    // Redisplay cursor
    set_cursor_visible(handle, true);

    // Place cursor at origin of rustbox display region
    set_cursor_location(handle, Location {x: 0, y: display_line});

    // Scroll console until cursor is just below the rustbox display region. Kludgy
    let visible_size = visible_size(handle);
    for i in 0..visible_size.height {
        println!("");
    }

    // TODO: Restore original window parameters buffer size, window size, font
    // ...
}
