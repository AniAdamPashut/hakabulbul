mod gdt;

pub fn init() -> Result<(), ()> {
    gdt::init()?;

    Ok(())
}