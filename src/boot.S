// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2021-2022 Andre Richter <andre.o.richter@gmail.com>

//--------------------------------------------------------------------------------------------------
// Definitions
//--------------------------------------------------------------------------------------------------

// Load the address of a symbol into a register, PC-relative.
//
// The symbol must lie within +/- 4 GiB of the Program Counter.
//
// # Resources
//
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_REL register, symbol
	adrp	\register, \symbol
	add	\register, \register, #:lo12:\symbol
.endm

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
.section .text._start

//------------------------------------------------------------------------------
// fn _start()
//------------------------------------------------------------------------------
_start:
	// Only proceed on the boot core. Park it otherwise.
	mrs   x1, mpidr_el1
    and   x1, x1, #3
    cmp   x1, #0
    bne   .L_parking_loop

	// If execution reaches here, it is the boot core.

	/*=============================================================*/
    /*      Enable FP/SIMD at EL1                                  */
    /*=============================================================*/
    mov x0, #3 << 20
    msr cpacr_el1, x0           /* Enable FP/SIMD at EL1 */

	// are we already in EL3?
    mrs  x9, CurrentEL
    cmp x9, #12
    beq drop_el1_secure

    // are we already in EL1
    cmp x9, #4
    beq el1_secure

    // otherwise, switch from EL2 to EL1
    mov x10, #2147483648
    msr hcr_el2, x10

    mov x11, #709
    msr spsr_el2, x11

    adr x12, el1_secure
    msr elr_el2, x12

    ADR_REL	x0, __boot_core_stack_end_exclusive
    msr SP_EL1, x0 // set the stack pointer for EL1

    eret

drop_el1_secure:

/* Try drop from el3 to el1 secure */

/*=============================================================*/
/*      Initialize sctlr_el1                                   */
/*=============================================================*/
mov x0, xzr
orr x0, x0, #(1 << 29)          /* Checking http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0500d/CIHDIEBD.html */
orr x0, x0, #(1 << 28)          /* Bits 29,28,23,22,20,11 should be 1 (res1 on documentation) */
orr x0, x0, #(1 << 23)
orr x0, x0, #(1 << 22)
orr x0, x0, #(1 << 20)
orr x0, x0, #(1 << 11)
msr sctlr_el1, x0

/*=============================================================*/
/*      Initialize scr_el3                                     */
/*=============================================================*/
mrs x0, scr_el3
orr x0, x0, #(1<<10)        /* Lower EL is 64bits */
msr scr_el3, x0

/*=============================================================*/
/*      Initialize spsr_el3                                    */
/*=============================================================*/
mov x0, xzr
mov x0, #0b00101            /* EL1 */
orr x0, x0, #(1 << 8)       /* Enable SError and External Abort. */
orr x0, x0, #(1 << 7)       /* IRQ interrupt Process state mask. */
orr x0, x0, #(1 << 6)       /* FIQ interrupt Process state mask. */
msr spsr_el3, x0

/*=============================================================*/
/*      Initialize elr_el3                                     */
/*=============================================================*/
adr x0, el1_secure
msr elr_el3, x0
ADR_REL	x0, __boot_core_stack_end_exclusive
msr SP_EL1, x0 // set the stack pointer for EL1

eret

el1_secure:
    	// Initialize DRAM.
    msr SPSel, #1
	ADR_REL	x0, __bss_start
	ADR_REL x1, __bss_end_exclusive

.L_bss_init_loop:
	cmp	x0, x1
	b.eq	.L_prepare_rust
	stp	xzr, xzr, [x0], #16
	b	.L_bss_init_loop

	// Prepare the jump to Rust code.
.L_prepare_rust:
	// Set the stack pointer.
	ADR_REL	x0, __boot_core_stack_end_exclusive
	mov	sp, x0

    bl _kmain

	// Infinitely wait for events (aka "park the core").
.L_parking_loop:
	wfe
	b	.L_parking_loop

.size	_start, . - _start
.type	_start, function
.global	_start
