# NES 2.0 Mapper 453

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_453) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_453)

**NES 2.0 Mapper 453** denotes the **Realtec 8042** multicart circuit board, used for the _3-in-1 1995 World Competitive Champion Edition (DG-002)_ multicart. It supports both AOROM and UNROM banking modes. 

## Data Latch ($8000-$FFFF), write
    
    
    D~[.PpM .BBB]
        |||  +++- PRG A16..A14 in UNROM mode,
        |||       PRG A17..A15 in AOROM mode
        ||+------ Mirroring
        ||         0=V, 1=H in UNROM mode,
        ||         CIRAM A10 in AOROM mode
        ++------- Lock Pp if either bit is set
        |+------- PRG A17 in UNROM mode
        +-------- PRG A18
        +-------- PRG banking mode
                   0=UNROM, 1=AOROM
    

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
