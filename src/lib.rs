#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(format_args_nl)]

use core::panic::PanicInfo;

#[macro_use]
mod screen;
mod gdt;

use screen::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pr_error!("KERNEL PANICK!!!");
    pr_error!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    unsafe {
        gdt::write_gdt_entry(0, 0, 0, 0);
        gdt::write_gdt_entry(1, 0xffff, gdt::GDT_ACCESS_CODE_PL0, gdt::GDT_SEG_GRANULAR_FLAGS);
        gdt::write_gdt_entry(2, 0xffff, gdt::GDT_ACCESS_DATA_PL0, gdt::GDT_SEG_GRANULAR_FLAGS);
        gdt::write_gdt_entry(3, 0xffff, gdt::GDT_ACCESS_STACK_PL0, gdt::GDT_SEG_GRANULAR_FLAGS);
        gdt::write_gdt_entry(4, 0xffff, gdt::GDT_ACCESS_CODE_PL3, gdt::GDT_SEG_GRANULAR_FLAGS);
        gdt::write_gdt_entry(5, 0xffff, gdt::GDT_ACCESS_DATA_PL3, gdt::GDT_SEG_GRANULAR_FLAGS);
        gdt::write_gdt_entry(6, 0xffff, gdt::GDT_ACCESS_STACK_PL3, gdt::GDT_SEG_GRANULAR_FLAGS);
        gdt::reload_gdt();
    }
    printk!(LogLevel::Info, "42\n");
    for i in 0..4 {
        pr_debug!("DEBUG MESSAGE {}!", i);
        pr_info!("INFO MESSAGE {}!", i);
        pr_warn!("WARN MESSAGE {}!", i);
        printk!(LogLevel::Error, "ERROR MESSAGE {}!", i);
    }

    // unsafe {
    //     let test_ptr = 0x0011_0000 as *mut u32;
    //     pr_debug!("Test pointer == {:#x}", *test_ptr);
    //     *test_ptr = 0x42424242u32;
    //     // core::ptr::write_volatile(test_ptr, 0x42424242u32);
    //     pr_debug!("Test pointer == {:#x}", *test_ptr);
    // }

    loop {}
}
