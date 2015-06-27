extern crate winapi;
extern crate kernel32;

pub mod wincon;
pub mod display;

pub use self::wincon::*;
pub use self::display::*;

use super::style::{Color, Style};

pub fn attr_translate(fg: Color, bg: Color, style: Style) -> u16
{
	12312
}
