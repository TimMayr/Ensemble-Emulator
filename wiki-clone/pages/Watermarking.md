# Watermarking

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Watermarking) | View [other pages](Special_AllPages.xhtml#Watermarking)

**Watermarking** is [defined by Wikipedia](https://en.wikipedia.org/wiki/Digital_watermarking "wikipedia:Digital watermarking") as "the process of embedding information into a digital signal in a way that is difficult to remove." In some cases, the developer of an unreleased NES program wants to distribute copies to beta testers but still [trace](https://en.wikipedia.org/wiki/Traitor_tracing "wikipedia:Traitor tracing") any [leaked](https://en.wikipedia.org/wiki/Internet_leak "wikipedia:Internet leak") copies of the program to the tester who broke the non-disclosure agreement. There are several ways to produce binaries that can be traced back to a particular recipient. 

## Contents

  * 1 Shuffling
  * 2 Instruction encoding
  * 3 Graphics changes
  * 4 Compression
  * 5 See also
  * 6 External links



## Shuffling

One way to make each copy unique is to shuffle, or randomly rearrange, pieces of a program at compile time. 

A code preprocessor can randomize the order of statically allocated variables in a program. This causes the addresses embedded in the code to change every time the program is compiled. It has benefits beyond watermarking: as the program is shuffled, a randomly chosen variable acts as a [canary](https://en.wikipedia.org/wiki/Buffer_overflow_protection "wikipedia:Buffer overflow protection") for the variable before it, and the effects of a [buffer overflow](https://en.wikipedia.org/wiki/buffer_overflow "wikipedia:buffer overflow") may become more apparent. 

A code preprocessor can shuffle the order of subroutines or lookup tables in a program. Watch out: A common technique on the NES is the "[inline](https://en.wikipedia.org/wiki/Inline_expansion "wikipedia:Inline expansion") [tail call](https://en.wikipedia.org/wiki/tail_call "wikipedia:tail call")", in which a subroutine doesn't return but instead falls off the end into the following subroutine. You'll need to take this into account when adding markup to control the preprocessor. 

A code preprocessor can shuffle the order of instructions in a subroutine. In a lot of cases, the order of instructions doesn't matter, such as `LDA` vs. `CLC`, or `LDX` vs. `LDY` where neither is indexed, or `STA (d),Y` vs. `DEX`, or STA of the same value to several different variables. Writes to some [PPU registers](PPU_registers.xhtml "PPU registers") act similarly: when setting up [PPUCTRL](PPU_registers.xhtml "PPUCTRL") ($2000) and [PPUMASK](PPU_registers.xhtml "PPUMASK") ($2001) at the end of vertical blanking, it doesn't matter which is written first. The markup for such "bundles of instructions" resembles the markup for stop bits in [explicitly parallel instruction computing](https://en.wikipedia.org/wiki/explicitly_parallel_instruction_computing "wikipedia:explicitly parallel instruction computing"). Adding the required markup also has a benefit beyond watermarking: thinking about what instructions affect others forces a code review, which allows a programmer to refamiliarize himself with a code base and possibly discover defects. 

A common method to cope with bus conflicts on discrete [mappers](Mapper.xhtml "Mapper") brings in another trick. For example, a game using UNROM might load from a table and write back to that table to make sure that the written bits match the bits in ROM. (See the `banktable` example at [Programming UNROM](Programming_UNROM.xhtml "Programming UNROM").) But if you shuffle the data in banks 0 through 6 and shuffle the bank numbers in the table, you can make 7! = 5040 different binaries from this alone. 

Even in a game no bigger than [NROM](NROM.xhtml "NROM")-128, shuffling alone allows for more distinct binaries than the number of atoms in the known universe squared. With the size of NES games and with modern solid archiving tools such as 7-Zip, you can save each binary that you send out to each tester and still not fill a 4 GB USB flash drive. As long as the binary doesn't get leaked to someone with the knowledge to disassemble and reassemble a binary (as in [SMBDis](http://www.romhacking.net/docs/344/)), computing the [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance "wikipedia:Hamming distance") between the leaked copy and your saved copies is likely to result in a high-confidence match to the leaker. 

## Instruction encoding

A few instructions have [multiple encodings](Programming_with_unofficial_opcodes.xhtml#Duplicated_instructions "Programming with unofficial opcodes"). A code preprocessor can introduce any of several [NOP instructions](Programming_with_unofficial_opcodes.xhtml#NOPs "Programming with unofficial opcodes") at random points in a non-time-critical subroutine. 

One may also add official do-(near)-nothing instructions such as EOR #0, ORA #0, AND #$FF, an INX/DEX pair, etc. Another watermarking padding stratagem would be, before a load instruction for a register (LDA, LDX, LDY, PLA, TAX, TAY, TSX, TXA, or TYA), to introduce instructions affecting that register; the load makes irrelevant all changes to the register's contents. One should, of course, take care to ensure that such new instructions do not have undesired flag changes altering execution flow. 

The addresses of the PPU's ports, almost every mapper without [bus conflicts](Bus_conflict.xhtml "Bus conflict"), and the internal WRAM are all [incompletely decoded](Mirroring.xhtml "Mirroring"). Because the NES PPU is configured to ignore A12 through A3, each PPU port appears 1024 times in the range $2000-$3FFF. Likewise, the [MMC3](MMC3.xhtml "MMC3") ignores A12 through A1, and each port appears 4096 times. The [MMC1](MMC1.xhtml "MMC1") ignores A14 through A0 for the first four writes and A12 through A0 for the last, or 73 bits in all, but many games use only one subroutine to handle all writes to each of its four ports. The internal RAM appears only 4 times in $0000-$1FFF, but any non-zero-page instruction could be changed to use one of the mirrors. A code preprocessor could randomize these address bits in any instruction that reads or writes these ports. This would also serve to hinder a cracker's use of an in-emulator debugger that doesn't take mirroring into account. 

Use of multiple instruction encoding and multiple address decoding might also be used if you want to store the program and data in the same place (although this is difficult), or to use the mirrored registers for some purpose in a mapper that you might make up or in a debugger. 

## Graphics changes

Graphics can depend on the build: 

  * Choose one of several alternatives for [grass](http://www.petesqbsite.com/sections/tutorials/tuts/tsugumo/chapter1.htm) and other [noisy tiles](http://www.petesqbsite.com/sections/tutorials/tuts/tsugumo/chapter3.htm)
  * Rearrange the order in which sets of tiles appear in the CHR ROM
  * Tester's name or something derived from tester's name on the title screen. This is easy to remove, but it acts as a deterrent.
  * Tester's name or something derived from tester's name on a sign in a building in the game



## Compression

If your program includes compressed data, you can change the interpretation of bits in the data format. For example, in RLE [tile compression](Tile_compression.xhtml "Tile compression"), the sense of bits denoting a run of repeated pixels vs. bits denoting a run of several literal pixels can be inverted. 

## See also

  * [Concentration Room](User_Tepples_Concentration_Room.xhtml "User:Tepples/Concentration Room"): Version 0.02 includes a Python program to shuffle source code, and an example of how to work it into a makefile.



## External links

  * Based on BBS topics [6046](http://forums.nesdev.org/viewtopic.php?t=6046) and [6197](http://forums.nesdev.org/viewtopic.php?t=6197)


