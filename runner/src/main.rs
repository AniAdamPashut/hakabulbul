#![no_std]
#![no_main]

use kernel::{println, recolor, vga::{Color, ColorCode}};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Cocofanto Elephanto");
    println!("Brr Brr patapim");
    
    recolor!(ColorCode::new(Color::Cyan, Color::DarkGray));
    println!("");
    println!("1 + 1 = 2");
    loop {}
}
