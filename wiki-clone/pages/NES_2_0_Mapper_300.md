# NES 2.0 Mapper 300

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_300) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_300)

NES 2.0 Mapper 300 is used for _Golden 190-in-1_ multicart. Its UNIF board name is **BMC-190in1**. It is similar to [INES Mapper 214](INES_Mapper_214.xhtml "INES Mapper 214") except that mirroring is selectable 

## Address latch ($8000-$FFFF)
    
    
    Mask: $8000
    
    A~FEDC BA98 7654 3210
      -------------------
      .... .... ...G GG.M
                   | || +- Select nametable mirroring type
                   | ||     0: Vertical
                   | ||     1: Horizontal
                   +-++--- Select 16 KiB PRG-ROM bank at CPU $8000-$BFFF mirrored at $C000-$FFFF
                           and 8 KiB CHR-ROM bank at PPU $0000-$1FFF
    

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
