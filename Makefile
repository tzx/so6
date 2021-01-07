KERNEL = target/riscv64gc-unknown-none-elf/debug/so6
QEMU = qemu-system-riscv64

kernel: $(KERNEL)

clean:
	cargo clean

$(KERNEL):
	cargo build

QEMUOPTS = -machine virt -cpu rv64 -smp 1 -m 128M -nographic -bios none -kernel $(KERNEL)
QEMUGDB = -s -S -d int

qemu: $(KERNEL)
	$(QEMU) $(QEMUOPTS)

qemu-gdb: $(KERNEL)
	$(QEMU) $(QEMUOPTS) $(QEMUGDB)
