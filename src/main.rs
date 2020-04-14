#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rustos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    rustos::init();

    println!("hello");
    println!("world");
    println!("some numbers: {} and {}", 1, 1.3376);

    println!("Running debug interrupt...");
    x86_64::instructions::interrupts::int3();
    println!("Done handling interrupt.");

    #[cfg(test)]
    test_main();
    rustos::halt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustos::halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}
