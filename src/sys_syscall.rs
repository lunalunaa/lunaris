use core::arch::global_asm;

use crate::term::TERM_GLOBAL;

global_asm!(include_str!("exception.S"));

extern "C" {
    //fn switch(old_context: *const Kontext, new_context: *const Kontext, retval: u64);
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExceptionFrame {
    x0: u64,
    x1: u64,
    x2: u64,
    x3: u64,
    x4: u64,
    x5: u64,
    x6: u64,
    x7: u64,
    x8: u64,
    x9: u64,
    x10: u64,
    x11: u64,
    x12: u64,
    x13: u64,
    x14: u64,
    x15: u64,
    x16: u64,
    x17: u64,
    x18: u64,
    x29: u64,
    x30: u64,
    xzr: u64,
    esr: u64,
    spsr: u64,
}

fn kcreate() -> i8 {
    todo!()
}

fn kmy_tid() -> i8 {
    unsafe {
        TERM_GLOBAL.put_slice(b"exception success\n");
        TERM_GLOBAL.flush_all();
    }
    return 0;
}

fn kmy_parent_tid() -> i8 {
    todo!()
}

fn kyield() -> i8 {
    todo!()
}

fn kexit() -> i8 {
    todo!()
}

/// Look up which syscall to excute and excute it
#[no_mangle]
pub extern "C" fn syscall(exception_frame: *mut ExceptionFrame) -> ! {
    //let exception_num = cpu::registers::ESR_EL1.read(ESR_EL1::ISS);

    kmy_tid();

    loop {}
}
