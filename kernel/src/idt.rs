use core::ptr;
use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};

pub static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub unsafe fn load_idt() {
    let idt_ptr: *mut InterruptDescriptorTable = ptr::addr_of_mut!(IDT);

    let idt_ref: &'static InterruptDescriptorTable = &*idt_ptr;

    (*idt_ptr).load();

}

pub unsafe fn set_idt(index: u8, handler: extern "x86-interrupt" fn(InterruptStackFrame)) {
    let idt_ptr: *mut InterruptDescriptorTable = ptr::addr_of_mut!(IDT);
    (&mut (*idt_ptr))[index].set_handler_fn(handler);
}
