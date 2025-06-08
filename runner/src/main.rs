#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;
use libkernel::vga::VGABuffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut vga_buffer = VGABuffer::new();

    writeln!(vga_buffer, "Brr Brr Patapim").expect("you ate a bolbol");
    
    loop {}
}
