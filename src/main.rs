#![no_main]
#![no_std]

use aarch64_cpu::asm;
use core::arch::global_asm;

global_asm!(include_str!("boot.S"));

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe()
    }
}

mod setup;

#[no_mangle]
extern "C" fn kmain() -> ! {
    wait_forever()
}
