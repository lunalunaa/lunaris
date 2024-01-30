use super::tasks::CPU_GLOBAL;
use crate::user::main;
use aarch64_cpu::{
    asm,
    registers::{Writeable, ELR_EL1, SPSR_EL1, SP_EL0, VBAR_EL1},
};
use core::cell::UnsafeCell;

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe()
    }
}

#[inline(always)]
unsafe fn exception_setup() {
    extern "Rust" {
        static vector_table_start: UnsafeCell<()>;
    }

    VBAR_EL1.set(vector_table_start.get() as u64);
    asm::barrier::isb(asm::barrier::SY);
}

/// Sets up the ELR_EL1 and SP_EL0
#[inline(always)]
pub fn el0_setup(func: u64, sp: u64) {
    SPSR_EL1.write(
        SPSR_EL1::A::Masked
            + SPSR_EL1::F::Masked
            + SPSR_EL1::M::EL0t
            + SPSR_EL1::I::Masked
            + SPSR_EL1::D::Masked,
    );
    ELR_EL1.set(func);
    SP_EL0.set(sp);
    unsafe {
        exception_setup();
    }
}

#[no_mangle]
extern "C" fn _kmain() -> ! {
    CPU_GLOBAL.scheduler.create(1, None, main::main);
    CPU_GLOBAL.scheduler.run();
    wait_forever()
}
