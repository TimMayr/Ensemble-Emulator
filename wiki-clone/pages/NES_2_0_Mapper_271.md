# NES 2.0 Mapper 271

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_271) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_271)

NES 2.0 Mapper 271 is used for the TXC 4-in-1 multicart (MGC-026). It's basically an oversize [GNROM](GxROM.xhtml "GNROM") with one of the PRG bits also serving as a mirroring control. Its UNIF board name is **BMC-22026**. 

## Data latch
    
    
    Mask: $8000
    
    Bit 7654 3210
        ---------
        ..PP CCCC
          || ++++- Select 8 KiB CHR-ROM bank at PPU $0000-$1FFF
          ++------ Select 32 KiB PRG-ROM bank at CPU $8000-$FFFF
          +------- Select nametable mirroring
                   0: Horizontal
                   1: Vertical
    

Categories: [GNROM-like mappers](Category_GNROM_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
