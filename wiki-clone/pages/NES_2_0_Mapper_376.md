# NES 2.0 Mapper 376

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_376) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_376)

**NES 2.0 Mapper 376** denotes J.Y. Company's **YY841155C** circuit board, used for one one known multicart: 

  * _1996 大金鋼專輯 8-in-1_ (JY-041)



Similarly to [NES 2.0 Mapper 361](NES_2_0_Mapper_361.xhtml "NES 2.0 Mapper 361"), it uses an [MMC3](MMC3.xhtml "MMC3") clone with two outer bank registers at $7000/$7001. This variant adds an NROM PRG mode. 

## Outer Bank register #0 ($7000, write)
    
    
    Mask: $F001
    
    D~7654 3210
      ---------
      UaV. .bbb
      |||   +++- In NROM mode: PRG A14..16
      |+|------- PRG/CHR A17
      +-+------- UV  PRG banking mode
                 --------------------
                 10  NROM-128
                 11  NROM-256
                 0x  MMC3
    

Regardless of the PRG banking mode, CHR-ROM A10..A16 will come from the MMC3. 

## Outer Bank register #1 ($7001, write)
    
    
    Mask: $F001
    
    D~7654 3210
      ---------
      .... ...B
              +- PRG/CHR A18
    

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
