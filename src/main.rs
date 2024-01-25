#![no_main]
#![no_std]
#![allow(dead_code)]
#![feature(asm_const)]
mod boot;
mod setup;
mod sys_syscall;
mod syscall;
mod tasks;
mod term;
