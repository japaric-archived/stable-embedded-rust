# `stable-embedded-rust`

> The closest we are to embedded no-std binaries on stable Rust

This is a minimal re-implementation of [`cortex-m-rt`] that replaces most of the unstable features
used there with stable counterparts. There's no loss in the main functionality provided by
[`cortex-m-rt`] but there's quite a drastic change in the interface presented to the end user cf.
the `examples` directory.

[`cortex-m-rt`]: https://docs.rs/cortex-m-rt/0.3.13/cortex_m_rt/

## What works

- Booting
- Initializing RAM (.bss+.data)
- Valid vector table (for simplicity, device specific interrupts have been omitted in this
  experiment)
- Individually overriding an exception handler
- Declaring a default exception handler
- Assembly via external assembly files

All that works without optimization (+ incr. comp and parallel codegen), with optimization and with
LTO.

For more details about how and why this differs from [`cortex-m-rt`] check the [embedonomicon].

[embedonomicon]: https://github.com/japaric/embedonomicon

## What doesn't work

This doesn't work on stable *yet*.

There are three (\*) unstable features that still tie this experiment to stable: the `panic_fmt`
lang item, the `compiler_builtins` crate and Xargo. We hope that we'll have stable replacement for
those by the time the 2018 edition is released.

(\*) There's a stable workaround for the unstable `compiler_builtins` crate and that's linking to
`libgcc.a` and `libc.a`. Unfortunately that workaround is not forward compatible with the idea of
having the compiler intrinsics being provided by the `core` crate, which is the most likely solution
to the existence of the unstable `compiler_builtins` crate. cf. `examples/{intrinsics,mem}-cb.rs`

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
