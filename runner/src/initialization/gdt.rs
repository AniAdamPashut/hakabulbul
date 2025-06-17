use kernel::gdt::{GlobalDescriptorTable, SegmentDescriptor};

fn kernel_code_segment() -> SegmentDescriptor {
    let mut cs: SegmentDescriptor = SegmentDescriptor::new();
    *cs.set_base(0)
        .set_limit(0xFFFFF)
        .set_access(0b1001_1010)
        .set_flags(0b0100)
}

fn kernel_data_segment() -> SegmentDescriptor {
    let mut ds: SegmentDescriptor = SegmentDescriptor::new();
    *ds.set_base(0)
        .set_limit(0xFFFFF)
        .set_access(0b1001_1010)
        .set_flags(0b0100)
}

#[allow(static_mut_refs)]
fn init_gdt() -> Result<(), ()> {
    static mut gdt: GlobalDescriptorTable<12> = GlobalDescriptorTable::new();

    unsafe {
        gdt
            .append(kernel_code_segment())?
            .append(kernel_data_segment())?
            .load()
    }
}