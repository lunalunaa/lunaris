// Typical exception vector table code.
.section .text

.align 11

vector_table_start:

.org 0x000
curr_el_sp0_sync:
    b syscall_handler

.org 0x080
	b syscall_handler
.org 0x100
	b syscall_handler
.org 0x180
	b syscall_handler

// Current exception level with SP_ELx, x > 0.
.org 0x200
	b syscall_handler
.org 0x280
	b syscall_handler
.org 0x300
	b syscall_handler
.org 0x380
	b syscall_handler

// Lower exception level, AArch64
.org 0x400 // this is the one
	b syscall_handler
.org 0x480
	b syscall_handler
.org 0x500
	b syscall_handler
.org 0x580
	b syscall_handler

// Lower exception level, AArch32
.org 0x600
	b syscall_handler
.org 0x680
	b syscall_handler
.org 0x700
	b syscall_handler
.org 0x780
	b syscall_handler
.org 0x800


syscall_handler:
    // we are using user stack rn
    sub sp, sp, #192
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x29, [sp, #144]
    stp x30, xzr, [sp, #160]

    mrs x0, ESR_EL1
    mrs x1, SPSR_EL1
    stp x0, x1, [sp, #176]

    mov x0, sp
    msr SPSel, #1 // let the kernel use SP_EL1
    bl syscall


/*
syscall_ret:
    ldp x0, x1, [sp, #0]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x29, [sp, #144]
    ldp x30, xzr, [sp, #160]
    add sp, sp, #192
    eret

// only need to save callee saved registers?? because before we jump here (from rust code), the other registers will be saved automatically
sys_to_usr:
    sub sp, sp, #192
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x29, [sp, #144]
    stp x30, xzr, [sp, #160]

    mrs x0, ESR_EL1
    mrs x1, FAR_EL1
    stp x0, x1, [sp, #176]

    mov x0, sp
    msr SPSel, #1 // let the kernel use SP_EL1
    bl syscall
*/
.global vector_table_start
.size	vector_table_start, . - vector_table_start
