#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rustos::{exit_qemu, serial_print, serial_println, QemuExitCode};

fn should_fail() {
    serial_print!("should fail... ");
    assert_eq!(0, 1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests.", tests.len());
    for test in tests {
        test();
        serial_println!("[test did not panic");
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
}
