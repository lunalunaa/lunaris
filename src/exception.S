// Typical exception vector table code.
.section .text

.align 11

vector_table_start:

.org 0x000
curr_el_sp0_sync:
    b __syscall_handler

.org 0x080
	b __syscall_handler
.org 0x100
	b __syscall_handler
.org 0x180
	b __syscall_handler

// Current exception level with SP_ELx, x > 0.
.org 0x200
	b __syscall_handler
.org 0x280
	b __syscall_handler
.org 0x300
	b __syscall_handler
.org 0x380
	b __syscall_handler

// Lower exception level, AArch64
.org 0x400 // this is the one
	b __syscall_handler
.org 0x480
	b __syscall_handler
.org 0x500
	b __syscall_handler
.org 0x580
	b __syscall_handler

// Lower exception level, AArch32
.org 0x600
	b __syscall_handler
.org 0x680
	b __syscall_handler
.org 0x700
	b __syscall_handler
.org 0x780
	b __syscall_handler
.org 0x800


__syscall_handler:
    // we are using user stack rn

    msr SPSel, #0 // let the kernel use SP_EL0
    sub sp, sp, #288
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x19, [sp, #144]
    stp x20, x21, [sp, #160]
    stp x22, x23, [sp, #176]
    stp x24, x25, [sp, #192]
    stp x26, x27, [sp, #208]
    stp x28, x29, [sp, #224]
    stp x30, xzr, [sp, #240]

    mrs x0, ESR_EL1
    mrs x1, SPSR_EL1
    stp x0, x1, [sp, #256]
    mrs x0, ELR_EL1
    stp x0, x0, [sp, #272]
    bl get_kernel_sp
    mov x1, x0
    mov x0, sp
    msr SPSel, #1
    mov sp, x1
    bl syscall

__syscall_ret:
    msr SPSel, #0
    ldp x0, x1, [sp, #256]
    msr ESR_EL1, x0
    msr SPSR_EL1, x1
    ldp x0, x1, [sp, #0]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x19, [sp, #144]
    ldp x20, x21, [sp, #160]
    ldp x22, x23, [sp, #176]
    ldp x24, x25, [sp, #192]
    ldp x26, x27, [sp, #208]
    ldp x28, x29, [sp, #224]
    ldp x30, xzr, [sp, #240]
    add sp, sp, #288
    isb
    eret

.global vector_table_start
.global __syscall_ret
.size	vector_table_start, . - vector_table_start
