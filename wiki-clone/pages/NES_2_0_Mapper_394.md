# NES 2.0 Mapper 394

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_394) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_394)

**NES 2.0 Mapper 394** denotes Realtec's **HSK007** multicart PCB. Unusually, it can be set to function either as an [MMC3](MMC3.xhtml "MMC3") clone or as a [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"). 

## Mode Register ($5009, write)
    
    
    Mask: unknown, but A3 must be set
    
    D~7654 3210
      ---------
      ...M MCBP
         | |||+- PRG A18
         | ||+-- PRG/CHR A19
         | |+--- CHR A18
         +-+---- Mapper Mode
                 0: BNROM
                 1: MMC3
                 2: J.Y. Company ASIC
                 3: Same as 2
    
    Power-on value: $0F
    

## Outer Bank Register ($500B, write)
    
    
    Mask: unknown, but A3 must be set
    
    D~7654 3210
      ---------
      cC.p PBB.
      || | |++-- PRG A16..A15 in BNROM mode
      || | +---- PRG A17 if p=0
      || +------ PRG A17 in MMC3 mode
      ||          0: From Outer Bank Register (128 KiB inner bank)
      ||          1: From MMC3 (256 KiB inner bank)
      |+-------- CHR A17 if c=0
      +--------- CHR A17 in MMC3 mode
                  0: From Outer Bank Register (128 KiB inner bank)
                  1: From MMC3 (256 KiB inner bank)
    
    Power-on value: $90
    

In J.Y. Company ASIC Mode, PRG/CHR A17 always come from the J.Y. Company ASIC. 

Categories: [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multi-ASIC mappers](Category_Multi_ASIC_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
