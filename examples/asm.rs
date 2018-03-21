//! Stable assembly via external assembly file

#![no_main]
#![no_std]

#[macro_use(exception, main)]
extern crate glue;
extern crate sh;

use core::fmt::Write;

main!(main);

fn main() -> ! {
    writeln!(sh::stdout().unwrap(), "Hello, world!").unwrap();

    loop {}
}

exception!(default_handler, dh);

fn dh() {
    loop {}
}
