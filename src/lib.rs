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

    for i in 0..26 {
        write!(screen, "Hello, {}!\n", i).expect("Write failed");
    }

    loop {}
}
