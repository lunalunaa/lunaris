use core::arch::asm;

const EXCEPTION_CODE_CREATE: u16 = 1;
const EXCEPTION_CODE_MY_TID: u16 = 2;
const EXCEPTION_CODE_MY_PARENT_TID: u16 = 3;
const EXCEPTION_CODE_YIELD: u16 = 4;
const EXCEPTION_CODE_EXIT: u16 = 5;

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
