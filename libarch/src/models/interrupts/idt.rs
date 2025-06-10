use crate::models::address::VirtualAddress;
use crate::models::DescriptorTablePointer;        
use crate::instructions::tables::load_idt;

use super::InterruptHandler;
use super::InterruptHandlerWithErrorCode;
use super::descriptors::InterruptDescriptor;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct InterruptDescriptorTable {
    pub divide_error: InterruptDescriptor<InterruptHandler>,
    pub debug_error: InterruptDescriptor<InterruptHandler>,
    pub nmi: InterruptDescriptor<InterruptHandler>,
    pub breakpoint: InterruptDescriptor<InterruptHandler>,
    pub overflow: InterruptDescriptor<InterruptHandler>,
    pub bound: InterruptDescriptor<InterruptHandler>,
    pub invalid_opcode: InterruptDescriptor<InterruptHandler>,
    pub device_unavailable: InterruptDescriptor<InterruptHandler>,
    pub double_fault: InterruptDescriptor<InterruptHandlerWithErrorCode>,
    pub coprocessor_segment_overrun: InterruptDescriptor<InterruptHandler>,
    pub invalid_tss: InterruptDescriptor<InterruptHandlerWithErrorCode>,
    pub segment_not_present: InterruptDescriptor<InterruptHandlerWithErrorCode>,
    pub stack_segment_fault: InterruptDescriptor<InterruptHandlerWithErrorCode>,
    pub general_protection: InterruptDescriptor<InterruptHandlerWithErrorCode>,
    pub page_fault: InterruptDescriptor<InterruptHandlerWithErrorCode>,
    _reserved1: InterruptDescriptor<InterruptHandler>,

    pub fpu_error: InterruptDescriptor<InterruptHandler>,
    pub alignment_error: InterruptDescriptor<InterruptHandlerWithErrorCode>,
    pub machine_check: InterruptDescriptor<InterruptHandler>,
    pub simd_error: InterruptDescriptor<InterruptHandler>,
    pub virutalization_error: InterruptDescriptor<InterruptHandler>,
    pub control_protection_error: InterruptDescriptor<InterruptHandlerWithErrorCode>,
    _reserved2: [InterruptDescriptor<InterruptHandler>; 6],

    pub hv_injection_exception: InterruptDescriptor<InterruptHandler>,
    pub vmm_communication_exception: InterruptDescriptor<InterruptHandlerWithErrorCode>,
    pub security_exception: InterruptDescriptor<InterruptHandlerWithErrorCode>,
    _reserved3: InterruptDescriptor<InterruptHandler>,

    pub custom_interrupts: [InterruptDescriptor<InterruptHandler>; 256 - 32],
}

impl InterruptDescriptorTable {
    #[inline]
    pub fn new() -> Self {
        InterruptDescriptorTable {
            divide_error: InterruptDescriptor::missing(),
            debug_error: InterruptDescriptor::missing(),
            nmi: InterruptDescriptor::missing(),
            breakpoint: InterruptDescriptor::missing(),
            overflow: InterruptDescriptor::missing(),
            bound: InterruptDescriptor::missing(),
            invalid_opcode: InterruptDescriptor::missing(),
            device_unavailable: InterruptDescriptor::missing(),
            double_fault: InterruptDescriptor::missing(),
            coprocessor_segment_overrun: InterruptDescriptor::missing(),
            invalid_tss: InterruptDescriptor::missing(),
            segment_not_present: InterruptDescriptor::missing(),
            stack_segment_fault: InterruptDescriptor::missing(),
            general_protection: InterruptDescriptor::missing(),
            page_fault: InterruptDescriptor::missing(),
            _reserved1: InterruptDescriptor::missing(),
            fpu_error: InterruptDescriptor::missing(),
            alignment_error: InterruptDescriptor::missing(),
            machine_check: InterruptDescriptor::missing(),
            simd_error: InterruptDescriptor::missing(),
            virutalization_error: InterruptDescriptor::missing(),
            control_protection_error: InterruptDescriptor::missing(),
            _reserved2: [InterruptDescriptor::missing(); 6],
            hv_injection_exception: InterruptDescriptor::missing(),
            vmm_communication_exception: InterruptDescriptor::missing(),
            security_exception: InterruptDescriptor::missing(),
            _reserved3: InterruptDescriptor::missing(),
            custom_interrupts: [InterruptDescriptor::missing(); 256 - 32],
        }
    }

    pub fn load(&'static self) {
        unsafe { self.load_unsafe() }
    }

    #[inline]
    pub unsafe fn load_unsafe(&self) {
        unsafe {
            load_idt(&self.pointer());
        }
    }
    

    fn pointer(&self) -> DescriptorTablePointer {
        use core::mem::size_of;
        DescriptorTablePointer {
            base: VirtualAddress::new(self as *const _ as u64),
            limit: (size_of::<Self>() - 1) as u16,
        }
    }
}
