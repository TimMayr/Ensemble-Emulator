# NES 2.0 Mapper 370

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_370) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_370)

**NES 2.0 Mapper 370** denotes the **F600** circuit board, used for the _Golden Mario Party II - Around the World 6-in-1_ multicart. It uses an [MMC3](MMC3.xhtml "MMC3") clone with an outer bank register that also selects [TKROM](TxROM.xhtml "TKROM") and [TLSROM](TLSROM.xhtml "TLSROM") configurations (the latter for a Mario-themed _Armadillo_ hack). 

## Outer Bank register ($5000-$5FFF, write)
    
    
    Mask: $F000
    
    A~FEDC BA98 7654 3210
      -------------------
      0101 .... ..pP PcCC
                  || |+++- CHR A17-A19
                  || |+--- CHR Mask: 0=128K, 1=256 K
                  ++-+---- PRG A17-A19
                  +------- PRG Mask: 0=128K, 1=256 K
    

[TLSROM](TLSROM.xhtml "TLSROM") nametable mirroring is used when cCC is 001, otherwise normal [MMC3](MMC3.xhtml "MMC3") mirroring is used. 

## Solder Pad register ($5000-5FFF, read)
    
    
    Mask: $F000
    
    D~7654 3210
      ---------
      V... ....
      +--------- Solder pad setting
    

The solder pad setting selects one of two different menus. 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
