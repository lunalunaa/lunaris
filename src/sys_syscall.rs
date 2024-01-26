use crate::{
    syscall::{
        EXCEPTION_CODE_CREATE, EXCEPTION_CODE_EXIT, EXCEPTION_CODE_MY_PARENT_TID,
        EXCEPTION_CODE_MY_TID, EXCEPTION_CODE_YIELD,
    },
    tasks::{Context, Task, CPU_GLOBAL},
    term::TERM_GLOBAL,
};
use aarch64_cpu as cpu;
use core::arch::global_asm;
use cpu::registers::{Readable, ESR_EL1};

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
    pub x19: u64,
    pub x20: u64,
    pub x21: u64,
    pub x22: u64,
    pub x23: u64,
    pub x24: u64,
    pub x25: u64,
    pub x26: u64,
    pub x27: u64,
    pub x28: u64,
    pub x29: u64,
    pub x30: u64,
    pub xzr: u64,
    pub esr: u64,
    pub spsr: u64,
    pub elr: u64,
    pub elr_dup: u64,
}

unsafe fn kcreate(task: &mut Task) -> i8 {
    let trap_frame = &*task.trap_frame.unwrap();
    return CPU_GLOBAL.scheduler.create(
        trap_frame.x0 as usize,
        Some(task.id),
        core::mem::transmute(trap_frame.x1),
    );
}

unsafe fn kmy_tid(task: &mut Task) -> i8 {
    TERM_GLOBAL.put_slice(b"kernel: kmy_tid called\n");
    TERM_GLOBAL.flush_all();
    TERM_GLOBAL.put_slice(b"kernel: my id is: ");
    TERM_GLOBAL.put_u_dec(task.id as usize);
    TERM_GLOBAL.put_slice_flush(b"\n");
    return task.id as i8;
}

unsafe fn kmy_parent_tid(task: &mut Task) -> i8 {
    if task.parent.is_some() {
        return task.parent.unwrap() as i8;
    } else {
        return -1;
    }
}

unsafe fn kyield(task: &mut Task) -> i8 {
    task.trap_frame = None;
    return 0;
}

unsafe fn kexit(_task: &mut Task) -> i8 {
    CPU_GLOBAL.scheduler.curr_active().take();
    return 0;
}

#[no_mangle]
pub extern "C" fn get_kernel_sp() -> u64 {
    unsafe { CPU_GLOBAL.scheduler.curr_active().unwrap().kernel_sp }
}

/// Look up which syscall to excute and excute it
#[no_mangle]
pub unsafe extern "C" fn syscall(exception_frame: *mut ExceptionFrame) -> ! {
    extern "C" {
        fn __switch_to_scheduler(old_context: *mut Context, new_context: *mut Context) -> !;
    }

    TERM_GLOBAL.put_slice_flush(b"syscall received\n");

    let task = CPU_GLOBAL.scheduler.curr_active_mut().unwrap();
    task.trap_frame = Some(exception_frame);
    let exception_num = cpu::registers::ESR_EL1.read(ESR_EL1::ISS);
    let ret = match exception_num {
        EXCEPTION_CODE_MY_TID => kmy_tid(task),
        EXCEPTION_CODE_CREATE => kcreate(task),
        EXCEPTION_CODE_MY_PARENT_TID => kmy_parent_tid(task),
        EXCEPTION_CODE_EXIT => kexit(task),
        EXCEPTION_CODE_YIELD => kyield(task),
        _ => todo!(),
    };

    let frame = unsafe { &mut *exception_frame };
    frame.x0 = ret as u64;

    __switch_to_scheduler(
        task.context.as_mut().unwrap() as *mut Context,
        &mut CPU_GLOBAL.context as *mut Context,
    );
}

// todo: kernel stack needs to be restored
