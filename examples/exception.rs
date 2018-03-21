//! Example of overriding an exception handler

#![no_main]
#![no_std]

#[macro_use(exception, main)]
extern crate glue;

use core::ptr;

main!(main);

fn main() -> ! {
    loop {}
}

exception!(default_handler, dh);

fn dh() {
    loop {}
}

exception!(hard_fault, hf);

fn hf() {
    // just to tell this handler apart from the other handlers
    unsafe {
        static mut X: u32 = 0;

        ptr::read_volatile(&X);
    }

    loop {}
}
