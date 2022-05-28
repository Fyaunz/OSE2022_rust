# Risc-V Rust OS

<https://osblog.stephenmarz.com/ch0.html>
<https://os.phil-opp.com/>
<https://github.com/sgmarz/osblog/blob/master/risc_v/src/lds/virt.lds>
<https://github.com/skyzh/core-os-riscv/blob/master/kernel/src/uart.rs>
<https://docs.rust-embedded.org/book/start/qemu.html>

UART
<https://www.lammertbies.nl/comm/info/serial-uart>

## Questions

- Why use mret in setup
- How to avoid race-conditions in UART/ Kernel

### Answered

- Why align to 16?
  - `ALIGN(4096) tells the linker to align the current memory location (which is
       0x8000_0000 + text section + rodata section) to 4096 bytes. This is because our paging
       system's resolution is 4,096 bytes or 4 KiB.`
- >ram AT>ram?
- sdata .sbss
- use wfi?
  - Wait for interrupts

## GDB

<https://stackoverflow.com/questions/2420813/using-gdb-to-single-step-assembly-code-outside-specified-executable-causes-error>

- gdbtui. Or run gdb with the -tui switch. Or press C-x C-a after entering gdb.
- layout asm
- Press C-x s
- use si ni
- use gdb-multiarch!

## LLDB

Don't use it!
<https://lldb.llvm.org/use/map.html>

## RISC-V

<https://github.com/riscv/riscv-isa-manual/#readme>
<https://github.com/rust-embedded/riscv>
