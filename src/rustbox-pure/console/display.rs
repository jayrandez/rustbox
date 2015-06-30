use super::wincon::*;
use std::thread;
use std::thread::JoinHandle;

/*
BEGINNING OF CODE FOR RESIZE HOOK

static mut console_window: HWND = 0 as HWND;
static mut console_handle: Handle = Handle {input: 0 as HANDLE, output: 0 as HANDLE};

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

pub fn hook_monitor(handle: Handle) -> JoinHandle<()>
{
    unsafe { console_handle = handle; }
    unsafe { set_buffer_size(console_handle, Size {width: 800, height: 800}); }

    thread::spawn(move || {
        store_hwnd();
        set_scroll_enable(false);

        co_initialize();

        let hook = event_hook();
        while let Some(msg) = get_message() {}

        event_unhook(hook);
        co_uninitialize();
    })
}

extern fn callback(hWinEventHook: HWINEVENTHOOK, event: DWORD, hwnd: HWND, idObject: LONG, idChild: LONG, dwEventThread: DWORD, dwmsEventTime: DWORD) -> () {
    let for_console = unsafe { hwnd == console_window };

    if(for_console) {
        match(event) {
            EVENT_SYSTEM_MOVESIZESTART => resize_start(),
            EVENT_SYSTEM_MOVESIZEEND => resize_stop(),
            _ => {}
        }
    }
}

pub fn resize_start() {
}

pub fn resize_stop() {
}*/

pub fn begin_display(handle: Handle) -> (Size, usize) {
    /* Begin display should set up the console with the necessary properties (buffer capacity,
    window size, font), and display a blank region for rustbox to use, while preserving the
    original console contents above this region. */
    set_buffer_size(handle, Size {width: 800, height: 800});
    set_scroll_enable(handle, false);

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
