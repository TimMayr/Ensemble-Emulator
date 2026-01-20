# CPU unofficial opcodes

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/CPU_unofficial_opcodes) | View [other pages](Special_AllPages.xhtml#CPU_unofficial_opcodes)

**Unofficial opcodes** , sometimes called **illegal opcodes** or **undocumented opcodes** , are [CPU instructions](6502_instructions.xhtml "6502 instructions") that were officially left unused by the original design. The 6502 family datasheet from MOS Technology does not specify or document their function, but they actually do perform various operations. 

Some of these instructions are useful; some are not predictable; some do nothing but burn cycles; some halt the CPU until reset. Most NMOS 6502 cores interpret them the same way, although there are slight differences with the less stable instructions. CMOS variants of the 6502 handle them completely differently, and later CPUs in the same family (e.g. 65C02, HuC6280, 65C816) were free to implement new instructions in the place of the unused ones. 

An [accurate](Accuracy.xhtml "Accuracy") NES emulator must implement all instructions, not just the official ones. A small number of games use them (see below). 

## Contents

  * 1 Arrangement
  * 2 Games using unofficial opcodes
    * 2.1 Homebrew games
  * 3 See also
  * 4 External links
  * 5 References



## Arrangement

The microcode of the 6502 is compressed into a 130-entry decode ROM. Instead of 256 entries telling how to process each separate opcode, it's encoded as combinational logic post-processing the output of a "sparse" ROM that acts in some ways like a programmable logic array (PLA). Each entry in the ROM means "if these bits are on, and these bits are off, do things on these six cycles."[1]

Many instructions activate multiple lines of the decode ROM at once. Often this is on purpose, such as one line for the addressing mode and one for the opcode part. But many of the unofficial opcodes simultaneously trigger parts of the ROM that were intended for completely unrelated instructions. 

Perhaps the pattern is easier to see by shuffling the 6502's opcode matrix. This table lists all 6502 opcodes, 32 columns per row. The columns are colored by bits 1 and 0: 00 red, 01 green, 10 blue, and 11 gray. 

| +00 | +01 | +02 | +03 | +04 | +05 | +06 | +07 | +08 | +09 | +0A | +0B | +0C | +0D | +0E | +0F | +10 | +11 | +12 | +13 | +14 | +15 | +16 | +17 | +18 | +19 | +1A | +1B | +1C | +1D | +1E | +1F   
---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---  
00 | BRK  
| ORA  
(d,x) | **STP**  
| **SLO**  
(d,x) | **NOP**  
d | ORA  
d | ASL  
d | **SLO**  
d | PHP  
| ORA  
#i | ASL  
| **ANC**  
#i | **NOP**  
a | ORA  
a | ASL  
a | **SLO**  
a | BPL  
*+d | ORA  
(d),y | **STP**  
| **SLO**  
(d),y | **NOP**  
d,x | ORA  
d,x | ASL  
d,x | **SLO**  
d,x | CLC  
| ORA  
a,y | **NOP**  
| **SLO**  
a,y | **NOP**  
a,x | ORA  
a,x | ASL  
a,x | **SLO**  
a,x   
20 | JSR  
a | AND  
(d,x) | **STP**  
| **RLA**  
(d,x) | BIT  
d | AND  
d | ROL  
d | **RLA**  
d | PLP  
| AND  
#i | ROL  
| **ANC**  
#i | BIT  
a | AND  
a | ROL  
a | **RLA**  
a | BMI  
*+d | AND  
(d),y | **STP**  
| **RLA**  
(d),y | **NOP**  
d,x | AND  
d,x | ROL  
d,x | **RLA**  
d,x | SEC  
| AND  
a,y | **NOP**  
| **RLA**  
a,y | **NOP**  
a,x | AND  
a,x | ROL  
a,x | **RLA**  
a,x   
40 | RTI  
| EOR  
(d,x) | **STP**  
| **SRE**  
(d,x) | **NOP**  
d | EOR  
d | LSR  
d | **SRE**  
d | PHA  
| EOR  
#i | LSR  
| **ALR**  
#i | JMP  
a | EOR  
a | LSR  
a | **SRE**  
a | BVC  
*+d | EOR  
(d),y | **STP**  
| **SRE**  
(d),y | **NOP**  
d,x | EOR  
d,x | LSR  
d,x | **SRE**  
d,x | CLI  
| EOR  
a,y | **NOP**  
| **SRE**  
a,y | **NOP**  
a,x | EOR  
a,x | LSR  
a,x | **SRE**  
a,x   
60 | RTS  
| ADC  
(d,x) | **STP**  
| **RRA**  
(d,x) | **NOP**  
d | ADC  
d | ROR  
d | **RRA**  
d | PLA  
| ADC  
#i | ROR  
| **ARR**  
#i | JMP  
(a) | ADC  
a | ROR  
a | **RRA**  
a | BVS  
*+d | ADC  
(d),y | **STP**  
| **RRA**  
(d),y | **NOP**  
d,x | ADC  
d,x | ROR  
d,x | **RRA**  
d,x | SEI  
| ADC  
a,y | **NOP**  
| **RRA**  
a,y | **NOP**  
a,x | ADC  
a,x | ROR  
a,x | **RRA**  
a,x   
80 | **NOP**  
#i | STA  
(d,x) | **NOP**  
#i | **SAX**  
(d,x) | STY  
d | STA  
d | STX  
d | **SAX**  
d | DEY  
| **NOP**  
#i | TXA  
| **XAA**  
#i | STY  
a | STA  
a | STX  
a | **SAX**  
a | BCC  
*+d | STA  
(d),y | **STP**  
| **AHX**  
(d),y | STY  
d,x | STA  
d,x | STX  
d,y | **SAX**  
d,y | TYA  
| STA  
a,y | TXS  
| **TAS**  
a,y | **SHY**  
a,x | STA  
a,x | **SHX**  
a,y | **AHX**  
a,y   
A0 | LDY  
#i | LDA  
(d,x) | LDX  
#i | **LAX**  
(d,x) | LDY  
d | LDA  
d | LDX  
d | **LAX**  
d | TAY  
| LDA  
#i | TAX  
| **LAX**  
#i | LDY  
a | LDA  
a | LDX  
a | **LAX**  
a | BCS  
*+d | LDA  
(d),y | **STP**  
| **LAX**  
(d),y | LDY  
d,x | LDA  
d,x | LDX  
d,y | **LAX**  
d,y | CLV  
| LDA  
a,y | TSX  
| **LAS**  
a,y | LDY  
a,x | LDA  
a,x | LDX  
a,y | **LAX**  
a,y   
C0 | CPY  
#i | CMP  
(d,x) | **NOP**  
#i | **DCP**  
(d,x) | CPY  
d | CMP  
d | DEC  
d | **DCP**  
d | INY  
| CMP  
#i | DEX  
| **AXS**  
#i | CPY  
a | CMP  
a | DEC  
a | **DCP**  
a | BNE  
*+d | CMP  
(d),y | **STP**  
| **DCP**  
(d),y | **NOP**  
d,x | CMP  
d,x | DEC  
d,x | **DCP**  
d,x | CLD  
| CMP  
a,y | **NOP**  
| **DCP**  
a,y | **NOP**  
a,x | CMP  
a,x | DEC  
a,x | **DCP**  
a,x   
E0 | CPX  
#i | SBC  
(d,x) | **NOP**  
#i | **ISC**  
(d,x) | CPX  
d | SBC  
d | INC  
d | **ISC**  
d | INX  
| SBC  
#i | NOP  
| **SBC**  
#i | CPX  
a | SBC  
a | INC  
a | **ISC**  
a | BEQ  
*+d | SBC  
(d),y | **STP**  
| **ISC**  
(d),y | **NOP**  
d,x | SBC  
d,x | INC  
d,x | **ISC**  
d,x | SED  
| SBC  
a,y | **NOP**  
| **ISC**  
a,y | **NOP**  
a,x | SBC  
a,x | INC  
a,x | **ISC**  
a,x   
  
Key: a is a 16-bit absolute address, and d is an 8-bit zero page address. Entries in bold represent unofficial opcodes. 

But if we rearrange it so that columns with the same bits 1-0 are close together, correlations become easier to see: 

| +00 | +04 | +08 | +0C | +10 | +14 | +18 | +1C | +01 | +05 | +09 | +0D | +11 | +15 | +19 | +1D | +02 | +06 | +0A | +0E | +12 | +16 | +1A | +1E | +03 | +07 | +0B | +0F | +13 | +17 | +1B | +1F   
---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---  
00 | BRK  
| **NOP**  
d | PHP  
| **NOP**  
a | BPL  
*+d | **NOP**  
d,x | CLC  
| **NOP**  
a,x | ORA  
(d,x) | ORA  
d | ORA  
#i | ORA  
a | ORA  
(d),y | ORA  
d,x | ORA  
a,y | ORA  
a,x | **STP**  
| ASL  
d | ASL  
| ASL  
a | **STP**  
| ASL  
d,x | **NOP**  
| ASL  
a,x | **SLO**  
(d,x) | **SLO**  
d | **ANC**  
#i | **SLO**  
a | **SLO**  
(d),y | **SLO**  
d,x | **SLO**  
a,y | **SLO**  
a,x   
20 | JSR  
a | BIT  
d | PLP  
| BIT  
a | BMI  
*+d | **NOP**  
d,x | SEC  
| **NOP**  
a,x | AND  
(d,x) | AND  
d | AND  
#i | AND  
a | AND  
(d),y | AND  
d,x | AND  
a,y | AND  
a,x | **STP**  
| ROL  
d | ROL  
| ROL  
a | **STP**  
| ROL  
d,x | **NOP**  
| ROL  
a,x | **RLA**  
(d,x) | **RLA**  
d | **ANC**  
#i | **RLA**  
a | **RLA**  
(d),y | **RLA**  
d,x | **RLA**  
a,y | **RLA**  
a,x   
40 | RTI  
| **NOP**  
d | PHA  
| JMP  
a | BVC  
*+d | **NOP**  
d,x | CLI  
| **NOP**  
a,x | EOR  
(d,x) | EOR  
d | EOR  
#i | EOR  
a | EOR  
(d),y | EOR  
d,x | EOR  
a,y | EOR  
a,x | **STP**  
| LSR  
d | LSR  
| LSR  
a | **STP**  
| LSR  
d,x | **NOP**  
| LSR  
a,x | **SRE**  
(d,x) | **SRE**  
d | **ALR**  
#i | **SRE**  
a | **SRE**  
(d),y | **SRE**  
d,x | **SRE**  
a,y | **SRE**  
a,x   
60 | RTS  
| **NOP**  
d | PLA  
| JMP  
(a) | BVS  
*+d | **NOP**  
d,x | SEI  
| **NOP**  
a,x | ADC  
(d,x) | ADC  
d | ADC  
#i | ADC  
a | ADC  
(d),y | ADC  
d,x | ADC  
a,y | ADC  
a,x | **STP**  
| ROR  
d | ROR  
| ROR  
a | **STP**  
| ROR  
d,x | **NOP**  
| ROR  
a,x | **RRA**  
(d,x) | **RRA**  
d | **ARR**  
#i | **RRA**  
a | **RRA**  
(d),y | **RRA**  
d,x | **RRA**  
a,y | **RRA**  
a,x   
80 | **NOP**  
#i | STY  
d | DEY  
| STY  
a | BCC  
*+d | STY  
d,x | TYA  
| **SHY**  
a,x | STA  
(d,x) | STA  
d | **NOP**  
#i | STA  
a | STA  
(d),y | STA  
d,x | STA  
a,y | STA  
a,x | **NOP**  
#i | STX  
d | TXA  
| STX  
a | **STP**  
| STX  
d,y | TXS  
| **SHX**  
a,y | **SAX**  
(d,x) | **SAX**  
d | **XAA**  
#i | **SAX**  
a | **AHX**  
(d),y | **SAX**  
d,y | **TAS**  
a,y | **AHX**  
a,y   
A0 | LDY  
#i | LDY  
d | TAY  
| LDY  
a | BCS  
*+d | LDY  
d,x | CLV  
| LDY  
a,x | LDA  
(d,x) | LDA  
d | LDA  
#i | LDA  
a | LDA  
(d),y | LDA  
d,x | LDA  
a,y | LDA  
a,x | LDX  
#i | LDX  
d | TAX  
| LDX  
a | **STP**  
| LDX  
d,y | TSX  
| LDX  
a,y | **LAX**  
(d,x) | **LAX**  
d | **LAX**  
#i | **LAX**  
a | **LAX**  
(d),y | **LAX**  
d,y | **LAS**  
a,y | **LAX**  
a,y   
C0 | CPY  
#i | CPY  
d | INY  
| CPY  
a | BNE  
*+d | **NOP**  
d,x | CLD  
| **NOP**  
a,x | CMP  
(d,x) | CMP  
d | CMP  
#i | CMP  
a | CMP  
(d),y | CMP  
d,x | CMP  
a,y | CMP  
a,x | **NOP**  
#i | DEC  
d | DEX  
| DEC  
a | **STP**  
| DEC  
d,x | **NOP**  
| DEC  
a,x | **DCP**  
(d,x) | **DCP**  
d | **AXS**  
#i | **DCP**  
a | **DCP**  
(d),y | **DCP**  
d,x | **DCP**  
a,y | **DCP**  
a,x   
E0 | CPX  
#i | CPX  
d | INX  
| CPX  
a | BEQ  
*+d | **NOP**  
d,x | SED  
| **NOP**  
a,x | SBC  
(d,x) | SBC  
d | SBC  
#i | SBC  
a | SBC  
(d),y | SBC  
d,x | SBC  
a,y | SBC  
a,x | **NOP**  
#i | INC  
d | NOP  
| INC  
a | **STP**  
| INC  
d,x | **NOP**  
| INC  
a,x | **ISC**  
(d,x) | **ISC**  
d | **SBC**  
#i | **ISC**  
a | **ISC**  
(d),y | **ISC**  
d,x | **ISC**  
a,y | **ISC**  
a,x   
  
Thus the 00 (red) block is mostly control instructions, 01 (green) is ALU operations, and 10 (blue) is read-modify-write (RMW) operations and data movement instructions involving X. The RMW instructions (all but row 80 and A0) in columns +06, +0E, +16, and +1E have the same addressing modes as the corresponding ALU operations. 

The 11 (gray) block is unofficial opcodes combining the operations of instructions from the ALU and RMW blocks. all of them having the same addressing mode as the corresponding ALU opcode. The RMW+ALU instructions that affect memory are easiest to understand because their RMW part completes during the opcode and the ALU part completes during the next opcode's fetch. Column +0B, on the other hand, has no extra cycles; everything completes during the next opcode's fetch. This causes instructions to have strange mixing properties. Some even [differ based on analog effects](Visual6502wiki_6502_Opcode_8B__XAA__ANE_.xhtml "Visual6502wiki/6502 Opcode 8B \(XAA, ANE\)"). 

## Games using unofficial opcodes

The use of unofficial opcodes is rare in NES games. It appears to occur mostly in late or unlicensed titles: 

  * _Beauty and the Beast_ (E) (1994) uses `$80` (a 2-byte NOP).[2]
  * _Disney's Aladdin_ (E) (December 1994) uses `$07` (SLO). This is Virgin's port of the Game Boy game, itself a port of the Genesis game, not any of the [pirate originals](https://bootleggames.fandom.com/wiki/Special:PrefixIndex/Aladdin).
  * _Dynowarz: Destruction of Spondylus_ (April 1990) uses 1-byte NOPs `$DA` and `$FA` on the first level when your dino throws his fist.
  * _F-117A Stealth Fighter_ uses `$89` (a 2-byte NOP).
  * _文字广场+排雷_ (romanized in GoodNES as Cantonese "Gaau Hok Gwong Cheung (Ch)"): After selecting the left game (排雷) from this 2-in-1 multicart, a glitchy [32 KiB bankswitch](INES_Mapper_242.xhtml "INES Mapper 242") causes the CPU to get lost in non-code ROM space that only with correct emulation of unofficial opcodes, including $8B (XAA immediate), will have it eventually reach a BRK instruction that properly branches to that game's reset handler.
  * _Infiltrator_ uses `$89` (a 2-byte NOP).
  * _Ninja Jajamaru-kun_ uses `$04` (a 2-byte NOP) when your ninja collides with an enemy, as a consequence of branching into the middle of an instruction.
  * _Puzznic_ (all regions) (US release November 1990) uses `$89` (a 2-byte NOP).
  * _Super Cars_ (U) (February 1991) uses `$B3` (LAX).



### Homebrew games

  * The MUSE music engine, used in _Driar_ and _STREEMERZ: Super Strength Emergency Squad Zeta_ , uses `$8F` (SAX), `$B3` (LAX), and `$CB` (AXS).[3]
  * _[Attribute Zone](User_Zzo38_Attribute_Zone.xhtml "User:Zzo38/Attribute Zone")_ uses `$0B` (ANC), `$2F` (RLA), `$4B` (ALR), `$A7` (LAX), `$B3` (LAX), `$CB` (AXS), `$D3` (DCP) and `$DB` (DCP).
  * The port of _Zork_ to the Famicom uses a few unofficial opcodes.
  * _Eyra, the Crow Maiden_ uses several unofficial opcodes.



## See also

  * [Programming with unofficial opcodes](Programming_with_unofficial_opcodes.xhtml "Programming with unofficial opcodes")



## External links

  * [6502 opcode matrix including unofficial opcodes](http://www.oxyron.de/html/opcodes02.html)
  * [65C02](http://www.oxyron.de/html/opcodesc02.html) and [65816](http://www.oxyron.de/html/opcodes816.html)
  * [Illegal opcodes](https://en.wikipedia.org/wiki/Illegal_opcode "wikipedia:Illegal opcode") at Wikipedia.
  * [65xx Processor Data](http://www.romhacking.net/documents/318/)
  * [6502_cpu.txt](http://nesdev.org/6502_cpu.txt)



## References

  1. ↑ Michael Steil. "[How MOS 6502 Illegal Opcodes really work](https://www.pagetable.com/?p=39)". _Pagetable_ , 2008-07-29. Accessed 2019-11-10.
  2. ↑ [puNES 0.20 changelog](http://forums.nesdev.org/viewtopic.php?f=3&t=6928) indicating $80 opcode in _Beauty and the Beast_.
  3. ↑ <http://forums.nesdev.org/viewtopic.php?p=100957#p100957>


