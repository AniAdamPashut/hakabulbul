use core::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VirtualAddress(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub struct VirtualAddressInvalid(u64);

impl VirtualAddress {
       #[inline]
    pub const fn new(addr: u64) -> VirtualAddress {
        match Self::try_new(addr) {
            Ok(v) => v,
            Err(_) => panic!("virtual address must be sign extended in bits 48 to 64"),
        }
    }

    #[inline]
    pub const fn try_new(addr: u64) -> Result<VirtualAddress, VirtualAddressInvalid> {
        let v = Self::new_truncate(addr);
        if v.0 == addr {
            Ok(v)
        } else {
            Err(VirtualAddressInvalid(addr))
        }
    }

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

    #[inline]
    pub const fn zero() -> VirtualAddress {
        VirtualAddress(0)
    }

    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn from_ptr<T: ?Sized>(ptr: *const T) -> Self {
        Self::new(ptr as *const () as u64)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub const fn as_ptr<T>(self) -> *const T {
        self.as_u64() as *const T
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub const fn as_mut_ptr<T>(self) -> *mut T {
        self.as_ptr::<T>() as *mut T
    }

    #[inline]
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Add<u64> for VirtualAddress {
    type Output = Self;

    #[inline]
    fn add(self, rhs: u64) -> Self::Output {
        VirtualAddress::try_new(
            self.0
                .checked_add(rhs)
                .expect("attempt to add with overflow"),
        )
        .expect("attempt to add resulted in non-canonical virtual address")
    }
}

impl Add<usize> for VirtualAddress {
    type Output = Self;
    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        self + rhs as u64
    }
}
