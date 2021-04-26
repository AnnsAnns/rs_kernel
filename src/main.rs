#![no_std]
#![no_main]

mod vga_buffer;

use vga_buffer::Writer;
use core::{panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer::new();

    loop {
        writer.write_string("Poggers Moment ");
    }
}