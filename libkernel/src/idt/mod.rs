#[derive(Debug, Clone)]
#[repr(C)]
pub struct InterruptGate {
    offset_high: u16,
    options: u16
}