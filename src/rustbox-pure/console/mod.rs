extern crate winapi;
extern crate kernel32;

pub mod wincon;
pub mod display;

pub use self::wincon::*;
pub use self::display::*;

use std::mem;
use super::style;
use super::style::{Color, Style};
use super::event::{Event, Mouse};

use self::winapi::{
	DWORD,
	INPUT_RECORD, PINPUT_RECORD,
    MOUSE_EVENT_RECORD, KEY_EVENT_RECORD, WINDOW_BUFFER_SIZE_RECORD,
    KEY_EVENT, MOUSE_EVENT, WINDOW_BUFFER_SIZE_EVENT,
    MOUSE_MOVED, MOUSE_WHEELED, FROM_LEFT_1ST_BUTTON_PRESSED, RIGHTMOST_BUTTON_PRESSED,
    FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, FOREGROUND_INTENSITY,
	BACKGROUND_RED, BACKGROUND_GREEN, BACKGROUND_BLUE, BACKGROUND_INTENSITY
};

pub fn translate_event(raw_event: RawEvent) -> Option<Event> {
    match(raw_event.record.EventType as DWORD) {
        MOUSE_EVENT => translate_mouse_event(raw_event.record.Event),
        KEY_EVENT => translate_key_event(unsafe {
            mem::transmute::<MOUSE_EVENT_RECORD, KEY_EVENT_RECORD>(raw_event.record.Event)
        }),
        _ => None
    }
}

fn translate_mouse_event(raw_event: MOUSE_EVENT_RECORD) -> Option<Event> {
    let (x, y) = (raw_event.dwMousePosition.X as i32, raw_event.dwMousePosition.Y as i32);

    match(raw_event.dwEventFlags) {
        0 => {
            match(raw_event.dwButtonState) {
                0 => Some(Event::MouseEvent(Mouse::Release, x, y)),
                FROM_LEFT_1ST_BUTTON_PRESSED => Some(Event::MouseEvent(Mouse::Left, x, y)),
                RIGHTMOST_BUTTON_PRESSED => Some(Event::MouseEvent(Mouse::Right, x, y)),
                _ => Some(Event::MouseEvent(Mouse::Middle, x, y))
            }
        }
        MOUSE_MOVED => {
            Some(Event::MouseEvent(Mouse::Move, x, y))
        }
        MOUSE_WHEELED => {
            let magnitude = (raw_event.dwButtonState >> 16) as i16;

            if magnitude > 0 { Some(Event::MouseEvent(Mouse::WheelUp, x, y)) }
            else { Some(Event::MouseEvent(Mouse::WheelDown, x, y)) }
        }
        _ => None
    }
}

fn translate_key_event(raw_event: KEY_EVENT_RECORD) -> Option<Event> {
    None
}

pub fn translate_attr(fg: Color, bg: Color, style: Style) -> u16
{
	let mut attr: u16 = 0;

	/* This is pretty inefficient since attr_translate has to be called for each change_cell.
	But, not sure whether it's a good idea to have two separate implementations for Color enum
	and Style bitfield */

	attr = attr | match fg {
		Color::Default => (FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE),
		Color::Black => 0,
		Color::Red => (FOREGROUND_RED | FOREGROUND_INTENSITY),
		Color::Green => (FOREGROUND_GREEN | FOREGROUND_INTENSITY),
		Color::Yellow => (FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_INTENSITY),
		Color::Blue => (FOREGROUND_BLUE | FOREGROUND_INTENSITY),
		Color::Magenta => (FOREGROUND_RED | FOREGROUND_BLUE | FOREGROUND_INTENSITY),
		Color::Cyan => (FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY),
		Color::White => (FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY),
	} as u16;

	attr = attr | match bg {
		Color::Default => (BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE),
		Color::Black => 0,
		Color::Red => (BACKGROUND_RED | BACKGROUND_INTENSITY),
		Color::Green => (BACKGROUND_GREEN | BACKGROUND_INTENSITY),
		Color::Yellow => (BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_INTENSITY),
		Color::Blue => (BACKGROUND_BLUE | BACKGROUND_INTENSITY),
		Color::Magenta => (BACKGROUND_RED | BACKGROUND_BLUE | BACKGROUND_INTENSITY),
		Color::Cyan => (BACKGROUND_GREEN | BACKGROUND_BLUE | BACKGROUND_INTENSITY),
		Color::White => (BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE | BACKGROUND_INTENSITY),
	} as u16;

	if style.contains(style::RB_REVERSE) {
		attr = ((attr >> 4) & 0x00FF) | ((attr << 4) & 0xFF00);
	}

	return attr;
}
