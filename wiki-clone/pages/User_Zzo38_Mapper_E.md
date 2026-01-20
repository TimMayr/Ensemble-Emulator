# User:Zzo38/Mapper E

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Mapper_E) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Mapper_E)

  
This is a mapper for multigame cartridges, although it has other features useful for other purposes, too. Compare also [iNES Mapper 028](Action_53_mapper.xhtml "INES Mapper 028"). 

You may be able to make multiple programs using mappers such as: 

  * [NROM](NROM.xhtml "NROM")
  * [CNROM](CNROM.xhtml "CNROM")
  * [GNROM](GxROM.xhtml "GNROM")
  * [UNROM](UxROM.xhtml "UNROM")
  * [UxROM](UxROM.xhtml "UxROM")
  * [AxROM](AxROM.xhtml "AxROM")
  * [BNROM](INES_Mapper_034.xhtml "BNROM")
  * [MHROM](GxROM.xhtml "MHROM")
  * [iNES Mapper 029](INES_Mapper_029.xhtml "INES Mapper 029")
  * [iNES Mapper 038](INES_Mapper_038.xhtml "INES Mapper 038")
  * [iNES Mapper 070](INES_Mapper_070.xhtml "INES Mapper 070")
  * [iNES Mapper 078](INES_Mapper_078.xhtml "INES Mapper 078")
  * [iNES Mapper 087](INES_Mapper_087.xhtml "INES Mapper 087")
  * [iNES Mapper 089](INES_Mapper_089.xhtml "INES Mapper 089")
  * [UN1ROM](INES_Mapper_094.xhtml "INES Mapper 094")
  * [iNES Mapper 101](INES_Mapper_101.xhtml "INES Mapper 101")
  * [iNES Mapper 107](INES_Mapper_107.xhtml "INES Mapper 107")
  * [iNES Mapper 140](INES_Mapper_140.xhtml "INES Mapper 140")
  * [iNES Mapper 152](INES_Mapper_152.xhtml "INES Mapper 152")
  * [iNES Mapper 184](INES_Mapper_184.xhtml "INES Mapper 184")
  * [iNES Mapper 203](INES_Mapper_203.xhtml "INES Mapper 203")
  * [iNES Mapper 218](INES_Mapper_218.xhtml "INES Mapper 218")
  * [Color Dreams](Color_Dreams.xhtml "Color Dreams")
  * [JF-13](INES_Mapper_086.xhtml "INES Mapper 086")



Depending on ROM/RAM sizes and submapper numbers, it might not be possible to use all combinations. 

Expansion audio is not supported. Audio registers in the mapper should not be written to, since they might conflict with others in this mapper. 

Some mappers may have bus conflicts, but this one doesn't, so programs relying on bus conflicts will not run properly. 

## Contents

  * 1 Registers
    * 1.1 Bankswitching masks
    * 1.2 Control register
    * 1.3 User register
  * 2 Supervisor ROM
  * 3 PRG RAM
  * 4 Trainer ROM
  * 5 ROM/RAM sizes
    * 5.1 Type 5,0,0
  * 6 Examples



## Registers

### Bankswitching masks
    
    
    [0101 .... .... ..xy]
    

Writing to these four registers (of eight bits each, regardless of ROM size) controls the bankswitching masks: 

  * The "x" bit is 0 for PRG banks or 1 for CHR banks (CHR banks can be ROM or RAM; see below section).
  * The "y" bit is 0 for AND masks or 1 for XOR masks.



This produces four ports visible to the program: 

  * $5000: PRG AND mask
  * $5001: PRG XOR mask
  * $5002: CHR AND mask
  * $5003: CHR XOR mask



### Control register
    
    
    [0101 .... .... .1..]
    

Writing to this register will write the control register (one of the mask registers will also be set at the same time). 
    
    
     7654 3210
    [BBUE EEMM]
     |||| ||||
     |||| ||++- Nametable mirroring
     |||+-++--- CIRAM enable
     ||+------- User register at $6000-$7FFF
     ++-------- PRG bank mode
    

The meaning of nametable mirroring controls is determining what CIRAM A10 is connected to: 

  * 00 = PA10
  * 01 = PA11
  * 10 = PA12
  * 11 = PA13 XOR bit4 of user register (bit3 instead, if bit3 of submapper number is set)



The CIRAM enable bits are as follows (if the bit is set, CIRAM is enabled): 

  * bit2 = PPU address $0000-$0FFF
  * bit3 = PPU address $1000-$1FFF
  * bit4 = PPU address $2000-$3FFF



The user registers at $6000-$7FFF means, if this bit is set, writes to $6000-$FFFF will write the user register; if cleared, the user register is at $8000-$FFFF. 

The PRG bank mode can be: 

  * 00 = 32K in $8000-$FFFF; let PRG ROM A14 signal from cartridge connector signal
  * 01 = Fixed bank (as if user register = $FF) in $C000-$FFFF, switchable bank in $8000-$BFFF, fix PRG ROM A14 high
  * 10 = Fixed bank (as if user register = $FF) in $C000-$FFFF, switchable bank in $8000-$BFFF, fix PRG ROM A14 low
  * 11 = Fixed bank (as if user register = $00) in $8000-$BFFF, switchable bank in $C000-$FFFF, fix PRG ROM A14 high



### User register

The user register is mapped at either $6000-$FFFF or $8000-$FFFF. There is no bus conflicts. Calling the registers "pa" (PRG AND), "px" (PRG XOR), "ca" (CHR AND), "cx" (CHR XOR), and "u" (User), the bank selection is as follows: 

  * 32K PRG bank at $8000-$FFFF: (u AND pa) XOR px
  * 8K CHR bank at $0000-$1FFF and $2000-$3FFF: (u AND ca) XOR cx



If the ROM is small enough that eight bits are not used, the bits of the result are ORed together to determine what bank to select. 

## Supervisor ROM

Reading from $5000-$5FFF reads from PRG ROM as if the PRG AND mask and PRG XOR mask are both zero (then it will be the same data at $D000-$DFFF). 

## PRG RAM

There is 8K PRG RAM at $6000-$7FFF (optional feature). Sometimes the user register is also mapped here, but it might not be. There is no bus conflicts. 

## Trainer ROM

If trainer ROM and PRG RAM are both present, then the $5000-$5FFF area is the trainer ROM (mirrored to fill the entire address range). This may be used to program individual games which are on such a cartridge, so that they can still be tested in the emulator, and then later you can combine them into one without a trainer ROM. 

## ROM/RAM sizes

PRG bankswitching is ROM only, although CHR may be ROM and/or RAM. Subtypes of bankswitching are bit1 of the submapper number for PRG, and bit0 for CHR. 

In the table below, "ROM" and "RAM" means the number of bits needed to select the bank; "-" means the ROM/RAM is not present. The column "sub" means the submapper number. 
    
    
     ROM   RAM   sub   calc
       -     0     0   No bankswitching
       -     0     1   MIR=[7]
       -     1     0   RAM=[76543210]
       -     2     0   RAM=[7531 6420]
       -     2     1   RAM=[7654 3210]
       0     -     0   No bankswitching
       0     -     1   SC=[7]
       0     0     0   EN=[76543210]
       0     0     1   Split ROM=[7531] RAM=[6420]
       0     1     0   EN=[7] RAM=[6543210]
       0     1     1   EN=[0] RAM=[7654321]
       1     -     0   ROM=[76543210]
       1     0     0   EN=[75] ROM=[643210]
       1     0     1   Split ROM=[64 210] RAM=[7531]
       2     -     0   ROM=[7531 6420]
       2     -     1   ROM=[7654 3210]
       3     -     0   ROM=[70 642 531]
       3     -     1   ROM=[72 641 530]
       4     -     0   ROM=[73 62 51 40]
       4     -     1   ROM=[75 64 31 20]
       4     0     0   EN=[76] ROM=[3 2 51 40]
       4     0     1   EN=[75] ROM=[63 2 51 40]
       4     1     0   EN=[76] ROM=[3 2 51 40] RAM=[543210]
       4     1     1   EN=[75] ROM=[63 2 51 40] RAM=[743210]
       4     2     0   EN=[76] ROM=[3 2 51 40] RAM=[542 310]
       4     2     1   EN=[76] ROM=[3 2 51 40] RAM=[531 420]
       4     3     0   EN=[76] ROM=[3 2 51 40] RAM=[52 41 30]
       5     -     0   ROM=[7 3 62 51 40]
       5     -     1   ROM=[7 0 64 53 21]
       5     0     0   Special
       6     -     0   ROM=[7 6 5 42 31 0]
       6     -     1   ROM=[7 6 53 42 1 0]
       6     0     0   EN=[6] ROM=[5 4 3 2 1 0] SC=[7]
       7     -     0   ROM=[7 6 5 43 2 1 0]
       7     -     1   ROM=[7 65 4 3 2 1 0]
       7     0     0   EN=[7] ROM=[6 5 4 3 2 1 0]
       7     0     1   EN=[0] ROM=[7 6 5 4 3 2 1]
       7     1     0   EN=[7] ROM=[6 5 4 3 2 1 0] RAM=[6543210]
       7     1     1   EN=[0] ROM=[7 6 5 4 3 2 1] RAM=[7654321]
       7     2     0   EN=[7] ROM=[6 5 4 3 2 1 0] RAM=[641 5320]
       7     2     1   EN=[0] ROM=[7 6 5 4 3 2 1] RAM=[652 7431]
       8     -     0   ROM=[7 6 5 4 3 2 1 0]
       8     -     1   ROM=[7 6 5 4 3 2 1 0] MIR=[3]
    

Notation in brackets means, any numbers without spaces between means those bits are ORed together; if there are spaces, that designates the separate bits of the bank number. "EN" means if RAM is enabled. "MIR" means the bit0 of the control register will be XOR by this value. "SC" means if nametable mirroring is set to PA12, it instead uses this value for CIRAM A10. "Split" means $0xxx is ROM, $1xxx is RAM, and 4K banks are used. 

### Type 5,0,0

This is a special type, acting differently than the above, as follows: 

  * CHR-RAM is enabled if bit5 of control register is cleared.
  * The CHR banks in ROM mode are 4K each.
  * ROM bank at $0xxx: `ROM=[p q 7 6 5 4]`
  * ROM bank at $1xxx: `ROM=[p q 3 2 1 0]`
  * In the above, "p" and "q" are bit1 and bit0 of the PRG bank number.



## Examples
