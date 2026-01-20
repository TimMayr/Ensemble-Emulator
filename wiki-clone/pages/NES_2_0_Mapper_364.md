# NES 2.0 Mapper 364

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_364) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_364)

**NES 2.0 Mapper 364** is used for an early MMC3-based multicart PCB (**JY830832C**) from J.Y. Company, preceding the [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"). A compatible PCB has the code **9053**. 

  * _1994 Super HiK 3-in-1 (JY-007)_
  * _1996 Super HiK Gold Card (LT-953)_



## Outer Bank Register ($6000-$7FFF)
    
    
    Mask: $E000
    
    D~7654 3210
      ---------
      .PSC ....
       ||+------ CHR A18
       |+------- PRG/CHR A17 mode
       |          0: Preserve MMC3's PRG/CHR A17
       |          1: Force PRG/CHR A17 to 0
       +-------- PRG A18
    

This register is accessible regardless of whether WRAM is enabled in the MMC3 clone's A001 register. It may be possible that there is another bit, unused on the only discovered cartridge using this mapper, that selects the bit value which PRG/CHR A17 is forced to when S=1. 

## MMC3-compatible registers
    
    
    Mask: $E001
    
    See [MMC3](MMC3.xhtml "MMC3").
    

## See also

[NES 2.0 Mapper 361](NES_2_0_Mapper_361.xhtml "NES 2.0 Mapper 361") describes a simpler variant with a fixed outer bank size. 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
