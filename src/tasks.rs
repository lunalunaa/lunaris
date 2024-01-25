use derive_more::Constructor;
use heapless::{binary_heap::Max, BinaryHeap};

use crate::sys_syscall::ExceptionFrame;

const TASK_SIZE: usize = 50;
const OUT_OF_DESCRIPTORS: i8 = -2;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum TaskRunState {
    Active,
    Ready,
    Exited,
    SendBlocked,
    ReceiveBlocked,
    ReplyBlocked,
    EventBlocked,
}

enum Request {}

#[derive(Eq, Constructor, Debug)]
pub struct Task {
    id: u8,
    priority: usize,
    cnt: usize,
    parent: Option<&'static Task>,
    run_state: TaskRunState,
    trap_frame: Option<ExceptionFrame>,
    fn_ptr: fn(),
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
}

pub static mut TASK_QUEUE_GLOBAL: Scheduler = Scheduler::new();

impl Scheduler {
    pub const fn new() -> Self {
        Scheduler {
            active_task: None,
            ready_queue: BinaryHeap::new(),
            cnt: 0,
        }
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

    pub fn schedule(&mut self) -> Option<Task> {
        return self.ready_queue.pop();
    }

    pub fn activate(&mut self, task: &Task) -> Request {
        // do the context switching
        todo!()
    }

    pub fn handle(&mut self, request: Request) {
        // do the context switching
        todo!()
    }
}
