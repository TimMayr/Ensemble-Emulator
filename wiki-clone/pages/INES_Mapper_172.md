# INES Mapper 172

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_172) | View [other pages](Special_AllPages.xhtml#INES_Mapper_172)

INES Mapper 172 is used to denote the Super Mega P-4070 board, used by the following games: 

  * _1991 賭馬 Racing_ (1991 Dǔmǎ Racing, "Enjoyable Horse Racing")
  * _麻将方块_ (Mahjong Block) (original release)
  * _Venice Beach Volley_ (Super Mega release)



## Contents

  * 1 Banks
  * 2 Registers
  * 3 Errata
  * 4 Similar Mappers
  * 5 See also



## Banks

  * PPU $0000-$1FFF: 8 KB switchable CHR ROM bank



## Registers

Mapper 172 uses a custom IC (marked "JV001") serving as a latch, adder and inverter. There are five registers: Input (6 bits), Output (6 bits), Register (6 bits), Mode (1 bit) and Invert (1 bit). 
    
    
    Mask: $E103
    Read $4100-$4103: [..RR RRRR]: Read Register. Bits 4-5 are inverted if Invert==1. 
                      Bits 6-7 are open bus. Note that the bit order D0-D5 is _reversed_.
    Write $4100: When Mode==0: Bits 0-5 of Register := Input, bits 0-3 being inverted if Invert==1.
                 When Mode==1: Bits 0-3 of Register incremented by one, bits 4-5 unaffected.
    Write $4101: Invert := Written value bit 5.
    Write $4102: Input := Written value bits 0-5. Note that the bit order D0-D5 is _reversed_.
    Write $4103: Mode := Written value bit 5.
    Write $8000-$FFFF: Output := Register, and nametable mirroring is changed; written value is ignored.
    

In Mapper 172, bits 0/1 of the 8 KiB CHR ROM bank number (CHR A13/A14) come from Output 5/4, or 0/1 after taking the reverse bit order into account: 
    
    
    8 KiB CHR-ROM bank number := (Output &3);
    

Games will check the lower six bits of $4100 for the correct value after several increment and inversion operations as a copy-protection measure. Unlike most of the other boards using the JV001/TXC 05-00002-010 ICs, this board allows nametable mirroring to be controlled by software. When writing to $8000-$FFFF, nametable mirroring is changed to Horizontal if Invert was clear, and to Vertical if Invert was set. 

## Errata

  * The TXC re-release of _麻将方块 (Mahjong Block)_ uses [INES Mapper 132](INES_Mapper_132.xhtml "INES Mapper 132").



## Similar Mappers

The board is identical to [INES Mapper 136](INES_Mapper_136.xhtml "INES Mapper 136") except that the CPU bits D0-D5 are connected in reverse order, and that Mapper 136 has hard-wired nametable mirroring. 

## See also

[PCB images](http://forums.nesdev.org/viewtopic.php?f=3&t=15961&p=213362#p210703)
