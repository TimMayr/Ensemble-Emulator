# NES 2.0 Mapper 456

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_456) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_456)

**NES 2.0 Mapper 456** denotes the [MMC3](MMC3.xhtml "MMC3")-based Realtec **K6C3001A** multicart PCB, used for the _(ST-401) Super 4-in-1_ multicart. It is basically an oversized version of [INES Mapper 047](INES_Mapper_047.xhtml "INES Mapper 047") with the outer bank register moved from $6000 to $4100. 

## Extra Register ($4100, write)
    
    
    Mask: $E100
    
    D~[.... ..BB]
              ++- PRG/CHR A18..A17
    Value on reset: $00
    

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
