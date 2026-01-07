use crate::idt::IDT;
use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};
use crate::{println, print};
use core::ptr;
use x86_64::set_general_handler;

pub unsafe fn isr_init() {
    unsafe {
        let idt_ptr: *mut InterruptDescriptorTable = ptr::addr_of_mut!(IDT);
        set_general_handler!((&mut (*idt_ptr)), general_handler);
        set_general_handler!((&mut (*idt_ptr)), divide_by_zero_handler, 0);
        set_general_handler!((&mut (*idt_ptr)), invalid_opcode_handler, 6);
        set_general_handler!((&mut (*idt_ptr)), double_fault_handler, 8);
        set_general_handler!((&mut (*idt_ptr)), general_protection_fault_handler, 13);
        set_general_handler!((&mut (*idt_ptr)), page_fault_handler, 14);
    }
}
fn general_handler(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
    println!("General Handler ERROR!!!!");
    println!("{:#?}", stack_frame);
    println!("handler isr {}", index);
    if let Some(code) = error_code {
        println!("Error Code: {}", code);
    }
    loop {};
}

fn divide_by_zero_handler(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
    println!("Divide by Zero ERROR!!!!");
    println!("{:#?}", stack_frame);
    println!("handler isr {}", index);
    if let Some(code) = error_code {
        println!("Error Code: {}", code);
    }
    loop {};
}

fn invalid_opcode_handler(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
    println!("Invalid Opcode ERROR!!!!");
    println!("{:#?}", stack_frame);
    println!("handler isr {}", index);
    if let Some(code) = error_code {
        println!("Error Code: {}", code);
    }
    loop {};
}

fn double_fault_handler(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
    println!("Double Fault ERROR!!!!");
    println!("{:#?}", stack_frame);
    println!("handler isr {}", index);
    if let Some(code) = error_code {
        println!("Error Code: {}", code);
    }
    loop {};
}

fn general_protection_fault_handler(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
    println!("General Protection Fault ERROR!!!!");
    println!("{:#?}", stack_frame);
    println!("handler isr {}", index);
    if let Some(code) = error_code {
        println!("Error Code: {}", code);
    }
    loop {};
}

fn page_fault_handler(stack_frame: InterruptStackFrame, index: u8, error_code: Option<u64>) {
    println!("Page Fault ERROR!!!!");
    println!("{:#?}", stack_frame);
    println!("handler isr {}", index);
    if let Some(code) = error_code {
        println!("Error Code: {}", code);
    }
    loop {};
}
