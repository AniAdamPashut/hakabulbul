use crate::models::{address::VirtualAddress, segments::{structs::SegmentSelector, Msr}};

pub trait Segment {
    fn get_reg() -> SegmentSelector;
    unsafe fn set_reg(sl: SegmentSelector);
}

pub trait WideSegment {
    const BASE: Msr;
    fn read_base() -> VirtualAddress;
    unsafe fn write_base(base: VirtualAddress);
}