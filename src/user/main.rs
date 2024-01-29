#![no_main]
#![no_std]
#![feature(asm_const)]
#![allow(unused)]

use crate::{
    kernel::syscall::{Create, Exit, MyParentTid, MyTid, Yield},
    println,
};

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

pub fn main() -> ! {
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
