#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(format_args_nl)]
#![feature(core_io_borrowed_buf)]

use core::panic::PanicInfo;
use core::arch::asm;

#[macro_use]
mod screen;
mod gdt;
mod stack_dump;
mod io;
mod kb_scancodes;
mod microshell;
mod fixed_string;

use screen::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // pr_error!("KERNEL PANICK!!!");
    pr_error!("{}", info);
    loop {}
}

const LOGO_42: &str = r#"              @@@@@@@@     @@@@@@@ @@@@@@@@
            @@@@@@@@       @@@@@   @@@@@@@@
          @@@@@@@@         @@@     @@@@@@@@
        @@@@@@@@           @       @@@@@@@@
      @@@@@@@@                    @@@@@@@@
    @@@@@@@@                    @@@@@@@@
   @@@@@@@@                   @@@@@@@@
 @@@@@@@@                   @@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@   @@@@@@@@       @
@@@@@@@@@@@@@@@@@@@@@@@@   @@@@@@@@     @@@
@@@@@@@@@@@@@@@@@@@@@@@@   @@@@@@@@   @@@@@
                @@@@@@@@   @@@@@@@@ @@@@@@@
                @@@@@@@@
                @@@@@@@@
                @@@@@@@@                   "#;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let test_stack: [u8; 8] = [b'H', b'e', b'l', b'l', b'o', b' ', b'm', b'8'];
    // Setup GDT
    unsafe {
        gdt::write_gdt_entry(0, 0, 0, 0);
        gdt::write_gdt_entry(1, 0xffff, gdt::GDT_ACCESS_CODE_PL0, gdt::GDT_SEG_GRANULAR_FLAGS);
        gdt::write_gdt_entry(2, 0xffff, gdt::GDT_ACCESS_DATA_PL0, gdt::GDT_SEG_GRANULAR_FLAGS);
        gdt::write_gdt_entry(3, 0xffff, gdt::GDT_ACCESS_STACK_PL0, gdt::GDT_SEG_GRANULAR_FLAGS);
        gdt::write_gdt_entry(4, 0xffff, gdt::GDT_ACCESS_CODE_PL3, gdt::GDT_SEG_GRANULAR_FLAGS);
        gdt::write_gdt_entry(5, 0xffff, gdt::GDT_ACCESS_DATA_PL3, gdt::GDT_SEG_GRANULAR_FLAGS);
        gdt::write_gdt_entry(6, 0xffff, gdt::GDT_ACCESS_STACK_PL3, gdt::GDT_SEG_GRANULAR_FLAGS);
        gdt::reload_gdt(7);
    }
    microshell::init_shell();

    for line in LOGO_42.split('\n') {
        pr_info!("{}", line);
    }
    for i in 0..1 {
        pr_debug!("DEBUG MESSAGE {}!", i);
        pr_info!("INFO MESSAGE {}!", i);
        pr_warn!("WARN MESSAGE {}!", i);
        printkln!(LogLevel::Error, "ERROR MESSAGE {}!", i);
    }
    pr_debug!("Address of test string: 0x{:08x}", &raw const test_stack as u32);
    stack_dump::stack_dump(128);

    let mut ebp: usize;
    let mut esp: usize;
    unsafe {
        asm!("mov {:e}, ebp", out(reg) ebp);
        asm!("mov {:e}, esp", out(reg) esp);
    }
    pr_debug!("esp: 0x{:08x}, ebp: 0x{:08x}", esp, ebp);
    for align in [4, 8, 16] {
        pr_debug!("{:2} bytes alignment: ESP={:5}, EBP={:5}", align, esp % align == 0, ebp % align == 0);
    }

    loop {
        let ps2_status = io::inb(0x64);
        if ps2_status & 1 == 1 {
            kb_scancodes::on_ps2_kb_input();
        }
    }
}

#[no_mangle]
pub extern "cdecl" fn reboot() -> ! {
    // Snippet of code from OsDev. Use the 8042 keyboard controller to pulse the CPU reset pin.
    let mut good: u8 = 0x02;
    while good & 0x02 != 0 {
        good = io::inb(0x64);
    }
    io::outb(0x64, 0xFE);
    // Should be unreachable beyond this point, if for some reason the reboot doesn't work,
    // just halt the CPU
    unsafe { core::arch::asm!("hlt"); }
    // Infinite loop to show rust that we're never exiting this function
    loop {}
}
