# NES 2.0 Mapper 361

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_361) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_361)

NES 2.0 Mapper 361 is used for an early MMC3-based multicart PCB (YY841101C) from J.Y. Company, preceding the [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"): 

  * _1994 Ball 4-in-1 Series (JY-009)_
  * _1996 Super HiK 4-in-1 (JY-018)_
  * _1995 Super HiK 4-in-1 (JY-019)_
  * _Game 4-in-1 (OK-411)_



## Contents

  * 1 Outer Bank Register ($7000)
  * 2 MMC3-compatible registers
  * 3 Note
  * 4 See also



## Outer Bank Register ($7000)
    
    
    Mask: unknown
    
    D~7654 3210
      ---------
      OOOO ....
      ++++------ Select outer 128 KiB PRG-ROM and CHR-ROM bank
    

This register is accessible regardless of whether WRAM is enabled in the MMC3 clone's A001 register. 

## MMC3-compatible registers
    
    
    Mask: $E001
    
    See [MMC3](MMC3.xhtml "MMC3").
    

## Note

[FCEUX emulates this PCB under mapper number 205](https://github.com/TASVideos/fceux/blob/master/src/boards/mmc3.cpp) as "OK-411" together with the GN-45 PCB, both of which are in turn different from the [iNES Mapper 205](INES_Mapper_205.xhtml "INES Mapper 205") described in that mapper's wiki article. 

## See also

[NES 2.0 Mapper 364](NES_2_0_Mapper_364.xhtml "NES 2.0 Mapper 364") describes a slightly more complex variant supporting a selectable outer bank size. 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
