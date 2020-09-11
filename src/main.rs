#![no_std]                              // don't link the Rust standard library
#![no_main]                             // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(jax_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate rlibc;

use jax_os::println;
use core::panic::PanicInfo; // panic implementation

/// This function is called on a panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

/// Panic handler when in test mode.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    jax_os::test_panic(info)
}

#[no_mangle]    // don't mangle the name of this function since it is `_start`
pub extern "C" fn _start() -> ! {
    println!("Hello, world{} Welcome to {}{}", "!", "JaxOS", ".");

    #[cfg(test)]
    test_main();
    
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}