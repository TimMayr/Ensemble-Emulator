# NES 2.0 Mapper 297

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_297) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_297)

NES 2.0 Mapper 297 denotes the TXC 01-22110-000 board, used for the multicart _2-in-1 Uzi Lightgun_ (MGC-002). Its hardware can function as an [MMC1](MMC1.xhtml "MMC1") or as [INES Mapper 070](INES_Mapper_070.xhtml "INES Mapper 070"). 

## Contents

  * 1 Registers
    * 1.1 Mode register ($4120)
    * 1.2 Data latch in Mapper 70 mode ($8000-$FFFF)
    * 1.3 MMC1 registers in Mapper 1 mode ($8000-$FFFF)
  * 2 Nametable mirroring
  * 3 See also



# Registers

## Mode register ($4120)

Mask: $E100 
    
    
    7654 3210
    ---------
    .... ..AM
           |+- Mapper mode and outer 128 KiB PRG/CHR-ROM bank
           |   0: [Mapper 70](INES_Mapper_070.xhtml "INES Mapper 070"), Latch at $8000-$FFFF, PRG/CHR A17=0
           |   1: [Mapper 1](MMC1.xhtml "INES Mapper 001"), MMC1-compatible registers at $8000-$FFFF, PRG/CHR A17=1
           +-- PRG A16 in Mapper 70 mode
    

## Data latch in Mapper 70 mode ($8000-$FFFF)

See [INES Mapper 070](INES_Mapper_070.xhtml "INES Mapper 070"). 

## MMC1 registers in Mapper 1 mode ($8000-$FFFF)

See [INES Mapper 001](MMC1.xhtml "INES Mapper 001"). 

# Nametable mirroring

Mirroring is fixed to Vertical by way of an additional wire connecting CIRAM A10 to PA10. 

# See also

[PCB image and analysis](http://forums.nesdev.org/viewtopic.php?f=9&t=17067)

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
