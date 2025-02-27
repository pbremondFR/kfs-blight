#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;

#[macro_use]
mod screen;

use screen::Screen;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let mut screen = Screen::new();

    for i in 0..4 {
        pr_debug!(screen, "DEBUG MESSAGE {}!\n", i);
        pr_info!(screen, "INFO MESSAGE {}!\n", i);
        pr_warn!(screen, "WARN MESSAGE {}!\n", i);
        pr_error!(screen, "ERROR MESSAGE {}!\n", i);
    }

    loop {}
}
