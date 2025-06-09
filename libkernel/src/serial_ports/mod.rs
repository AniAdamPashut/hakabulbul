use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

const COM1: u16 = 0x3F8;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(COM1) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}


#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! com1_send {
    ($($arg:tt)*) => {
        $crate::serial_ports::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! com1_sendln {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::com1_send!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::com1_send!(
        concat!($fmt, "\n"), $($arg)*));
}