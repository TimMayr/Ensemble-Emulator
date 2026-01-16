# CPU registers

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/CPU_registers) | View [other pages](Special_AllPages.xhtml#CPU_registers)

The registers on the NES CPU are just like on the 6502. There is the accumulator, 2 indexes, a program counter, the stack pointer, and the status register. Unlike many CPU families, members do not have generic groups of registers like say, R0 through R7. 

## Contents

  * 1 Accumulator
  * 2 Indexes
  * 3 Program Counter
  * 4 Stack Pointer
  * 5 Status Register



### Accumulator

**A** is byte-wide and along with the [arithmetic logic unit](https://en.wikipedia.org/wiki/arithmetic_logic_unit "wikipedia:arithmetic logic unit") (ALU), supports using the status register for carrying, overflow detection, and so on. 

### Indexes

**X** and **Y** are byte-wide and used for several addressing modes. They can be used as loop counters easily, using INC/DEC and branch instructions. Not being the accumulator, they have limited addressing modes themselves when loading and saving. 

### Program Counter

The 2-byte program counter **PC** supports 65536 direct (unbanked) memory locations, however not all values are sent to the cartridge. It can be accessed either by allowing CPU's internal fetch logic increment the address bus, an interrupt (NMI, Reset, IRQ/BRQ), and using the RTS/JMP/JSR/Branch instructions. 

### Stack Pointer

**S** is byte-wide and can be accessed using interrupts, pulls, pushes, and transfers. It indexes into a 256-byte [stack](Stack.xhtml "Stack") at $0100-$01FF. 

### Status Register

**P** has 6 bits used by the ALU but is byte-wide. PHP, PLP, arithmetic, testing, and branch instructions can access this register. See [status flags](Status_flags.xhtml "Status flags"). 
