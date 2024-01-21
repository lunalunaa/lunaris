#![no_main]
#![no_std]
#![allow(dead_code)]
#![feature(asm_const)]
mod setup;
mod syscall;
mod tasks;
mod term;

use aarch64_cpu::asm;
use core::{arch::global_asm, ptr::addr_of};
use panic_halt as _;
use term::Term;

#[cfg(feature = "default")]
global_asm!(include_str!("boot.S"));
#[cfg(feature = "lab")]
global_asm!(include_str!("boot_lab.S"));

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe()
    }
}

#[no_mangle]
extern "C" fn _kmain() -> ! {
    let mut term = Term::init();
    //term.put_u(unsafe { addr_of!(syscall::TRAP_FRAME) as usize });
    term.put_slice(b"hello\n");
    term.flush_all();
    wait_forever()
}
