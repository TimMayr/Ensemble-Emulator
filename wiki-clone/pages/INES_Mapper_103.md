# INES Mapper 103

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_103) | View [other pages](Special_AllPages.xhtml#INES_Mapper_103)

iNES Mapper 103 describes a specific pirate port of the FDS version of Doki Doki Panic. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG bank ($8000-$8FFF)
    * 2.2 Mirroring control ($E000-$EFFF)
    * 2.3 RAM disable ($F000-$FFFF)
  * 3 Hardware
  * 4 See also



## Banks

  * CPU $6000-$7FFF: 8 KiB RAM, or 8 KiB controllable ROM bank.
  * CPU $8000-$B7FF: 14 KiB ROM, starting at ROM offset 0x18000
  * CPU $B800-$D7FF: 8 KiB RAM, or 8 KiB fixed ROM bank, starting at ROM offset 0x1B800
  * CPU $D800-$FFFF: 10 KiB ROM, starting at ROM offset 0x1D800



Writes to regions where RAM can be mapped will always write to RAM, even if RAM isn't enabled for reading. 

## Registers

Registers are not cleared on reset, and state is unknown on power-up. No bus conflicts. 

### PRG bank ($8000-$8FFF)

Mask: $F000 
    
    
    7  bit  0
    ---- ----
    .... PPPP
         ||||
         ++++-- select 8 KiB PRG bank at $6000
    

### Mirroring control ($E000-$EFFF)

Mask: $F000 
    
    
    7  bit  0
    ---- ----
    .... m...
         |
         +----- 0=V (PPU A10), 1=H (PPU A11)
    

### RAM disable ($F000-$FFFF)

Mask: $F000 
    
    
    7  bit  0
    ---- ----
    ...d ....
       |
       +------- 0=RAM available at $6000-$7FFF and $B800-$D7FF
                1=ROM instead
    

## Hardware

This hardware involves three [7400](7400.xhtml "7400"), two [7410](7410.xhtml "7410"), a [7432](7432.xhtml "7432"), a [7474](7474.xhtml "7474"), a [74138](74138.xhtml "74138"), a [74157](74157.xhtml "74157"), and a [74161](74161.xhtml "74161"). 

## See also

  * [Krzysiobal's reverse engineered documentation](https://forums.nesdev.org/viewtopic.php?f=9&t=18956)



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
