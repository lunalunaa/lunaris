#![no_main]
#![no_std]
#![feature(asm_const)]

use syscall::MyTid;
use term::TERM_GLOBAL;
mod boot;
mod setup;
mod sys_syscall;
mod syscall;
mod tasks;
mod term;

fn main() -> ! {
    unsafe {
        TERM_GLOBAL.put_slice(b"EL0 transition success\n");
        TERM_GLOBAL.put_slice(b"My tid is: ");
        let tid = MyTid();
        TERM_GLOBAL.put_u_dec(tid as usize);
        TERM_GLOBAL.put_slice_flush(b"\n");
        TERM_GLOBAL.flush_all();
    }

    loop {}
}
