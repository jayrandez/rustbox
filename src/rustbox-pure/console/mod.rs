pub mod wincon;
pub mod display;
pub mod translate;
mod api;

use self::api::{HANDLE, HWND, INPUT_RECORD};

#[derive(Clone, Copy)]
pub struct DisplayInfo {
    pub handle: Handle,
    stdout_buffer: HANDLE,
	pub visible_size: Size,
	pub display_line: usize
}

#[derive(Clone, Copy)]
pub struct Handle {
	pub window: HWND,
	pub input: HANDLE,
	pub output: HANDLE
}

#[derive(Clone, Copy)]
pub struct RawEvent {
    pub record: INPUT_RECORD
}

#[derive(Clone, Copy)]
pub struct Size {
    pub width: usize,
    pub height: usize
}

#[derive(Clone, Copy)]
pub struct Location {
    pub x: usize,
    pub y: usize
}

pub use self::display::{
    begin_display,
    finish_display
};

pub use self::translate::{
    translate_attr,
    translate_event
};

pub use self::wincon::{
    set_mode,
    visible_size,
    write_characters,
    write_attributes,
    set_cursor_visible,
    set_cursor_location,
    read_input
};
