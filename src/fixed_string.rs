pub struct FixedString<const N: usize> {
	buf: [u8; N],
	size: usize,
}

impl<const N: usize> FixedString<N> {
	pub fn new() -> Self {
		Self{ buf: [0; N], size: 0 }
	}

	pub fn len(&self) -> usize {
		self.size
	}

	pub fn as_str(&self) -> Result<&str, core::str::Utf8Error> {
		core::str::from_utf8(&self.buf[..self.size])
	}

	pub unsafe fn as_str_unchecked(&self) -> &str {
		unsafe { core::str::from_utf8_unchecked(&self.buf[..self.size]) }
	}
}

impl<const N: usize> core::fmt::Write for FixedString<N> {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		let bytes = s.as_bytes();
		if self.size + bytes.len() > N {
			return Err(core::fmt::Error);
		}
		self.buf[self.size..self.size + bytes.len()].copy_from_slice(bytes);
		self.size += bytes.len();
		Ok(())
	}
}
