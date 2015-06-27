extern crate time;
extern crate rustbox;

use std::default::Default;

use rustbox::{RustBox, Color, Style};
use rustbox::Key;

fn main()
{
    let mut rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    loop {
        let width = rustbox.width();
        let height = rustbox.height();

        let x1 = 2;
        let x2 = width - 1 - 2;
        let y1 = 2;
        let y2 = height - 1 - 2;

        //rustbox.clear();
        if x2 > x1 && y2 > y1 {
            fill_rect(&mut rustbox, x1, y1, x2, y2, Color::Red);
            draw_rect(&mut rustbox, x1-1, y1-1, x2+1, y2+1, Color::White);
            draw_rect(&mut rustbox, x1-2, y1-2, x2+2, y2+2, Color::Black);
        }

        match rustbox.peek_event(time::Duration::milliseconds(30), false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Char('q')) => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }

        rustbox.present();
    }
}

fn draw_rect(rb:&mut RustBox, x1:usize, y1:usize, x2:usize, y2:usize, color:Color)
{
    for x in x1..(x2 + 1) {
        rb.change_cell(x, y1, 219 as u32, color, Color::Black, rustbox::RB_NORMAL);
        rb.change_cell(x, y2, 219 as u32, color, Color::Black, rustbox::RB_NORMAL);
    }
    for y in (y1 + 1)..y2 {
        rb.change_cell(x1, y, 219 as u32, color, Color::Black, rustbox::RB_NORMAL);
        rb.change_cell(x2, y, 219 as u32, color, Color::Black, rustbox::RB_NORMAL);
    }

}

fn fill_rect(rb:&mut RustBox, x1:usize, y1:usize, x2:usize, y2:usize, color:Color)
{
    for x in x1..(x2 + 1) {
        for y in y1..(y2 + 1) {
            rb.change_cell(x, y, 219 as u32, color, Color::Black, rustbox::RB_NORMAL);
        }
    }
}
