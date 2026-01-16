# NES 2.0 Mapper 382

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_382) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_382)

**NES 2.0 Mapper 382** denotes the **830928C** circuit board, used on a 512 KiB _5-in-1_ and a 1 MiB _9-in-1_ multicart containing the [BNROM](INES_Mapper_034.xhtml "BNROM") game _Journey to the West_ and Capcom/Konami [UNROM](UxROM.xhtml "UNROM") games. 

The written address sets the mode and outer bank number, the data latch the inner bank number. The data latch is subject to bus conflicts, making it difficult to write a desired inner bank value as long as the outer bank is not locked. 

## Outer Bank register ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    A~FEDC BA98 7654 3210
      -------------------
      1... .... ..LM UPPP
                  || |+++- PRG A19..A17 (outer 128 KiB PRG-ROM bank)
                  || +---- 0: UNROM mode
                  ||       1: BNROM mode
                  |+------ 0: Vertical mirroring
                  |        1: Horizontal mirroring
                  +------- 1: Lock outer bank
    Power-on value: $00
    

## Inner Bank register ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    D~7654 3210 (bus conflicts)
      ---------
      .... .ABB
            |++- In BNROM mode: PRG A16..A15 (inner 32 KiB PRG-ROM bank)
            +++- In UNROM mode: PRG A16..A14 (inner 16 KiB PRG-ROM bank
                                              at CPU $8000-$BFFF)
    

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
