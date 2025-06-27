use core::arch::asm;
use kernel::tables::gdt::{GlobalDescriptorTable, SegmentDescriptor};

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
        .set_access(0b1001_0010)
        .set_flags(0b0100)
}

pub struct SegmentSelectors {
    pub code: u16,
    pub data: u16
}

#[allow(static_mut_refs)]
fn init_gdt() -> Result<SegmentSelectors, ()> {
    static mut GDT: GlobalDescriptorTable<12> = GlobalDescriptorTable::new();
    unsafe {
        let cs = GDT.append(kernel_code_segment())?;
        let ds = GDT.append(kernel_data_segment())?;
        GDT.load()?;
        Ok(SegmentSelectors {code: cs, data: ds})
    }
}

fn set_segments(segments: SegmentSelectors) {
    unsafe {
        asm!(
            "push {cs:r}",
            "lea rax, [6969f + rip]", // 6969f means the next occurence of the 6969 label
            "push rax",
            "retfq",
            "6969:", // This 6969 is just a label name because rust doesn't allow them to have letters 
            cs = in(reg) u64::from(segments.code << 3), // for rpl of zero
            options(preserves_flags, nostack),
        );

        asm!(concat!("mov ds, {0:x}"), in(reg) segments.data, options(nostack, preserves_flags));
        asm!(concat!("mov ss, {0:x}"), in(reg) segments.data, options(nostack, preserves_flags));
        asm!(concat!("mov es, {0:x}"), in(reg) segments.data, options(nostack, preserves_flags));
        asm!(concat!("mov fs, {0:x}"), in(reg) segments.data, options(nostack, preserves_flags));
        asm!(concat!("mov gs, {0:x}"), in(reg) segments.data, options(nostack, preserves_flags));
    }
}

pub fn init() -> Result<(), ()> {
    let segments = init_gdt()?;
    set_segments(segments);
    Ok(())
}