# NES 2.0 Mapper 550

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_550) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_550)

**NES 2.0 Mapper 550** denotes the **JY820845C** circuit board, used by the _7-in-1 1993 Chess Series (JY-015)_ multicart. It combines an [MMC1](MMC1.xhtml "MMC1") clone with a PAL for outer bank and optional MHROM inner bank switching. 

## Outer Bank Register ($7000-$7FFF, write)
    
    
    Mask: $F000
    
    A~FEDC BA98 7654 3210
      -------------------
      0111 .... .... LBBb
                     |+++- PRG A18..16 (MHROM mode)
                     |++-- PRG A18..17 (MMC1 mode)
                     |++-- CHR A16..15
                     |++-- 0-2: MHROM mode
                     |     3:   MMC1 mode
                     +---- 1: Lock outer bank register
    

In GNROM mode, the latch register selects the inner 2x32 KiB PRG and 4x8 CHR banks. In MMC1 mode, the MMC1 selects the inner 4x32/8x16 KiB PRG and 4x8/8x4 KiB CHR banks. The Outer Bank Register overlaps 8 KiB of WRAM in the $6000-$7FFF address range. 

## MHROM Latch Register ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    D~7654 3210
      ---------
      ...P ..CC
         |   ++- CHR A14..13
         +------ PRG A15     
    

The MMC1 still sees the writes to the latch register in MHROM; as a result, the menu writes to the latch register five times to keep the MMC1's serial shift register in a consistent state, at least until the Outer Bank Register's Lock bit is set. It is not known whether the MHROM Latch Register suffers from bus conflicts. 

Nametable mirroring is decided the by MMC1 even in MHROM mode. 

Categories: [GNROM-like mappers](Category_GNROM_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
