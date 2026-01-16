# NES 2.0 Mapper 393

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_393) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_393)

**NES 2.0 Mapper 393** denotes the **820720C** multicart PCB. 

# Registers

## MMC3-compatible registers ($8000-$FFFF, write)

These registers function identically to a normal [MMC3](MMC3.xhtml "MMC3"), except in UNROM/BNROM mode. 

## Outer Bank Register ($6000-$7FFF, write)
    
    
    Mask: $E000
     
    A~FEDC BA98 7654 3210
      --------- ---------
      011. .... ..MM CPPP
                  || |+++- PRG A19..A17
                  || +---- CHR Mode
                  ||        0: CHR-ROM with MMC3 banking
                  ||        1: 8 KiB unbanked CHR-RAM
                  ++------ PRG Mode
                            0: MMC3 (PRG A13-A16)
                            1: same as 0
                            2: BNROM (PRG A15-A16 from MMC3 R6,
                               PRG A13-A14=CPU A13-A14)
                            3: UNROM (PRG A14-A16 from $8000-$FFFF latch,
                               PRG A13=CPU A13) 
    

Selected by the MMC3's PRG-RAM /CE, WRAM must be enabled and writable in the MMC3's A001 register for Outer Bank Register writes to be recognized. 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
