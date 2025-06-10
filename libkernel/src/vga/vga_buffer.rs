use core::fmt::Arguments;
use core::fmt::Write;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

pub use super::colors::Color;
pub use super::colors::ColorCode;
use super::chars::VGAChar;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub(crate) struct Buffer {
    pub chars: [[Volatile<VGAChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VGABuffer{
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    pub(crate)buffer: &'static mut Buffer,
}

lazy_static! {
    pub static ref VGA_BUFFER: Mutex<VGABuffer> = Mutex::new(VGABuffer::new());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    use core::fmt::Write;
    VGA_BUFFER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! recolor {    
    ($color:expr) => {
        $crate::vga::_change_color($color);
    };
}

#[doc(hidden)]
pub fn _change_color(color: ColorCode) {
    VGA_BUFFER.lock().color_code = color;
}

const VGA_BUFFER_ADDRESS: usize = 0xb8000;

impl VGABuffer {
    pub fn new() -> VGABuffer {
        VGABuffer {
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::new(Color::Yellow, Color::Black),
            buffer: unsafe { &mut *(VGA_BUFFER_ADDRESS as *mut Buffer) },
        }
    }
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(VGAChar::new(byte, color_code));
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        self.column_position = 0;
        self.row_position += 1;
    }
    
}


impl Write for VGABuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
        Ok(())
    }
}