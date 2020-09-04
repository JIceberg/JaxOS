#![no_std]                              // don't link the Rust standard library
#![no_main]                             // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate rlibc;

mod vga_buffer;
mod serial;

use core::panic::PanicInfo; // panic implementation

/// This function is called on a panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

/// This is the panic handler in test mode.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// Closes the QEMU system through the use of exit codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
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

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests{}", tests.len(), ".");
    for test in tests {
        // run tests using our custom Testable trait
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[passed]");
    }
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