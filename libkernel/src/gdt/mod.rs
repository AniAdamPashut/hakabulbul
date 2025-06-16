#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GDTEntry { 
    pointer_high: u8,
    options: u8,
    pub access: u8,
    pointer_middle: u8,
    pointer_low: u16,
    limit: u16,
}

#[derive(Debug)]
#[repr(C)]
pub struct GlobalDescriptorTable {
    table: [GDTEntry; 12],
    length: u64
}

impl GDTEntry {
    pub const fn new() -> GDTEntry {
        GDTEntry { pointer_high: 0, options: 0, access: 0, pointer_middle: 0, pointer_low: 0, limit: 0 }
    }

    pub fn set_base(&mut self, base_address: u32) -> &mut Self {
        self.pointer_low = base_address as u16;
        self.pointer_middle = (base_address >> 16) as u8;
        self.pointer_high = (base_address >> 24) as u8;
        self
    }
    
    pub fn set_limit(&mut self, limit: u32) -> &mut Self {

        self 
    }
}

impl GlobalDescriptorTable {
    pub const fn new() -> GlobalDescriptorTable {
        GlobalDescriptorTable { table: [GDTEntry::new(); 12], length: 1 }
    }
}