use super::colors::ColorCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VGAChar {
    pub(crate) ascii_character: u8,
    pub(crate) color_code: ColorCode,
}


impl VGAChar {
    pub fn new(ascii_character: u8, color_code: ColorCode) -> VGAChar {
        VGAChar { ascii_character, color_code }
    }
}