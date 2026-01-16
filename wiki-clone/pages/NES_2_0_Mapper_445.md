# NES 2.0 Mapper 445

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_445) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_445)

**NES 2.0 Mapper 445** denotes the **DG574B** [MMC3](MMC3.xhtml "MMC3")-compatible multicart circuit board. 

## Contents

  * 1 Outer PRG Bank ($5000, write)
  * 2 Outer CHR Bank ($5001, write)
  * 3 Mode ($5002, write)
  * 4 Lock ($5003, write)
  * 5 MMC3-compatible registers



## Outer PRG Bank ($5000, write)
    
    
    D~[.PPP ....]  Address mask: $F003
        +++------- PRG A19..A17
    

## Outer CHR Bank ($5001, write)
    
    
    D~[.CCC ....]  Address mask: $F003
        +++------- CHR A19..A17
    

## Mode ($5002, write)
    
    
    D~[...? c.?p]  Address Mask: $F003
            |  +- PRG A17 Mode
            |      0: from MMC3
            |      1: from Outer PRG Bank
            +---- CHR A17 Mode
                   0: from MMC3
    

## Lock ($5003, write)
    
    
    D~[..L. ....]  Address Mask: $F003
         +------- 1: Lock registers $5000-$5003
    

## MMC3-compatible registers

See [MMC3](MMC3.xhtml "MMC3"). 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
