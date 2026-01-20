# INES Mapper 059

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_059) | View [other pages](Special_AllPages.xhtml#INES_Mapper_059)

iNES Mapper 59 is used for a multicart with the UNIF board names **BMC-T3H53** and **BMC-D1038**. 

## Address Latch ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    A~FEDC BA98 7654 3210
    ---------------------
      1... ..LD SPPp MCCC
             || |||| |+++- Select 8 KiB CHR-ROM bank at PPU $0000-$1FFF
             || |||| +---- Select nametable mirroring type
             || ||||        0: Vertical
             || ||||        1: Horizontal
             || |+++------ Select 16 KiB PRG-ROM bank at CPU $8000-$BFFF/$C000-$FFFF
             || +--------- Select PRG A14 behavior
             ||             0: Substitute p with CPU A14 (NROM-256, 32 KiB PRG-ROM)
             ||             1: Leave p unchanged (NROM-128, 16 KiB PRG-ROM)
             |+----------- Jumper Read Control
             |              0: CPU $8000-$FFFF returns PRG-ROM content
             |              1: CPU $8000-$FFFF returns Jumper/DIP switch value
             +------------ Lock bit
                            1: Ignore further writes to the latch until reset
    

## Jumper Value ($8000-$FFFF, read)
    
    
    Mask: $8000
    
    D~7654 3210
      ---------
      OOOO OOJJ
      |||| ||++- Current jumper value, selecting menu variants with different game counts
      ++++-++--- Open bus
    

## Errata

NESten's Mapper DLL v1.0 contained an incorrect implementation of T3H53 as iNES mapper 059 and UNIF mapper BMC-T3H53. The UNIF implementation was improved in NESten's Mapper DLL v1.2 while the iNES implementation was not, causing confusion about what kind of cartridge the unimproved mapper 59 was supposed to be. As Nintendulator's mapper DLL is based on NESten's mapper DLL, this was carried over to Nintendulator. Presumably as a result of this confusion, FCEUX and others put the correct implementation of T3H53 at [INES Mapper 060](INES_Mapper_060.xhtml "INES Mapper 060"), displacing the "Reset-based NROM-128 4-in-1" in the process. Therefore, a .NES ROM file denoting mapper 60 that is not a reset-based multicart is actually for mapper 59, as described above. 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
