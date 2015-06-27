use std::cmp::{max, min};
use super::style::{Color, Style};

pub struct Cell
{
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub sty: Style
}

impl Copy for Cell{}

impl Clone for Cell
{
    fn clone(&self) -> Cell { *self }
}

pub struct CellBuffer
{
    pub width: usize,
    pub height: usize,
    pub char_buffer: Vec<u8>,
    pub attr_buffer: Vec<u16>
}

impl CellBuffer
{
    pub fn new(width: usize, height: usize, fill_char: u8, fill_attr: u16) -> CellBuffer
    {
        let buffer_size: usize = width * height;

        CellBuffer {
            width: width,
            height: height,
            char_buffer: vec![fill_char; buffer_size],
            attr_buffer: vec![fill_attr; buffer_size]
        }
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize, fill_char: u8, fill_attr: u16)
    {
        let old_size = self.height * self.width;
        let new_size = new_width * new_height;

        if new_size > old_size {
            // Add extra room in array before any stretch/squeeze.
            self.char_buffer.resize(new_size, fill_char);
            self.attr_buffer.resize(new_size, fill_attr);
        }

        // If the width has changed in addition to the height, the contents must be shifted
        if(new_width < self.width) {
            self.squeeze(new_width, new_height, fill_char, fill_attr);
        }
        else if(new_width > self.width) {
            self.stretch(new_width, new_height, fill_char, fill_attr);
        }

        if new_size < old_size {
            // Remove extra room in array which is now garbage.
            self.char_buffer.truncate(new_size);
            self.attr_buffer.truncate(new_size);
        }

        self.width = new_width;
        self.height = new_height;
    }

    fn stretch(&mut self, new_width: usize, new_height: usize, fill_char: u8, fill_attr: u16)
    {
        let char_buffer_slice = self.char_buffer.as_mut_slice();
        let attr_buffer_slice = self.attr_buffer.as_mut_slice();

        for line in (0..min(self.height, new_height)).rev() {
            let dest_index = line * new_width;
            let src_index = line * self.width;

            for i in (0..self.width).rev() {
                char_buffer_slice[dest_index + i] = char_buffer_slice[src_index + i];
                attr_buffer_slice[dest_index + i] = attr_buffer_slice[src_index + i];
            }
            for i in self.width..new_width {
                char_buffer_slice[dest_index + i] = fill_char;
                attr_buffer_slice[dest_index + i] = fill_attr;
            }
        }
        for line in self.height..new_height
        {
            let dest_index = line * new_width;
            for i in 0..new_width {
                char_buffer_slice[dest_index + i] = fill_char;
                attr_buffer_slice[dest_index + i] = fill_attr;
            }
        }
    }

    fn squeeze(&mut self, new_width: usize, new_height: usize, fill_char: u8, fill_attr: u16)
    {
        let char_buffer_slice = self.char_buffer.as_mut_slice();
        let attr_buffer_slice = self.attr_buffer.as_mut_slice();

        for line in 0..min(self.height, new_height) {
            let dest_index = line * new_width;
            let src_index = line * self.width;

            for i in 0..new_width {
                char_buffer_slice[dest_index + i] = char_buffer_slice[src_index + i];
                attr_buffer_slice[dest_index + i] = attr_buffer_slice[src_index + i];
            }
        }
        for line in self.height..new_height {
            let dest_index = line * new_width;

            for i in 0..new_width {
                char_buffer_slice[dest_index + i] = fill_char;
                attr_buffer_slice[dest_index + i] = fill_attr;
            }
        }
    }

    pub fn resize_blindly(&mut self, new_width: usize, new_height: usize)
    {
        self.char_buffer.resize(new_width * new_height, 0);
        self.attr_buffer.resize(new_width * new_height, 0);

        self.width = new_width;
        self.height = new_height;
    }
}
