# INES Mapper 142

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_142) | View [other pages](Special_AllPages.xhtml#INES_Mapper_142)

iNES Mapper 142 represents a PCB used for several of Kaiser's unlicensed ports of FDS games. Its UNIF board name is **UNL-KS7032** , and it features the [KS202 ASIC](VRC3_pinout.xhtml "VRC3 pinout"), which is an upgrade to Konami's [VRC3](VRC3.xhtml "VRC3"). 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 IRQ Reload value ($8000-$BFFF)
    * 2.2 IRQ Enable ($C000-$CFFF)
    * 2.3 IRQ Acknowledge ($D000-$DFFF)
    * 2.4 Bank select ($E000-$EFFF)
    * 2.5 Bank data ($F000-$FFFF)
  * 3 See also



## Banks

  * CPU $6000-$7FFF: 8 KB switchable PRG ROM bank
  * CPU $8000-$9FFF: 8 KB switchable PRG ROM bank
  * CPU $A000-$BFFF: 8 KB switchable PRG ROM bank
  * CPU $C000-$DFFF: 8 KB switchable PRG ROM bank
  * CPU $E000-$FFFF: 8 KB PRG ROM bank, fixed to the last bank.
  * PPU $0000-$1FFF: 8 KB CHR RAM, unbanked



## Registers

### IRQ Reload value ($8000-$BFFF)

Writes set IRQ reload value, in four four-bit words, same as [VRC3](VRC3.xhtml "VRC3")

### IRQ Enable ($C000-$CFFF)

Kaiser's SMB2J writes $02 or $05 to enable, and $00 to disable, indicating that bits 1 and 2 may function similarly to the VRC3. 

### IRQ Acknowledge ($D000-$DFFF)

Any write to this register will acknowledge the pending IRQ, like VRC3. Unlike a VRC3, $C000 bit 0 must not be moved to bit 1, otherwise SMB2J will freeze. 

### Bank select ($E000-$EFFF)

Select one of four bank registers to update on next write to $F000. 
    
    
    7  bit  0
    ---- ----
    .... .RRR
          |||
          +++- Specify which bank register to update on next write to Bank Data register
               1: Select 8 KB PRG bank at CPU $8000-$9FFF
               2: Select 8 KB PRG bank at CPU $A000-$BFFF
               3: Select 8 KB PRG bank at CPU $C000-$DFFF
               4: Select 8 KB PRG bank at CPU $6000-$7FFF
               6: Writes to $F000 have no effect
               0,5,7: unknown
    

### Bank data ($F000-$FFFF)
    
    
    7  bit  0
    ---- ----
    .... DDDD
         ||||
         ++++- New bank value, based on last value written to Bank select register (mentioned above)
    

## See also

  * [PCB photos](https://forums.nesdev.org/viewtopic.php?p=242685#p242685)
  * [iNES Mapper 056](INES_Mapper_056.xhtml "INES Mapper 056") augments this ASIC with some additional hardware



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
