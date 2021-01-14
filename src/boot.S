# https://riscv.org/technical/specifications/
# At least one mhartid is 0 from Privileged Spec 3.15, so we will use
# that to handle all the work
# We don't currently need virtual memory so set satp to 0
# _start is linked in the linker script as the entry
# Reason for norelax for gp:
# https://www.sifive.com/blog/all-aboard-part-3-linker-relaxation-in-riscv-toolchain
# We don't have the gp yet so we can't relax something we don't have yet

.option norvc

.section .text.init
.global _start
_start:
    csrr        t0, mhartid
    bnez        t0, 4f

    csrw	satp, zero
    
.option push
.option norelax
    la		gp, _global_pointer
.option pop
    # Set stack
    la     sp, _stack_end
    # Enable interrupts
    li		t0, (0b11 << 11) | (1 << 7) | (1 << 3)
    csrw	mstatus, t0
    # Jump to kernel_init when we mret
    la		t1, kernel_init
    csrw	mepc, t1
    la		t2, asm_trap_vector
    csrw	mtvec, t2
    li		t3, (1 << 3) | (1 << 7) | (1 << 11)
    csrw	mie, t3
    la		ra, 4f
    mret
asm_trap_vector:
    mret
4:
    wfi
    j 4b
