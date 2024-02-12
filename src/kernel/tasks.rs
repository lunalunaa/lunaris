use aarch64_cpu as cpu;
use derive_more::Constructor;
use heapless::{binary_heap::Max, BinaryHeap};
use spin::Mutex;

use super::{boot::el0_setup, sys_syscall::ExceptionFrame};

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
    const fn new() -> Context {
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
    cnt: usize,
    pub parent: Option<u8>,
    pub run_state: TaskRunState,
    pub trap_frame: Option<*mut ExceptionFrame>,
    pub context: Option<Context>,
    pub kernel_sp: u64,
    pub starting_sp: u64,
    pub fn_ptr: fn() -> !,
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

pub struct CPU {
    pub scheduler: Scheduler,
    pub context: Mutex<Context>,
}

impl CPU {
    const fn init() -> Self {
        Self {
            scheduler: Scheduler::new(),
            context: Mutex::new(Context::new()),
        }
    }
}

unsafe impl Send for CPU {}
unsafe impl Sync for CPU {}

pub static CPU_GLOBAL: CPU = CPU::init();

pub struct Scheduler {
    pub active_task: Mutex<Option<Task>>,
    ready_queue: Mutex<BinaryHeap<Task, Max, TASK_SIZE>>,
    cnt: usize,
    num_tasks: Mutex<u64>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Scheduler {
            active_task: Mutex::new(None),
            ready_queue: Mutex::new(BinaryHeap::new()),
            cnt: usize::max_value(),
            num_tasks: Mutex::new(0),
        }
    }

    pub fn task_num(&self) -> u64 {
        *self.num_tasks.lock()
    }

    pub fn create(&self, priority: usize, parent: Option<u8>, fn_ptr: fn() -> !) -> i8 {
        let mut num = self.num_tasks.lock();
        *num += 1;
        let task = Task {
            id: *num as u8,
            priority,
            cnt: self.cnt - 1,
            parent,
            run_state: TaskRunState::Ready,
            trap_frame: None,
            context: None,
            kernel_sp: KERNEL_STACK_START - *num * PER_TASK_KERNEL_STACK_SIZE,
            starting_sp: USER_STACK_START - *num * USER_STACK_SIZE,
            fn_ptr,
        };

        if self.push(task).is_ok() {
            *num as i8
        } else {
            OUT_OF_DESCRIPTORS
        }
    }

    pub fn push(&self, mut task: Task) -> Result<(), Task> {
        task.cnt -= 1;
        self.ready_queue.lock().push(task)
    }

    /// Check the priority of the current running task and the task to be scheduled.
    pub fn schedule(&self) -> Option<Task> {
        self.ready_queue.lock().pop()
    }

    pub unsafe fn activate(&self, mut task: Task) {
        extern "C" {
            fn __syscall_ret() -> !;
            fn __switch_to_task(old_context: *mut Context, new_context: *mut Context);
        }

        if task.run_state == TaskRunState::Exited {
            return;
        }

        if task.trap_frame.is_some() {
            let frame_ptr = task.trap_frame.unwrap();
            let frame = &*frame_ptr;
            el0_setup(frame.elr, frame_ptr as u64);
            let mut active_task = self.active_task.lock();
            *active_task = Some(task);
            core::mem::drop(active_task);
            __syscall_ret();
        }
        let task_starting_sp = task.starting_sp;
        el0_setup(task.fn_ptr as u64, task_starting_sp);
        if task.context.is_none() {
            let context = Context::new();
            task.context = Some(context);
        }
        let mut active_task = self.active_task.lock();
        *active_task = Some(task);
        let active_task_context =
            active_task.as_mut().unwrap().context.as_mut().unwrap() as *mut Context;
        let mut cpu_context = CPU_GLOBAL.context.lock();
        let cpu_context_ptr = &mut *cpu_context as *mut Context;
        core::mem::drop(active_task);
        core::mem::drop(cpu_context);
        __switch_to_task(cpu_context_ptr, active_task_context);
        self.reschedule();
    }

    pub fn reschedule(&self) {
        let task = self.active_task.lock().take();
        if let Some(task) = task {
            self.push(task).unwrap();
        }
    }

    pub unsafe fn run(&self) {
        loop {
            if let Some(task) = self.schedule() {
                self.activate(task);
            } else {
                cpu::asm::wfe(); // wait for event
            }
        }
    }
}
