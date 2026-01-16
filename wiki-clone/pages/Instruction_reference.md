# Instruction reference

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Instruction_reference) | View [other pages](Special_AllPages.xhtml#Instruction_reference)

Official 6502 Instructions  ADC | AND | ASL | BCC | BCS | BEQ | BIT | BMI | BNE | BPL | BRK | BVC | BVS | CLC  
---|---|---|---|---|---|---|---|---|---|---|---|---|---  
CLD | CLI | CLV | CMP | CPX | CPY | DEC | DEX | DEY | EOR | INC | INX | INY | JMP  
JSR | LDA | LDX | LDY | LSR | NOP | ORA | PHA | PHP | PLA | PLP | ROL | ROR | RTI  
RTS | SBC | SEC | SED | SEI | STA | STX | STY | TAX | TAY | TSX | TXA | TXS | TYA  
  
## Official instructions by type

Type  | Instructions   
---|---  
Access  | LDA | STA | LDX | STX | LDY | STY |  |   
Transfer  | TAX | TXA | TAY | TYA |  |  |  |   
Arithmetic  | ADC | SBC | INC | DEC | INX | DEX | INY | DEY  
Shift  | ASL | LSR | ROL | ROR |  |  |  |   
Bitwise  | AND | ORA | EOR | BIT |  |  |  |   
Compare  | CMP | CPX | CPY |  |  |  |  |   
Branch  | BCC | BCS | BEQ | BNE | BPL | BMI | BVC | BVS  
Jump  | JMP | JSR | RTS | BRK | RTI |  |  |   
Stack  | PHA | PLA | PHP | PLP | TXS | TSX |  |   
Flags  | CLC | SEC | CLI | SEI | CLD | SED | CLV |   
Other  | NOP |  |  |  |  |  |  |   
  
## Official instructions

### ADC - Add with Carry

`A = A + memory + C`

ADC adds the carry flag and a memory value to the accumulator. The carry flag is then set to the carry value coming out of bit 7, allowing values larger than 1 byte to be added together by carrying the 1 into the next byte's addition. This can also be thought of as unsigned overflow. It is common to clear carry with CLC before adding the first byte to ensure it is in a known state, avoiding an off-by-one error. The overflow flag indicates whether signed overflow or underflow occurred. This happens if both inputs are positive and the result is negative, or both are negative and the result is positive. 

Flag | New value | Notes   
---|---|---  
[C - Carry](Status_flags.xhtml#C "Status flags") | result > $FF | If the result overflowed past $FF (wrapping around), unsigned overflow occurred.   
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0 |   
[V - Overflow](Status_flags.xhtml#V "Status flags") | (result ^ A) & (result ^ memory) & $80 | If the result's sign is different from both A's and memory's, signed overflow (or underflow) occurred.   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7 |   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[#Immediate](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $69 | 2 | 2   
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $65 | 2 | 3   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $75 | 2 | 4   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $6D | 3 | 4   
[Absolute,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $7D | 3 | 4 (5 if page crossed)   
[Absolute,Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $79 | 3 | 4 (5 if page crossed)   
[(Indirect,X)](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $61 | 2 | 6   
[(Indirect),Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $71 | 2 | 5 (6 if page crossed)   
  
See also: SBC, CLC

* * *

### AND - Bitwise AND

`A = A & memory`

This ANDs a memory value and the accumulator, bit by bit. If both input bits are 1, the resulting bit is 1. Otherwise, it is 0. 

AND truth table  A | memory | result   
---|---|---  
0 | 0 | 0   
0 | 1 | 0   
1 | 0 | 0   
1 | 1 | 1   
Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[#Immediate](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $29 | 2 | 2   
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $25 | 2 | 3   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $35 | 2 | 4   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $2D | 3 | 4   
[Absolute,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $3D | 3 | 4 (5 if page crossed)   
[Absolute,Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $39 | 3 | 4 (5 if page crossed)   
[(Indirect,X)](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $21 | 2 | 6   
[(Indirect),Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $31 | 2 | 5 (6 if page crossed)   
  
See also: ORA, EOR

* * *

### ASL - Arithmetic Shift Left

`value = value << 1`, or visually: ` C <- [76543210] <- 0`

ASL shifts all of the bits of a memory value or the accumulator one position to the left, moving the value of each bit into the next bit. Bit 7 is shifted into the carry flag, and 0 is shifted into bit 0. This is equivalent to multiplying an unsigned value by 2, with carry indicating overflow. 

This is a read-modify-write instruction, meaning that its addressing modes that operate on memory first write the original value back to memory before the modified value. This extra write can matter if targeting a hardware register. 

Flag | New value   
---|---  
[C - Carry](Status_flags.xhtml#C "Status flags") | value bit 7   
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Accumulator](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $0A | 1 | 2   
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $06 | 2 | 5   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $16 | 2 | 6   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $0E | 3 | 6   
[Absolute,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $1E | 3 | 7   
  
See also: LSR, ROL, ROR

* * *

### BCC - Branch if Carry Clear

`PC = PC + 2 + memory (signed)`

If the carry flag is clear, BCC branches to a nearby location by adding the relative offset to the program counter. The offset is signed and has a range of [-128, 127] relative to the first byte _after_ the branch instruction. Branching further than that requires using a JMP instruction, instead, and branching over that JMP when carry is set with BCS. 

The carry flag has different meanings depending on the context. BCC can be used after a compare to branch if the register is less than the memory value, so it is sometimes called BLT for Branch if Less Than. It can also be used after SBC to branch if the unsigned value underflowed or after ADC to branch if it did _not_ overflow. 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Relative](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $90 | 2 | 2 (3 if branch taken, 4 if page crossed)*  
  
See also: BCS, JMP

* * *

### BCS - Branch if Carry Set

`PC = PC + 2 + memory (signed)`

If the carry flag is set, BCS branches to a nearby location by adding the branch offset to the program counter. The offset is signed and has a range of [-128, 127] relative to the first byte _after_ the branch instruction. Branching further than that requires using a JMP instruction, instead, and branching over that JMP when carry is clear with BCC. 

The carry flag has different meanings depending on the context. BCS can be used after a compare to branch if the register is greater than or equal to the memory value, so it is sometimes called BGE for Branch if Greater Than or Equal. It can also be used after ADC to branch if the unsigned value overflowed or after SBC to branch if it did _not_ underflow. 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Relative](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $B0 | 2 | 2 (3 if branch taken, 4 if page crossed)*  
  
See also: BCC, JMP

* * *

### BEQ - Branch if Equal

`PC = PC + 2 + memory (signed)`

If the zero flag is set, BEQ branches to a nearby location by adding the branch offset to the program counter. The offset is signed and has a range of [-128, 127] relative to the first byte _after_ the branch instruction. Branching further than that requires using a JMP instruction, instead, and branching over that JMP when zero is clear with BNE. 

Comparison uses this flag to indicate if the compared values are equal. All instructions that change A, X, or Y also implicitly set or clear the zero flag depending on whether the register becomes 0. 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Relative](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $F0 | 2 | 2 (3 if branch taken, 4 if page crossed)*  
  
See also: BNE, JMP

* * *

### BIT - Bit Test

`A & memory`

BIT modifies flags, but does not change memory or registers. The zero flag is set depending on the result of the accumulator AND memory value, effectively applying a bitmask and then checking if any bits are set. Bits 7 and 6 of the memory value are loaded directly into the negative and overflow flags, allowing them to be easily checked without having to load a mask into A. 

Because BIT only changes CPU flags, it is sometimes used to trigger the read side effects of a hardware register without clobbering any CPU registers, or even to waste cycles as a 3-cycle NOP. As an advanced trick, it is occasionally used to hide a 1- or 2-byte instruction in its operand that is only executed if jumped to directly, allowing two code paths to be interleaved. However, because the instruction in the operand is treated as an address from which to read, this carries risk of triggering side effects if it reads a hardware register. This trick can be useful when working under tight constraints on space, time, or register usage. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[V - Overflow](Status_flags.xhtml#V "Status flags") | memory bit 6   
[N - Negative](Status_flags.xhtml#N "Status flags") | memory bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Zero page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $24 | 2 | 3   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $2C | 3 | 4   
  
See also: AND

* * *

### BMI - Branch if Minus

`PC = PC + 2 + memory (signed)`

If the negative flag is set, BMI branches to a nearby location by adding the branch offset to the program counter. The offset is signed and has a range of [-128, 127] relative to the first byte _after_ the branch instruction. Branching further than that requires using a JMP instruction, instead, and branching over that JMP when negative is clear with BPL. 

All instructions that change A, X, or Y implicitly set or clear the negative flag based on bit 7 (the sign bit). 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Relative](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $30 | 2 | 2 (3 if branch taken, 4 if page crossed)*  
  
See also: BPL, JMP

* * *

### BNE - Branch if Not Equal

`PC = PC + 2 + memory (signed)`

If the zero flag is clear, BNE branches to a nearby location by adding the branch offset to the program counter. The offset is signed and has a range of [-128, 127] relative to the first byte _after_ the branch instruction. Branching further than that requires using a JMP instruction, instead, and branching over that JMP when negative is set with BEQ. 

Comparison uses this flag to indicate if the compared values are equal. All instructions that change A, X, or Y also implicitly set or clear the zero flag depending on whether the register becomes 0. 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Relative](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $D0 | 2 | 2 (3 if branch taken, 4 if page crossed)*  
  
See also: BEQ, JMP

* * *

### BPL - Branch if Plus

`PC = PC + 2 + memory (signed)`

If the negative flag is clear, BPL branches to a nearby location by adding the branch offset to the program counter. The offset is signed and has a range of [-128, 127] relative to the first byte _after_ the branch instruction. Branching further than that requires using a JMP instruction, instead, and branching over that JMP when negative is set with BMI. 

All instructions that change A, X, or Y implicitly set or clear the negative flag based on bit 7 (the sign bit). 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Relative](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $10 | 2 | 2 (3 if branch taken, 4 if page crossed)*  
  
See also: BMI, JMP

* * *

### BRK - Break (software IRQ)

`push PC + 2 to stack`  
`push NV11DIZC flags to stack`  
`PC = ($FFFE)`

BRK triggers an interrupt request (IRQ). IRQs are normally triggered by external hardware, and BRK is the only way to do it in software. Like a typical IRQ, it pushes the current program counter and processor flags to the stack, sets the interrupt disable flag, and jumps to the IRQ handler. Unlike a typical IRQ, it sets the break flag in the flags byte that is pushed to the stack (like PHP) and it triggers an interrupt even if the interrupt disable flag is set. Notably, the return address that is pushed to the stack skips the byte after the BRK opcode. For this reason, BRK is often considered a 2-byte instruction with an unused immediate. 

Unfortunately, a 6502 bug allows the BRK IRQ to be overridden by an NMI occurring at the same time. In this case, only the NMI handler is called; the IRQ handler is skipped. However, the break flag is still set in the flags byte pushed to the stack, so the NMI handler can detect that this occurred (albeit slowly) by checking this flag. 

Because BRK uses the value $00, any byte in a programmable ROM can be overwritten with a BRK instruction to send execution to an IRQ handler. This is useful for patching one-time programmable ROMs. BRK can also be used as a system call mechanism, and the unused byte can be used by software as an argument (although it is inconvenient to access). In the context of NES games, BRK is often most useful as a crash handler, where the unused program space is filled with $00 and the IRQ handler displays debugging information or otherwise handles the crash in a clean way. 

Flag | New value | Notes   
---|---|---  
[I - Interrupt disable](Status_flags.xhtml#I "Status flags") | 1 | This is set to 1 after the old flags are pushed to the stack. The effect of changing this flag is _not_ delayed.   
[B - Break](Status_flags.xhtml#B "Status flags") | Pushed as 1 | This flag exists only in the flags byte pushed to the stack, not as real state in the CPU.   
Addressing mode | Opcode | Bytes | Cycles | Notes   
---|---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $00 | 1 | 7 | Although BRK only uses 1 byte, its return address skips the following byte.   
[#Immediate](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $00 | 2 | 7 | Because BRK skips the following byte, it is often considered a 2-byte instruction.   
  
See also: RTI, PHP

* * *

### BVC - Branch if Overflow Clear

`PC = PC + 2 + memory (signed)`

If the overflow flag is clear, BVC branches to a nearby location by adding the branch offset to the program counter. The offset is signed and has a range of [-128, 127] relative to the first byte _after_ the branch instruction. Branching further than that requires using a JMP instruction, instead, and branching over that JMP when overflow is set with BVS. 

Unlike zero, negative, and even carry, overflow is modified by very few instructions. It is most often used with the BIT instruction, particularly for polling hardware registers. It is also sometimes used for signed overflow with ADC and SBC. The standard 6502 chip allows an external device to set overflow using a pin, enabling software to poll for that event, but this is not present on the NES' 2A03. 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Relative](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $50 | 2 | 2 (3 if branch taken, 4 if page crossed)*  
  
See also: BVS, JMP

* * *

### BVS - Branch if Overflow Set

`PC = PC + 2 + memory (signed)`

If the overflow flag is set, BVS branches to a nearby location by adding the branch offset to the program counter. The offset is signed and has a range of [-128, 127] relative to the first byte _after_ the branch instruction. Branching further than that requires using a JMP instruction, instead, and branching over that JMP when overflow is clear with BVC. 

Unlike zero, negative, and even carry, overflow is modified by very few instructions. It is most often used with the BIT instruction, particularly for polling hardware registers. It is also sometimes used for signed overflow with ADC and SBC. The standard 6502 chip allows an external device to set overflow using a pin, enabling software to poll for that event, but this is not present on the NES' 2A03 CPU. 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Relative](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $70 | 2 | 2 (3 if branch taken, 4 if page crossed)*  
  
See also: BVC, JMP

* * *

### CLC - Clear Carry

`C = 0`

CLC clears the carry flag. In particular, this is usually done before adding the low byte of a value with ADC to avoid adding an extra 1. 

Flag | New value   
---|---  
[C - Carry](Status_flags.xhtml#C "Status flags") | 0   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $18 | 1 | 2   
  
See also: SEC

* * *

### CLD - Clear Decimal

`D = 0`

CLD clears the decimal flag. The decimal flag normally controls whether binary-coded decimal mode (BCD) is enabled, but this mode is permanently disabled on the NES' 2A03 CPU. However, the flag itself still functions and can be used to store state. 

Flag | New value   
---|---  
[D - Decimal](Status_flags.xhtml#D "Status flags") | 0   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $D8 | 1 | 2   
  
See also: SED

* * *

### CLI - Clear Interrupt Disable

`I = 0`

CLI clears the interrupt disable flag, enabling the CPU to handle hardware IRQs. The effect of changing this flag is delayed one instruction because the flag is changed after IRQ is polled, allowing the next instruction to execute before any pending IRQ is detected and serviced. This flag has no effect on NMI, which (as the "non-maskable" name suggests) cannot be ignored by the CPU. 

Flag | New value | Notes   
---|---|---  
[I - Interrupt disable](Status_flags.xhtml#I "Status flags") | 0 | The effect of changing this flag is delayed 1 instruction.   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $58 | 1 | 2   
  
See also: SEI

* * *

### CLV - Clear Overflow

`V = 0`

CLV clears the overflow flag. There is no corresponding SEV instruction; instead, setting overflow is exposed on the 6502 CPU as a pin controlled by external hardware, and not exposed at all on the NES' 2A03 CPU. 

Flag | New value   
---|---  
[V - Overflow](Status_flags.xhtml#V "Status flags") | 0   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $B8 | 1 | 2   
  
* * *

### CMP - Compare A

`A - memory`

CMP compares A to a memory value, setting flags as appropriate but not modifying any registers. The comparison is implemented as a subtraction, setting carry if there is no borrow, zero if the result is 0, and negative if the result is negative. However, carry and zero are often most easily remembered as inequalities. 

Note that comparison does _not_ affect overflow. 

Flag | New value   
---|---  
[C - Carry](Status_flags.xhtml#C "Status flags") | A >= memory   
[Z - Zero](Status_flags.xhtml#Z "Status flags") | A == memory   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[#Immediate](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $C9 | 2 | 2   
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $C5 | 2 | 3   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $D5 | 2 | 4   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $CD | 3 | 4   
[Absolute,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $DD | 3 | 4 (5 if page crossed)   
[Absolute,Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $D9 | 3 | 4 (5 if page crossed)   
[(Indirect,X)](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $C1 | 2 | 6   
[(Indirect),Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $D1 | 2 | 5 (6 if page crossed)   
  
See also: CPX, CPY

* * *

### CPX - Compare X

`X - memory`

CPX compares X to a memory value, setting flags as appropriate but not modifying any registers. The comparison is implemented as a subtraction, setting carry if there is no borrow, zero if the result is 0, and negative if the result is negative. However, carry and zero are often most easily remembered as inequalities. 

Note that comparison does _not_ affect overflow. 

Flag | New value   
---|---  
[C - Carry](Status_flags.xhtml#C "Status flags") | X >= memory   
[Z - Zero](Status_flags.xhtml#Z "Status flags") | X == memory   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[#Immediate](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $E0 | 2 | 2   
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $E4 | 2 | 3   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $EC | 3 | 4   
  
See also: CMP, CPY

* * *

### CPY - Compare Y

`Y - memory`

CPY compares Y to a memory value, setting flags as appropriate but not modifying any registers. The comparison is implemented as a subtraction, setting carry if there is no borrow, zero if the result is 0, and negative if the result is negative. However, carry and zero are often most easily remembered as inequalities. 

Note that comparison does _not_ affect overflow. 

Flag | New value   
---|---  
[C - Carry](Status_flags.xhtml#C "Status flags") | Y >= memory   
[Z - Zero](Status_flags.xhtml#Z "Status flags") | Y == memory   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[#Immediate](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $C0 | 2 | 2   
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $C4 | 2 | 3   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $CC | 3 | 4   
  
See also: CMP, CPX

* * *

### DEC - Decrement Memory

`memory = memory - 1`

DEC subtracts 1 from a memory location. Notably, there is no version of this instruction for the accumulator; ADC or SBC must be used, instead. 

This is a read-modify-write instruction, meaning that it first writes the original value back to memory before the modified value. This extra write can matter if targeting a hardware register. 

Note that decrement does _not_ affect carry nor overflow. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $C6 | 2 | 5   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $D6 | 2 | 6   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $CE | 3 | 6   
[Absolute,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $DE | 3 | 7   
  
See also: INC, ADC, SBC

* * *

### DEX - Decrement X

`X = X - 1`

DEX subtracts 1 from the X register. Note that it does _not_ affect carry nor overflow. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $CA | 1 | 2   
  
See also: INX

* * *

### DEY - Decrement Y

`Y = Y - 1`

DEY subtracts 1 from the Y register. Note that it does _not_ affect carry nor overflow. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $88 | 1 | 2   
  
See also: INY

* * *

### EOR - Bitwise Exclusive OR

`A = A ^ memory`

EOR exclusive-ORs a memory value and the accumulator, bit by bit. If the input bits are different, the resulting bit is 1. If they are the same, it is 0. This operation is also known as XOR. 

6502 doesn't have bitwise NOT instruction, but using EOR with value $FF has the same behavior, inverting every bit of the other value. In fact, EOR can be thought of as NOT with a bitmask; all of the 1 bits in one value have the effect of inverting the corresponding bit in the other value, while 0 bits do nothing. 

EOR truth table  A | memory | result   
---|---|---  
0 | 0 | 0   
0 | 1 | 1   
1 | 0 | 1   
1 | 1 | 0   
Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[#Immediate](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $49 | 2 | 2   
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $45 | 2 | 3   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $55 | 2 | 4   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $4D | 3 | 4   
[Absolute,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $5D | 3 | 4 (5 if page crossed)   
[Absolute,Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $59 | 3 | 4 (5 if page crossed)   
[(Indirect,X)](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $41 | 2 | 6   
[(Indirect),Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $51 | 2 | 5 (6 if page crossed)   
  
See also: AND, ORA

* * *

### INC - Increment Memory

`memory = memory + 1`

INC adds 1 to a memory location. Notably, there is no version of this instruction for the accumulator; ADC or SBC must be used, instead. 

This is a read-modify-write instruction, meaning that it first writes the original value back to memory before the modified value. This extra write can matter if targeting a hardware register. 

Note that increment does _not_ affect carry nor overflow. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $E6 | 2 | 5   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $F6 | 2 | 6   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $EE | 3 | 6   
[Absolute,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $FE | 3 | 7   
  
See also: DEC, ADC, SBC

* * *

### INX - Increment X

`X = X + 1`

INX adds 1 to the X register. Note that it does _not_ affect carry nor overflow. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $E8 | 1 | 2   
  
See also: DEX

* * *

### INY - Increment Y

`Y = Y + 1`

INY adds 1 to the Y register. Note that it does _not_ affect carry nor overflow. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $C8 | 1 | 2   
  
See also: DEY

* * *

### JMP - Jump

`PC = memory`

JMP sets the program counter to a new value, allowing code to execute from a new location. If you wish to be able to return from that location, JSR should normally be used, instead. 

The indirect addressing mode uses the operand as a pointer, getting the new 2-byte program counter value from the specified address. Unfortunately, because of a CPU bug, if this 2-byte variable has an address ending in $FF and thus crosses a page, then the CPU fails to increment the page when reading the second byte and thus reads the wrong address. For example, JMP ($03FF) reads $03FF and _$0300_ instead of $0400. Care should be taken to ensure this variable does not cross a page. 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $4C | 3 | 3   
[(Indirect)](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $6C | 3 | 5   
  
See also: JSR

* * *

### JSR - Jump to Subroutine

`push PC + 2 to stack`  
`PC = memory`

JSR pushes the current program counter to the stack and then sets the program counter to a new value. This allows code to call a function and return with RTS back to the instruction after the JSR. 

Notably, the return address on the stack points 1 byte before the start of the next instruction, rather than directly at the instruction. This is because RTS increments the program counter before the next instruction is fetched. This differs from the return address pushed by interrupts and used by RTI, which points directly at the next instruction. 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $20 | 3 | 6   
  
See also: RTS, JMP, RTI

* * *

### LDA - Load A

`A = memory`

LDA loads a memory value into the accumulator. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[#Immediate](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $A9 | 2 | 2   
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $A5 | 2 | 3   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $B5 | 2 | 4   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $AD | 3 | 4   
[Absolute,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $BD | 3 | 4 (5 if page crossed)   
[Absolute,Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $B9 | 3 | 4 (5 if page crossed)   
[(Indirect,X)](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $A1 | 2 | 6   
[(Indirect),Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $B1 | 2 | 5 (6 if page crossed)   
  
See also: STA

* * *

### LDX - Load X

`X = memory`

LDX loads a memory value into the X register. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[#Immediate](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $A2 | 2 | 2   
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $A6 | 2 | 3   
[Zero Page,Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $B6 | 2 | 4   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $AE | 3 | 4   
[Absolute,Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $BE | 3 | 4 (5 if page crossed)   
  
See also: STX

* * *

### LDY - Load Y

`Y = memory`

LDX loads a memory value into the Y register. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[#Immediate](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $A0 | 2 | 2   
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $A4 | 2 | 3   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $B4 | 2 | 4   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $AC | 3 | 4   
[Absolute,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $BC | 3 | 4 (5 if page crossed)   
  
See also: STY

* * *

### LSR - Logical Shift Right

`value = value >> 1`, or visually: ` 0 -> [76543210] -> C`

LSR shifts all of the bits of a memory value or the accumulator one position to the right, moving the value of each bit into the next bit. 0 is shifted into bit 7, and bit 0 is shifted into the carry flag. This is equivalent to dividing an unsigned value by 2 and rounding down, with the remainder in carry. 

This is a read-modify-write instruction, meaning that its addressing modes that operate on memory first write the original value back to memory before the modified value. This extra write can matter if targeting a hardware register. 

Flag | New value   
---|---  
[C - Carry](Status_flags.xhtml#C "Status flags") | value bit 0   
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Accumulator](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $4A | 1 | 2   
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $46 | 2 | 5   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $56 | 2 | 6   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $4E | 3 | 6   
[Absolute,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $5E | 3 | 7   
  
See also: ASL, ROL, ROR

* * *

### NOP - No Operation

NOP has no effect; it merely wastes space and CPU cycles. This instruction can be useful when writing timed code to delay for a desired amount of time, as padding to ensure something does or does not cross a page, or to disable code in a binary. 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $EA | 1 | 2   
  
* * *

### ORA - Bitwise OR

`A = A | memory`

ORA inclusive-ORs a memory value and the accumulator, bit by bit. If either input bit is 1, the resulting bit is 1. Otherwise, it is 0. 

OR truth table  A | memory | result   
---|---|---  
0 | 0 | 0   
0 | 1 | 1   
1 | 0 | 1   
1 | 1 | 1   
Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[#Immediate](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $09 | 2 | 2   
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $05 | 2 | 3   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $15 | 2 | 4   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $0D | 3 | 4   
[Absolute,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $1D | 3 | 4 (5 if page crossed)   
[Absolute,Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $19 | 3 | 4 (5 if page crossed)   
[(Indirect,X)](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $01 | 2 | 6   
[(Indirect),Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $11 | 2 | 5 (6 if page crossed)   
  
See also: AND, EOR

* * *

### PHA - Push A

`($0100 + SP) = A`  
`SP = SP - 1`

PHA stores the value of A to the current stack position and then decrements the stack pointer. 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $48 | 1 | 3   
  
See also: PLA

* * *

### PHP - Push Processor Status

`($0100 + SP) = NV11DIZC`  
`SP = SP - 1`

PHP stores a byte to the stack containing the 6 status flags and B flag and then decrements the stack pointer. The B flag and extra bit are both pushed as 1. The bit order is NV1BDIZC (high to low). 

Flag | New value | Notes   
---|---|---  
[B - Break](Status_flags.xhtml#B "Status flags") | Pushed as 1 | This flag exists only in the flags byte pushed to the stack, not as real state in the CPU.   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $08 | 1 | 3   
  
See also: PLP

* * *

### PLA - Pull A

`SP = SP + 1`  
`A = ($0100 + SP)`

PLA increments the stack pointer and then loads the value at that stack position into A. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $68 | 1 | 4   
  
See also: PHA

* * *

### PLP - Pull Processor Status

`SP = SP + 1`  
`NVxxDIZC = ($0100 + SP)`

PLP increments the stack pointer and then loads the value at that stack position into the 6 status flags. The bit order is NVxxDIZC (high to low). The B flag and extra bit are ignored. Note that the effect of changing I is delayed one instruction because the flag is changed after IRQ is polled, delaying the effect until IRQ is polled in the next instruction like with CLI and SEI. 

Flag | New value | Notes   
---|---|---  
[C - Carry](Status_flags.xhtml#C "Status flags") | result bit 0 |   
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result bit 1 |   
[I - Interrupt disable](Status_flags.xhtml#I "Status flags") | result bit 2 | The effect of changing this flag is delayed 1 instruction.   
[D - Decimal](Status_flags.xhtml#D "Status flags") | result bit 3 |   
[V - Overflow](Status_flags.xhtml#V "Status flags") | result bit 6 |   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7 |   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $28 | 1 | 4   
  
See also: PHP

* * *

### ROL - Rotate Left

`value = value << 1 through C`, or visually: ` C <- [76543210] <- C`

ROL shifts a memory value or the accumulator to the left, moving the value of each bit into the next bit and treating the carry flag as though it is both above bit 7 and below bit 0. Specifically, the value in carry is shifted into bit 0, and bit 7 is shifted into carry. Rotating left 9 times simply returns the value and carry back to their original state. 

This is a read-modify-write instruction, meaning that its addressing modes that operate on memory first write the original value back to memory before the modified value. This extra write can matter if targeting a hardware register. 

Flag | New value   
---|---  
[C - Carry](Status_flags.xhtml#C "Status flags") | value bit 7   
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Accumulator](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $2A | 1 | 2   
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $26 | 2 | 5   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $36 | 2 | 6   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $2E | 3 | 6   
[Absolute,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $3E | 3 | 7   
  
See also: ROR, ASL, LSR

* * *

### ROR - Rotate Right

`value = value >> 1 through C`, or visually: ` C -> [76543210] -> C`

ROR shifts a memory value or the accumulator to the right, moving the value of each bit into the next bit and treating the carry flag as though it is both above bit 7 and below bit 0. Specifically, the value in carry is shifted into bit 7, and bit 0 is shifted into carry. Rotating right 9 times simply returns the value and carry back to their original state. 

This is a read-modify-write instruction, meaning that its addressing modes that operate on memory first write the original value back to memory before the modified value. This extra write can matter if targeting a hardware register. 

Flag | New value   
---|---  
[C - Carry](Status_flags.xhtml#C "Status flags") | value bit 0   
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Accumulator](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $6A | 1 | 2   
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $66 | 2 | 5   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $76 | 2 | 6   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $6E | 3 | 6   
[Absolute,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $7E | 3 | 7   
  
See also: ROL, ASL, LSR

* * *

### RTI - Return from Interrupt

`pull NVxxDIZC flags from stack`  
`pull PC from stack`

RTI returns from an interrupt handler, first pulling the 6 status flags from the stack and then pulling the new program counter. The flag pulling behaves like PLP except that changes to the interrupt disable flag apply immediately instead of being delayed 1 instruction. This is because the flags change before IRQs are polled for the instruction, not after. The PC pulling behaves like RTS except that the return address is the exact address of the next instruction instead of 1 byte before it. 

Flag | New value | Notes   
---|---|---  
[C - Carry](Status_flags.xhtml#C "Status flags") | pulled flags bit 0 |   
[Z - Zero](Status_flags.xhtml#Z "Status flags") | pulled flags bit 1 |   
[I - Interrupt disable](Status_flags.xhtml#I "Status flags") | pulled flags bit 2 | The effect of changing this flag is _not_ delayed.   
[D - Decimal](Status_flags.xhtml#D "Status flags") | pulled flags bit 3 |   
[V - Overflow](Status_flags.xhtml#V "Status flags") | pulled flags bit 6 |   
[N - Negative](Status_flags.xhtml#N "Status flags") | pulled flags bit 7 |   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $40 | 1 | 6   
  
See also: BRK, PLP, RTS

* * *

### RTS - Return from Subroutine

`pull PC from stack`  
`PC = PC + 1`

RTS pulls an address from the stack into the program counter and then increments the program counter. It is normally used at the end of a function to return to the instruction after the JSR that called the function. However, RTS is also sometimes used to implement jump tables (see [Jump table](Jump_table.xhtml "Jump table") and [RTS Trick](RTS_Trick.xhtml "RTS Trick")). 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $60 | 1 | 6   
  
See also: JSR, PLA

* * *

### SBC - Subtract with Carry

`A = A - memory - ~C`, or equivalently: `A = A + ~memory + C`

SBC subtracts a memory value and the bitwise NOT of carry from the accumulator. It does this by _adding_ the bitwise NOT of the memory value using ADC. This implementation detail explains the backward nature of carry; SBC subtracts 1 more when carry is _clear_ , not when it's set, and carry is cleared when it underflows and set otherwise. As with ADC, carry allows the borrow from one subtraction to be carried into the next subtraction, allowing subtraction of values larger than 1 byte. It is common to set carry with SEC before subtracting the first byte to ensure it is in a known state, avoiding an off-by-one error. 

Overflow works the same as with ADC, except with an inverted memory value. Therefore, overflow or underflow occur if the result's sign is different from A's and the same as the memory value's. 

Flag | New value | Notes   
---|---|---  
[C - Carry](Status_flags.xhtml#C "Status flags") | ~(result < $00) | If the result underflowed below $00 (wrapping around), unsigned underflow occurred.   
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0 |   
[V - Overflow](Status_flags.xhtml#V "Status flags") | (result ^ A) & (result ^ ~memory) & $80 | If result's sign is different from A's and the same as memory's, signed overflow (or underflow) occurred.   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7 |   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[#Immediate](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $E9 | 2 | 2   
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $E5 | 2 | 3   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $F5 | 2 | 4   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $ED | 3 | 4   
[Absolute,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $FD | 3 | 4 (5 if page crossed)   
[Absolute,Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $F9 | 3 | 4 (5 if page crossed)   
[(Indirect,X)](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $E1 | 2 | 6   
[(Indirect),Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $F1 | 2 | 5 (6 if page crossed)   
  
See also: ADC, SEC

* * *

### SEC - Set Carry

`C = 1`

SEC sets the carry flag. In particular, this is usually done before subtracting the low byte of a value with SBC to avoid subtracting an extra 1. 

Flag | New value   
---|---  
[C - Carry](Status_flags.xhtml#C "Status flags") | 1   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $38 | 1 | 2   
  
See also: CLC

* * *

### SED - Set Decimal

`D = 1`

SED sets the decimal flag. The decimal flag normally controls whether binary-coded decimal mode (BCD) is enabled, but this mode is permanently disabled on the NES' 2A03 CPU. However, the flag itself still functions and can be used to store state. 

Flag | New value   
---|---  
[D - Decimal](Status_flags.xhtml#D "Status flags") | 1   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $F8 | 1 | 2   
  
See also: CLD

* * *

### SEI - Set Interrupt Disable

`I = 1`

SEI sets the interrupt disable flag, preventing the CPU from handling hardware IRQs. The effect of changing this flag is delayed one instruction because the flag is changed after IRQ is polled, allowing an IRQ to be serviced between this and the next instruction if the flag was previously 0. 

Flag | New value | Notes   
---|---|---  
[I - Interrupt disable](Status_flags.xhtml#I "Status flags") | 1 | The effect of changing this flag is delayed 1 instruction.   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $78 | 1 | 2   
  
See also: CLI

* * *

### STA - Store A

`memory = A`

STA stores the accumulator value into memory. 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $85 | 2 | 3   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $95 | 2 | 4   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $8D | 3 | 4   
[Absolute,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $9D | 3 | 5   
[Absolute,Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $99 | 3 | 5   
[(Indirect,X)](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $81 | 2 | 6   
[(Indirect),Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $91 | 2 | 6   
  
See also: LDA

* * *

### STX - Store X

`memory = X`

STX stores the X register value into memory. 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $86 | 2 | 3   
[Zero Page,Y](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $96 | 2 | 4   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $8E | 3 | 4   
  
See also: LDX

* * *

### STY - Store Y

`memory = Y`

STY stores the Y register value into memory. 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Zero Page](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $84 | 2 | 3   
[Zero Page,X](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $94 | 2 | 4   
[Absolute](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $8C | 3 | 4   
  
See also: LDY

* * *

### TAX - Transfer A to X

`X = A`

TAX copies the accumulator value to the X register. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $AA | 1 | 2   
  
See also: TXA, TAY, TYA

* * *

### TAY - Transfer A to Y

`Y = A`

TAY copies the accumulator value to the Y register. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $A8 | 1 | 2   
  
See also: TYA, TAX, TXA

* * *

### TSX - Transfer Stack Pointer to X

`X = SP`

TSX copies the stack pointer value to the X register. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $BA | 1 | 2   
  
See also: TXS

* * *

### TXA - Transfer X to A

`A = X`

TXA copies the X register value to the accumulator. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $8A | 1 | 2   
  
See also: TAX, TAY, TYA

* * *

### TXS - Transfer X to Stack Pointer

`SP = X`

TXS copies the X register value to the stack pointer. 

Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $9A | 1 | 2   
  
See also: TXS

* * *

### TYA - Transfer Y to A

`A = Y`

TYA copies the Y register value to the accumulator. 

Flag | New value   
---|---  
[Z - Zero](Status_flags.xhtml#Z "Status flags") | result == 0   
[N - Negative](Status_flags.xhtml#N "Status flags") | result bit 7   
Addressing mode | Opcode | Bytes | Cycles   
---|---|---|---  
[Implied](https://www.nesdev.org/w/index.php?title=Addressing_modes&action=edit&redlink=1 "Addressing modes \(page does not exist\)") | $98 | 1 | 2   
  
See also: TAY, TAX, TXA

* * *

### Note

For Relative addressing, the document <https://www.nesdev.org/6502_cpu.txt> seems to represent branch instructions as having 5 possible cycles, however 2-4 cycles as noted on this page is correct. 
    
    
           1     PC      R  fetch opcode, increment PC
           2     PC      R  fetch operand, increment PC
           3     PC      R  Fetch opcode of next instruction,
                            If branch is taken, add operand to PCL.
                            Otherwise increment PC.
           4+    PC*     R  Fetch opcode of next instruction.
                            Fix PCH. If it did not change, increment PC.
           5!    PC      R  Fetch opcode of next instruction,
                            increment PC.
    

These notes help clarify the way the cycles are represented in the document: 

  * If the branch is not taken, cycle 3 shown here is actually cycle 1 of the next instruction (the branch instruction ending after 2 cycles).
  * If the branch is taken and does not cross a page boundary, cycle 4 shown here is cycle 1 of the next instruction (the branch instruction ending after 3 cycles).
  * If a page boundary is crossed, cycle 5 shown here is cycle 1 of the next instruction (the branch instruction ending after 4 cycles).


