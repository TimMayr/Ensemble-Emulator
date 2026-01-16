# INES Mapper 041

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_041) | View [other pages](Special_AllPages.xhtml#INES_Mapper_041)

iNES Mapper 041 represents Caltron 6-in-1, a discrete logic multicart. It appears to have been designed to hold up to four each unmodified [CNROM](CNROM.xhtml "CNROM") and [NROM](NROM.xhtml "NROM") games (with NROMs' CHR in unused portions of the CNROM games). 

## Contents

  * 1 Overview
  * 2 Registers
    * 2.1 Outer Bank Select ($6000-$67FF)
    * 2.2 Inner CHR Bank Select ($8000-$FFFF), bus conflicts



## Overview

  * PRG ROM size: 256 KiB
  * PRG ROM bank size: 32 KiB
  * PRG RAM: Impossible
  * CHR capacity: 128 KiB ROM
  * CHR bank size: 8 KiB inner / 32 KiB outer
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Mapper-controlled H or V
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): partly



## Registers

Both registers are cleared on powerup, or while the reset button is pressed. 

### Outer Bank Select ($6000-$67FF)
    
    
    15   11   7  bit  0 (address lines)
    ---- ---- ---- ----
    0110 0xxx xxMC CEPP
                ││ ││││
                ││ │├┴┴── Select 32KiB PRG ROM bank for CPU $8000─$FFFF
                ││ ││
                ││ │└──── If high, enable Inner Bank Select
                │└─┴───── Select 32KiB outer CHR ROM bank
                └──────── Mirroring (0=Vertical, 1=Horizontal)
    

### Inner CHR Bank Select ($8000-$FFFF), bus conflicts
    
    
    7  bit  0
    ---- ----
    xxZZ xxcc
      ││   ││
      ││   └┴── Select 8KiB inner bank of outer 32KiB CHR bank chosen above
      └┴─────── Extant but unused
    

Note that the Inner CHR Bank Select only can be written while the PRG ROM bank is 4, 5, 6, or 7. This means that execution must pass through one of the upper four PRG banks to choose the inner CHR bank before playing any games in the lower four PRG banks. 

Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
