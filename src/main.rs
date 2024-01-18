#![no_main]
#![no_std]

use aarch64_cpu::asm;
use core::arch::global_asm;
use panic_halt as _;
use setup::{UARTLines, UART};

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

mod setup;

#[no_mangle]
extern "C" fn _kmain() -> ! {
    let mut uart_console = UART::new(UARTLines::Console);
    uart_console.init();

    uart_console.println("\x1b[2J");
    uart_console.println("Hello");
    wait_forever()
}
