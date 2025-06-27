#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga;
pub mod tables;
pub mod serial_ports;

#[cfg(test)]
pub(crate) mod tests;

#[cfg(not(test))]
pub mod panic;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();
    
    loop {}
}
