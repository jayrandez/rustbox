extern crate winapi;
extern crate kernel32;

pub mod wincon;
pub mod display;

pub use self::wincon::*;
pub use self::display::*;
