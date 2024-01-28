#![no_main]
#![no_std]
#![feature(asm_const)]
#![allow(unused)]

mod asm;
mod boot;
mod setup;
mod sys_syscall;
mod syscall;
mod tasks;
mod term;

use syscall::{Create, Exit, MyParentTid, MyTid, Yield};

fn other() -> ! {
    let tid = MyTid();
    let parent_tid = MyParentTid();

    println!("my task id: {}", tid);
    println!("my parent id: {}", parent_tid);

    Yield();
    println!("my task id: {}", tid);
    println!("my parent id: {}", parent_tid);

    Exit();
}

fn main() -> ! {
    let task_1 = Create(0, other);
    println!("Created: {}", task_1);

    let task_2 = Create(0, other);
    println!("Created: {}", task_2);

    let task_3 = Create(2, other);
    println!("Created: {}", task_3);

    let task_4 = Create(2, other);
    println!("Created: {}", task_4);

    println!("First User Task: exiting");

    Exit();
}
