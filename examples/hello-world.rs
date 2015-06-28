extern crate rustbox;

use std::default::Default;

use rustbox::{style, Color, RustBox};
use rustbox::Key;

fn main() {
    let mut rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Hello, world!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");
    loop {
        //rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Char('q')) => { break; },
                    Some(Key::Char(c)) => { println!("{}", c)},
                    Some(Key::Ctrl(c)) => { println!("Ctrl + {}", c)},
                    Some(Key::Tab) => { println!("Tab")},
                    Some(Key::Enter) => { println!("Enter")},
                    Some(Key::F(i)) => { println!("F-{}", i)},
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}
