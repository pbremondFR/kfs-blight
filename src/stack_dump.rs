use crate::screen::printk;
use crate::screen::LogLevel;
use core::slice::from_raw_parts;
use core::arch::asm;

// 00000000  23 20 42 4c 49 47 48 54  20 62 79 20 4a 6f 6c 61  |# BLIGHT by Jola|

fn get_ascii_representation(c: u8) -> char {
	if c >= 32 && c <= 126 { c.into() } else { '.' }
}

fn dump_slice(slice: &[u8], count: usize) {
	printk!(LogLevel::Debug, "{:08x} ", count);
	for i in 0..16 {
		if i % 8 == 0 {
			printk!(LogLevel::Debug, " ");
		}
		if i < slice.len() {
			printk!(LogLevel::Debug, "{:02x} ", slice[i]);
		} else {
			printk!(LogLevel::Debug, "   ");
		}
	}
	printk!(LogLevel::Debug, " |");
	for &c in slice.into_iter() {
		printk!(LogLevel::Debug, "{}", get_ascii_representation(c));
	}
	printk!(LogLevel::Debug, "|\n");
}

pub extern "C" fn dump_address(size_to_dump: usize, ptr: usize) {
	let slice;
	unsafe {
		slice = from_raw_parts(ptr as *const u8, size_to_dump);
	}
	let mut count = 0;
	while count < size_to_dump {
		let end = size_to_dump.min(count+16);
		let subslice = &slice[count..end];
		dump_slice(subslice, count + ptr);
		count += subslice.len();
	}
	printkln!(LogLevel::Debug, "{:08x}", count + ptr);
}

// Force inline expansion to keep the esp from the caller.
// Technically Rust doesn't 100% guarantee expansion, but according to the book,
// "in practice #[inline(always)] will cause inlining in all but the most
// exceptional cases". For this 2-liner, we should be good to go.
#[inline(always)]
pub extern "C" fn stack_dump(size_to_dump: usize) {
	let esp: u32;
	let ebp: u32;
	unsafe {
		asm!("mov {:e}, esp", out(reg) esp);
		asm!("mov {:e}, ebp", out(reg) ebp);
	}
	pr_debug!("STACK DUMP: esp 0x{:08x}, ebp 0x{:08x}", esp, ebp);
	dump_address(size_to_dump, esp as usize);
}

// Force inline expansion to keep the esp from the caller.
// Technically Rust doesn't 100% guarantee expansion, but according to the book,
// "in practice #[inline(always)] will cause inlining in all but the most
// exceptional cases". For this 2-liner, we should be good to go.
#[inline(always)]
pub fn stack_dump_cmd(mut args: core::str::SplitAsciiWhitespace) {
	if let Ok(size) = args.next().unwrap_or("128").parse() {
		stack_dump(size);
	}
}
