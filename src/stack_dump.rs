extern "C" {
	static blight_stack: *const u8;
}

const STACK_SIZE: usize = 16384;

use crate::screen::*;
use core::slice::from_raw_parts;
use core::arch::asm;

// 00000000  23 20 42 4c 49 47 48 54  20 62 79 20 4a 6f 6c 61  |# BLIGHT by Jola|

fn get_ascii_representation(c: u8) -> char {
	if c >= 32 && c <= 126 { c.into() } else { '.' }
}

fn dump_slice(slice: &[u8], count: usize) -> usize {
	let mut count = count; // Could also add mut to parameter declaration but that kinda sucks
	printk!(LogLevel::Debug, "{:08x} ", count);
	for (i, c) in slice.iter().enumerate() {
		if i % 8 == 0 {
			printk!(LogLevel::Debug, " ");
		}
		printk!(LogLevel::Debug, "{:02x} ", c);
		count += 1;
	}
	printk!(LogLevel::Debug, " |");
	for &c in slice.into_iter() {
		printk!(LogLevel::Debug, "{}", get_ascii_representation(c));
	}
	printk!(LogLevel::Debug, "|\n");
	return count;
}

pub fn stack_dump(size_to_dump: usize) {
	let slice;
	let esp: usize;
	unsafe {
		asm!("mov {:e}, esp", out(reg) esp);
		slice = from_raw_parts(esp as *const u8, size_to_dump);
	}
	let mut count = 0;
	while count < size_to_dump {
		let end = size_to_dump.min(count+16);
		let subslice = &slice[count..end];
		dump_slice(subslice, count + esp);
		count += subslice.len();
	}
	printkln!(LogLevel::Debug, "{:08x}", count + esp);
}
