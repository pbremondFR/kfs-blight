use core::fmt::{self, Write};
use core::intrinsics::volatile_copy_nonoverlapping_memory;

pub const VGA_BUFFER: usize = 0xb8000;
pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 24; // 1 line less to allow space for the input line
pub const VGA_BUFFER_SIZE: usize = VGA_WIDTH * 2 * VGA_HEIGHT;

macro_rules! pr_debug {
    ($($arg:tt)*) => {
        printkln!(LogLevel::Debug, $($arg)*)
    };
}

macro_rules! pr_info {
    ($($arg:tt)*) => {
        printkln!(LogLevel::Info, $($arg)*)
    };
}

macro_rules! pr_warn {
    ($($arg:tt)*) => {
        printkln!(LogLevel::Warn, $($arg)*)
    };
}

macro_rules! pr_error {
    ($($arg:tt)*) => {
        printkln!(LogLevel::Error, $($arg)*)
    };
}

macro_rules! printk {
    ($level:expr, $($arg:tt)*) => {
        printk($level, format_args!($($arg)*))
    }
}

macro_rules! printkln {
    ($level:expr, $($arg:tt)*) => {
        #[allow(unused_must_use)]
        printk($level, format_args_nl!($($arg)*))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum VgaColor {
    Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGrey = 7,
	DarkGrey = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	LightMagenta = 13,
	LightBrown = 14,
	White = 15
}

pub enum LogLevel {
    Debug = 0,
    Info,
    Warn,
    Error
}

pub struct Screen {
    buf: [[u8; VGA_BUFFER_SIZE]; 4],
    active: usize,
    line: [usize; 4],
    pos: [usize; 4],
    color: VgaColor,
}

static mut SCREEN: Screen = Screen { buf: [[0; VGA_BUFFER_SIZE]; 4], active: 0, line: [0; 4], pos: [0; 4], color: VgaColor::White };

#[allow(static_mut_refs)]
#[allow(unused_must_use)]
pub fn printk(level: LogLevel, fmt: fmt::Arguments) -> fmt::Result {
    const LEVEL_COLORS: [VgaColor; 4] = [VgaColor::LightGrey, VgaColor::LightCyan, VgaColor::LightRed, VgaColor::Red];
    unsafe {
        SCREEN.set_color(LEVEL_COLORS[level as usize]);
        fmt::write(&mut SCREEN, fmt)
    }
}

#[allow(static_mut_refs)]
#[allow(unused_must_use)]
pub fn switch(active: usize) {
    unsafe {
        SCREEN.switch_active(active);
    }
}

impl Screen {
    pub fn new() -> Self {
        Screen { buf: [[0; VGA_BUFFER_SIZE]; 4], active: 0, line: [0; 4], pos: [0; 4], color: VgaColor::White }
    }

    pub fn set_color(&mut self, color: VgaColor) {
        self.color = color;
    }

    // XXX: Alternative trick: we'll probably use transmute often...
    // pub fn set_color(&mut self, color: u8) {
    //     self.color = unsafe { core::mem::transmute(color) };
    // }

    pub fn switch_active(&mut self, active: usize) {
        if active > 3 {
            self.active = 3;
        } else {
            self.active = active;
        }
        let vga_buffer = VGA_BUFFER as *mut u8;
        unsafe {
            volatile_copy_nonoverlapping_memory(vga_buffer, self.buf[self.active].as_ptr(), VGA_BUFFER_SIZE);
        }
    }

    pub fn scroll_up(&mut self) {
        if self.line[self.active] < VGA_HEIGHT {
            self.line[self.active] += 1;

        } else {
            self.line[self.active] -= 1;

            for i in 0..(VGA_HEIGHT - 1) {
                for j in 0..VGA_WIDTH {
                    self.buf[self.active][(j + i * VGA_WIDTH) * 2] = self.buf[self.active][(j + (i + 1) * VGA_WIDTH) * 2];
                    self.buf[self.active][(j + i * VGA_WIDTH) * 2 + 1] = self.buf[self.active][(j + (i + 1) * VGA_WIDTH) * 2 + 1];
                }
            }
            for i in 0..VGA_WIDTH {
                self.buf[self.active][(i + (VGA_HEIGHT - 1) * VGA_WIDTH) * 2 ] = 0;
                self.buf[self.active][(i + (VGA_HEIGHT - 1) * VGA_WIDTH) * 2 + 1] = 0;
            }
        }
    }
}

impl Write for Screen {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.len() > VGA_WIDTH {
            return Err(fmt::Error);
        }

        if self.line[self.active] == VGA_HEIGHT {
            self.scroll_up();
        }

        for byte in s.bytes() {
            if byte == b'\n' || self.pos[self.active] == VGA_WIDTH * 2 {
                self.pos[self.active] = 0;
                self.scroll_up();
                continue;
            }
            self.buf[self.active][(self.pos[self.active] + self.line[self.active] * VGA_WIDTH) * 2 ] = byte;
            self.buf[self.active][(self.pos[self.active] + self.line[self.active] * VGA_WIDTH) * 2 + 1] = self.color as u8;
            self.pos[self.active] += 1;
        }

        let vga_buffer = VGA_BUFFER as *mut u8;

        unsafe {
            volatile_copy_nonoverlapping_memory(vga_buffer, self.buf[self.active].as_ptr(), VGA_BUFFER_SIZE);
        }

        Ok(())
    }

    fn write_fmt(&mut self, fmt_args: fmt::Arguments) -> fmt::Result {
        fmt::write(self, fmt_args)
    }
}
