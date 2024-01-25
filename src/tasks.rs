use core::arch::asm;

use aarch64_cpu as cpu;
use derive_more::Constructor;
use heapless::{binary_heap::Max, BinaryHeap};

use crate::{boot::el0_setup, sys_syscall::ExceptionFrame};

const TASK_SIZE: usize = 50;
const OUT_OF_DESCRIPTORS: i8 = -2;

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
    pub context: Option<*mut Context>,
    pub fn_ptr: fn(),
}

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

pub struct Scheduler {
    active_task: Option<Task>,
    ready_queue: BinaryHeap<Task, Max, TASK_SIZE>,
    cnt: usize,
    context: Context,
}

pub static mut SCHEDULER_GLOBAL: Scheduler = Scheduler::new();

impl Scheduler {
    pub const fn new() -> Self {
        Scheduler {
            active_task: None,
            ready_queue: BinaryHeap::new(),
            cnt: 0,
            context: Context::new(),
        }
    }

    pub fn curr_active(&self) -> Option<&Task> {
        self.active_task.as_ref()
    }

    pub fn create(&mut self, priority: usize, parent: Option<&'static Task>, fn_ptr: fn()) -> i8 {
        self.cnt -= 1;
        let id = self.ready_queue.len();
        let task = Task {
            id: id as u8,
            priority,
            cnt: self.cnt,
            parent,
            run_state: TaskRunState::Ready,
            context: None,
            trap_frame: None,
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

    pub fn schedule(&mut self) -> Option<Task> {}

    pub fn activate(&mut self, mut task: Task) {
        extern "C" {
            fn __syscall_ret();
            fn __switch_to_task(old_context: *mut Context, new_context: *mut Context);
        }

        // todo

        if task.trap_frame.is_some() {
            unsafe {
                let frame = &*task.trap_frame.unwrap();
                el0_setup(frame.elr);
                self.active_task = Some(task);
                __syscall_ret();
            }
        } else {
            unsafe {
                el0_setup(task.fn_ptr as u64);
                __switch_to_task(self.context as *mut Context);
            }
        }
    }

    pub fn handle(&mut self, request: Request) {
        // do the context switching
        todo!()
    }

    pub fn run(&mut self) {
        loop {
            if let Some(task) = self.schedule() {
                self.activate(task);
            } else {
                cpu::asm::wfe(); // wait for event
            }
        }
    }
}
