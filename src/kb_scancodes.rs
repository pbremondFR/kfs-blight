use crate::io;
use crate::screen::*;
use core::str;

pub const SCANCODES_CHARS: [u8; 256] = make_scancodes();
const BACKSPACE: u8 = 0x0e;
const ENTER: u8 = 0x1c;
const ESCAPE: u8 = 0x01;

const fn make_scancodes() -> [u8; 256] {
    let mut scancodes: [u8; 256] = [b'.'; 256];

    // Number row
    scancodes[0x02] = b'1';
    scancodes[0x03] = b'2';
    scancodes[0x04] = b'3';
    scancodes[0x05] = b'4';
    scancodes[0x06] = b'5';
    scancodes[0x07] = b'6';
    scancodes[0x08] = b'7';
    scancodes[0x09] = b'8';
    scancodes[0x0a] = b'9';
    scancodes[0x0b] = b'0';
    scancodes[0x0c] = b'-'; // Hyphen
    scancodes[0x0d] = b'='; // Equals

    // Top row
    scancodes[0x10] = b'Q';
    scancodes[0x11] = b'W';
    scancodes[0x12] = b'E';
    scancodes[0x13] = b'R';
    scancodes[0x14] = b'T';
    scancodes[0x15] = b'Y';
    scancodes[0x16] = b'U';
    scancodes[0x17] = b'I';
    scancodes[0x18] = b'O';
    scancodes[0x19] = b'P';
    scancodes[0x1a] = b'['; // Opening bracket
    scancodes[0x1b] = b']'; // Closing bracket

    // Home row
    scancodes[0x1e] = b'A';
    scancodes[0x1f] = b'S';
    scancodes[0x20] = b'D';
    scancodes[0x21] = b'F';
    scancodes[0x22] = b'G';
    scancodes[0x23] = b'H';
    scancodes[0x24] = b'J';
    scancodes[0x25] = b'K';
    scancodes[0x26] = b'L';
    scancodes[0x27] = b';'; // Semicolon
    scancodes[0x28] = b'\''; // Apostrophe
    scancodes[0x29] = b'`'; // Backtick
    scancodes[0x2b] = b'\\'; // Backslash

    // Bottom row
    scancodes[0x2c] = b'Z';
    scancodes[0x2d] = b'X';
    scancodes[0x2e] = b'C';
    scancodes[0x2f] = b'V';
    scancodes[0x30] = b'B';
    scancodes[0x31] = b'N';
    scancodes[0x32] = b'M';
    scancodes[0x33] = b','; // Comma
    scancodes[0x34] = b'.'; // Period
    scancodes[0x35] = b'/'; // Forward slash
    scancodes[0x37] = b'*'; // Keypad *
    scancodes[0x39] = b' '; // Space

    // Keypad numbers and operations
    scancodes[0x47] = b'7'; // Keypad 7
    scancodes[0x48] = b'8'; // Keypad 8
    scancodes[0x49] = b'9'; // Keypad 9
    scancodes[0x4a] = b'-'; // Keypad -
    scancodes[0x4b] = b'4'; // Keypad 4
    scancodes[0x4c] = b'5'; // Keypad 5
    scancodes[0x4d] = b'6'; // Keypad 6
    scancodes[0x4e] = b'+'; // Keypad +
    scancodes[0x4f] = b'1'; // Keypad 1
    scancodes[0x50] = b'2'; // Keypad 2
    scancodes[0x51] = b'3'; // Keypad 3
    scancodes[0x52] = b'0'; // Keypad 0

    return scancodes;
}

struct InputBuffer {
	buf: [u8; 80],
	len: usize
}

static mut INPUT_BUFFER: InputBuffer = InputBuffer{buf: [0; 80], len: 0};

pub fn on_ps2_kb_input() {
	let code = io::inb(0x60);
	let char: Option<u8> = match SCANCODES_CHARS[code as usize] {
		b'.' => None,
		_ => Some(SCANCODES_CHARS[code as usize])
	};
	if let Some(char) = char {
		unsafe {
			if INPUT_BUFFER.len < 80 {
				INPUT_BUFFER.buf[INPUT_BUFFER.len] = char;
				INPUT_BUFFER.len += 1;
			}
		}
	} else if code == BACKSPACE{
		unsafe {
			if INPUT_BUFFER.len > 0 {
				INPUT_BUFFER.len -= 1;
			}
		}
	} else if code == ENTER {
		unsafe { INPUT_BUFFER.len = 0; }
	} else if code == ESCAPE {
		unsafe { INPUT_BUFFER.len = 0; }
	} else {
		return;
	}
	unsafe {
		let slice_to_print = &INPUT_BUFFER.buf[0..INPUT_BUFFER.len];
		let s =  str::from_utf8_unchecked(slice_to_print);
		pr_debug!("{}", s);
	}
}
