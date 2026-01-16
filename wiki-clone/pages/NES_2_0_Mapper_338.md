# NES 2.0 Mapper 338

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_338) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_338)

NES 2.0 mapper 338 is used for a _16-in-1_ and a _200/300/600/1000-in-1_ multicart. Its UNIF board name is **BMC-SA005-A**. 

## Address Latch ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    A~FEDC BA98 7654 3210
      -------------------
      1... .... .... MBBB
                     ++++- Select 16 KiB PRG-ROM bank at CPU $8000-$BFFF and $C000-$FFFF
                     |     and 8 KiB CHR-ROM bank at PPU $0000-$1FFF
                     +---- Select nametable mirroring type
                            0: Horizontal
                            1: Vertical
    

## Similar mappers

[INES Mapper 200](INES_Mapper_200.xhtml "INES Mapper 200") is similar but with the mirroring bit's meaning flipped, and the mirroring bit being also used as a fourth bank bit. 

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
