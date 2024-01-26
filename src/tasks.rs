use core::arch::global_asm;

use aarch64_cpu as cpu;
use cpu::registers::Writeable;
use derive_more::Constructor;
use heapless::{binary_heap::Max, BinaryHeap};

use crate::{boot::el0_setup, sys_syscall::ExceptionFrame, term::TERM_GLOBAL};

global_asm!(include_str!("switch.S"));

const TASK_SIZE: usize = 50;
const OUT_OF_DESCRIPTORS: i8 = -2;
const PER_TASK_KERNEL_STACK_SIZE: u64 = 0x400;
const USER_STACK_SIZE: u64 = 0x800;
const KERNEL_STACK_START: u64 = 0x20000;
const USER_STACK_START: u64 = 0x50000;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum TaskRunState {
    Active,
    Ready,
    Exited,
    SendBlocked,
    ReceiveBlocked,
    ReplyBlocked,
    EventBlocked,
}

pub enum Request {}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Context {
    x19: u64,
    x20: u64,
    x21: u64,
    x22: u64,
    x23: u64,
    x24: u64,
    x25: u64,
    x26: u64,
    x27: u64,
    x28: u64,
    x29: u64,
    x30: u64,
    sp: u64,
}

impl Context {
    const fn new() -> Self {
        Self {
            x19: 0,
            x20: 0,
            x21: 0,
            x22: 0,
            x23: 0,
            x24: 0,
            x25: 0,
            x26: 0,
            x27: 0,
            x28: 0,
            x29: 0,
            x30: 0,
            sp: 0,
        }
    }
}

#[derive(Eq, Constructor, Debug)]
pub struct Task {
    pub id: u8,
    pub priority: usize,
    pub cnt: usize,
    pub parent: Option<&'static Task>,
    pub run_state: TaskRunState,
    pub trap_frame: Option<*mut ExceptionFrame>,
    pub context: Option<Context>,
    pub kernel_sp: u64,
    pub starting_sp: u64,
    pub fn_ptr: fn() -> !,
}

static mut KERNEL_SP: u64 = 0x1000;

impl Ord for Task {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| self.cnt.cmp(&other.cnt))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

pub struct CPU {
    pub scheduler: Scheduler,
    pub context: Context,
}

impl CPU {
    const fn init() -> Self {
        Self {
            scheduler: Scheduler::new(),
            context: Context::new(),
        }
    }
}

pub static mut CPU_GLOBAL: CPU = CPU::init();

pub struct Scheduler {
    active_task: Option<Task>,
    ready_queue: BinaryHeap<Task, Max, TASK_SIZE>,
    cnt: usize,
    context: Context,
}

impl Scheduler {
    pub const fn new() -> Self {
        Scheduler {
            active_task: None,
            ready_queue: BinaryHeap::new(),
            cnt: usize::max_value(),
            context: Context::new(),
        }
    }

    pub const fn task_num(&self) -> u64 {
        (usize::max_value() - self.cnt) as u64
    }

    pub fn curr_active(&self) -> Option<&Task> {
        self.active_task.as_ref()
    }

    pub fn curr_active_mut(&mut self) -> Option<&mut Task> {
        self.active_task.as_mut()
    }

    pub fn create(
        &mut self,
        priority: usize,
        parent: Option<&'static Task>,
        fn_ptr: fn() -> !,
    ) -> i8 {
        self.cnt -= 1;
        let id = self.ready_queue.len();
        let task = Task {
            id: id as u8,
            priority,
            cnt: self.cnt,
            parent,
            run_state: TaskRunState::Ready,
            trap_frame: None,
            context: None,
            kernel_sp: KERNEL_STACK_START - self.task_num() * PER_TASK_KERNEL_STACK_SIZE,
            starting_sp: USER_STACK_START - self.task_num() * USER_STACK_SIZE,
            fn_ptr,
        };

        if self.ready_queue.push(task).is_ok() {
            return id as i8;
        } else {
            return OUT_OF_DESCRIPTORS;
        }
    }

    pub fn push(&mut self, mut task: Task) -> Result<(), Task> {
        task.cnt -= 1;
        self.ready_queue.push(task)
    }

    /// Check the priority of the current running task and the task to be scheduled.
    pub fn schedule(&mut self) -> Option<Task> {
        return self.ready_queue.pop();
    }

    pub fn activate(&mut self, mut task: Task) {
        extern "C" {
            fn __syscall_ret();
            fn __switch_to_task(old_context: *mut Context, new_context: *mut Context);
        }

        let task_starting_sp = task.starting_sp;

        if task.trap_frame.is_some() {
            unsafe {
                let frame_ptr = task.trap_frame.unwrap();
                el0_setup((*frame_ptr).elr, frame_ptr as u64);
                self.active_task = Some(task);
                __syscall_ret();
            }
        } else {
            unsafe {
                let task_starting_sp = task.starting_sp;
                el0_setup(task.fn_ptr as u64, task_starting_sp);
                if task.context.is_none() {
                    let context = Context::new();
                    task.context = Some(context);
                    self.active_task = Some(task);

                    TERM_GLOBAL.put_slice_flush(b"task activated!\n");

                    __switch_to_task(
                        &mut CPU_GLOBAL.context as *mut Context,
                        self.active_task.as_mut().unwrap().context.as_mut().unwrap()
                            as *mut Context,
                    );

                    // syscall returns here
                    // reschedule the task by making it inactive
                    let task = self.active_task.take();
                    self.push(task.unwrap()).unwrap();
                }
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Some(task) = self.schedule() {
                self.activate(task);
                unsafe { TERM_GLOBAL.put_slice_flush(b"switched to scheduler") };
            } else {
                cpu::asm::wfe(); // wait for event
            }
        }
    }
}
