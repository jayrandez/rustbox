use super::keyboard::Key;

use std::error::Error;
use std::fmt;
use num::FromPrimitive;

#[derive(Clone, Copy)]
pub enum Event {
    KeyEventRaw(u8, u16, u32),
    KeyEvent(Option<Key>),
    ResizeEvent(i32, i32),
    MouseEvent(Mouse, i32, i32),
    NoEvent
}

#[derive(Debug)]
pub enum EventError {
   TermboxError,
   Unknown(isize),
}

pub type EventResult = Result<Event, EventError>;

impl fmt::Display for EventError {
   fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
      write!(fmt, "{}", self.description())
   }
}

impl Error for EventError {
   fn description(&self) -> &str {
      match *self {
         EventError::TermboxError => "Error in Termbox",
         // I don't know how to format this without lifetime error.
         // EventError::Unknown(n) => &format!("There was an unknown error. Error code: {}", n),
         EventError::Unknown(_) => "Unknown error in Termbox",
      }
   }
}

impl FromPrimitive for EventError {
   fn from_i64(n: i64) -> Option<EventError> {
      match n {
         -1 => Some(EventError::TermboxError),
         n => Some(EventError::Unknown(n as isize)),
      }
   }

   fn from_u64(n: u64) -> Option<EventError> {
      Some(EventError::Unknown(n as isize))
   }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Mouse {
    Left,
    Right,
    Middle,
    Release,
    WheelUp,
    WheelDown,
    Move
}
