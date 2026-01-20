# NES 2.0 Mapper 381

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_381) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_381)

**NES 2.0 Mapper 381** denotes the **KN-42** PCB, used by the _2-in-1 High Standard Game (BC-019)_ reset-based multicart containing two (modified) [UOROM](UxROM.xhtml "UOROM") games, in this case, the two Codemasters "Bignose" titles. 

## Data Latch ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    D~7654 3210
      ---------
      ...b .aaa
         |  +++- PRG A15..A17
         +------ PRG A14
    

The actual 16 KiB bank number is therefore _aaab_. PRG A14..A17 are forced to $F if CPU A14=1, just like UOROM. PRG A18 is flipped on each reset cycle, which is detected through interruption of the M2 signal. The data latch exhibits bus conflicts. 

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
