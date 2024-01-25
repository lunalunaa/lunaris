use crate::{
    syscall::EXCEPTION_CODE_MY_TID,
    tasks::{Task, SCHEDULER_GLOBAL},
    term::TERM_GLOBAL,
};
use aarch64_cpu as cpu;
use core::arch::global_asm;
use cpu::registers::{Readable, ESR_EL1, SPSR_EL1};

use core::ptr;

global_asm!(include_str!("exception.S"));

#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExceptionFrame {
    pub x0: u64,
    pub x1: u64,
    pub x2: u64,
    pub x3: u64,
    pub x4: u64,
    pub x5: u64,
    pub x6: u64,
    pub x7: u64,
    pub x8: u64,
    pub x9: u64,
    pub x10: u64,
    pub x11: u64,
    pub x12: u64,
    pub x13: u64,
    pub x14: u64,
    pub x15: u64,
    pub x16: u64,
    pub x17: u64,
    pub x18: u64,
    pub x29: u64,
    pub x30: u64,
    pub xzr: u64,
    pub esr: u64,
    pub spsr: u64,
    pub elr: u64,
}

fn kcreate(task: &Task) -> i8 {
    todo!()
}

unsafe fn kmy_tid(task: &Task) -> i8 {
    TERM_GLOBAL.put_slice(b"my_tid called\n");
    TERM_GLOBAL.flush_all();
    TERM_GLOBAL.put_slice(b"my_id is: ");
    TERM_GLOBAL.put_u_dec(task.id as usize);
    TERM_GLOBAL.flush_all();
    return task.id as i8;
}

unsafe fn kmy_parent_tid(task: &Task) -> i8 {
    todo!()
}

unsafe fn kyield(task: &Task) -> i8 {
    todo!()
}

unsafe fn kexit(task: &Task) -> i8 {
    todo!()
}

/// Look up which syscall to excute and excute it
#[no_mangle]
pub unsafe extern "C" fn syscall(exception_frame: *mut ExceptionFrame) -> ! {
    if let Some(task) = SCHEDULER_GLOBAL.curr_active() {
        let exception_num = cpu::registers::ESR_EL1.read(ESR_EL1::ISS);

        let ret = match exception_num {
            EXCEPTION_CODE_MY_TID => kmy_tid(task),
            _ => todo!(),
        };
        let frame = unsafe { &mut *exception_frame };
        frame.x0 = ret as u64;
    } else {
    }

    loop {}
}
