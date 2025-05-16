#[repr(packed)]
pub struct GdtDescriptor {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    limit_granularity: u8,
    base_high: u8,
}

// In the access flags
const SEG_DESCTYPE_BIT: u8 = 0x04;
const SEG_PRES_BIT: u8 = 0x07;
const SEG_PRIV_SHIFT: u8 = 0x05;
// In the granularity flags
const SEG_LONG_BIT: u8 = 0x05;
const SEG_SIZE_BIT: u8 = 0x06;
const SEG_GRAN_BIT: u8 = 0x07;

// In the access flags
#[inline] pub const fn seg_desctype(x: u8) -> u8	{ (x & 1) << SEG_DESCTYPE_BIT }		// Descriptor type (0 for system, 1 for code/data)
#[inline] pub const fn seg_pres(x: u8) -> u8		{ (x & 1) << SEG_PRES_BIT }			// Present
#[inline] pub const fn seg_priv(x: u8) -> u8		{ (x & 0x03) << SEG_PRIV_SHIFT }	// Set privilege level (0 - 3)
// In the granularity flags
#[inline] pub const fn seg_long(x: u8) -> u8	{ (x & 1) << SEG_LONG_BIT }			// Long mode
#[inline] pub const fn seg_size(x: u8) -> u8	{ (x & 1) << SEG_SIZE_BIT }			// Size (0 for 16-bit, 1 for 32)
#[inline] pub const fn seg_gran(x: u8) -> u8	{ (x & 1) << SEG_GRAN_BIT }			// Granularity (0 for 1B - 1MB, 1 for 4KB - 4GB)

// Segment types
const SEG_DATA_RD: u8 = 0x00;        // Read-Only
const SEG_DATA_RDA: u8 = 0x01;       // Read-Only, accessed
const SEG_DATA_RDWR: u8 = 0x02;      // Read/Write
const SEG_DATA_RDWRA: u8 = 0x03;     // Read/Write, accessed
const SEG_DATA_RDEXPD: u8 = 0x04;    // Read-Only, expand-down
const SEG_DATA_RDEXPDA: u8 = 0x05;   // Read-Only, expand-down, accessed
const SEG_DATA_RDWREXPD: u8 = 0x06;  // Read/Write, expand-down
const SEG_DATA_RDWREXPDA: u8 = 0x07; // Read/Write, expand-down, accessed
const SEG_CODE_EX: u8 = 0x08;        // Execute-Only
const SEG_CODE_EXA: u8 = 0x09;       // Execute-Only, accessed
const SEG_CODE_EXRD: u8 = 0x0A;      // Execute/Read
const SEG_CODE_EXRDA: u8 = 0x0B;     // Execute/Read, accessed
const SEG_CODE_EXC: u8 = 0x0C;       // Execute-Only, conforming
const SEG_CODE_EXCA: u8 = 0x0D;      // Execute-Only, conforming, accessed
const SEG_CODE_EXRDC: u8 = 0x0E;     // Execute/Read, conforming
const SEG_CODE_EXRDCA: u8 = 0x0F;    // Execute/Read, conforming, accessed

pub const GDT_ACCESS_CODE_PL0: u8 =  (seg_desctype(1) | seg_pres(1) | seg_priv(0)) as u8 | SEG_CODE_EXRD;
pub const GDT_ACCESS_DATA_PL0: u8 =  (seg_desctype(1) | seg_pres(1) | seg_priv(0)) as u8 | SEG_DATA_RDWR;
// Expand down flags aren't needed because we're not doing segmented memory, but flat paging instead
pub const GDT_ACCESS_STACK_PL0: u8 = (seg_desctype(1) | seg_pres(1) | seg_priv(0)) as u8 | SEG_DATA_RDWR;
pub const GDT_ACCESS_CODE_PL3: u8 =  (seg_desctype(1) | seg_pres(1) | seg_priv(3)) as u8 | SEG_CODE_EXRD;
pub const GDT_ACCESS_DATA_PL3: u8 =  (seg_desctype(1) | seg_pres(1) | seg_priv(3)) as u8 | SEG_DATA_RDWR;
pub const GDT_ACCESS_STACK_PL3: u8 = (seg_desctype(1) | seg_pres(1) | seg_priv(3)) as u8 | SEG_DATA_RDWR;

// Segment flags for granularity (bits 52-55)
pub const GDT_SEG_GRANULAR_FLAGS: u8 = seg_long(0) | seg_size(1) | seg_gran(1);

pub unsafe fn write_gdt_entry(gdt_index: usize, limit: u32, access: u8, granular: u8) {
	let gdt_ptr = 0x0000_0800 as *mut GdtDescriptor;
	let entry = gdt_ptr.add(gdt_index);
	(*entry).limit_low = (limit & 0xffff) as u16;
	(*entry).base_low = 0;
	(*entry).base_middle = 0;
	(*entry).access = access;
	(*entry).limit_granularity = (((limit >> 16) & 0x0f) | granular as u32 & 0xf0) as u8;
	(*entry).base_high = 0;
}

extern "C" {
	pub fn reload_gdt();
}
