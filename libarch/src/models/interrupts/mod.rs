use super::address::VirtualAddress;

mod idt;
mod stack;
mod descriptors;

pub use descriptors::InterruptDescriptor;
pub use descriptors::DescriptorOptions;
pub use stack::InterruptStackFrame;
pub use idt::InterruptDescriptorTable;

type InterruptHandler = fn(InterruptStackFrame);
type InterruptHandlerWithErrorCode = fn(InterruptStackFrame, error_code: u64);

impl<F: HandlerFuncType> InterruptDescriptor<F> {
    #[inline]
    pub fn set_handler(&mut self, handler: F) -> &mut DescriptorOptions {
        unsafe { self.set_handler_addr(handler.to_virt_addr()) }
    }
}

pub unsafe trait HandlerFuncType {
    fn to_virt_addr(self) -> VirtualAddress;
}

macro_rules! impl_handler_func_type {
    ($f:ty) => {
        unsafe impl HandlerFuncType for $f {
            #[inline]
            fn to_virt_addr(self) -> VirtualAddress {
                // Casting a function pointer to u64 is fine, if the pointer
                // width doesn't exeed 64 bits.
                VirtualAddress::new(self as u64)
            }
        }
    };
}

impl_handler_func_type!(InterruptHandler);
impl_handler_func_type!(InterruptHandlerWithErrorCode);
// impl_handler_func_type!(PageFaultHandlerFunc);
// impl_handler_func_type!(DivergingHandlerFunc);
// impl_handler_func_type!(DivergingHandlerFuncWithErrCode);