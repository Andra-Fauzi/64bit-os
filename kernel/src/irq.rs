use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};
use crate::pic::pic_send_eoi;
use crate::pic;
use crate::idt::{IDT, set_idt};
use crate::{println, print};
use crate::port;
use pc_keyboard::{Keyboard, layouts, ScancodeSet1, HandleControl, DecodedKey};
use core::ptr;
use crate::terminal;

extern "x86-interrupt" fn timer_handler(_stack: InterruptStackFrame) {
    unsafe  {
        pic_send_eoi(0);
    }
}

static mut KEYBOARD: Keyboard<layouts::Us104Key, ScancodeSet1> = Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore);

extern "x86-interrupt" fn keyboard_handler(_stack: InterruptStackFrame) {
    unsafe  {
        let kb: *mut Keyboard<layouts::Us104Key, ScancodeSet1> = ptr::addr_of_mut!(KEYBOARD);
        let status = port::inb(0x64);
        if status & 0x1 != 0{
            let scancode = port::inb(0x60);
            if scancode & 0x80 != 0{
                pic_send_eoi(1);
                return;
            }
            if let Ok(Some(event)) = (*kb).add_byte(scancode) {
                if let Some(key) = (*kb).process_keyevent(event) {
                    match key {
                        DecodedKey::Unicode('\u{8}') => {
                            terminal::remove_before();
                        }
                        DecodedKey::Unicode(c) => {
                            print!("{}", c);
                        }
                        _ => {}
                    }
                }
            }
        }
        pic_send_eoi(1);
    }
}

pub fn irq_init() {
    unsafe {
        pic::remap(0x20, 0x28);
        //set_general_handler!(&mut (*idt_ptr), timer_handler, 32);
        //set_general_handler!(&mut (*idt_ptr), keyboard_handler, 33);
        set_idt(32, timer_handler);
        set_idt(33, keyboard_handler);
    }
    x86_64::instructions::interrupts::enable();
}

