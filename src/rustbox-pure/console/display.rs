use rustbox::console::{Handle, Size, Location, DisplayInfo};
use rustbox::console::wincon;

pub fn begin_display() -> Handle {
    let handle = Handle {
		window: wincon::window_handle(),
		input: wincon::stdin_buffer(),
		output: wincon::create_buffer()
	};

    // Set console to use alternate output buffer.
    wincon::set_buffer(handle.output);

    // Make window resizeable.
    wincon::set_buffer_size(handle, Size {width: 800, height: 800});

    // Disable the scrollbars.
    wincon::set_scroll_enable(handle, false);

    // Hide cursor by default.
    wincon::set_cursor_visible(handle, false);

    wincon::set_font(handle, 12);

    return handle;
}

pub fn finish_display(handle: Handle) {
    // Restore stdout buffer as the displayed buffer.
    wincon::set_buffer(wincon::stdout_buffer());

    // Release the handle to the alternate output buffer.
    wincon::finish_buffer(handle.output);
}
