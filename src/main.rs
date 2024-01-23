#![no_main]
#![no_std]
#![allow(dead_code)]
#![feature(asm_const)]
mod setup;
mod syscall;
mod tasks;
mod term;

use aarch64_cpu as cpu;
use core::arch::global_asm;
use cpu::{
    asm,
    registers::{Readable, Writeable, HCR_EL2, SCTLR_EL1, SPSR_EL2},
};
use panic_halt as _;
use term::Term;

#[cfg(feature = "default")]
global_asm!(include_str!("boot.S"));
#[cfg(feature = "lab")]
global_asm!(include_str!("boot_lab.S"));

const EXCEPTION_VECTOR_TABLE_ADDR: u64 = 0x40000000;

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe()
    }
}

#[inline(always)]
fn el1_setup(boot_core_stack_end_exclusive: u64) {
    cpu::registers::HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);
    cpu::registers::SCTLR_EL1.write(
        SCTLR_EL1::NTWE::DontTrap
            + SCTLR_EL1::NTWI::DontTrap
            + SCTLR_EL1::UMA::DontTrap
            + SCTLR_EL1::M::Disable,
    );
    cpu::registers::SPSR_EL2.write(
        SPSR_EL2::A::Masked
            + SPSR_EL2::F::Masked
            + SPSR_EL2::M::EL1h
            + SPSR_EL2::I::Masked
            + SPSR_EL2::D::Masked,
    );
    cpu::registers::ELR_EL2.set(main as u64);
    cpu::registers::SP_EL1.set(boot_core_stack_end_exclusive);
    cpu::asm::eret();
}

fn exception_setup() {
    cpu::registers::VBAR_EL1.set(EXCEPTION_VECTOR_TABLE_ADDR);
}

fn main() -> ! {
    let mut term = Term::init();
    //term.put_u(unsafe { addr_of!(syscall::TRAP_FRAME) as usize });
    // term.put_u_hex(aarch64_cpu::registers::SP.get() as usize);
    term.put_slice(b"current EL: ");
    term.put_u_hex(cpu::registers::CurrentEL.get() as usize);
    term.put_slice(b"\n");
    term.flush_all();
    loop {}
}

#[no_mangle]
extern "C" fn _kmain(boot_core_stack_end_exclusive: u64) -> ! {
    el1_setup(boot_core_stack_end_exclusive);
    loop {}
}
