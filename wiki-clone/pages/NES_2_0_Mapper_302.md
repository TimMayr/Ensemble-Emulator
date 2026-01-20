# NES 2.0 Mapper 302

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_302) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_302)

NES 2.0 Mapper 302 is used for Kaiser's ROM cartridge conversion of the Japanese Famicom Disk System version of _Gyruss_. Its UNIF board name is **UNL-KS7057**. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Mirroring select ($8000-$9FFF)
    * 2.2 PRG-ROM Bank ($B000-$E003)



# Banks

  * CPU $6000-$67FF: 2 KiB switchable PRG-ROM bank
  * CPU $6800-$6FFF: 2 KiB switchable PRG-ROM bank
  * CPU $7000-$77FF: 2 KiB switchable PRG-ROM bank
  * CPU $7800-$7FFF: 2 KiB switchable PRG-ROM bank
  * CPU $8000-$87FF: 2 KiB switchable PRG-ROM bank
  * CPU $8800-$8FFF: 2 KiB switchable PRG-ROM bank
  * CPU $9000-$97FF: 2 KiB switchable PRG-ROM bank
  * CPU $9800-$9FFF: 2 KiB switchable PRG-ROM bank
  * CPU $A000-$BFFF: 8 KiB fixed PRG-ROM bank #$0D
  * CPU $C000-$FFFF: 16 KiB fixed PRG-ROM bank #$07
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM



# Registers

## Mirroring select ($8000-$9FFF)
    
    
    Mask: unknown, probably $E000
    
    D~7654 3210
      ---------
      .... ...M
              +- Select nametable mirroring type
                 0: Vertical
                 1: Horizontal
    

## PRG-ROM Bank ($B000-$E003)
    
    
    Mask: unknown, probably $F003
    
    A~FEDC BA98 7654 3210
      -------------------
      1RRR .... .... ..RN
       |||             |+- 0: Select low nibble
       |||             |   1: Select high nibble
       +++-------------+-- Select register number
                           $B000/$B001: Set 2 KiB PRG-ROM bank at CPU $8000-$87FF
                           $B002/$B003: Set 2 KiB PRG-ROM bank at CPU $8800-$8FFF
                           $C000/$C001: Set 2 KiB PRG-ROM bank at CPU $9000-$97FF
                           $C002/$C003: Set 2 KiB PRG-ROM bank at CPU $9800-$9FFF
                           $D000/$D001: Set 2 KiB PRG-ROM bank at CPU $6000-$67FF
                           $D002/$D003: Set 2 KiB PRG-ROM bank at CPU $6800-$6FFF
                           $E000/$E001: Set 2 KiB PRG-ROM bank at CPU $7000-$77FF
                           $E002/$E003: Set 2 KiB PRG-ROM bank at CPU $7800-$7FFF
    

Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml)
