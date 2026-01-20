# NES 2.0 Mapper 457

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_457) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_457)

**NES 2.0 Mapper 457** denotes the [MMC3](MMC3.xhtml "MMC3")-based **810431C** multicart PCB, used for the _92 年超高K 黄金卡 35-in-1 刺激大驚險旅風侠_ multicart. It is basically an oversized version of [INES Mapper 047](INES_Mapper_047.xhtml "INES Mapper 047") with a selectable outer PRG and CHR bank size. Alternatively, it can be thought of as a simplified version of [INES Mapper 052](INES_Mapper_052.xhtml "INES Mapper 052") without independent PRG/CHR control. 

## Extra Register ($6000-$7FFF, write)
    
    
    Mask: $E000
    
    D~[.... SBBB]
            |+++- PRG/CHR A19..A17
            +---- 0: 128 KiB outer PRG and CHR bank size
                  1: 256 KiB outer PRG and CHR bank size
    Value on reset: $00
    

## Notes

  * WRAM must be enabled in $A001.7 before writing to the extra register.
  * Nestopia Plus! has assigned mapper number **2786** for this circuit board.



Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
