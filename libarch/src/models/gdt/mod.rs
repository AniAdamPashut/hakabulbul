use core::{fmt, sync::atomic::{AtomicU64, Ordering}};

use crate::{instructions::tables::load_gdt, models::{address::VirtualAddress, segments::SegmentSelector, DescriptorTablePointer}};

mod descriptor;
mod tss;

pub use tss::TaskStateSegment;
pub use descriptor::Descriptor;

#[repr(transparent)]
pub struct GDTEntry(AtomicU64);

#[derive(Debug, Clone)]
pub struct GlobalDescriptorTable<const MAX: usize = 8> {
    table: [GDTEntry; MAX],
    len: usize,
}


impl GDTEntry {
    // Create a new Entry from a raw value.
    const fn new(raw: u64) -> Self {
        let raw = AtomicU64::new(raw);
        Self(raw)
    }

    /// The raw bits for this entry. Depending on the [`Descriptor`] type, these
    /// bits may correspond to those in [`DescriptorFlags`].
    pub fn raw(&self) -> u64 {
        // TODO: Make this const fn when AtomicU64::load is const.
        let raw = self.0.load(Ordering::SeqCst);
        raw
    }
}

impl Clone for GDTEntry {
    fn clone(&self) -> Self {
        Self::new(self.raw())
    }
}

impl PartialEq for GDTEntry {
    fn eq(&self, other: &Self) -> bool {
        self.raw() == other.raw()
    }
}

impl Eq for GDTEntry {}

impl fmt::Debug for GDTEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display inner value as hex
        write!(f, "Entry({:#018x})", self.raw())
    }
}


impl GlobalDescriptorTable {
    /// Creates an empty GDT with the default length of 8.
    pub const fn new() -> Self {
        Self::empty()
    }
}

impl Default for GlobalDescriptorTable {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const MAX: usize> GlobalDescriptorTable<MAX> {
    #[inline]
    pub const fn empty() -> Self {
        assert!(MAX > 0, "A GDT cannot have 0 entries");
        assert!(MAX <= (1 << 13), "A GDT can only have at most 2^13 entries");

        const NULL: GDTEntry = GDTEntry::new(0);
        Self {
            table: [NULL; MAX],
            len: 1,
        }
    }

    /// Panics if:
    /// * the provided slice has more than `MAX` entries
    /// * the provided slice is empty
    /// * the first entry is not zero
    #[inline]
    pub const fn from_raw_entries(slice: &[u64]) -> Self {
        let len = slice.len();
        let mut table = Self::empty().table;
        let mut idx = 0;

        assert!(len > 0, "cannot initialize GDT with empty slice");
        assert!(slice[0] == 0, "first GDT entry must be zero");
        assert!(
            len <= MAX,
            "cannot initialize GDT with slice exceeding the maximum length"
        );

        while idx < len {
            table[idx] = GDTEntry::new(slice[idx]);
            idx += 1;
        }

        Self { table, len }
    }

    /// The resulting slice may contain system descriptors, which span two [`Entry`]s.
    #[inline]
    pub fn entries(&self) -> &[GDTEntry] {
        &self.table[..self.len]
    }

    /// Note that depending on the type of the [`Descriptor`] this may append
    /// either one or two new [`Entry`]s to the table.
    ///
    /// Panics if the GDT doesn't have enough free entries.
    #[inline]
    pub fn append(&mut self, entry: Descriptor) -> SegmentSelector {
        let index = match entry {
            Descriptor::UserSegment(value) => {
                if self.len > self.table.len().saturating_sub(1) {
                    panic!("GDT full")
                }
                self.push(value)
            }
            Descriptor::SystemSegment(value_low, value_high) => {
                if self.len > self.table.len().saturating_sub(2) {
                    panic!("GDT requires two free spaces to hold a SystemSegment")
                }
                let index = self.push(value_low);
                self.push(value_high);
                index
            }
        };
        SegmentSelector::new(index as u16, entry.dpl())
    }

    #[inline]
    pub fn load(&'static self) {
        unsafe { self.load_unsafe() };
    }

    /// # Safety
    ///
    /// Unlike `load` this function will not impose a static lifetime constraint
    /// this means its up to the user to ensure that there will be no modifications
    /// after loading and that the GDT will live for as long as it's loaded.
    #[inline]
    pub unsafe fn load_unsafe(&self) {
        unsafe {
            load_gdt(&self.pointer());
        }
    }

    #[inline]
    fn push(&mut self, value: u64) -> usize {
        let index = self.len;
        self.table[index] = GDTEntry::new(value);
        self.len += 1;
        index
    }

    /// Returns the value of the limit for a gdt pointer. It is one less than the number of bytes of the table.
    pub const fn limit(&self) -> u16 {
        use core::mem::size_of;
        // 0 < self.next_free <= MAX <= 2^13, so the limit calculation
        // will not underflow or overflow.
        (self.len * size_of::<u64>() - 1) as u16
    }

    fn pointer(&self) -> DescriptorTablePointer {
        DescriptorTablePointer {
            base: VirtualAddress::new(self.table.as_ptr() as u64),
            limit: self.limit(),
        }
    }
}
