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
    bne   exit

	// If execution reaches here, it is the boot core.

	    // are we already in EL1?
    mrs  x9, CurrentEL
    and  x9, x9, #8
    cbz  x9, el1_entry

    // otherwise, switch to EL1 by fake exception to return from
    ldr x10, =18446744071562067968
    msr hcr_el2, x10

    ldr x11, =709
    msr spsr_el2, x11

    adr x12, el1_entry
    msr elr_el2, x12

    eret

el1_entry:
    // configure processor and mmu
    ldr x13, =819268096
    msr sctlr_el1, x13

	// Initialize DRAM.
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

	// Jump to Rust code.
	b	_kmain

	// Infinitely wait for events (aka "park the core").
.L_parking_loop:
	wfe
	b	.L_parking_loop

exit:
    // shouldn't be reached
    wfi
    b  exit

.size	_start, . - _start
.type	_start, function
.global	_start
