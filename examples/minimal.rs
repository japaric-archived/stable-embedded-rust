//! Minimal no-std binary

#![no_main]
#![no_std]

#[macro_use(exception, main)]
extern crate glue;

// User must declare what's the entry point for this program
// (stable workaround for `#[lang = "start"]` / `#[lang = "termination"]`)
main!(main);

fn main() -> ! {
    loop {}
}

// User must declare a default exception handler
// (stable workaround for `#[linkage = "weak"]`
exception!(default_handler, dh);

fn dh() {
    loop {}
}
