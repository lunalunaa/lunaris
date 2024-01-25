use core::arch::asm;

pub const EXCEPTION_CODE_CREATE: u64 = 1;
pub const EXCEPTION_CODE_MY_TID: u64 = 2;
pub const EXCEPTION_CODE_MY_PARENT_TID: u64 = 3;
pub const EXCEPTION_CODE_YIELD: u64 = 4;
pub const EXCEPTION_CODE_EXIT: u64 = 5;

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
        asm!("svc {N}", "mov {ret}, x0", ret = out(reg) _, N = const EXCEPTION_CODE_MY_TID);
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
