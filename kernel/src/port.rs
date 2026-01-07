use x86_64::instructions::port::Port;

pub unsafe fn outb(port: u16, value: u8) {
    unsafe {
        let mut port: Port<u8> = Port::new(port);
        port.write(value);
    }
}

pub unsafe fn inb(port: u16) -> u8 {
    unsafe {
        let mut port: Port<u8> = Port::new(port);
        port.read()
    }
}
