use core::arch::asm;

use crate::models::{segments::SegmentSelector, DescriptorTablePointer};


pub unsafe fn load_idt(table: &DescriptorTablePointer) {
    unsafe {
        asm!("lidt [{}]", in(reg) table, options(readonly, nostack, preserves_flags));
    }
}

pub unsafe fn load_gdt(table: &DescriptorTablePointer) {
    unsafe {
        asm!("lgdt [{}]", in(reg) table, options(readonly, nostack, preserves_flags));
    }
}

pub unsafe fn load_tss(sel: SegmentSelector) {
    unsafe {
        asm!("ltr {0:x}", in(reg) sel.0, options(nostack, preserves_flags));
    }
}

