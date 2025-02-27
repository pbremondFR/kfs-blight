use core::fmt::{self, Write};
use core::ptr::copy;

macro_rules! VGA_BUFFER {
    () => {0xb8000}
}
macro_rules! VGA_WIDTH {
    () => {80}
}
macro_rules! VGA_HIGHT {
    () => {25}
}
macro_rules! VGA_BUFFER_SIZE {
    () => {VGA_WIDTH!() * 2 * VGA_HIGHT!()}
}

macro_rules! pr_debug {
    ($dst:expr, $($arg:tt)*) => {
        $dst.set_color(0x7);
        write!($dst, $($arg)*).expect("Write failed");
    };
}

macro_rules! pr_info {
    ($dst:expr, $($arg:tt)*) => {
        $dst.set_color(0xb);
        write!($dst, $($arg)*).expect("Write failed");
        $dst.set_color(0);
    }
}

macro_rules! pr_warn {
    ($dst:expr, $($arg:tt)*) => {
        $dst.set_color(0xc);
        write!($dst, $($arg)*).expect("Write failed");
        $dst.set_color(0);
    }
}

macro_rules! pr_error {
    ($dst:expr, $($arg:tt)*) => {
        $dst.set_color(0x4);
        write!($dst, $($arg)*).expect("Write failed");
        $dst.set_color(0);
    }
}


pub struct Screen {
    buf: [u8; VGA_BUFFER_SIZE!()],
    line: usize,
    pos: usize,
    color: u8,
}

impl Screen {
    pub fn new() -> Self {
        Screen { buf: [0; VGA_BUFFER_SIZE!()], line: 0, pos: 0, color: 0x0 }
    }

    pub fn set_color(&mut self, color: u8) {
        self.color = color;
    }

    pub fn scroll_up(&mut self) {
        if self.line < VGA_HIGHT!() {
            self.line += 1;

        } else {
            self.line -= 1;

            for i in 0..(VGA_HIGHT!() - 1) {
                for j in 0..VGA_WIDTH!() {
                    self.buf[(j + i * VGA_WIDTH!()) * 2] = self.buf[(j + (i + 1) * VGA_WIDTH!()) * 2];
                    self.buf[(j + i * VGA_WIDTH!()) * 2 + 1] = self.buf[(j + (i + 1) * VGA_WIDTH!()) * 2 + 1];
                }
            }
            for i in 0..VGA_WIDTH!() {
                self.buf[(i + 24 * VGA_WIDTH!()) * 2 ] = 0;
                self.buf[(i + 24 * VGA_WIDTH!()) * 2 + 1] = 0;
            }
        }
    }
}

impl Write for Screen {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.len() > VGA_WIDTH!() {
            return Err(fmt::Error);
        }
        
        if self.line == VGA_HIGHT!(){
            self.scroll_up();
        }

        for byte in s.bytes() {
            if byte == b'\n' || self.pos == VGA_WIDTH!() * 2 {
                self.pos = 0;
                self.scroll_up();
                continue;
            } 
            self.buf[(self.pos + self.line * VGA_WIDTH!()) * 2 ] = byte;
            self.buf[(self.pos + self.line * VGA_WIDTH!()) * 2 + 1] = self.color;
            self.pos += 1;
        }

        let vga_buffer = VGA_BUFFER!() as *mut u8;
        
        unsafe {
            copy(self.buf.as_ptr(), vga_buffer, VGA_BUFFER_SIZE!());
        }
        
        Ok(())
    }

    fn write_fmt(&mut self, fmt_args: fmt::Arguments) -> fmt::Result {
        fmt::write(self, fmt_args)
    }
}
