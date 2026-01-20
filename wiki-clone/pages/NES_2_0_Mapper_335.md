# NES 2.0 Mapper 335

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_335) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_335)

NES 2.0 mapper 335 is used for a _10-in-1_ multicart. Its UNIF board name is **BMC-CTC-09**. 

## PRG-ROM and Mirroring Register ($C000-$FFFF)
    
    
    Mask: probably $C000
    
    D~7654 3210
      ---------
      ..MB pPPP
        || |+++- Select 32 KiB PRG-ROM bank at CPU $8000-$FFFF
        || +---- Select 16 KiB PRG-ROM bank if B=1
        |+------ Select PRG A14 Mode
        |         0: PRG A14=CPU A14 (NROM-256)
        |         1: PRG A14=p (NROM-128)
        +------- Select nametable mirroring mode
                  0: Vertical
                  1: Horizontal         
    

## CHR-ROM Register ($8000-$BFFF)
    
    
    Mask: probably $C000
    
    D~7654 3210
      ---------
      .... CCCC
           ++++- Select 8 KiB CHR-ROM bank at PPU $0000-$1FFF
    

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
