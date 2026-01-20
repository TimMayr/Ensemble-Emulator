# Taito X1-017

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Taito_X1-017) | View [other pages](Special_AllPages.xhtml#Taito_X1_017)

[iNES Mapper 082](Taito_X1_017.xhtml "INES Mapper 082") and [NES 2.0 Mapper 552](Taito_X1_017.xhtml "NES 2.0 Mapper 552") represent boards using [Taito's X1-017 mapper IC](Taito_X1_017_pinout.xhtml "Taito X1-017 pinout"), which provides something a little more sophisticated than [MMC6](MMC6.xhtml "MMC6"). It loses the indirect addressing, adds a 3rd 8 KiB ROM slice, and has 5 KiB of battery-backed RAM. 

The cartridge connector's /IRQ line _is_ connected to the mapper IC, but no commercial games used it. It's only recently been reverse-engineered. 

Comparatively strong pull-downs inside the ASIC mean that there is no functional open bus in games that use this hardware: all reads from locations that would be open bus read back as 0, including disabled RAM and both ASIC and 2A03 registers. 

NES 2.0 Mapper 552 represents the actual way the mask ROM is connected and is thus the correct bank order, while iNES Mapper 082 represents the bank order as it was understood before January 2020 when the mapper was reverse-engineered. 

## Contents

  * 1 Overview
  * 2 Banks
  * 3 Registers
    * 3.1 CHR Select 0 ($7EF0)
    * 3.2 CHR Select 1 ($7EF1)
    * 3.3 CHR Select 2 ($7EF2)
    * 3.4 CHR Select 3 ($7EF3)
    * 3.5 CHR Select 4 ($7EF4)
    * 3.6 CHR Select 5 ($7EF5)
    * 3.7 CHR Mode / Mirroring Control ($7EF6)
    * 3.8 PRG RAM enable 0 ($7EF7)
    * 3.9 PRG RAM enable 1 ($7EF8)
    * 3.10 PRG RAM enable 2 ($7EF9)
    * 3.11 PRG Select 0/1/2 ($7EFA/$7EFB/$7EFC)
    * 3.12 IRQ
      * 3.12.1 IRQ Latch ($7EFD)
      * 3.12.2 IRQ Control ($7EFE)
      * 3.12.3 IRQ Acknowledge and reload ($7EFF)
  * 4 Known Games
  * 5 See Also
  * 6 References



## Overview

  * PRG ROM size: 512 KiB
  * PRG ROM bank size: 8 KiB
  * PRG RAM: Yes, internal, battery backed.
  * CHR capacity: 256 KiB ROM
  * CHR bank size: 1 KiB and 2 KiB
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Controlled by mapper
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): no



## Banks

  * CPU $6000-$73FF: 5 KiB PRG RAM divided into 3 protection regions
  * CPU $7EF0-$7EFF: Mapper registers
  * CPU $8000-$9FFF: 8 KB switchable PRG ROM bank
  * CPU $A000-$BFFF: 8 KB switchable PRG ROM bank
  * CPU $C000-$DFFF: 8 KB switchable PRG ROM bank
  * CPU $E000-$FFFF: 8 KB PRG ROM bank, fixed to the last bank
  * PPU $0000-$07FF (or $1000-$17FF): 2 KB switchable CHR bank
  * PPU $0800-$0FFF (or $1800-$1FFF): 2 KB switchable CHR bank
  * PPU $1000-$13FF (or $0000-$03FF): 1 KB switchable CHR bank
  * PPU $1400-$17FF (or $0400-$07FF): 1 KB switchable CHR bank
  * PPU $1800-$1BFF (or $0800-$0BFF): 1 KB switchable CHR bank
  * PPU $1C00-$1FFF (or $0C00-$0FFF): 1 KB switchable CHR bank



## Registers

### CHR Select 0 ($7EF0)
    
    
    7  bit  0
    CCCC CCC.
    |||| |||
    ++++-+++-- Select 2 KiB CHR ROM at PPU $0000 or $1000
    

### CHR Select 1 ($7EF1)
    
    
    7  bit  0
    CCCC CCC.
    |||| |||
    ++++-+++-- Select 2 KiB CHR ROM at PPU $0800 or $1800
    

### CHR Select 2 ($7EF2)
    
    
    7  bit  0
    CCCC CCCC
    |||| ||||
    ++++-++++- Select 1 KiB CHR ROM at PPU $1000 or $0000
    

### CHR Select 3 ($7EF3)
    
    
    7  bit  0
    CCCC CCCC
    |||| ||||
    ++++-++++- Select 1 KiB CHR ROM at PPU $1400 or $0400
    

### CHR Select 4 ($7EF4)
    
    
    7  bit  0
    CCCC CCCC
    |||| ||||
    ++++-++++- Select 1 KiB CHR ROM at PPU $1800 or $0800
    

### CHR Select 5 ($7EF5)
    
    
    7  bit  0
    CCCC CCCC
    |||| ||||
    ++++-++++- Select 1 KiB CHR ROM at PPU $1C00 or $0C00
    

### CHR Mode / Mirroring Control ($7EF6)
    
    
    7  bit  0
    .... ..CM
           ||
           |+- Mirroring (0:Horizontal, 1:Vertical)
           +-- CHR A12 inversion (0: two 2 KB banks at $0000-$0FFF,
                                     four 1 KB banks at $1000-$1FFF;
                                  1: two 2 KB banks at $1000-$1FFF,
                                     four 1 KB banks at $0000-$0FFF)
    

### PRG RAM enable 0 ($7EF7)
    
    
    7  bit  0
    XXXX XXXX
    |||| ||||
    ++++-++++- Write $CA to enable RAM from $6000 to $67FF, write anything else to disable
    

### PRG RAM enable 1 ($7EF8)
    
    
    7  bit  0
    XXXX XXXX
    |||| ||||
    ++++-++++- Write $69 to enable RAM from $6800 to $6FFF, write anything else to disable
    

### PRG RAM enable 2 ($7EF9)
    
    
    7  bit  0
    XXXX XXXX
    |||| ||||
    ++++-++++- Write $84 to enable RAM from $7000 to $73FF, write anything else to disable
    

### PRG Select 0/1/2 ($7EFA/$7EFB/$7EFC)

Selects the 8 KiB PRG-ROM bank at CPU $8000 ($7EFA)/$A000 ($7EFB)/$C000 ($7EFC). 

iNES Mapper 082: 
    
    
    D~7654 3210
      ---------
      ..DC BA..
        || |+--- PRG A13
        || +---- PRG A14
        |+------ PRG A15
        +------- PRG A16
    

Mapper 82 does not support more than 128KiB of PRG, due to its interpretation of the order of bits in the register. One translation assumes that bit 6 denotes PRG A17, but that bit is not connected to any ASIC pin, and therefore that translation cannot work on real hardware regardless of how the ASIC is connected to ROM. 

NES 2.0 Mapper 552: 
    
    
    D~7654 3210
      ---------
      ..AB CDEF
        || |||+- PRG A18
        || ||+-- PRG A17
        || |+--- PRG A16
        || +---- PRG A15
        |+------ PRG A14
        +------- PRG A13
    

In other words, NES 2.0 Mapper 552 reflects the fact that the PRG mask ROM address lines A13-A18 are connected in reverse order, something previously seen on the [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml#Mode_Select_.28.24D000.29 "J.Y. Company ASIC") in PRG banking mode 3. 

### IRQ

The X1-017's IRQ functionality was not used by the commercial games, and only reverse-engineered in January 2020. No emulators support the IRQ as of January 2020. 

It effectively only provides the ability to schedule an IRQ up to 4100 cycles in the future, short enough that even a top status bar will require a "spacing" IRQ, and its 16x prescaler is too coarse for close raster effects. Furthermore, acknowledging the IRQ reloads the counter, so it subsequent IRQs will tend to creep forward due to the 6502's variable IRQ latency. 

#### IRQ Latch ($7EFD)

An eight-bit wide register, specifying the reload value for the IRQ. 

#### IRQ Control ($7EFE)
    
    
    7  bit  0
    .... .MIC
          |||
          ||+- 1: Enable counting
          ||   0: Disable counting and reload counter, ([$7EFD]+2)*16 if [$7EFD] is nonzero; 17 if it's zero.
          |+-- 1: Enable asserting /IRQ, 0: don't.
          +--- Unknown. Counting only works if this bit is 0.
    

The IRQ will count down while counting is enabled. It's possible to generate multiple IRQs in a row by toggling the "I" bit above before acknowledging the interrupt. 

#### IRQ Acknowledge and reload ($7EFF)

Any write to this register will de-assert IRQ and reload the counter. If [$7EFD] is nonzero, the value reloaded is ([$7EFD]+1)*16. If [$7EFD] is zero, the value reloaded is 1. 

## Known Games

  * SD Keiji: Blader (ＳＤ刑事ブレイダー)
  * Kyuukyoku Harikiri Stadium 1989 Edition (究極ハリキリスタジアム平成元年版)
  * Kyuukyoku Harikiri Stadium III (究極ハリキリスタジアムIII)
  * Kyuukyoku Harikiri Koushien (究極ハリキリ甲子園)



## See Also

  * [Taito X1-005](INES_Mapper_080.xhtml "INES Mapper 080")
  * [Taito X1-005 with alternate mirroring control](INES_Mapper_207.xhtml "INES Mapper 207")



## References

  * BootGod mentions the RAM protect registers: [[1]](https://forums.nesdev.org/viewtopic.php?p=30165#p30165)
  * Krzysiobal reverse-engineered the IRQ: [[2]](https://forums.nesdev.org/viewtopic.php?p=246602#p246602)



Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
