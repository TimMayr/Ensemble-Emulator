# NES 2.0 Mapper 404

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_404) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_404)

**NES 2.0 Mapper 404** denotes the **JY012005** [MMC1](MMC1.xhtml "MMC1")-based multicart circuit board, used only for the _1998 Super HiK 8-in-1 (JY-021B)_ multicart. 

## Outer Bank and Mode Register ($6000-$7FFF, write)
    
    
    Mask: $E000
    
    D~7654 3210
      ---------
      LM.. ?BBb
      ||   |+++- PRG/CHR A19..A17
      ||   +---- Always set to 1 by the menu
      |+-------- 0: PRG A17=MMC1 A17 (256 KiB inner PRG bank)
      |          1: PRG A17=b        (128 KiB inner PRG bank)
      +--------- 1: Lock outer bank register
    
    Power-on value: $00
    

## MMC1-compatible registers ($8000-$FFFF)

Mask: $E000 

See [MMC1](MMC1.xhtml "MMC1"). 

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
