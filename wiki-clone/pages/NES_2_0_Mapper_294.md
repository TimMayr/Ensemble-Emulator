# NES 2.0 Mapper 294

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_294) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_294)

**NES 2.0 Mapper 294** denotes the **63-1601** multicart PCB, used for the _強捧 16-in-1 (1682)_ multicart consisting of 2 MiB worth of UNROM games. 

## Outer Bank Register ($4100, write)
    
    
    Mask: $C100
    D~[...M PPPP]
          | ++++- PRG A20..A17
          +------ Mirroring
                   0=Vertical
                   1=Horizontal
    

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
