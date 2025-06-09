pub mod vga_buffer;
mod colors;
mod chars;

pub use vga_buffer::VGABuffer;
pub use vga_buffer::Color;
pub use vga_buffer::ColorCode;
pub use vga_buffer::_print;
pub use vga_buffer::_change_color;