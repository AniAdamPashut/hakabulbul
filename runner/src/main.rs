#![no_std]
#![no_main]

use kernel::{
    com1_sendln, println, recolor, vga::{
        Color, 
        ColorCode
    }
};


// Model GDT in rust
// Model IDT in rust
// Impl a generic exception handler. print "Oh No! Exception" or something.


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Cocofanto Elephanto");
    println!("Brr Brr Patapim");

    recolor!(ColorCode::new(Color::Cyan, Color::DarkGray));
    println!("");
    println!("1 + 1 = 2");

    com1_sendln!("Hello host");
    loop {}
}
