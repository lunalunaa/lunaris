use crate::{main, syscall::MyTid, term::TERM_GLOBAL};
use aarch64_cpu as cpu;
use core::{arch::global_asm, cell::UnsafeCell};
use cpu::{
    asm,
    registers::{Writeable, HCR_EL2, SCR_EL3, SCTLR_EL1, SPSR_EL1, SPSR_EL2, SPSR_EL3},
};
use panic_halt as _;

#[cfg(feature = "default")]
global_asm!(include_str!("boot_alt.S"));
#[cfg(feature = "lab")]
global_asm!(include_str!("boot_lab.S"));

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe()
    }
}

// no use for this function for now
#[inline(always)]
pub fn el1_setup(boot_core_stack_end_exclusive: u64) {
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
    cpu::registers::ELR_EL2.set(el0_setup as u64);
    cpu::registers::SP_EL1.set(boot_core_stack_end_exclusive);
    cpu::registers::SP_EL0.set(boot_core_stack_end_exclusive - 0x5000);
    cpu::asm::eret();
}

#[inline(always)]
unsafe fn exception_setup() {
    extern "Rust" {
        static vector_table_start: UnsafeCell<()>;
    }

    cpu::registers::VBAR_EL1.set(vector_table_start.get() as u64);
    asm::barrier::isb(asm::barrier::SY);
}

#[inline(always)]
pub fn el0_setup(func: u64) {
    cpu::registers::SPSR_EL1.write(
        SPSR_EL1::A::Masked
            + SPSR_EL1::F::Masked
            + SPSR_EL1::M::EL0t
            + SPSR_EL1::I::Masked
            + SPSR_EL1::D::Masked,
    );
    cpu::registers::ELR_EL1.set(func);
    unsafe {
        exception_setup();
    }
    cpu::asm::eret();
}

#[no_mangle]
extern "C" fn _kmain(boot_core_stack_end_exclusive: u64) -> ! {
    unsafe { TERM_GLOBAL.put_slice(b"EL0 transition success\n") };
    cpu::registers::SP_EL0.set(boot_core_stack_end_exclusive - 5000);
    el0_setup(main as u64);
    loop {}
}
