# AxROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/AxROM) | View [other pages](Special_AllPages.xhtml#AxROM)

**AxROM**

**Company** | Nintendo, Rare, others   
---|---  
**Games** | [34 in NesCartDB](https://nescartdb.com/search/advanced?ines=7)  
**Complexity** | Discrete logic   
**Boards** | AMROM, ANROM,   
AN1ROM, AOROM, others   
**PRG ROM capacity** | 256K   
**PRG ROM window** | 32K   
**PRG RAM capacity** | None   
**PRG RAM window** | n/a   
**CHR capacity** | 8K   
**CHR window** | n/a   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | 1 page switchable   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | AMROM/AOROM only   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [007](AxROM.xhtml "INES Mapper 007")  
  
**NESCartDB**

[iNES 007](https://nescartdb.com/search/advanced?ines=7)  
---  
[AxROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=-A%25ROM)  
[AMROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=AMROM)  
[ANROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=ANROM)  
[AN1ROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=AN1ROM)  
[AOROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=AOROM)  
  
  
The generic designation **AxROM** refers to Nintendo cartridge boards **NES-AMROM** , **NES-ANROM** , **NES-AN1ROM** , **NES-AOROM** , their [HVC](Family_Computer.xhtml "Famicom") counterparts, and clone boards. AxROM and compatible boards are implemented in [iNES](INES.xhtml "INES") format with **iNES Mapper 7**. 

## Contents

  * 1 Board types
  * 2 Overview
  * 3 Banks
  * 4 Solder pad config
  * 5 Registers
    * 5.1 Bank select ($8000-$FFFF)
  * 6 Hardware
  * 7 Various notes
  * 8 Variants
  * 9 See also



## Board types

The following AxROM boards are known to exist: 

Board | PRG ROM | Bus conflicts   
---|---|---  
AMROM | 128 KB | Yes   
ANROM | 128 KB | No   
AN1ROM | 64 KB | No   
AOROM | 128 / 256 KB | Depends on +CE wiring   
  
## Overview

  * PRG ROM size: up to 256 KB
  * PRG ROM bank size: 32 KB
  * PRG RAM: None
  * CHR capacity: 8 KB RAM
  * CHR bank size: Not bankswitched
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Single-screen, mapper-selectable
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): AMROM/AOROM only



## Banks

  * CPU $8000-$FFFF: 32 KB switchable PRG ROM bank



## Solder pad config

No solder pad config is needed on the AxROM board family. 

## Registers

### Bank select ($8000-$FFFF)
    
    
    7  bit  0
    ---- ----
    xxxM xPPP
       |  |||
       |  +++- Select 32 KB PRG ROM bank for CPU $8000-$FFFF
       +------ Select 1 KB VRAM page for all 4 nametables
    

## Hardware

The AxROM boards contain a [74HC161](74161.xhtml "74161") binary counter used as a quad D latch (4-bit register). The ANROM and AN1ROM boards also contains a [74HC02](7402.xhtml "7402") which is used to disable the PRG ROM during writes, thus avoiding [bus conflicts](Bus_conflict.xhtml "Bus conflict"). 

## Various notes

On the AOROM board, special mask ROMs with an additional positive CE on pin 31 (which is connected to PRG R/W) can be used to prevent bus conflicts without an additional chip. Some 128 KB games were made with AOROM to save the cost of a 74HC02. It seems that only _Double Dare_ and _Wheel of Fortune_ employ this trick noticeably--that is, if emulated with bus conflicts enabled, the games will glitch. [The list of AxROM games in NesCartDB](https://nescartdb.com/search/advanced?ines_op=equal&ines=7) lacks quality coverage of the PCB backs for the AOROM games, so it is hard to determine yet which games may be wired this way. 

It is likely that every retail AOROM game could be emulated correctly without emulating bus conflicts. 

## Variants

[BNROM](INES_Mapper_034.xhtml "BNROM") is the same as AMROM except it uses a fixed horizontal or vertical mirroring. 

Some emulators allow bit 3 to be used to select up to 512 KB of PRG ROM for an oversized AxROM. In hardware this could be implemented by using an octal latch in place of the quad latch ([74HC377](74377.xhtml "74377")). 

The pirate multicart and unlicensed music game _Hot Dance 2000_ uses a 512 KB AxROM variant. 

A ROM Hack of Battletoads expanded the ROM size to 512 KB. 

## See also

  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) by \Firebug\, information about mapper's initial state is inaccurate.



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers with bus conflicts](Category_Mappers_with_bus_conflicts.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with single-screen mirroring](Category_Mappers_with_single_screen_mirroring.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
