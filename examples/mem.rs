//! Verify that `memcpy` and friends work

// `memcpy` et al. are provided by libc.a instead of by the unstable compiler-builtins crate

#![no_main]
#![no_std]

#[macro_use(exception, main)]
extern crate glue;

use core::ptr;

main!(main);

fn main() -> ! {
    static mut A: [u32; 32] = [0; 32];
    static mut B: [u32; 32] = [0; 32];

    unsafe {
        ptr::write_bytes(&mut A, 0, A.len()); // memclr
        ptr::read_volatile(&A[0]);

        ptr::write_bytes(&mut B, 1, B.len()); // memset
        ptr::read_volatile(&B[0]);

        A.copy_from_slice(&B); // memcpy
        ptr::read_volatile(&A[0]);
    }

    loop {}
}

exception!(default_handler, dh);

fn dh() {
    loop {}
}
