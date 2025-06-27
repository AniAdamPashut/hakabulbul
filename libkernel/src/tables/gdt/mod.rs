use core::{arch::asm, mem::transmute};

use crate::tables::TablePointer;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SegmentDescriptor { 
    pub pointer_high: u8,
    pub options: u8,
    pub access: u8,
    pub pointer_middle: u8,
    pub pointer_low: u16,
    pub limit: u16,
}

#[derive(Debug)]
#[repr(C)]
pub struct GlobalDescriptorTable<const MAX: usize> {
    table: [u64; MAX],
    length: usize
}

impl SegmentDescriptor {
    pub const fn new() -> SegmentDescriptor {
        SegmentDescriptor { pointer_high: 0, options: 0, access: 0, pointer_middle: 0, pointer_low: 0, limit: 0 }
    }

    pub fn set_base(&mut self, base_address: u32) -> &mut Self {
        self.pointer_low = base_address as u16;
        self.pointer_middle = (base_address >> 16) as u8;
        self.pointer_high = (base_address >> 24) as u8;
        self
    }
    
    pub fn set_limit(&mut self, limit: u32) -> &mut Self {
        self.limit = limit as u16;
        let new_options: u8 = self.options & 0b1111_0000;
        self.options = new_options | (limit >> 16 & 0xF) as u8;
        self 
    }

    pub fn set_flags(&mut self, flags: u8) -> &mut Self {
        self.options = (flags << 4) | self.options & 0xF; 
        self
    }

    pub fn set_access(&mut self, access: u8) -> &mut Self {
        self.access = access;
        self
    }
}

impl<const MAX: usize> GlobalDescriptorTable<MAX> {
    pub const fn new() -> GlobalDescriptorTable<MAX> {
        GlobalDescriptorTable { table: [0u64; MAX], length: 1 }
    }

    pub fn append(&mut self, segment: SegmentDescriptor) -> Result<u16, ()> {
        if self.length >= MAX {
            return Err(())
        }
        let index = self.length as u16;
        unsafe {
            self.table[self.length] = transmute(segment);
        }
        self.length += 1;
        Ok(index)
    }

    pub fn load(&mut self) -> Result<(), ()> {
        let offset = self.table.as_ptr() as u64;
        let limit = (size_of::<u64>() * self.length - 1) as u16;
        
        let table_pointer = TablePointer { offset, limit };

        unsafe {
            asm!("lgdt [{}]", in (reg) &table_pointer, options(readonly, nostack, preserves_flags));
        }
        Ok(())
    }
}