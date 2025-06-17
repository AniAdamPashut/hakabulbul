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
    table: [SegmentDescriptor; MAX],
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
        GlobalDescriptorTable { table: [SegmentDescriptor::new(); MAX], length: 1 }
    }

    pub fn append(&mut self, segment: SegmentDescriptor) -> Result<&mut GlobalDescriptorTable<MAX>, ()> {
        if self.length >= MAX {
            return Err(())
        }

        self.table[self.length] = segment;
        self.length += 1;
        Ok(self)
    }

    pub fn load(&mut self) -> Result<(), ()> {
        unimplemented!("implement load");
        Ok(())
    }
}