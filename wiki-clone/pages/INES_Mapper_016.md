# INES Mapper 016

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_016) | View [other pages](Special_AllPages.xhtml#INES_Mapper_016)

iNES Mapper 016 is used for some of the [Bandai FCG boards](Bandai_FCG_board.xhtml "Bandai FCG board"), namely, boards with the [FCG-1 ASIC](Bandai_FCG_pinout.xhtml "Bandai FCG pinout") that supports no EEPROM, and the [LZ93D50 ASIC](Bandai_LZ93D50_pinout.xhtml "Bandai LZ93D50 pinout") with no or 256 bytes of EEPROM. 

[INES Mapper 016 submapper table](INES_Mapper_016_Submapper_table.xhtml "INES Mapper 016/Submapper table")  
---  
Submapper # | Meaning | Note   
0 | Unspecified | Emulate both FCG-1/2 and LZ93D50 chips in their respective CPU address ranges.   
_1_ | _LZ93D50 with 128 byte serial EEPROM (24C01)_ | _Deprecated, use[INES Mapper 159](INES_Mapper_159.xhtml "INES Mapper 159") instead._  
_2_ | _Datach Joint ROM System_ | _Deprecated, use[INES Mapper 157](INES_Mapper_157.xhtml "INES Mapper 157") instead._  
_3_ | _8 KiB of WRAM instead of serial EEPROM_ | _Deprecated, use[INES Mapper 153](INES_Mapper_153.xhtml "INES Mapper 153") instead._  
4 | FCG-1/2 | Responds only in the CPU $6000-$7FFF address range; IRQ counter is not latched.   
5 | LZ93D50 with no or 256-byte serial EEPROM (24C02) | Responds only in the CPU $8000-$FFFF address range; IRQ counter is latched.   
  
## Contents

  * 1 Game List
  * 2 Banks
  * 3 Registers
    * 3.1 Read Serial EEPROM ($6000-$7FFF read, **Submapper 5** only)
    * 3.2 CHR-ROM Bank Select ($6000-$6007 write, **Submapper 4** ; $8000-$8007 write, **Submapper 5**)
    * 3.3 PRG-ROM Bank Select ($6008 write, **Submapper 4** ; $8008 write, **Submapper 5**)
    * 3.4 Nametable Mirroring Type Select ($6009 write, **Submapper 4** ; $8009 write, **Submapper 5**)
    * 3.5 IRQ Control ($600A write, **Submapper 4** ; $800A write, **Submapper 5**)
    * 3.6 IRQ Latch/Counter ($600B-$600C write, **Submapper 4** ; $800B-$800C write, **Submapper 5**)
    * 3.7 EEPROM Control ($800D write, **Submapper 5** only)
  * 4 Similar Mappers
  * 5 See Also



# Game List

Name | Chip | EEPROM | NES 2.0 Submapper | [NES 2.0 Byte 10](NES_2_0.xhtml#Byte_10_.28RAM_size.29 "NES 2.0")  
---|---|---|---|---  
_Akuma-kun: Makai no Wana_ | FCG-2 | - | 4 | $00   
_Crayon Shin-chan: Ora to Poi Poi_ | LZ93D50 | - | 5 | $00   
_Dragon Ball: Daimaou Fukkatsu_ | FCG-1 | - | 4 | $00   
_Dragon Ball 3: Gokuu Den_ | FCG-2 | - | 4 | $00   
_Dragon Ball Z II: Gekishin Freezer!!_ | LZ93D50 | 24C02 | 5 | $20   
_Dragon Ball Z III: Ressen Jinzou Ningen_ | LZ93D50 | 24C02 | 5 | $20   
_Dragon Ball Z Gaiden: Saiya-jin Zetsumetsu Keikaku_ | LZ93D50 | 24C02 | 5 | $20   
_Famicom Jump: Hero Retsuden_ | FCG-2 | - | 4 | $00   
_Meimon! Dai-3 Yakyuu-bu_ | FCG-1 | - | 4 | $00   
_Nishimura Kyoutarou Mystery: Blue Train Satsujin Jiken_ | FCG-1 | - | 4 | $00   
_Rokudenashi Blues_ | LZ93D50 | 24C02 | 5 | $20   
_Sakigake!! Otoko Juku: Shippu 1-gou Sei_ | FCG-1 | - | 4 | $00   
_SD Gundam Gaiden - Knight Gundam Monogatari 2: Hikari no Kishi_ | LZ93D50 | 24C02 | 5 | $20   
_SD Gundam Gaiden - Knight Gundam Monogatari 3: Densetsu no Kishidan_ | LZ93D50 | 24C02 | 5 | $20   
  
# Banks

  * CPU $8000-$BFFF: 16 KiB switchable PRG ROM bank
  * CPU $C000-$FFFF: 16 KiB PRG ROM bank, fixed to the last bank
  * PPU $0000-$03FF: 1 KiB switchable CHR ROM bank
  * PPU $0400-$07FF: 1 KiB switchable CHR ROM bank
  * PPU $0800-$0BFF: 1 KiB switchable CHR ROM bank
  * PPU $0C00-$0FFF: 1 KiB switchable CHR ROM bank
  * PPU $1000-$13FF: 1 KiB switchable CHR ROM bank
  * PPU $1400-$17FF: 1 KiB switchable CHR ROM bank
  * PPU $1800-$1BFF: 1 KiB switchable CHR ROM bank
  * PPU $1C00-$1FFF: 1 KiB switchable CHR ROM bank



# Registers

## Read Serial EEPROM ($6000-$7FFF read, **Submapper 5** only)
    
    
    Mask: $E000
    
    7  bit  0
    ---- ----
    xxxE xxxx
    |||| ||||
    +++|-++++- Open bus
       +------ Data out from I²C EEPROM
    

## CHR-ROM Bank Select ($6000-$6007 write, **Submapper 4** ; $8000-$8007 write, **Submapper 5**)
    
    
    Mask: $E00F (**Submapper 4**), $800F (**Submapper 5**)
    
    7  bit  0
    ---- ----
    CCCC CCCC
    |||| ||||
    ++++-++++-- 1 KiB CHR-ROM bank number
    

  * $xxx0: Select 1 KiB CHR-ROM bank at PPU $0000-$03FF
  * $xxx1: Select 1 KiB CHR-ROM bank at PPU $0400-$07FF
  * $xxx2: Select 1 KiB CHR-ROM bank at PPU $0800-$0BFF
  * $xxx3: Select 1 KiB CHR-ROM bank at PPU $0C00-$0FFF
  * $xxx4: Select 1 KiB CHR-ROM bank at PPU $1000-$13FF
  * $xxx5: Select 1 KiB CHR-ROM bank at PPU $1400-$17FF
  * $xxx6: Select 1 KiB CHR-ROM bank at PPU $1800-$1BFF
  * $xxx7: Select 1 KiB CHR-ROM bank at PPU $1C00-$1FFF



## PRG-ROM Bank Select ($6008 write, **Submapper 4** ; $8008 write, **Submapper 5**)
    
    
    Mask: $E00F (**Submapper 4**), $800F (**Submapper 5**)
    
    7  bit  0
    ---- ----
    .... PPPP
         ||||
         ++++-- Select 16 KiB PRG-ROM bank at CPU $8000-$BFFF   
    

## Nametable Mirroring Type Select ($6009 write, **Submapper 4** ; $8009 write, **Submapper 5**)
    
    
    Mask: $E00F (**Submapper 4**), $800F (**Submapper 5**)
    
    7  bit  0
    ---- ----
    .... ..MM
           ||
           ++-- Select nametable mirroring type
                 0: Vertical
                 1: Horizontal
                 2: One-screen, page 0
                 3: One-screen, page 1
    

## IRQ Control ($600A write, **Submapper 4** ; $800A write, **Submapper 5**)
    
    
    Mask: $E00F (**Submapper 4**), $800F (**Submapper 5**)
    
    7  bit  0
    ---- ----
    .... ...C
            |
            +-- IRQ counter control
                 0: Counting disabled
                 1: Counting enabled
    

  * Writing to this register acknowledges a pending IRQ.
  * On the LZ93D50 (**Submapper 5**), writing to this register also copies the latch to the actual counter.
  * If a write to this register enables counting while the counter is holding a value of zero, an IRQ is generated immediately.



## IRQ Latch/Counter ($600B-$600C write, **Submapper 4** ; $800B-$800C write, **Submapper 5**)
    
    
    Mask: $E00F (**Submapper 4**), $800F (**Submapper 5**)
    
       $C         $B
    7  bit  0  7  bit  0
    ---- ----  ---- ----
    CCCC CCCC  CCCC CCCC
    |||| ||||  |||| ||||
    ++++-++++--++++-++++-- Counter value (little-endian)
    

  * If counting is enabled, the counter decreases on every M2 cycle. When it holds a value of zero, an IRQ is generated.
  * On the FCG-1/2 (**Submapper 4**), writing to these two registers directly modifies the counter itself; all such games therefore disable counting before changing the counter value.
  * On the LZ93D50 (**Submapper 5**), these registers instead modify a latch that will only be copied to the actual counter when register $800A is written to.



## EEPROM Control ($800D write, **Submapper 5** only)
    
    
    Mask: $800F
    
    7  bit  0
    ---- ----
    RDC. ....
    |||
    ||+-------- I²C SCL
    |+--------- I²C SDA
    +---------- Direction bit (1=Enable Read)
    

  * This register only exists on the LZ93D50 (Submapper 5), and only has an effect if a 24C02 EEPROM is present.
  * Please refer to generic I²C tutorials and the 24C02 datasheet on how to operate or emulate this register correctly.



# Similar Mappers

  * [INES Mapper 153](INES_Mapper_153.xhtml "INES Mapper 153") replaces the serial EEPROM with 8 KiB of battery-backed WRAM, and uses CHR-RAM instead of CHR-ROM.
  * [INES Mapper 157](INES_Mapper_157.xhtml "INES Mapper 157") adds a barcode reader, support for a second EEPROM, and uses CHR-RAM instead of CHR-ROM.
  * [INES Mapper 159](INES_Mapper_159.xhtml "INES Mapper 159") replaces the 256 byte with an 128 byte serial EEPROM.



# See Also

A discrete logic clone of the FCG-2 has been found and [reverse-engineered by Krzysiobal](https://forums.nesdev.org/viewtopic.php?p=243449#p243449)

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
