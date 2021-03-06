#![feature(libc)]
#![feature(optin_builtin_traits)]
#![feature(convert)]
#![feature(vec_resize)]

extern crate libc;
extern crate num;
extern crate time;

#[macro_use]
extern crate bitflags;

pub use rustbox::*;

#[cfg(all(target_os="linux"))]
#[path="rustbox-c/mod.rs"]
pub mod rustbox;

#[cfg(all(target_os="macos"))]
#[path="rustbox-c/mod.rs"]
pub mod rustbox;

#[cfg(all(target_os="windows"))]
#[path="rustbox-pure/mod.rs"]
pub mod rustbox;
