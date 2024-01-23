use aarch64_cpu as cpu;
use core::arch::asm;
use cpu::registers::{Readable, ESR_EL1};

use crate::tasks::TASK_QUEUE_GLOBAL;

const EXCEPTION_CODE_CREATE: u16 = 1;
const EXCEPTION_CODE_MY_TID: u16 = 2;
const EXCEPTION_CODE_MY_PARENT_TID: u16 = 3;
const EXCEPTION_CODE_YIELD: u16 = 4;
const EXCEPTION_CODE_EXIT: u16 = 5;

#[repr(C)]
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
    fp: u64,
    lr: u64,
    xzr: u64,
    esr: u64,
}

#[no_mangle]
pub extern "C" fn exception(exception_frame: *mut ExceptionFrame) {
    let exception_num = cpu::registers::ESR_EL1.read(ESR_EL1::ISS);

    todo!()
}

pub extern "C" fn Create(priority: usize, func: fn()) -> i8 {
    let mut ret: u64 = 0;
    unsafe {
        asm!("svc {N}", "mov {ret} x0",ret = out(reg) _, N = const EXCEPTION_CODE_CREATE);
    }

    return ret as i8;
}

pub extern "C" fn MyTid() -> i8 {
    let mut ret: u64 = 0;
    unsafe {
        asm!("svc {N}", "mov {ret} x0",ret = out(reg) _, N = const EXCEPTION_CODE_MY_TID);
    }

    return ret as i8;
}

pub extern "C" fn MyParentTid() -> i8 {
    let mut ret: u64 = 0;
    unsafe {
        asm!("svc {N}", "mov {ret} x0",ret = out(reg) _, N = const EXCEPTION_CODE_MY_TID);
    }

    return ret as i8;
}

pub extern "C" fn Yield() -> ! {
    unsafe {
        asm!("svc {N}", N = const EXCEPTION_CODE_YIELD);
    }

    loop {}
}

pub extern "C" fn Exit() -> ! {
    unsafe {
        asm!("svc {N}", N = const EXCEPTION_CODE_EXIT);
    }

    loop {}
}
