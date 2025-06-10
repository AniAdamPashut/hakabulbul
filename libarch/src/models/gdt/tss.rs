use crate::models::address::VirtualAddress;

pub struct TaskStateSegment {
    _reserved1: u32,
    pub privilege_stack_table: [VirtualAddress; 3],
    _reserved2: u64,
    pub interrupt_stack_table: [VirtualAddress; 7],
    _reserved3: u64,
    _reserved4: u16,
    pub io_map_base: u16
}

impl TaskStateSegment {
    /// Creates a new TSS with zeroed privilege and interrupt stack table and an
    /// empty I/O-Permission Bitmap.
    ///
    /// As we always set the TSS segment limit to
    /// `size_of::<TaskStateSegment>() - 1`, this means that `iomap_base` is
    /// initialized to `size_of::<TaskStateSegment>()`.
    #[inline]
    pub const fn new() -> TaskStateSegment {
        TaskStateSegment {
            privilege_stack_table: [VirtualAddress::zero(); 3],
            interrupt_stack_table: [VirtualAddress::zero(); 7],
            io_map_base: size_of::<TaskStateSegment>() as u16,
            _reserved1: 0,
            _reserved2: 0,
            _reserved3: 0,
            _reserved4: 0,
        }
    }
}

impl Default for TaskStateSegment {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}