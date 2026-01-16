# INES Mapper 094

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_094) | View [other pages](Special_AllPages.xhtml#INES_Mapper_094)

**NESCartDB**

[iNES 094](https://nescartdb.com/search/advanced?ines=94)  
---  
[UN1ROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=UN1ROM)  
  
INES Mapper 094 is used to represent the board HVC-UN1ROM. _Senjou no Ookami_ (Japanese version of _Commando_) is the only game known to utilize this mapper. It is very similar to [UxROM](UxROM.xhtml "UxROM"), but the register is shifted by two bits. 

## Contents

  * 1 Overview
  * 2 Banks
  * 3 Registers
    * 3.1 Bank select ($8000-$FFFF)
  * 4 References



## Overview

  * PRG ROM size: 128 KB
  * PRG ROM bank size: 16 KB
  * PRG RAM: None
  * CHR capacity: 8 KB RAM
  * CHR bank size: Not bankswitched
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Solder pads select vertical or horizontal mirroring
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): Yes



## Banks

  * CPU $8000-$BFFF: 16 KB switchable PRG ROM bank
  * CPU $C000-$FFFF: 16 KB PRG ROM bank, fixed to the last bank



## Registers

### Bank select ($8000-$FFFF)
    
    
    7  bit  0
    ---- ----
    xxxP PPxx
       | ||
       +-++--- Select 16 KB PRG ROM bank for CPU $8000-$BFFF
    

## References

  * NES mapper list by Disch [[1]](http://www.romhacking.net/documents/362/)



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
