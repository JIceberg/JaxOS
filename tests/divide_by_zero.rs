#![no_std]
#![no_main]

use core::panic::PanicInfo;
use jax_os::{QemuExitCode, exit_qemu, serial_println, serial_print};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[panicked]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_case();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn test_case() {
    serial_print!("divide_by_zero::test_case...\t");
    fn divide(x: i32, y: i32) -> i32 {
        return x / y;
    }
    let x = divide(1, 0);
    jax_os::print!("x: {}", x);
}