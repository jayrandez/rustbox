use rustbox::console::{Handle, Size, Location, DisplayInfo};
use rustbox::console::wincon;

pub fn begin_display() -> DisplayInfo {
    /* Begin display should set up the console with the necessary properties (buffer capacity,
    window size, font), and display a blank region for rustbox to use, while preserving the
    original console contents above this region. */

    let handle = Handle {
		window: wincon::window_handle(),
		input: wincon::stdin_buffer(),
		output: wincon::stdout_buffer()
	};

    wincon::set_buffer_size(handle, Size {width: 800, height: 800});
    wincon::set_scroll_enable(handle, false);

    // Scroll console to clear a region at least the height of the visible window. Kludgy
    let visible_size = wincon::visible_size(handle);
    for i in 0..visible_size.height {
        println!("");
    }

    // Set display_line for the display to the y origin of the region just cleared
    let cursor_line = wincon::cursor_location(handle).y;
    let display_line = cursor_line - visible_size.height + 1;

    // By default, hide the cursor
    wincon::set_cursor_visible(handle, false);

    DisplayInfo {
		handle: handle,
		visible_size: visible_size,
		display_line: display_line
	}
}

pub fn finish_display(handle: Handle, display_line: usize) {
    /* Finish display should restore the original console properties (buffer capacity, window
    size, font), clear the display area used by rustbox, and place the cursor one line below
    the rustbox display. NOTE: May clear display area to be consistent with ncurses. */

    // Redisplay cursor
    wincon::set_cursor_visible(handle, true);

    // Place cursor at origin of rustbox display region
    wincon::set_cursor_location(handle, Location {x: 0, y: display_line});

    // Scroll console until cursor is just below the rustbox display region. Kludgy
    let visible_size = wincon::visible_size(handle);
    for i in 0..visible_size.height {
        println!("");
    }

    // TODO: Restore original window parameters buffer size, window size, font
    // ...
}
