use core::arch::asm;

pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

const EXIT_PORT: u16 = 0xf4;

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        asm!("out dx, eax", in("dx") EXIT_PORT, in("eax") exit_code as u32, options(nomem, nostack, preserves_flags));
    }
}