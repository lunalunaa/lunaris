use core::arch::asm;

use super::boot::wait_forever;

pub const EXCEPTION_CODE_CREATE: u64 = 1;
pub const EXCEPTION_CODE_MY_TID: u64 = 2;
pub const EXCEPTION_CODE_MY_PARENT_TID: u64 = 3;
pub const EXCEPTION_CODE_YIELD: u64 = 4;
pub const EXCEPTION_CODE_EXIT: u64 = 5;

#[allow(non_snake_case)]
pub fn Create(priority: usize, func: fn() -> !) -> i8 {
    let mut ret: i8;
    unsafe {
        asm!("svc {}", const EXCEPTION_CODE_CREATE, in("x0") priority, in("x1") func, lateout("x0") ret);
    }
    ret
}

#[allow(non_snake_case)]
pub fn MyTid() -> i8 {
    let mut ret: i8;
    unsafe {
        asm!("svc {}", const EXCEPTION_CODE_MY_TID, out("x0") ret);
    }
    ret
}

#[allow(non_snake_case)]
pub fn MyParentTid() -> i8 {
    let mut ret: i8;
    unsafe {
        asm!("svc {}", const EXCEPTION_CODE_MY_PARENT_TID, out("x0") ret);
    }
    ret
}

#[allow(non_snake_case)]
pub fn Yield() {
    unsafe {
        asm!("svc {}", const EXCEPTION_CODE_YIELD);
    }
}

#[allow(non_snake_case)]
pub fn Exit() -> ! {
    unsafe {
        asm!("svc {}", const EXCEPTION_CODE_EXIT);
    }

    wait_forever()
}
