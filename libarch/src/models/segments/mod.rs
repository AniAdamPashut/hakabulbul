use core::arch::asm;
use crate::models::address::VirtualAddress;

mod traits; 
mod structs;

pub use structs::*;
pub use traits::{Segment, WideSegment};

macro_rules! get_reg_impl {
    ($name:literal) => {
        #[inline]
        fn get_reg() -> SegmentSelector {
            let segment: u16;
            unsafe {
                asm!(concat!("mov {0:x}, ", $name), out(reg) segment, options(nomem, nostack, preserves_flags));
            }
            SegmentSelector(segment)
        }
    };
}

macro_rules! segment_impl {
    ($type:ty, $name:literal) => {
        impl Segment for $type {
            get_reg_impl!($name);

            #[inline]
            unsafe fn set_reg(sel: SegmentSelector) {
                unsafe {
                    asm!(concat!("mov ", $name, ", {0:x}"), in(reg) sel.0, options(nostack, preserves_flags));
                }
            }
        }
    };
}


macro_rules! segment64_impl {
    ($type:ty, $name:literal, $base:ty) => {
        impl WideSegment for $type {
            const BASE: Msr = <$base>::MSR;
            #[inline]
            fn read_base() -> VirtualAddress {
                unsafe {
                    let val: u64;
                    asm!(concat!("rd", $name, "base {}"), out(reg) val, options(nomem, nostack, preserves_flags));
                    VirtualAddress::new_unsafe(val)
                }
            }

            #[inline]
            unsafe fn write_base(base: VirtualAddress) {
                unsafe{
                    asm!(concat!("wr", $name, "base {}"), in(reg) base.as_u64(), options(nostack, preserves_flags));
                }
            }
        }
    };
}

impl Segment for CS {
    get_reg_impl!("cs");

    #[inline]
    unsafe fn set_reg(sel: SegmentSelector) {
        unsafe {
            asm!(
                "push {sel}",
                "lea {tmp}, [55f + rip]",
                "push {tmp}",
                "retfq",
                "55:",
                sel = in(reg) u64::from(sel.0),
                tmp = lateout(reg) _,
                options(preserves_flags),
            );
        }
    }
}


segment_impl!(SS, "ss");
segment_impl!(DS, "ds");
segment_impl!(ES, "es");
segment_impl!(FS, "fs");
segment64_impl!(FS, "fs", FsBase);
segment_impl!(GS, "gs");
segment64_impl!(GS, "gs", GsBase);

impl GS {
    /// Swap `KernelGsBase` MSR and `GsBase` MSR.
    ///
    /// ## Safety
    ///
    /// This function is unsafe because the caller must ensure that the
    /// swap operation cannot lead to undefined behavior.
    #[inline]
    pub unsafe fn swap() {
        unsafe {
            asm!("swapgs", options(nostack, preserves_flags));
        }
    }
}
