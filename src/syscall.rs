use core::arch::asm;

pub const EXCEPTION_CODE_CREATE: u64 = 1;
pub const EXCEPTION_CODE_MY_TID: u64 = 2;
pub const EXCEPTION_CODE_MY_PARENT_TID: u64 = 3;
pub const EXCEPTION_CODE_YIELD: u64 = 4;
pub const EXCEPTION_CODE_EXIT: u64 = 5;

#[inline(never)]
pub fn Create(priority: usize, func: fn() -> !) -> i8 {
    let ret: i32;
    unsafe {
        asm!("svc {}", const EXCEPTION_CODE_CREATE, out("x0") ret);
    }
    return ret as i8;
}

#[inline(never)]
pub fn MyTid() -> i8 {
    let ret: i32;
    unsafe {
        asm!("svc {}", const EXCEPTION_CODE_MY_TID, out("x0") ret);
    }
    return ret as i8;
}

#[inline(never)]
pub fn MyParentTid() -> i8 {
    let ret: i32;
    unsafe {
        asm!("svc {}", const EXCEPTION_CODE_MY_PARENT_TID, out("x0") ret);
    }
    return ret as i8;
}

#[inline(never)]
pub fn Yield() {
    unsafe {
        asm!("svc {N}", N = const EXCEPTION_CODE_YIELD);
    }
}

#[inline(never)]
pub fn Exit() -> ! {
    unsafe {
        asm!("svc {N}", N = const EXCEPTION_CODE_EXIT);
    }

    loop {}
}
