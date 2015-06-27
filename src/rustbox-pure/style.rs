#[derive(Clone, Copy, PartialEq)]
#[repr(C,u16)]
pub enum Color {
    Default =  0x00,
    Black =    0x01,
    Red =      0x02,
    Green =    0x03,
    Yellow =   0x04,
    Blue =     0x05,
    Magenta =  0x06,
    Cyan =     0x07,
    White =    0x08,
}

bitflags! {
    #[repr(C)]
    flags Style: u16 {
        const TB_NORMAL_COLOR = 0x000F,
        const RB_BOLD = 0x0100,
        const RB_UNDERLINE = 0x0200,
        const RB_REVERSE = 0x0400,
        const RB_NORMAL = 0x0000,
        const TB_ATTRIB = RB_BOLD.bits | RB_UNDERLINE.bits | RB_REVERSE.bits,
    }
}

impl Style {
    pub fn from_color(color: super::Color) -> Style {
        Style { bits: color as u16 & TB_NORMAL_COLOR.bits }
    }
}
