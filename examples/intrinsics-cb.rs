//! Ensure that linking both libgcc.a and compiler_builtins doesn't make things explode

// Unfortunately this does explode and poses a forward compatibility hazard

#![feature(compiler_builtins_lib)]
#![no_main]
#![no_std]

extern crate compiler_builtins;
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
