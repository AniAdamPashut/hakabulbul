#![no_std]
#![no_main]

use core::arch::asm;

use lazy_static::lazy_static;
use kernel::{arch::models::interrupts::{InterruptDescriptorTable, InterruptStackFrame}, println, recolor, vga::{Color, ColorCode}};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler(handle_breakpoint);
        idt
    };
}

fn handle_breakpoint(stack: InterruptStackFrame) {
    recolor!(ColorCode::new(Color::Green, Color::Pink));
    println!("Chimpanzini Bananini");
    println!("rip {}", stack.rip);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Cocofanto Elephanto");
    println!("Brr Brr Patapim");

    IDT.load();

    unsafe {
        asm!("int3");
    }

    recolor!(ColorCode::new(Color::Cyan, Color::DarkGray));
    println!("");
    println!("1 + 1 = 2");
    loop {}
}
