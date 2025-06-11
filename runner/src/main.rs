#![no_std]
#![no_main]


use core::arch::asm;

use lazy_static::lazy_static;
use kernel::{
    arch::{load_tss, models::{
        address::VirtualAddress, 
        gdt::{
            Descriptor, 
            GlobalDescriptorTable, 
            TaskStateSegment
        }, 
        interrupts::{
            InterruptDescriptorTable, 
            InterruptStackFrame
        }, 
        segments::{Segment, SegmentSelector, CS}
    }}, 
    com1_sendln, println, recolor, vga::{
        Color, 
        ColorCode
    }
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

struct Selectors {
    cs: SegmentSelector,
    tss: SegmentSelector,
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtualAddress::from_ptr(&raw const STACK);
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let cs = gdt.append(Descriptor::kernel_code_segment());
        com1_sendln!("{:?}", cs);
        let tss = gdt.append(Descriptor::tss_segment(&TSS));
        com1_sendln!("{:?}", cs);
        (gdt, Selectors {cs, tss})
    };
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler(handle_breakpoint);
        unsafe {
            idt.double_fault.set_handler(handle_double_fault).set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

fn handle_breakpoint(stack: InterruptStackFrame) {
    recolor!(ColorCode::new(Color::Green, Color::Pink));
    println!("Chimpanzini Bananini");
    println!("rip {}", stack.rip);
}

fn handle_double_fault(stack: InterruptStackFrame, error_code: u64) {
    recolor!(ColorCode::new(Color::Blue, Color::Magenta));
    println!("Tung Tung Tung Sahur");
    println!("err code {}", error_code);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Cocofanto Elephanto");
    println!("Brr Brr Patapim");

    IDT.load();
    GDT.0.load();

    unsafe {
        asm!("int3", options(nostack, preserves_flags));
    }

    recolor!(ColorCode::new(Color::Cyan, Color::DarkGray));
    println!("");
    println!("1 + 1 = 2");

    com1_sendln!("Hello host");
    loop {}
}
