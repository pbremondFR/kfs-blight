#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(core_intrinsics)]

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
        pr_debug!("DEBUG MESSAGE {}!\n", i);
        pr_info!("INFO MESSAGE {}!\n", i);
        pr_warn!("WARN MESSAGE {}!\n", i);
        printk!(LogLevel::Error, "ERROR MESSAGE {}!\n", i);
    }

    loop {}
}
