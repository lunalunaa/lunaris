#![no_main]
#![no_std]
#![feature(asm_const)]

use core::panic::PanicInfo;

use syscall::MyTid;
use term::TERM_GLOBAL;
mod boot;
mod setup;
mod sys_syscall;
mod syscall;
mod tasks;
mod term;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        TERM_GLOBAL.put_slice_flush(b"panicked!\n");
        TERM_GLOBAL.put_slice_flush(b"file = ");
        TERM_GLOBAL.put_slice_flush(info.location().unwrap().file().as_bytes());
        TERM_GLOBAL.put_slice_flush(b"\nline = ");
        TERM_GLOBAL.put_u_dec_flush(info.location().unwrap().line() as usize);
        TERM_GLOBAL.put_slice_flush(b"\n");
    }

    loop {}
}

fn main() -> ! {
    unsafe {
        TERM_GLOBAL.put_slice_flush(b"I am the first task\n");
        TERM_GLOBAL.put_slice_flush(b"My tid is: ");
        let tid = MyTid();
        TERM_GLOBAL.put_u_dec_flush(tid as usize);
        TERM_GLOBAL.put_slice_flush(b"\n");
        TERM_GLOBAL.put_slice_flush(b"system call returned!\n");
    }

    loop {}
}
