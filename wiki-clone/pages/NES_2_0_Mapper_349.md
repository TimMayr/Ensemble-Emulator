# NES 2.0 Mapper 349

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_349) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_349)

NES 2.0 mapper 349 is used for the _1994 Super HIK 14-in-1 (G-136)_ multicart. Its UNIF board name is **BMC-G-146**. CHR data is stored in 8 KiB of unbanked CHR-RAM. 

## Address Latch ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    A~FEDC BA98 7654 3210
      -------------------
      1... U... MSOO OPPp
           |    ||++-++++- Select 16 KiB PRG-ROM bank CPU $8000-$BFFF and $C000-$FFFF,
           |    ||         modified by U and S bits
           |    |+-------- Select PRG A14 Mode
           |    |           if U=1 (UNROM mode): 0: PRG A14=p, 1: PRG A14=1
           |    |           if U=0 (NROM mode):  0: PRG A14=PA14 (NROM-256), 1: PRG A14=p (NROM-128)
           |    +--------- Select nametable mirroring type
           |                0: Vertical
           |                1: Horizontal
           +-------------- Select PRG A14-A16 Mode
                            0: PPx (NROM), x determined by S bit
                            1: PPp OR S if PA14=0, 111b if PA14=1 (UNROM-like)
    

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
