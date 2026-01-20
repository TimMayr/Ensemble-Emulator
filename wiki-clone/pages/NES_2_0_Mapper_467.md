# NES 2.0 Mapper 467

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_467) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_467)

**NES 2.0 Mapper 467** denotes the **47-2** [MMC3](MMC3.xhtml "MMC3")-based circuit board, used for a _72-in-1_ multicart. It is effectively [INES Mapper 197](INES_Mapper_197.xhtml "INES Mapper 197") submapper 0 with an outer bank register and an NROM mode. 

## Outer Bank Register ($9000, write)
    
    
    Mask: $F000?
    
    D~[MSNQ QPPp]
       ||++-++++- PRG A18..A14
       ||+------- PRG banking mode
       ||          0=NROM-128
       ||          1=MMC3-based
       |+-------- PRG (if N=1)/CHR banking mode
       |           0=PRG A14..A13 from MMC3, PRG A19..A15 from NQQPP
       |             CHR A12..A10=PPU A12..A10, CHR A18..A13 from MMC3 CHR A17..A12, CHR A19=0
       |           1=PRG A16..A13 from MMC3, PRG A19..A17 from NQQ
       |             CHR A10=PPU A10, CHR A18..A11 from MMC3 CHR A17..A10, CHR A19=1
       +--------- Nametable mirroring
                   0=Vertical
                   1=Horizontal
    

The MMC3 clone ignores the PRG/CHR inversion bits D6 and D7 in the pointer register $8000, and its mirroring output is ignored in favor of the Outer Bank Register's. 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
