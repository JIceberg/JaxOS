#![no_std]      // don't link the Rust standard library
#![no_main]     // disable all Rust-level entry points

extern crate rlibc;

mod vga_buffer;

use core::panic::PanicInfo; // panic implementation

/// This function is called on a panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[no_mangle]    // don't mangle the name of this function since it is `_start`
pub extern "C" fn _start() -> ! {
    let mut index = 0;
    
    while index <= 100 {
        println!("Hello, World! Loading... {}% complete.", index);
        vga_buffer::WRITER.lock().clear_row(22);
        index += 1;
    }

    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}