#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VirtualAddress(u64);

pub struct VirtualAddressInvalid(u64);

impl VirtualAddress {
       #[inline]
    pub const fn new(addr: u64) -> VirtualAddress {
        match Self::try_new(addr) {
            Ok(v) => v,
            Err(_) => panic!("virtual address must be sign extended in bits 48 to 64"),
        }
    }

    /// Tries to create a new canonical virtual address.
    ///
    /// This function checks wether the given address is canonical
    /// and returns an error otherwise. An address is canonical
    /// if bits 48 to 64 are a correct sign
    /// extension (i.e. copies of bit 47).
    #[inline]
    pub const fn try_new(addr: u64) -> Result<VirtualAddress, VirtualAddressInvalid> {
        let v = Self::new_truncate(addr);
        if v.0 == addr {
            Ok(v)
        } else {
            Err(VirtualAddressInvalid(addr))
        }
    }

    /// Creates a new canonical virtual address, throwing out bits 48..64.
    ///
    /// This function performs sign extension of bit 47 to make the address
    /// canonical, overwriting bits 48 to 64. If you want to check whether an
    /// address is canonical, use [`new`](Self::new) or [`try_new`](Self::try_new).
    #[inline]
    pub const fn new_truncate(addr: u64) -> VirtualAddress {
        // By doing the right shift as a signed operation (on a i64), it will
        // sign extend the value, repeating the leftmost bit.
        VirtualAddress(((addr << 16) as i64 >> 16) as u64)
    }

    /// Creates a new virtual address, without any checks.
    ///
    /// ## Safety
    ///
    /// You must make sure bits 48..64 are equal to bit 47. This is not checked.
    #[inline]
    pub const unsafe fn new_unsafe(addr: u64) -> VirtualAddress {
        VirtualAddress(addr)
    }

    /// Creates a virtual address that points to `0`.
    #[inline]
    pub const fn zero() -> VirtualAddress {
        VirtualAddress(0)
    }

    /// Converts the address to an `u64`.
    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    /// Creates a virtual address from the given pointer
    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn from_ptr<T: ?Sized>(ptr: *const T) -> Self {
        Self::new(ptr as *const () as u64)
    }

    /// Converts the address to a raw pointer.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub const fn as_ptr<T>(self) -> *const T {
        self.as_u64() as *const T
    }

    /// Converts the address to a mutable raw pointer.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub const fn as_mut_ptr<T>(self) -> *mut T {
        self.as_ptr::<T>() as *mut T
    }

    /// Convenience method for checking if a virtual address is null.
    #[inline]
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
}