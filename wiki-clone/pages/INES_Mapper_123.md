# INES Mapper 123

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_123) | View [other pages](Special_AllPages.xhtml#INES_Mapper_123)

iNES Mapper 123 is used for the 卡聖 (Kǎshèng) H2288 board, an [MMC3](MMC3.xhtml "MMC3")-clone-bearing board with scrambled register indices, and an NROM-like PRG-ROM register that can override the MMC3 clone's PRG-ROM bank. Its UNIF board name is **UNL-H2288**. Used by: 

  * _Mortal Kombat 3_ (Kǎshèng/Hummer Team, not to be confused with _Mortal Kombat 3_ from SuperGame)
  * _Earthworm Jim 2_ (Kǎshèng/Shin-Shin Electronics, not to be confused with _Earthworm Jim 2_ from SuperGame)



## Contents

  * 1 Registers
    * 1.1 NROM Override Register ($5800)
    * 1.2 MMC3-compatible registers ($8000-$FFFF)
  * 2 Similar Mappers



# Registers

## NROM Override Register ($5800)
    
    
    Mask: $F800
    
    D~7654 3210
      ---------
      .M31 .2S0
       +++--+|+- Select 16 KiB PRG-ROM bank at CPU
       |     |   $8000-$BFFF and $C000-$FFFF (notice the bit order)
       |     +-- 0: Do not replace bit 0 (NROM-128)
       |         1: Replace bit 0 with CPU A14 (NROM-256)
       +-------- 0: Use PRG bank from MMC3; ignore $5800 bits 0-3/5
                 1: Ignore PRG bank from MMC3; apply $5800 bits 0-3/5
    

## MMC3-compatible registers ($8000-$FFFF)
    
    
    $8000 index written   MMC3 register meant
    -------------------   -------------------
    0                     0
    1                     3
    2                     1
    3                     5
    4                     6
    5                     7
    6                     2
    7                     4
    

Bits 6 and 7 of the value written to $8000 are unchanged. See [MMC3](MMC3.xhtml "MMC3") for further details. 

# Similar Mappers

  * iNES Mappers [114](INES_Mapper_114.xhtml "INES Mapper 114") and [115](INES_Mapper_115.xhtml "INES Mapper 115") move the $5800 register to $6000 while rearranging its bits and add an outer CHR-ROM bank register at $6001. Mapper 114 keeps the MMC3 index register scrambling (using the same pattern) and adds register address scrambling, while Mapper 115 drops all scrambling.



Categories: [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
