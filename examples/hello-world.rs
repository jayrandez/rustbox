extern crate rustbox;

use std::default::Default;

use rustbox::{style, Color, RustBox};
use rustbox::Key;

fn main() {
    let mut rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.set_clear_attributes(Color::Default, Color::Default, style::RB_NORMAL);
    rustbox.clear();
    rustbox.present();

    rustbox.set_cursor(1, 1);

    std::thread::sleep_ms(3000);

    println!("Before shutdown");

    rustbox.shutdown();

    println!("After shutdown");

    std::thread::sleep_ms(3000);

    rustbox.present();

    /*rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Hello, world!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");
    loop {
        rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Char('q')) => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }*/
}
