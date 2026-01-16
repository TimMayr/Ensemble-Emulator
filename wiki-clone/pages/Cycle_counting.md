# Cycle counting

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Cycle_counting) | View [other pages](Special_AllPages.xhtml#Cycle_counting)

It is often useful to delay a specific number of CPU cycles. Timing raster effects or generating PCM audio are some examples that might utilize this. This article outlines a few relevant techniques. 

## Contents

  * 1 Instruction timings
  * 2 Short delays
  * 3 Clockslide
  * 4 Resources
  * 5 References



## Instruction timings

You can use a comprehensive guide[1][2] as reference for instruction timings, but there are some rules-of-thumb[3] that can help remember most of them: 

  * There is a minimum of 2 cycles per instruction.
  * Each byte of memory read or written adds 1 more cycle to the instruction. This includes fetching the instruction, and each byte of its operand, then any memory it references.
  * Indexed instructions which cross a page take 1 extra cycle to adjust the high byte of the effective address first.
  * Read-modify-write instructions perform a dummy write during the "modify" stage and thus take 1 extra cycle.
  * Instructions that push data onto the stack take 1 extra cycle.
  * Instructions that pop data from the stack take 2 extra cycles, since they also need to pre-increment the stack pointer.
  * "Extra" cycles often include an extra read or write that usually does not affect the outcome.



Examples: 

Instructions | Cycles | Bytes | Details   
---|---|---|---  
`SEC` | 2 | 1 | opcode, but has to wait for the 2-cycle minimum.   
`AND #imm` | 2 | 2 | opcode + operand. Only affects registers.   
`LDA zp` | 3 | 2 | opcode + operand + byte fetched from zp.   
`STA abs` | 4 | 3 | opcode + 2 byte operand + byte written to abs.   
`LDA abs,X` | 4 or 5 | 3 | opcode + 2 byte operand + read from abs, but it delays 1 extra cycle if the addition of the X index causes a page crossing.   
`STA abs,X` | 5 | 3 | Like LDA abs,X, but assumes the worst case of page crossing and always spends 1 extra read cycle.   
`ASL zp` | 5 | 2 | opcode + operand + read from zp + write to zp, but it takes 1 extra cycle to modify the value.   
`LDA (indirect),Y` | 5 or 6 | 2 | opcode + operand + two reads from zp + read from indirect address. 1 extra cycle if a page is crossed.   
`STA (indirect),Y` | 6 | 2 | Like LDA (indirect),Y, but assumes the worst case of page crossing and always spends 1 extra read cycle.   
`PHA` | 3 | 1 | opcode + stack write, but requires 1 extra cycle to perform the stack operation.   
`RTS` | 6 | 1 | opcode + two stack reads, but requires 2 extra cycles to perform the stack operations, plus 1 cycle to post-increment the program counter (to compensate for the off-by-1 address pushed by JSR).   
  
## Short delays

Here are few ways to create short delays without side effects. As the shortest instruction time is 2 cycles, it is not possible to delay 1 cycle on its own. NOP is essential for 2 cycle delays. 3 cycle delays always take 2 bytes, but usually have some compromise. More options become available as delays become longer. 

Instructions | Cycles | Bytes | Side effects and notes   
---|---|---|---  
`NOP` | 2 | 1 |   
`JMP *+3` | 3 | 3 |   
`Bxx *+2` | 3 | 2 | None, but requires a known flag state (e.g. BCC if carry is known to be clear).   
`BIT zp` | 3 | 2 | Clobbers NVZ flags. Reads zp.   
`[IGN zp](Programming_with_unofficial_opcodes.xhtml#NOPs "Programming with unofficial opcodes")` | 3 | 2 | Reads zp. (Unofficial instruction.)   
`NOP, NOP` | 4 | 2   
`NOP, ...` | 5 | 3 | (... = 3 cycle delay of choice)   
`CLV, BVC *+2` | 5 | 3 | Clears V flag. (Can instead use C flag with CLC, BCC or SEC, BCS.)   
`NOP, NOP, NOP` | 6 | 3   
`PHP, PLP` | 7 | 2 | Modifies 1 byte of stack.   
`NOP, NOP, NOP, NOP` | 8 | 4   
`PHP, PLP, NOP` | 9 | 3 | Modifies 1 byte of stack.   
`PHP, CMP zp, PLP` | 10 | 4 | Modifies 1 byte of stack. Reads zp.   
`PHP, PLP, NOP, NOP` | 11 | 4 | Modifies 1 byte of stack.   
`JSR, RTS` | 12 | 3 | Modifies 2 bytes of stack. (Takes 3 bytes only if reusing an existing RTS; otherwise 4.)   
  
## Clockslide

A **clockslide**[4] is a sequence of instructions that wastes a small constant amount of cycles plus one cycle per executed byte, no matter whether it's entered on an odd or even address. With official instructions, one can construct a clockslide from CMP instructions: `... C9 C9 C9 C9 C5 EA` Disassemble from the start and you get `CMP #$C9 CMP #$C9 CMP $EA` (6 bytes, 7 cycles). Disassemble one byte in and you get `CMP #$C9 CMP #$C5 NOP` (5 bytes, 6 cycles). The entry point can be controlled with an indirect jump or the [RTS Trick](RTS_Trick.xhtml "RTS Trick") to precisely control raster effect or sample playback timing. 

CMP has a side effect of destroying most of the flags, but you can substitute other instructions with the same size and timing that preserve whichever flags/registers you need at the end of the slide. There are [unofficial instructions](CPU_unofficial_opcodes.xhtml "CPU unofficial opcodes") that can avoid altering any state: replace $C9 (CMP) with $89 or $80, which ignores an immediate operand, and replace $C5 with $04, $44, or $64, which ignore a read from the zero page. 

## Resources

  * [Delay code](Delay_code.xhtml "Delay code") \- various variable-cycle delays
  * [Fixed cycle delay](Fixed_cycle_delay.xhtml "Fixed cycle delay") \- shortest fixed-cycle delays
  * [Fixed-cycle delay code vending machine](https://bisqwit.iki.fi/utils/nesdelay.php) \- code for generating shortest-possible delay routines at compile-time.
  * [6502 vdelay](https://github.com/bbbradsmith/6502vdelay) \- code for delaying a variable number of cycles at run-time.



## References

  1. ↑ [Obelisk: 6502 instruction reference](https://www.nesdev.org/obelisk-6502-guide/reference.html)
  2. ↑ [6502_cpu.txt: cycle-by-cycle instruction behaviour](http://nesdev.org/6502_cpu.txt)
  3. ↑ [Forum: Is there a logic to instruction timings?](https://forums.nesdev.org/viewtopic.php?p=256515)
  4. ↑ [Clockslide: How to waste an exact number of clock cycles on the 6502](http://www.pagetable.com/?p=669)


