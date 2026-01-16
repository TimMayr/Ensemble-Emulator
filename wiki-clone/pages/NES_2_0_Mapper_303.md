# NES 2.0 Mapper 303

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_303) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_303)

NES 2.0 Mapper 303 is used for Kaiser's ROM cartridge conversion of the Famicom Disk System game _アルマナの奇跡_ (Almana no Kiseki). Its UNIF board name is **UNL-KS7017**. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG Bank Latch ($4A00-$4AFF)
    * 2.2 PRG Bank Latch Commit ($5100)
    * 2.3 Mirroring, IRQ ($4020, $4021, $4025 bit 3: write, $4030 bit 0: read)
  * 3 Note



# Banks

  * CPU $6000-$7FFF: 8 KiB unbanked PRG-RAM
  * CPU $8000-$BFFF: 16 KiB switchable PRG-ROM bank
  * CPU $C000-$FFFF: 16 KiB fixed PRG-ROM bank #2
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM



# Registers

## PRG Bank Latch ($4A00-$4AFF)
    
    
    Mask: probably $FF00
    
    A~FEDC BA98 7654 3210
      -------------------
      0100 1010 .P.. PP..
                 +---++-- Select 16 KiB PRG-ROM bank at CPU $8000-$BFFF
    

The new PRG bank number is not applied until register $5100 is written to. 

## PRG Bank Latch Commit ($5100)
    
    
    Mask: probably $FF00
    

## Mirroring, IRQ ($4020, $4021, $4025 bit 3: write, $4030 bit 0: read)

Identical to their respective equivalents on the [Famicom Disk System](Family_Computer_Disk_System.xhtml "Famicom Disk System"). 

# Note

The conversion retains all writes to the FDS sound channel registers, so an emulator could provide the expansion sound channel even though the original Kaiser cartridge did not. 

Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml)
