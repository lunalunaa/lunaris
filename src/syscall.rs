use core::arch::asm;

pub const EXCEPTION_CODE_CREATE: u64 = 1;
pub const EXCEPTION_CODE_MY_TID: u64 = 2;
pub const EXCEPTION_CODE_MY_PARENT_TID: u64 = 3;
pub const EXCEPTION_CODE_YIELD: u64 = 4;
pub const EXCEPTION_CODE_EXIT: u64 = 5;

pub fn Create(priority: usize, func: fn() -> !) -> i8 {
    let mut ret: i8;
    unsafe {
        asm!("svc {}", const EXCEPTION_CODE_CREATE, in("x0") priority, in("x1") func, lateout("x0") ret);
    }
    return ret;
}

pub fn MyTid() -> i8 {
    let mut ret: i8;
    unsafe {
        asm!("svc {}", const EXCEPTION_CODE_MY_TID, out("x0") ret);
    }
    return ret;
}

pub fn MyParentTid() -> i8 {
    let mut ret: i8;
    unsafe {
        asm!("svc {}", const EXCEPTION_CODE_MY_PARENT_TID, out("x0") ret);
    }
    return ret;
}

pub fn Yield() {
    unsafe {
        asm!("svc {}", const EXCEPTION_CODE_YIELD);
    }
}

pub fn Exit() -> ! {
    unsafe {
        asm!("svc {}", const EXCEPTION_CODE_EXIT);
    }

    loop {}
}
