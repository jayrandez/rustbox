extern crate rustbox;

use std::default::Default;

use rustbox::{style, Color, RustBox};
use rustbox::Key;

fn main() {
    let mut rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    loop{}

}
