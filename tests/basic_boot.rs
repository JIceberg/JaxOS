#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(jax_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

/// Panic handling
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    jax_os::test_panic(info);
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

use jax_os::println;

#[test_case]
fn test_println() {
    println!("Output");
}

#[test_case]
fn loop_assertion() {
    let mut i = 0;
    while i < 100 {
        i += 1;
    }
    assert_eq!(100, i, "Index value {} failed to reach {}", i, 100);
}

#[test_case]
fn hash_assertion() {
    let x = 3;
    let y = x;
    assert_eq!(x, y, "{}'s hash did not match {}", y, x);
}