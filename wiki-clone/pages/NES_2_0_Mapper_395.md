# NES 2.0 Mapper 395

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_395) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_395)

**NES 2.0 Mapper 395** denotes the [MMC3](MMC3.xhtml "MMC3")-compatible **Realtec 8210** multicart PCB. Both outer bank registers are selected through the MMC3's WRAM interface and overlay any actual WRAM. 

  * _Super Card 12-in-1_ (SPC002)
  * _Super Card 13-in-1_ (SPC003)
  * _Super Card 14-in-1_ (King006)
  * _Super Card 14-in-1_ (King007)



## Outer Bank Register 1 ($6000)
    
    
    Mask: $E010
    
    D~7654 3210
      ---------
      ..BA C...
        || +---- PRG A20
        ++------ PRG/CHR A19..18
        
    

## Outer Bank Register 2 ($6010)
    
    
    Mask: $E010
    
    D~7654 3210
      ---------
      LcCA p..B
      |||| |  +- PRG A17 if p=1
      |||| +---- 0=PRG A17 from MMC3 (256 KiB inner PRG bank)
      ||||       1=PRG A17 from B (128 KiB inner PRG bank)
      |||+------ CHR A17 if c=1
      ||+------- CHR A20
      |+-------- 0=CHR A17 from MMC3 (256 KiB inner CHR bank)
      |          1=CHR A17 from A (128 KiB inner CHR bank)
      +--------- 1=Lock outer bank
    

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
