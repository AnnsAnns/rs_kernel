#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

pub mod interrupts;
mod vga_buffer;

use core::{panic::PanicInfo};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("PoggersOS starting ...");

    interrupts::init_idt();

    x86_64::instructions::interrupts::int3();

    loop {
        panic!("Computer machine broke")
    }
}