unsafe extern "cdecl" {
	pub safe fn inb(port: u16) -> u8;
	pub safe fn inw(port: u16) -> u16;
	pub safe fn inl(port: u16) -> u32;
	pub safe fn outb(port: u16, value: u8);
	pub safe fn outw(port: u16, value: u16);
	pub safe fn outl(port: u16, value: u32);
}
