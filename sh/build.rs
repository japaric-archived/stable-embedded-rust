extern crate cc;

fn main() {
    cc::Build::new().file("asm.s").compile("asm");
}
