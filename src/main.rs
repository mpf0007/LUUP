#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#[warn(non_snake_case)]
use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle] // don't mangle the name of this function
             // this function is the entry point, since the linker looks for a function
             // named `_start` by default

pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
