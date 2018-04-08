#![feature(lang_items)]
#![no_std]

use core::ptr;

/// Macro use to override an exception handler
///
/// Usage: `exception!(exception_name, path::to::handler)`
///
/// Valid exception names:
///
/// - `nmi`
/// - `hard_fault`
/// - `mem_manage`
/// - `bus_fault`
/// - `usage_fault`
/// - `svcall`
/// - `pendsv`
/// - `systick`
#[macro_export]
macro_rules! exception {
    ($exception: ident, $path: path) => {
        #[no_mangle]
        pub extern "C" fn $exception() {
            let f: fn() = $path;
            f()
        }
    };
}

#[macro_export]
macro_rules! main {
    ($path: path) => {
        #[export_name = "main"]
        #[no_mangle]
        pub extern "C" fn __impl_main() -> ! {
            let f: fn() -> ! = $path;
            f()
        }
    };
}

#[link(name = "c")]
extern "C" {
    fn nmi();
    fn hard_fault();
    fn mem_manage();
    fn bus_fault();
    fn usage_fault();
    fn svcall();
    fn pendsv();
    fn systick();
}

#[doc(hidden)]
#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Option<unsafe extern "C" fn()>; 14] = [
    Some(nmi),
    Some(hard_fault),
    Some(mem_manage),
    Some(bus_fault),
    Some(usage_fault),
    None,
    None,
    None,
    None,
    Some(svcall),
    None,
    None,
    Some(pendsv),
    Some(systick),
];

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        fn main() -> !; // we can pick any signature we want

        static mut _sbss: u32;
        static mut _ebss: u32;

        static mut _sdata: u32;
        static mut _edata: u32;
        static _sidata: u32;
    }

    zero_bss(&mut _sbss, &mut _ebss);
    initialize_data(&mut _sdata, &mut _edata, &_sidata);

    main()
}

unsafe fn zero_bss(sbss: *mut u32, ebss: *mut u32) {
    let mut bss = sbss;
    while bss < ebss {
        // NOTE(ptr::write*) to force aligned stores
        // NOTE(volatile) to prevent the compiler from optimizing this into `memclr`
        ptr::write_volatile(bss, 0);
        bss = bss.offset(1);
    }
}

unsafe fn initialize_data(sdata: *mut u32, edata: *mut u32, sidata: *const u32) {
    let mut data = sdata;
    let mut idata = sidata;
    while data < edata {
        // NOTE(ptr::{read,write}) to force aligned loads and stores
        ptr::write(data, ptr::read(idata));
        data = data.offset(1);
        idata = idata.offset(1);
    }
}

#[doc(hidden)]
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = reset;

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(
    _args: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _col: u32,
) -> ! {
    loop {}
}
