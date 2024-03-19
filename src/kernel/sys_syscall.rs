use crate::kernel::{
    syscall::{
        EXCEPTION_CODE_CREATE, EXCEPTION_CODE_EXIT, EXCEPTION_CODE_MY_PARENT_TID,
        EXCEPTION_CODE_MY_TID, EXCEPTION_CODE_YIELD,
    },
    tasks::{Context, Task, TaskRunState, CPU_GLOBAL},
};
use aarch64_cpu as cpu;
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
    pub _elr_dup: u64,
}

unsafe fn kcreate(task: &mut Task) -> i8 {
    let trap_frame = &*task.trap_frame.unwrap();
    CPU_GLOBAL.scheduler.create(
        trap_frame.x0 as usize,
        Some(task.id),
        core::mem::transmute(trap_frame.x1),
    )
}

unsafe fn kmy_tid(task: &mut Task) -> i8 {
    task.id as i8
}

unsafe fn kmy_parent_tid(task: &mut Task) -> i8 {
    if let Some(parent) = task.parent {
        parent as i8
    } else {
        -1
    }
}

unsafe fn kyield(task: &mut Task) -> i8 {
    0
}

unsafe fn kexit(task: &mut Task) -> i8 {
    extern "C" {
        fn __switch_to_scheduler(old_context: *mut Context, new_context: *mut Context) -> !;
    }

    task.run_state = TaskRunState::Exited;

    let mut cpu_context = CPU_GLOBAL.context.lock();
    let cpu_context_ptr = &mut *cpu_context as *mut Context;
    core::mem::drop(cpu_context);

    // this is locked in syscall
    CPU_GLOBAL.scheduler.active_task.force_unlock();
    __switch_to_scheduler(
        task.context.as_mut().unwrap() as *mut Context,
        cpu_context_ptr,
    );
    0
}

#[no_mangle]
pub extern "C" fn get_kernel_sp() -> u64 {
    let active_task = CPU_GLOBAL.scheduler.active_task.lock();
    let ret = active_task.as_ref().unwrap().kernel_sp;
    core::mem::drop(active_task);
    ret
}

/// Look up which syscall to excute and excute it
#[no_mangle]
pub unsafe extern "C" fn syscall(exception_frame: *mut ExceptionFrame) -> ! {
    extern "C" {
        fn __switch_to_scheduler(old_context: *mut Context, new_context: *mut Context) -> !;
    }

    let mut task = CPU_GLOBAL.scheduler.active_task.lock();
    let exception_num = ESR_EL1.read(ESR_EL1::ISS);
    let task_ref = task.as_mut().unwrap();
    task_ref.trap_frame = Some(exception_frame);
    let ret = match exception_num {
        EXCEPTION_CODE_MY_TID => kmy_tid(task_ref),
        EXCEPTION_CODE_CREATE => kcreate(task_ref),
        EXCEPTION_CODE_MY_PARENT_TID => kmy_parent_tid(task_ref),
        EXCEPTION_CODE_EXIT => kexit(task_ref),
        EXCEPTION_CODE_YIELD => kyield(task_ref),
        _ => todo!(),
    };

    let frame = &mut *exception_frame;
    frame.x0 = ret as u64;

    let task_context = task.as_mut().unwrap().context.as_mut().unwrap() as *mut Context;
    let mut cpu_context = CPU_GLOBAL.context.lock();
    let cpu_context_ptr = &mut *cpu_context as *mut Context;

    core::mem::drop(cpu_context);
    core::mem::drop(task);

    __switch_to_scheduler(task_context, cpu_context_ptr);
}

// todo: kernel stack needs to be restored
