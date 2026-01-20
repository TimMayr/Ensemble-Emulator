# NES 2.0 Mapper 366

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_366) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_366)

NES 2.0 Mapper 366 is used for an MMC3-based multicart PCB (GN-45): 

  * _4-in-1 (K-3131GS)_
  * _4-in-1 (K-3131SS)_



## Contents

  * 1 Outer Bank Register ($6800)
  * 2 MMC3-compatible registers
  * 3 Note
  * 4 See also



## Outer Bank Register ($6800)
    
    
    Mask: unknown, probably $E0F0
    
    A~FEDC BA98 7654 3210
      -------------------
      0110 1000 LOOO ....
                |+++------ Select outer 128 KiB PRG-ROM and CHR-ROM bank
                +--------- 1=Lock outer bank
    

This register is accessible regardless of whether WRAM is enabled in the MMC3 clone's A001 register. It overlays the 8 KiB of WRAM that both multicarts have, making the Lock bit necessary to prevent the outer bank from changing once the game writes to WRAM. 

## MMC3-compatible registers
    
    
    Mask: $E001
    
    See [MMC3](MMC3.xhtml "MMC3").
    

## Note

[FCEUX emulates this PCB under mapper number 205](https://github.com/TASVideos/fceux/blob/master/src/boards/mmc3.cpp) together with "OK-411" ([NES 2.0 Mapper 361](NES_2_0_Mapper_361.xhtml "NES 2.0 Mapper 361")), both of which are in turn different from the [iNES Mapper 205](INES_Mapper_205.xhtml "INES Mapper 205") described in that mapper's wiki article. 

## See also

[NES 2.0 Mapper 361](NES_2_0_Mapper_361.xhtml "NES 2.0 Mapper 361") describes a simpler variant that latches the data bus instead of the address bus. 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
