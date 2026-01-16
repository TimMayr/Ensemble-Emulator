# INES Mapper 187

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_187) | View [other pages](Special_AllPages.xhtml#INES_Mapper_187)

iNES Mapper 187 is used for the 卡聖 (Kǎshèng) A98402 and similar boards, an [MMC3](MMC3.xhtml "MMC3")-clone-bearing board with an NROM-like PRG-ROM register that can override the MMC3 clone's PRG-ROM bank, and supports 512 KiB of CHR-ROM. Used by: 

  * _Street Fighter Zero 2_ (Kǎshèng/Hummer Team, not _Street Fighter Zero 2 '97_)
  * _The King of Fighters '96_ (Kǎshèng/Hummer Team)



Only King of Fighters '96 has 512 KiB of CHR-ROM; CHR A18 (the second half of CHR-ROM) is directly selected by PPU A12, regardless of what the MMC3 is set to do. This means that the "left" pattern table always uses the first and the "right" pattern table always uses the second 256 KiB half of CHR-ROM. 

## Contents

  * 1 Registers
    * 1.1 NROM Override Register ($5000)
    * 1.2 Protection Read ($5000-$5FFF)
    * 1.3 MMC3-compatible registers ($8000-$FFFF)
  * 2 Errata
  * 3 Similar Mappers



# Registers

## NROM Override Register ($5000)
    
    
    Mask: $F001
    
    D~7654 3210
      ---------
      M.NB BBb.
      | |+-+++-- Select 16 KiB PRG-ROM bank at CPU
      | |        $8000-$BFFF and $C000-$FFFF
      | +------- 0: Do not replace bit 1 (NROM-128)
      |          1: Replace bit 1 with CPU A14 (NROM-256)
      +--------- 0: Use PRG bank from MMC3; ignore $5000 bits 1-4/5
                 1: Ignore PRG bank from MMC3; apply $5000 bits 1-4/5
    

## Protection Read ($5000-$5FFF)

Mask: Unknown 

The actual values that are returned are unknown; _The King of Fighters '96_ reads from here and only expects bit 7 of the value being returned to be set. 

## MMC3-compatible registers ($8000-$FFFF)

Mask: $E001, see [MMC3](MMC3.xhtml "MMC3"). 

# Errata

  * GoodNES 3.23b has several [INES Mapper 121](INES_Mapper_121.xhtml "INES Mapper 121") and [INES Mapper 189](INES_Mapper_189.xhtml "INES Mapper 189") images erroneously set to 187.



# Similar Mappers

  * iNES Mappers [115](INES_Mapper_115.xhtml "INES Mapper 115") and [123](INES_Mapper_123.xhtml "INES Mapper 123") have similar NROM override registers, with Mapper 123's bits arranged a bit differently.
  * iNES Mapper [121](INES_Mapper_121.xhtml "INES Mapper 121") has the same CHR A18 behavior, drops the NROM override register, and adds more complex protection.



Categories: [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
