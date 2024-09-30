#![no_std]
#![no_main]

use core::cmp::min;
use core::panic::PanicInfo;
use multiboot2::{BootInformation, BootInformationHeader, FramebufferTag};

// #[allow(non_upper_case_globals)]
// #[allow(non_camel_case_types)]
// #[allow(non_snake_case)]
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn put_pixel(framebuffer_tag: &FramebufferTag, posx: u32, posy: u32, color: u32) {
    let bpp = framebuffer_tag.bpp() / 8;
    let pitch = framebuffer_tag.pitch();
    let framebuffer_address = (framebuffer_tag.address() + posx as u64 * bpp as u64 + posy as u64 * pitch as u64 ) as *mut u32;
    unsafe { *framebuffer_address = color };
}

#[no_mangle]
pub extern "C" fn kmain(mb_magic: u32, mbi_ptr: u32) -> ! {
    if mb_magic == multiboot2::MAGIC {
        let boot_info =
            unsafe { BootInformation::load(mbi_ptr as *const BootInformationHeader).unwrap() };
        let framebuffer_tag = boot_info.framebuffer_tag().unwrap().unwrap();
        let framebuffer_width = framebuffer_tag.width();
        let framebuffer_height = framebuffer_tag.height();

        for y in 0..framebuffer_height {
            for x in 0..framebuffer_width {
                let color = (min(x / (framebuffer_width/256),255) << 16) + (min(y/ (framebuffer_height/256),255) << 8) + 0;
                put_pixel(&framebuffer_tag, x, y, color);
            }
        }

        loop {}
    } else {
        panic!("Multiboot2 not supported!")
    }
}
