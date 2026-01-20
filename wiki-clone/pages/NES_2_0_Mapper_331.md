# NES 2.0 Mapper 331

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_331) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_331)

NES 2.0 mapper 331 is used for the _7-in-1 (NS03)_ multicart. Its UNIF board name is **BMC-12-IN-1**. 

## Inner Bank Register A ($A000-$BFFF)
    
    
    Mask: probably $E000
    
    D~7654 3210
      ---------
      CCCC CPPP
      |||| |+++- Select 16 KiB inner PRG-ROM bank at CPU $8000-$BFFF and CPU $C000-$FFFF
      |||| |     active when the PPU is rendering from $0000-$0FFF, subject to A14-A16 modification
      ++++-+---- Select 4 KiB inner CHR-ROM bank at PPU $0000-$0FFF
    

## Inner Bank Register B ($C000-$DFFF)
    
    
    Mask: probably $E000
    
    D~7654 3210
      ---------
      CCCC CPPP
      |||| |+++- Select 16 KiB inner PRG-ROM bank at CPU $8000-$BFFF and CPU $C000-$FFFF
      |||| |     active when the PPU is rendering from $1000-$1FFF, subject to A14-A16 modification
      ++++-+---- Select 4 KiB inner CHR-ROM bank at PPU $1000-$1FFF
    

## Outer Bank and Mode Register ($E000-$FFFF)
    
    
    Mask: probably $E000
    
    D~7654 3210
      ---------
      .... PMOO
           ||++- Select 128 KiB outer PRG-ROM and CHR-ROM bank
           |+--- Select nametable mirroring type
           |      0: Vertical
           |      1: Horizontal
           +---- Select PRG A14-A16 mode
                  0: Force PRG A14-A16 to 111b if CPU A14=1 (UNROM-like)
                  1: PRG A14=CPU A14 (NROM-256)
                  
    

It's probably a good idea to initialize the PPP bits of both inner bank registers to the same value to prevent the PRG-ROM bank at $8000-$BFFF from changing back and forth as the PPU is rendering. 

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
