use core::arch::asm;

use crate::models::DescriptorTablePointer;

pub unsafe fn load_idt(table: &DescriptorTablePointer) {
    unsafe {
        asm!("lidt [{}]", in(reg) table, options(readonly, nostack, preserves_flags));
    }
}