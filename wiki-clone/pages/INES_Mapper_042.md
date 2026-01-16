# INES Mapper 042

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_042) | View [other pages](Special_AllPages.xhtml#INES_Mapper_042)

iNES Mapper 042 was used for hacked FDS games converted to cartridge form. At least two examples are known: Ai Senshi Nicol and "Mario Baby" (really Bio Miracle Bokutte Upa). While they likely used different hardware, they can both be encompassed by a single mapper. 

## Contents

  * 1 Overview
  * 2 Banks
  * 3 Registers
    * 3.1 CHR Select ($8000)
    * 3.2 PRG Select ($E000)
    * 3.3 Mirroring Control ($E001)
    * 3.4 IRQ Control ($E002)
  * 4 See also



## Overview

  * PRG ROM size: 128 KiB
  * PRG ROM bank size: 8 KiB
  * PRG RAM: No
  * CHR capacity: 128KiB ROM or 8KiB RAM
  * CHR bank size: 8 KiB
  * Nametable mirroring: Mapper controlled
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): ??



## Banks

  * CPU $6000-$7FFF: 8KiB switchable PRG ROM bank
  * CPU $8000-$FFFF: Fixed to last 32 KiB of PRG ROM
  * PPU $0000-$1FFF: 8KiB switchable CHR ROM bank or 8KiB fixed CHR RAM



## Registers

Register addresses should be masked with $E003. 

### CHR Select ($8000)
    
    
    7  bit  0
    ---- ----
    xxxx CCCC
         ||||
         ++++- Select 8 KB CHR ROM bank for PPU $0000-$1FFF
    

This register is ignored on Bio Miracle Bokutte Upa, which uses 8KiB CHR RAM instead. 

### PRG Select ($E000)
    
    
    7  bit  0
    ---- ----
    xxxx PPPP
         ||||
         ++++- Select 8 KB PRG ROM bank for CPU $6000-$7FFF
    

### Mirroring Control ($E001)
    
    
    7  bit  0
    ---- ----
    xxxx Mxxx
         |
         +---- Mirroring (0: Vertical; 1: Horizontal)
    

### IRQ Control ($E002)
    
    
    7  bit  0
    ---- ----
    xxxx xxEx
           |
           +-- IRQ (0: Disable, Acknowledge, and Reset; 1: Enable)
    

The IRQ hardware counts M2 cycles and triggers after 24576 ($6000) have elapsed. FCEUX's source implies that the IRQ hardware is a 15-bit counter, and IRQ is asserted while the two MSBs are set, i.e. in the absence of CPU involvement, IRQ will be be not asserted for 24576 cycles, then asserted for 8192 cycles, and repeat endlessly. 

## See also

  * [Pirate game "Mario Baby" Mapper #42 Info](http://nesdev.org/42.txt) by The Mad Dumper.



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml)
