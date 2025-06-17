use core::panic::PanicInfo;

mod qemu_exit;
mod testable;

pub mod vga;
pub mod gdt;

use qemu_exit::exit_qemu;
use qemu_exit::QemuExitCode;

use testable::Testable;

pub fn test_runner(tests: &[&dyn Testable]) {
    use crate::com1_sendln;

    com1_sendln!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    com1_sendln!("Passed {} tests", tests.len());
    exit_qemu(QemuExitCode::Success);
}


#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    use crate::com1_sendln;

    com1_sendln!("{}", _info);
    exit_qemu(QemuExitCode::Failed);

    loop {}
}
