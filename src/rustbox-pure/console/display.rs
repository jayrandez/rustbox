fn beginDisplay(handle: Handle) -> (Size, usize) {
    /* Begin display should set up the console with the necessary properties (buffer capacity,
    window size, font), and display a blank region for rustbox to use, while preserving the
    original console contents above this region. */

    // TODO: Set up window parameters buffer size, window size, font
    // ...

    // Scroll console to clear a region at least the height of the visible window. Kludgy
    let visible_size = console::visible_size(self.handle);
    for i in 0..visible_size.height {
        println!("");
    }

    // Set display_line for the display to the y origin of the region just cleared
    let cursor_line = console::cursor_location(self.handle).y;
    let display_line = cursor_line - visible_size.height + 1;

    // By default, hide the cursor
    console::set_cursor_visible(self.handle, false);

    // TEMP
    console::fill_character(self.handle, b'A', visible_size.width * visible_size.height, Location {x: 0, y: display_line});
    console::fill_attribute(self.handle, 12312 as u16, visible_size.width * visible_size.height, Location {x: 0, y: display_line});

    return (visible_size, display_line);
}

fn finishDisplay(handle: Handle) {
    /* Finish display should restore the original console properties (buffer capacity, window
    size, font), clear the display area used by rustbox, and place the cursor one line below
    the rustbox display. NOTE: May clear display area to be consistent with ncurses. */

    // Redisplay cursor
    console::set_cursor_visible(self.handle, true);

    // Place cursor at origin of rustbox display region
    console::set_cursor_location(self.handle, Location {x: 0, y: self.display_line});

    // Scroll console until cursor is just below the rustbox display region. Kludgy
    let visible_size = console::visible_size(self.handle);
    for i in 0..visible_size.height {
        println!("");
    }

    // TODO: Restore original window parameters buffer size, window size, font
    // ...
}
