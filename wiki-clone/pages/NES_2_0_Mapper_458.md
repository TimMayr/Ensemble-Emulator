# NES 2.0 Mapper 458

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_458) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_458)

**NES 2.0 Mapper 458** denotes the [MMC3](MMC3.xhtml "MMC3")-based **K-3102** (submapper 0) and **GN-23** (submapper 1) multicart PCBs. The MMC3 is only used for CHR banking and mirroring, while all PRG banking is controlled by an extra register, similar to [NES 2.0 Mapper 259](NES_2_0_Mapper_259.xhtml "NES 2.0 Mapper 259"). 

## Extra Register ($6000-$7FFF, write)
    
    
    A~[011. .... d.DM PPPP]
                 | || ++++- PRG A17..A14
                 | || +---- 0: NROM-128, 1: NROM-256 (submapper 1)
                 | |+------ 0: NROM-128, 1: NROM-256 (submapper 0)
                 | +------- 0: Normal, 1: Replace CPU A4..0 with solder pad value (submapper 0)
                 +--------- 0: Normal, 1: Replace CPU A4..0 with solder pad value (submapper 1)
    Value on reset: $00
    

## Notes

  * WRAM must be enabled in $A001.7 before writing to the extra register.



Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
