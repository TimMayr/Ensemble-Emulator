# NES 2.0 Mapper 319

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_319) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_319)

NES 2.0 Mapper 319 denotes the **HP-898F** and **KD-7/9-E** circuit boards. Its UNIF board name is **BMC-HP898F** (with a different bank order). Used for: 

  * _Prima Soft 9999999-in-1_
  * _Olympic 2000 1000000-in-1_
  * _4-in-1 (0207)_
  * _4-in-1 (0210)_
  * _4-in-1 (KG-443)_
  * _9999999-in-1 (KD-6037-B)_



## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG-ROM Bank and Mirroring ($6004)
    * 2.2 CHR-ROM Bank ($6000)
    * 2.3 Solder Pad Register ($5FF0)
  * 3 UNIF bank order



# Banks

  * CPU $8000-$BFFF: 16 KiB switchable PRG-ROM bank
  * CPU $C000-$FFFF: 16 KiB switchable PRG-ROM bank
  * PPU $0000-$1FFF: 8 KiB switchable CHR-ROM bank



# Registers

## PRG-ROM Bank and Mirroring ($6004)
    
    
    Mask: $6004
    
    D~7654 3210
      ---------   
      MDaB C...
      |||| +---- PRG A15
      |||+------ PRG A16
      ||+------- PRG A14 if D=0
      |+-------- PRG A14 mode
      |           0: PRG A14=p (NROM-128)
      |           1: PRG A14=CPU A14 (NROM-256)
      +--------- Select nametable mirroring type
                  0: Horizontal
                  1: Vertical
    

## CHR-ROM Bank ($6000)
    
    
    Mask: $6004
    
    D~7654 3210
      ---------
      CCCC ....
      ||||
      ||||
      ++++------ Select 8 KiB CHR-ROM bank at PPU $0000-$1FFF
    

## Solder Pad Register ($5FF0)
    
    
    Mask: probably $F000
    
    D~7654 3210
      ---------
      .P.. ....
       +-------- Solder pad value
    

# UNIF bank order

The publicly-available UNIF ROM file of _Prima Soft 9999999-in-1_ has the order of the 16 KiB PRG-ROM banks slightly mixed up, so that the PRG A14 mode bit operates on A16 instead of A14. To obtain the correct bank order, use UNIF 16 KiB PRG banks 0, 4, 1, 5, 2, 6, 3, 7. 

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
