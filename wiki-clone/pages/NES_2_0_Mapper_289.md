# NES 2.0 Mapper 289

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_289) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_289)

**NES 2.0 Mapper 289** is used for at least three multicarts designated in UNIF as **BMC-60311C**. 

  * _(EW-16-1) Super HiK 17-in-1_
  * _1995 Champion Cassette 1200-in-1 - 94年超值珍藏版_
  * _Super 76-in-1_ (with Contra)



## Contents

  * 1 Mode Register ($6000), write
  * 2 Outer Bank Register ($6001), write
  * 3 Solder Pad Register ($6000, read)
  * 4 Data Latch ($8000-$FFFF), write



## Mode Register ($6000), write
    
    
    D~[.... MLPP]  Address mask: $E001
            ||++- PRG Banking Mode
            ||     0: NROM-128: PRG A14-A16 from Outer Bank Register
            ||     1: NROM-256: PRG A15-A16 from Outer Bank Register, PRG A14=CPU A14
            ||     2: UNROM: PRG A14-A16 from Data Latch when CPU A14=0, and 111b when CPU A14=1
            ||     3: PRG A14-16=1 regardless of CPU A14
            |+--- Protect CHR-RAM
            |      0: CHR-RAM writable
            |      1: CHR-RAM write-protected
            +---- Select nametable mirroring type
                   0: Vertical
                   1: Horizontal
    

## Outer Bank Register ($6001), write
    
    
    D~[.QQQ QPPp]  Address mask: $E001
        ||| |||+- PRG A14 in NROM-128 PRG Banking Mode
        ||| |++-- PRG A16..15 in NROM-128/-256 Banking Modes
        +++-+---- PRG A20..A17
    

## Solder Pad Register ($6000, read)
    
    
    D~[.... ..PP]  Address mask: $E000
       |||| ||++- Solder pad setting
       ++++-++--- Open Bus 
    

## Data Latch ($8000-$FFFF), write
    
    
    D~[.... .PPP]  Address mask: $8000
             +++- PRG A16..14 in UNROM PRG Banking Mode when CPU A14=0
    

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
