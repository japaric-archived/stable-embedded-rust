//! Verify that operations that require compiler intrinsics work

// Intrinsics are provided by libgcc.a instead of by the unstable compiler-builtins crate

#![no_main]
#![no_std]

#[macro_use(exception, main)]
extern crate glue;

use core::ptr;

main!(main);

fn main() -> ! {
    let a = unsafe { ptr::read_volatile(0x2000_0000 as *const u64) };
    let b = unsafe { ptr::read_volatile(0x2000_0008 as *const u64) };

    let c = a * b;

    unsafe { ptr::write_volatile(0x2000_0010 as *mut u64, c) }

    loop {}
}

exception!(default_handler, dh);

fn dh() {
    loop {}
}
