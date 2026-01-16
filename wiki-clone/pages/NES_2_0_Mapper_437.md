# NES 2.0 Mapper 437

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_437) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_437)

**NES 2.0 Mapper 437** denotes the NTDEC **TH2348** multicart circuit board for eight [UNROM](UxROM.xhtml "UNROM") games with an additional outer bank register. 

## Outer Bank register ($5000-$5FFF, write)
    
    
    A~[0101 .... .... MOOO]
                      |+++- PRG A19..A17
                      +---- Nametable mirroring
                            0: CIRAM A10=PPU A10 (vertical)
                            1: CIRAM A10=PPU A11 (horizontal)
    

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
