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
    failing_test();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn failing_test() {
    serial_print!("basic_panic::failing_test...\t");
    assert_eq!(1, 0);
}