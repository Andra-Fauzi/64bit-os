use crate::FRAMEBUFFER_REQUEST;

pub fn draw_pixel(x: u64, y: u64, color: u32) {
    if let Some(resp) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(fb) = resp.framebuffers().next() {
            let width = fb.width();
            let height = fb.height();
            if x > width || y > height {
                return;
            }
            let pitch = fb.pitch() as u64; // bytes per row
            let bpp = fb.bpp() / 8;        // bytes per pixel

            let offset = y as u64 * pitch + x as u64 * bpp as u64;

            unsafe {
                fb
                    .addr()
                    .add(offset as usize)
                    .cast::<u32>()
                    .write(color);
            }
        }
    }
}

pub fn draw_line(xpos: u64, ypos: u64, xdest: u64, ydest: u64, color: u32) {
    if let Some(resp) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(fb) = resp.framebuffers().next() {
            let width = fb.width();
            let height = fb.height();
            if xpos > width || xdest > width || ypos > height || ydest > height {
                return;
            }
            let pitch = fb.pitch() as u64; // bytes per row
            let bpp = fb.bpp() / 8;        // bytes per pixel
            let mut y: u64 = ypos;
            let mut x: u64 = xpos;
            loop {
                if y < ydest {
                    y += 1;
                }
                if x < xdest {
                    x += 1;
                }
                if x >= xdest && y >= ydest {
                    break;
                }
                let pixel_offset = y * pitch + x * bpp as u64;
                unsafe {
                    fb
                        .addr()
                        .add(pixel_offset as usize)
                        .cast::<u32>()
                        .write(color);
                }
            }
        }
    }
}

pub fn get_pixel(fb_addr: *const u8, pitch: usize, x: usize, y: usize) -> u32 {
    unsafe {
        let offset = y * pitch + x * 4;
        let ptr = fb_addr.add(offset) as *const u32;
        ptr.read_volatile()
    }
}

pub fn get_pixel_offset(fb_addr: *const u8, offset: usize) -> u32 {
    unsafe {
        let ptr = fb_addr.add(offset) as *const u32;
        ptr.read_volatile()
    }
}
