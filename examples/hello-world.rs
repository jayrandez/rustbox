extern crate rustbox;

use std::default::Default;
use std::thread;
use rustbox::{style, Color, RustBox};
use rustbox::Key;

fn main() {
    do_rustbox();
    println!("Rustbox finished.");
    thread::sleep_ms(1000);
}

fn do_rustbox() {
    let mut rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.change_cell(2, 2, 'H' as u32, Color::White, Color::Black, rustbox::RB_NORMAL);
    rustbox.change_cell(3, 2, 'i' as u32, Color::White, Color::Black, rustbox::RB_NORMAL);
    rustbox.change_cell(4, 2, '.' as u32, Color::White, Color::Black, rustbox::RB_NORMAL);
    rustbox.present();

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Hello, world!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");
    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Char('q')) => { break; },
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}
