extern crate winapi;
extern crate kernel32;

pub mod wincon;
pub mod display;

pub use self::wincon::*;
pub use self::display::*;

use self::winapi::{
	FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE, FOREGROUND_INTENSITY,
	BACKGROUND_RED, BACKGROUND_GREEN, BACKGROUND_BLUE, BACKGROUND_INTENSITY,
	COMMON_LVB_REVERSE_VIDEO, COMMON_LVB_UNDERSCORE
};

use super::style;
use super::style::{Color, Style};

pub fn attr_translate(fg: Color, bg: Color, style: Style) -> u16
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

	if style.contains(style::RB_UNDERLINE) {
		attr = attr | COMMON_LVB_UNDERSCORE as u16;
	}

	if style.contains(style::RB_REVERSE) {
		attr = attr | COMMON_LVB_REVERSE_VIDEO as u16;
	}

	return attr;
}
