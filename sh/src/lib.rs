//! Minimal implementation of `cortex-m-semihosting` used to show stable assembly

#![no_std]

use core::fmt;

extern "C" {
    fn syscall(nr: usize, args: usize) -> isize;
}

pub struct Stdout {
    fd: usize,
}

const OPEN: usize = 0x01;
const WRITE: usize = 0x05;

const W_TRUNC: usize = 4;

pub fn stdout() -> Result<Stdout, ()> {
    const TT: &[u8] = b":tt\0";

    let args = [TT.as_ptr() as usize, W_TRUNC, TT.len()];
    match unsafe { syscall(OPEN, args.as_ptr() as usize) } {
        -1 => Err(()),
        fd => Ok(Stdout { fd: fd as usize }),
    }
}

impl Stdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        let args = [self.fd, buf.as_ptr() as usize, buf.len()];
        match unsafe { syscall(WRITE, args.as_ptr() as usize) } {
            -1 => Err(()),
            // `n` is the number of bytes that were *not* written
            n => Ok(buf.len() - n as usize),
        }
    }

    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), ()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => return Err(()),
                Ok(n) => buf = &buf[n..],
                Err(()) => return Err(()),
            }
        }
        Ok(())
    }
}

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}
