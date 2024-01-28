#![no_main]
#![no_std]
#![feature(asm_const)]
#![allow(unused)]

mod asm;
mod boot;
mod setup;
mod sys_syscall;
mod syscall;
mod tasks;
mod term;

use core::{arch::global_asm, panic::PanicInfo};
use syscall::{Create, Exit, MyParentTid, MyTid, Yield};
use term::TERM_GLOBAL;

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

fn other() -> ! {
    let tid = MyTid();
    let parent_tid = MyParentTid();
    unsafe {
        TERM_GLOBAL.put_slice_flush(b"my task id: ");
        TERM_GLOBAL.put_int_flush(tid);
        TERM_GLOBAL.put_slice_flush(b"\nmy parent id: ");
        TERM_GLOBAL.put_int_flush(parent_tid);
        TERM_GLOBAL.put_slice_flush(b"\n");

        Yield();
        TERM_GLOBAL.put_slice_flush(b"my task id: ");
        TERM_GLOBAL.put_int_flush(tid);
        TERM_GLOBAL.put_slice_flush(b"\nmy parent id: ");
        TERM_GLOBAL.put_int_flush(parent_tid);
        TERM_GLOBAL.put_slice_flush(b"\n");
    }

    Exit();
}

fn main() -> ! {
    let task_1 = Create(0, other);
    unsafe {
        TERM_GLOBAL.put_slice_flush(b"Created: ");
        TERM_GLOBAL.put_int(task_1);
        TERM_GLOBAL.put_slice_flush(b"\n");
    }
    let task_2 = Create(0, other);
    unsafe {
        TERM_GLOBAL.put_slice_flush(b"Created: ");
        TERM_GLOBAL.put_int_flush(task_2);
        TERM_GLOBAL.put_slice_flush(b"\n");
    }
    let task_3 = Create(2, other);
    unsafe {
        TERM_GLOBAL.put_slice_flush(b"Created: ");
        TERM_GLOBAL.put_int_flush(task_3);
        TERM_GLOBAL.put_slice(b"\n");
    }
    let task_4 = Create(2, other);
    unsafe {
        TERM_GLOBAL.put_slice_flush(b"Crearted: ");
        TERM_GLOBAL.put_int_flush(task_4);
        TERM_GLOBAL.put_slice_flush(b"\n");
        TERM_GLOBAL.put_slice_flush(b"First User Task: exiting\n");
    }

    Exit();
}
