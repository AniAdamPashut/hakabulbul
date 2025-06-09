use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    use crate::println;

    println!("{}", _info);
    loop {}
}