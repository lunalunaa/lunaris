use core::arch::global_asm;

global_asm!(include_str!("boot.S"));
global_asm!(include_str!("exception.S"));
global_asm!(include_str!("switch.S"));