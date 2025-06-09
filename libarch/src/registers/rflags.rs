use core::arch::asm;

use bitflags::bitflags;

bitflags! {
    /// https://wiki.osdev.org/CPU_Registers_x86-64#RFLAGS_Register
    #[repr(transparent)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct RFlags: u64 {
        const ID = 1 << 21;
        /// Indicates that an external, maskable interrupt is pending.
        ///
        /// Used when virtual-8086 mode extensions (CR4.VME) or protected-mode virtual
        /// interrupts (CR4.PVI) are activated.
        const VIRTUAL_INTERRUPT_PENDING = 1 << 20;
        /// Virtual image of the INTERRUPT_FLAG bit.
        ///
        /// Used when virtual-8086 mode extensions (CR4.VME) or protected-mode virtual
        /// interrupts (CR4.PVI) are activated.
        const VIRTUAL_INTERRUPT = 1 << 19;
        /// Enable automatic alignment checking if CR0.AM is set. Only works if CPL is 3.
        const ALIGNMENT_CHECK = 1 << 18;
        /// Enable the virtual-8086 mode.
        const VIRTUAL_8086_MODE = 1 << 17;
        /// Allows to restart an instruction following an instruction breakpoint.
        const RESUME_FLAG = 1 << 16;
        /// Used by `iret` in hardware task switch mode to determine if current task is nested.
        const NESTED_TASK = 1 << 14;
        /// The high bit of the I/O Privilege Level field.
        ///
        /// Specifies the privilege level required for executing I/O address-space instructions.
        const IOPL_HIGH = 1 << 13;
        /// The low bit of the I/O Privilege Level field.
        ///
        /// Specifies the privilege level required for executing I/O address-space instructions.
        const IOPL_LOW = 1 << 12;
        const OVERFLOW_FLAG = 1 << 11;
        const DIRECTION_FLAG = 1 << 10;
        const INTERRUPT_FLAG = 1 << 9;
        const TRAP_FLAG = 1 << 8;
        const SIGN_FLAG = 1 << 7;
        const ZERO_FLAG = 1 << 6;
        const AUXILIARY_CARRY_FLAG = 1 << 4;
        const PARITY_FLAG = 1 << 2;
        const CARRY_FLAG = 1;
    }
}

impl RFlags {
    pub fn read() -> RFlags {
        RFlags::from_bits_truncate(RFlags::read_raw())
    }

    pub fn read_raw() -> u64 {
        let mut rflags: u64;
        unsafe {
            asm!(
                "pushfq",
                "pop {}", out(reg) rflags,
                options(nomem, preserves_flags)
            );
        }
        rflags
    }

    pub fn write(flags: RFlags) {
        let old_flags = RFlags::read_raw();
        let reserved_bits = old_flags & (RFlags::all().bits());
        let new_flags = reserved_bits | flags.bits();

        unsafe {
            RFlags::write_raw(new_flags)
        }
    } 

    pub unsafe fn write_raw(flags: u64) {
        unsafe {
            asm!(
                "push {}; popfq", in(reg) flags,
                options(nomem, preserves_flags)
            )
        }
    }
}