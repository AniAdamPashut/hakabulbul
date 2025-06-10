use crate::models::segments::SegmentSelector;
use crate::registers::RFlags;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct InterruptStackFrame {
    pub rip: u64,
    pub cs: SegmentSelector,
    _reserved1: [u8; 6],
    pub rflags: RFlags,
    pub rsp: u64,
    pub selector: SegmentSelector,
    _reserved2: [u8; 6],
}