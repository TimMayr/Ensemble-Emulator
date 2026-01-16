# NES 2.0 Mapper 341

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_341) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_341)

NES 2.0 mapper 341 is used for a simple _4-in-1_ multicart. Its UNIF board name is **BMC-TJ-03**. 

## Address Latch ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    A~FEDC BA98 7654 3210
      -------------------
      1... .BBB .... ..M.
            |||        +-- Select nametable mirroring type
            |||             0: Vertical
            |||             1: Horizontal
            +++----------- Select 16 KiB PRG-ROM bank at CPU $8000-$BFFF and $C000-$FFFF
                           and 8 KiB CHR-ROM bank at PPU $0000-$1FFF
    

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
