#![no_std]
#![no_main]

use bootloader::BootInfo;
use kernel::{
    com1_sendln, println, recolor, vga::{
        Color, 
        ColorCode
    }
};

mod initialization;

// Model IDT in rust
// Impl a generic exception handler. print "Oh No! Exception" or something.

#[unsafe(no_mangle)]
pub extern "C" fn _start(_bootInfo: &'static BootInfo) -> ! {
    com1_sendln!("{:?}", _bootInfo);
    if initialization::init() == Err(()) {
        panic!("Oh no, bad init");
    }
    println!("Cocofanto Elephanto");
    println!("Brr Brr Patapim");

    
    recolor!(ColorCode::new(Color::Cyan, Color::DarkGray));
    println!("");
    println!("1 + 1 = 2");

    com1_sendln!("Hello host");
    loop {}
}

