use core::marker::PhantomData;

use bit_field::BitField;

use crate::models::address::VirtualAddress;
use crate::models::privilege::PrivilegeLevel;
use crate::models::segments::{Segment, SegmentSelector, CS};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DescriptorOptions {
    cs: SegmentSelector,
    bits: u16,
}

impl DescriptorOptions {
    #[inline]
    const fn minimal() -> Self {
        DescriptorOptions {
            cs: SegmentSelector(0),
            bits: 0b1110_0000_0000, // Default to a 64-bit Interrupt Gate
        }
    }

    pub unsafe fn set_code_selector(&mut self, code: SegmentSelector) -> &mut Self {
        self.cs = code;
        self
    }
    
    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.bits.set_bit(15, present);
        self
    }

    fn present(&self) -> bool {
        self.bits.get_bit(15)
    }
        #[inline]
    pub fn set_privilege_level(&mut self, dpl: PrivilegeLevel) -> &mut Self {
        self.bits.set_bits(13..15, dpl as u16);
        self
    }

    fn privilege_level(&self) -> PrivilegeLevel {
        PrivilegeLevel::from_u16(self.bits.get_bits(13..15))
    }
    
    #[inline]
    pub unsafe fn set_stack_index(&mut self, index: u16) -> &mut Self {
        // The hardware IST index starts at 1, but our software IST index
        // starts at 0. Therefore we need to add 1 here.
        self.bits.set_bits(0..3, index + 1);
        self
    }

    fn stack_index(&self) -> Option<u16> {
        self.bits.get_bits(0..3).checked_sub(1)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct InterruptDescriptor<F> {
    pointer_low: u16,      // offset bits 0..15
    options: DescriptorOptions, // the options or somethign
    pointer_middle: u16,   // offset bits 16..31
    pointer_high: u32,     // offset bits 32..63
    reserved: u32,         // reserved
    _marker: PhantomData<F>
}

impl<F> InterruptDescriptor<F> {
    #[inline]
    pub const fn missing() -> Self {
        InterruptDescriptor {
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            options: DescriptorOptions::minimal(),
            reserved: 0,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub unsafe fn set_handler_addr(&mut self, addr: VirtualAddress) -> &mut DescriptorOptions {
        let addr = addr.as_u64();
        self.pointer_low = addr as u16;
        self.pointer_middle = (addr >> 16) as u16;
        self.pointer_high = (addr >> 32) as u32;

        self.options = DescriptorOptions::minimal();
        unsafe { self.options.set_code_selector(CS::get_reg()) };
        self.options.set_present(true);
        &mut self.options
    }

    /// Returns the virtual address of this IDT entry's handler function.
    #[inline]
    pub fn handler_addr(&self) -> VirtualAddress {
        let addr = self.pointer_low as u64
            | ((self.pointer_middle as u64) << 16)
            | ((self.pointer_high as u64) << 32);
        // addr is a valid VirtAddr, as the pointer members are either all zero,
        // or have been set by set_handler_addr (which takes a VirtAddr).
        VirtualAddress::new_truncate(addr)
    }
}
