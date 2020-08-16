#![no_std]      // don't link the Rust standard library
#![no_main]     // disable all Rust-level entry points

extern crate rlibc;

mod vga_buffer;

use core::panic::PanicInfo; // panic implementation

/// This function is called on a panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &str = "Hello, World!";

#[no_mangle]    // don't mangle the name of this function since it is `_start`
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something(HELLO); // using the vga_buffer, we can send a string to the screen

    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}