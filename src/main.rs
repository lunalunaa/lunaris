#![no_main]
#![no_std]
mod setup;
mod term;

use aarch64_cpu::asm;
use core::arch::global_asm;
use panic_halt as _;
use setup::{UARTLine, UART};

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
    let mut uart_console = UART::new(UARTLine::Console);
    uart_console.init();

    uart_console.println(b"\x1b[2J");
    uart_console.println(b"Hello");
    wait_forever()
}
