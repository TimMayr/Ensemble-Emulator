# NES 2.0 Mapper 378

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_378) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_378)

**NES 2.0 Mapper 378** denotes a circuit board with unknown ID, used for an 8-in-1 512 KiB multicart containing one [AxROM](AxROM.xhtml "AxROM") and two [UNROM](UxROM.xhtml "UNROM") games. 

## Outer Bank register ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    A~FEDC BA98 7654 3210
      -------------------
      1... .... ..MS cbaa
                  || |+++- PRG A15-A17
                  || |+--- UNROM only: 0=Vertical, 1=Horizontal mirroring
                  || +---- UNROM only: PRG A14 when CPU A14=0
                  |+------ CIRAM A10 in AxROM mode
                  +------- PRG A18, also:
                            0: AxROM mode: PRG A14=CPU A14
                            1: UNROM mode: PRG A14=c if CPU A14=0,
                                           PRG A14..16=7 if CPU A14=1
    

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
