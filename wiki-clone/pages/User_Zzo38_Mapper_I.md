# User:Zzo38/Mapper I

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Mapper_I) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Mapper_I)

  
This is design for trying to simplify (and speed up) Z-code interpreter programs (and possibly other kind of interpreters, too). It can be made with four 74xx series ICs (it is completely untested). 

  * PRG ROM size: Main ROM = 16K or 32K; ExROM = 16K, 32K, 64K, or 128K.
  * PRG ROM bank size: Main ROM = not bankswitched; ExROM = 2 bytes if 128K, 1 byte if less than 128K
  * PRG RAM size: 8K, 16K, 32K, 64K, or 128K (ExRAM; optionally battery-backed)
  * PRG RAM bank size: 1 byte
  * CHR capacity: 8K ROM
  * CHR bank size: 8K
  * Nametable mirroring: Hardwired
  * Bus conflicts: Yes (but usually not significant)



Note that in the iNES file, ExROM comes first and is then followed by the main ROM. Please note that 32K ExROM and 16K main ROM is not a valid combination. 

## Contents

  * 1 Memory map
  * 2 Mapper registers
  * 3 Wiring
  * 4 Examples
  * 5 Oversize/undersize
  * 6 Implementations



## Memory map

CPU: 

  * $1000-$1FFF: Mapper registers overlapping console's internal RAM
  * $3000-$3FFF: Mapper registers overlapping PPU registers
  * $5000-$5FFF: Mapper registers
  * $7000-$7FFF: Mapper registers (mirror of $5000-$5FFF)
  * $8000-$FFFF: 16K or 32K PRG ROM, fixed to the last bank



PPU: 

  * $0000-$1FFF: One 8K bank of CHR ROM; not bankswitched
  * $2000-$3FFF: CIRAM



## Mapper registers

There are two 8-bit registers, the low address register and the high address register. 

Low address register is mapped at: 
    
    
    [0..1 .... ...1 ....]
    

High address register is mapped at: 
    
    
    [0..1 .... ..1. ....]
    

Writing to any such address writes those registers; reading from the addresses will instead write to the mapper register whatever data happens to be read from that address (possibly a mirror of the Famicom's internal 2K RAM). 

Note that if bit4 and bit5 are both set, then both registers will be written. 

ExROM is mapped at: 
    
    
    [0..1 1... .... .1.x]
    

A15-A8 are loaded from the high address register, A7-A0 from the low address register, and A16 is from the "x" above. 

ExRAM is mapped at: 
    
    
    [0..1 1... .... .0.x]
    

A15-A8 are loaded from the high address register, A7-A0 from the low address register, and A16 is from the "x" above. 

Also note that accessing ExROM/ExRAM may also write to mapper registers if bit4/bit5 of the address the CPU is using to access it happen to be set (although race conditions may result if you are not careful). 

The R/W signal is not used for mapper registers, except when accessing ExRAM. 

## Wiring

The following cartridge signals are used: /ROMSEL A12 M2 A0 A2 A4 A5 A11 D0-D7 R/W 

  * D0-D7 are wired to inputs of both latches, as well as to RAM and both ROMs.
  * R/W is wired to the RAM only.



Other wires: 
    
    
    wire0 = /ROMSEL&A12
    wire1 = wire0&M2
    wire2 = wire1~&A11
    wire3 = wire2~&wire2
    Latch1.E = wire1&A4
    Latch1.D7:D0 = D7:D0
    Latch2.E = wire1&A5
    Latch2.D7:D0 = D7:D0
    ExROM.D7:D0 = D7:D0
    ExROM.A16 = A0
    ExROM.A15:A8 = Latch2.Q7:Q0
    ExROM.A7:A0 = Latch1.Q7:Q0
    ExROM./CS = wire3~&A2
    ExRAM.D7:D0 = D7:D0
    ExRAM.A15:A8 = Latch2.Q7:Q0
    ExRAM.A7:A0 = Latch1.Q7:Q0
    ExRAM./CS = ExROM./CS~&wire3
    ExRAM.R/W = R/W
    

This is untested; I hope it works and that I haven't made a mistake in typing it into the computer. 

## Examples

Assuming variables are assigned as follows: 

  * $10 = bit7-bit0 of instruction pointer
  * $20 = bit15-bit8 of instruction pointer
  * $0E = bit16 of instruction pointer



Further assume that the high 64K of a 128K story file is in the low 64K of ExROM, and the low 64K of the story file in the high 64K of ExROM. 

To advance instruction pointer and read the byte, you can do something like: 
    
    
        INC $1010
        BNE L1
        INC $1020
        BNE L1
        INC <$0E
    L1  LDX <$0E
        LDA $5803,X
    

And then if interpreting an instruction such as GET or PUT or whatever that accesses other parts of the RAM, the high address register can be reset simply by the `BIT $1020` (or `IGN $1020`) instruction. And then when reading the next instruction byte, the `INC $1010` will reset the low address register too. 

An implementation of a PUT instruction might be something like this: 
    
    
        LDA <$12
        ASL A
        ROL <$22
        CLC
        ADC <$11
        STA $1011
        LDA <$22
        ADC <$21
        STA $1021
        LDA <$23
        STA $5801
        INC $1011
        BNE L1
        INC $1021
    L1  LDA <$13
        STA $5801
        IGN $1020
        JMP NXTINST
    

This adds up to 64 cycles. (By contrast my current MMC5 implementation takes up 90 cycles. This comparison isn't particularly good either, because on MMC5 there is also a lot more overhead per instruction.) 

## Oversize/undersize

Oversize extensions are obvious to allow A0 and A1 to be used, allowing up to 256K ROM and 256K RAM. 

Undersize extensions are also possible to mean not all bits of the bankswitching registers will be in use (or some might be ROM only or RAM only). 

## Implementations

  * In C, targeting Nintendulator: [http://forums.nesdev.org/viewtopic.php?f=22&t=12441](http://forums.nesdev.org/viewtopic.php?f=22&t=12441)


