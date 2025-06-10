use bit_field::BitField;

use crate::models::privilege::PrivilegeLevel;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct SegmentSelector(pub u16);

impl SegmentSelector {
    #[inline]
    pub const fn new(index: u16, rpl: PrivilegeLevel) -> SegmentSelector {
        SegmentSelector((index << 3) | (rpl as u16))
    }

    /// Can be used as a selector into a non-existent segment and assigned to segment registers,
    /// e.g. data segment register in ring 0
    pub const NULL: Self = Self::new(0, PrivilegeLevel::Ring0);

    #[inline]
    pub fn index(self) -> u16 {
        self.0 >> 3
    }

    #[inline]
    pub fn rpl(self) -> PrivilegeLevel {
        PrivilegeLevel::from_u16(self.0.get_bits(0..2))
    }

    #[inline]
    pub fn set_rpl(&mut self, rpl: PrivilegeLevel) {
        self.0.set_bits(0..2, rpl as u16);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Msr(u32);

impl Msr {
    /// Create an instance from a register.
    #[inline]
    pub const fn new(reg: u32) -> Msr {
        Msr(reg)
    }
}

#[derive(Debug)]
pub struct CS;

#[derive(Debug)]
pub struct SS;

#[derive(Debug)]
pub struct DS;

#[derive(Debug)]
pub struct ES;

#[derive(Debug)]
pub struct FS;

#[derive(Debug)]
pub struct FsBase;

#[derive(Debug)]
pub struct GS;

#[derive(Debug)]
pub struct GsBase;

impl FsBase {
    /// The underlying model specific register.
    pub const MSR: Msr = Msr(0xC000_0100);
}

impl GsBase {
    /// The underlying model specific register.
    pub const MSR: Msr = Msr(0xC000_0101);
}
