use core::arch::asm;


pub fn are_interrupts_enabled() -> bool {
    use crate::registers::{RFlags};

    RFlags::read().contains(RFlags::INTERRUPT_FLAG)
}

pub fn disable_interrupts() {
    unsafe {
        asm!("cli", options(nostack, preserves_flags))
    }
}

pub fn enable_interrupts() {
    unsafe {
        asm!("sti", options(nostack, preserves_flags))
    }
}

pub fn without_interrupts<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let saved_intpt_flag = are_interrupts_enabled();

    if saved_intpt_flag {
        disable_interrupts();
    }

    let ret = f();

    if saved_intpt_flag {
        enable_interrupts();
    }

    ret
}

#[inline]
pub fn int3() {
    unsafe {
        asm!("int3", options(nomem, nostack));
    }
}
