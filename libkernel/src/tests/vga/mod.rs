#[test_case]
#[allow(non_snake_case)]
pub fn write_buffer__valid_bytes__all_written() {
    use core::fmt::Write;
    use crate::vga::VGABuffer;

    // Arrange
    const BUFFER: &str = "HELLO";
    const ROW_NUMBER: usize = 0;
    let mut vga = VGABuffer::new();
    
    // Act
    let _ = vga.write_str(BUFFER);

    // Assert
    for i in 0..BUFFER.len() {
        assert_eq!(BUFFER.as_bytes()[i], vga.buffer.chars[ROW_NUMBER][i].read().ascii_character)
    }
}