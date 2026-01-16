# NES 2.0 Mapper 385

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_385) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_385)

**NES 2.0 Mapper 385** denotes the NTDEC **2779** circuit board, used for a simple _5-in-1_ multicart with 8 KiB of unbanked CHR-RAM containing NROM-128 games. It is similar to [INES Mapper 202](INES_Mapper_202.xhtml "INES Mapper 202") except that there is no NROM-128/256 switch, and that it uses CHR-RAM rather than CHR-ROM. 

## Address Latch ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    A~FEDC BA98 7654 3210
      -------------------
      .... .... .... BBBM
                     |||+- 0: Vertical, 1: Horizontal mirroring
                     +++-- PRG A14-A16 at CPU $8000-$BFFF and CPU $C000-$FFFF
    

## See also

[NESdev forum discussion](https://forums.nesdev.org/viewtopic.php?f=9&t=19198)

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
