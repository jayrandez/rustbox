use super::wincon::*;

pub fn beginDisplay(handle: Handle) -> (Size, usize) {
    /* Begin display should set up the console with the necessary properties (buffer capacity,
    window size, font), and display a blank region for rustbox to use, while preserving the
    original console contents above this region. */

    // TODO: Set up window parameters buffer size, window size, font
    // ...

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

    // TEMP
    fill_character(handle, b'A', visible_size.width * visible_size.height, Location {x: 0, y: display_line});
    fill_attribute(handle, 12312 as u16, visible_size.width * visible_size.height, Location {x: 0, y: display_line});

    return (visible_size, display_line);
}

pub fn finishDisplay(handle: Handle, display_line: usize) {
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
