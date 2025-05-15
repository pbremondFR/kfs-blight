#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(format_args_nl)]

use core::panic::PanicInfo;

#[macro_use]
mod screen;

use screen::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pr_error!("KERNEL PANICK!!!");
    pr_error!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    printk!(LogLevel::Info, "42\n");
    for i in 0..4 {
        pr_debug!("DEBUG MESSAGE {}!", i);
        pr_info!("INFO MESSAGE {}!", i);
        pr_warn!("WARN MESSAGE {}!", i);
        printk!(LogLevel::Error, "ERROR MESSAGE {}!", i);
    }

    unsafe {
        let test_ptr = 0x0011_0000 as *mut u32;
        pr_debug!("Test pointer == {:#x}", *test_ptr);
        *test_ptr = 0x42424242u32;
        // core::ptr::write_volatile(test_ptr, 0x42424242u32);
        pr_debug!("Test pointer == {:#x}", *test_ptr);
    }

    loop {}
}
