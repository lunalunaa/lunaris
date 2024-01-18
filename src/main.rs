#![no_main]
#![no_std]

use aarch64_cpu::asm;
use core::arch::global_asm;
use panic_halt as _;
use setup::{UARTLines, UART};
global_asm!(include_str!("boot.S"));

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

    uart_console.println("Hello");
    wait_forever()
}
