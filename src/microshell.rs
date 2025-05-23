use crate::screen::*;
use crate::io;
use crate::stack_dump;
use crate::screen::VGA_WIDTH;
use core::slice::from_raw_parts_mut;

pub struct ShellBuf {
	buf: [u8; VGA_WIDTH],
	len: usize,
}

impl ShellBuf {
	fn update_cursor(&self) {
		let x = self.len as u16;
		let y = VGA_HEIGHT as u16;
		let pos = (y * VGA_WIDTH as u16) + x;

		io::outb(0x3d4, 0x0f);
		io::outb(0x3d5, (pos & 0xff) as u8);
		io::outb(0x3d4, 0x0e);
		io::outb(0x3d5, ((pos >> 8) & 0xff) as u8);
	}

	unsafe fn flush_buffer(&self) {
		unsafe {
			let mut vga_buffer = VGA_BUFFER as *mut u8;
			vga_buffer = vga_buffer.add((VGA_HEIGHT) * VGA_WIDTH * 2);
			assert!(vga_buffer as usize == 757504);
			let vga_input_line = from_raw_parts_mut(vga_buffer, VGA_WIDTH * 2);

			let mut i: usize = 0;
			while i < self.len {
				vga_input_line[i * 2] = self.buf[i];
				vga_input_line[i * 2 + 1] = VgaColor::LightMagenta as u8;
				i += 1;
			}
			while i < VGA_WIDTH {
				vga_input_line[i * 2] = 0;
				vga_input_line[i * 2 + 1] = VgaColor::LightMagenta as u8;
				i += 1;
			}
		}
	}

	unsafe fn push_shell_char(&mut self, c: u8) -> bool {
		if self.len >= VGA_WIDTH {
			return false;
		} else {
			self.buf[self.len] = c;
			self.len += 1;
			self.flush_buffer();
			self.update_cursor();
			return true;
		}
	}

	unsafe fn pop_shell_char(&mut self) -> bool {
		if self.len == 0 {
			return false;
		} else {
			self.len -= 1;
			self.flush_buffer();
			self.update_cursor();
			return true;
		}

	}

	fn match_command(mut tokens: core::str::SplitAsciiWhitespace) {
		if let Some(cmd) = tokens.next() {
			match cmd {
				"STACK" => { stack_dump::stack_dump_cmd(tokens); },
				"REBOOT" => { crate::reboot(); }
				_ => { pr_error!("{}: not found", cmd); },
			}
		}
	}

	unsafe fn enter_cmd(&mut self) {
		let str = unsafe { core::str::from_utf8_unchecked(&self.buf[..self.len]) };
		let tokens = str.split_ascii_whitespace();
		Self::match_command(tokens);
		self.len = 0;
		self.flush_buffer();
		self.update_cursor();
	}

	unsafe fn clear_buffer(&mut self) {
		self.len = 0;
		self.flush_buffer();
		self.update_cursor();
	}

}

pub static mut SHELL_INPUT: ShellBuf = ShellBuf{buf: [0; VGA_WIDTH], len: 0};

pub fn init_shell() {
	// Fill buffer with color
	unsafe { clear_buffer(); }

	// Enable cursor, is 2 scanlines tall (between scanlines 13 and 15 of character)
	io::outb(0x3D4, 0x0A);
	io::outb(0x3D5, (io::inb(0x3D5) & 0xC0) | 13);

	io::outb(0x3D4, 0x0B);
	io::outb(0x3D5, (io::inb(0x3D5) & 0xE0) | 15);

	// Set cursor position to line 25, column 0
	let pos = (VGA_HEIGHT * VGA_WIDTH) as u16;
	io::outb(0x3d4, 0x0f);
	io::outb(0x3d5, (pos & 0xff) as u8);
	io::outb(0x3d4, 0x0e);
	io::outb(0x3d5, ((pos >> 8) & 0xff) as u8);
}

pub unsafe fn push_shell_char(c: u8) -> bool {
	unsafe {
		let shell = &raw mut SHELL_INPUT;
		(*shell).push_shell_char(c)
	}
}

pub unsafe fn pop_shell_char() -> bool {
	unsafe {
		let shell = &raw mut SHELL_INPUT;
		(*shell).pop_shell_char()
	}
}

pub unsafe fn enter_cmd() {
	unsafe {
		let shell = &raw mut SHELL_INPUT;
		(*shell).enter_cmd()
	}
}

pub unsafe fn clear_buffer() {
	unsafe {
		let shell = &raw mut SHELL_INPUT;
		(*shell).clear_buffer()
	}
}
