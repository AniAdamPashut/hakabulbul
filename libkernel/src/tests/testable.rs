use crate::com1_sendln;

#[allow(dead_code)]
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        let name = core::any::type_name::<T>();
        com1_sendln!("Running {}...\t", name);
        self();
        com1_sendln!("{} Passed!!", name);
    }
}
