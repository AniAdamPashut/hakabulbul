use crate::models::address::VirtualAddress;

pub mod address;
pub mod interrupts;
pub mod privilage;
pub mod segments;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(2))]
pub struct DescriptorTablePointer {
    /// The size of the table - 1 (in bytes)
    pub limit: u16,
    pub base: VirtualAddress,
}