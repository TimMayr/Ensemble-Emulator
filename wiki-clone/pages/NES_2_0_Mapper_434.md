# NES 2.0 Mapper 434

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_434) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_434)

**NES 2.0 Mapper 434** denotes the **S-009** PCB, used by multicarts consisting of several UNROM games. 

## Outer Bank Register ($6000-$7FFF)
    
    
    Mask: $E000
    
    D~7654 3210
      ---------
      ..M. .PPP
        |   +++- PRG A19..A17
        +------- 0: Horizontal mirroring
                 1: Vertical mirroring
    

## Inner Bank Register ($8000-$FFFF)
    
    
    Mask: $8000
    
    D~7654 3210 (bus conflicts)
      ---------
      .... .PPP
            +++- PRG A16..A14 if CPU A14=0
    

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
