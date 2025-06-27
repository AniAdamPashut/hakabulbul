pub mod gdt;
pub mod idt;

struct TablePointer {
    offset: u64,
    limit: u16
}