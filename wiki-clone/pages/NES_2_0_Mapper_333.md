# NES 2.0 Mapper 333

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_333) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_333)

NES 2.0 mapper 333 is used for the _New Star Super 8-in-1_ multicart. Its UNIF board name are **BMC-8-IN-1** and **BMC-NEWSTAR-GRM070-8IN1**. 

## Outer Bank and PRG Banking Mode Register ($9000-$9FFF, $B000-$BFFF, $D000-$DFFF, $F000-$FFFF, write)
    
    
    Mask: $9000
    
    D~7654 3210
      ---------
      ...P OOII
         | ||++- Select inner 32 KiB PRG-ROM bank at CPU $8000-$FFFF if P=0
         | ++--- Select 128 KiB outer PRG-ROM and CHR-ROM bank
         +------ Select PRG Banking Mode
                  0: NROM-256
                  1: MMC3
    

## MMC3-compatible registers ($8000-$8FFF, $A000-$AFFF, $C000-$CFFF, $E000-$EFFF, write)
    
    
    Mask: $F001
    
    See [MMC3](MMC3.xhtml "MMC3").
    

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
