# NROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NROM) | View [other pages](Special_AllPages.xhtml#NROM)

**NROM**

**Company** | Nintendo, others   
---|---  
**Boards** | NROM, HROM*, RROM, RTROM, SROM, STROM   
**PRG ROM capacity** | 16K or 32K   
**PRG ROM window** | n/a   
**PRG RAM capacity** | 2K or 4K in Family Basic only   
**PRG RAM window** | n/a   
**CHR capacity** | 8K   
**CHR window** | n/a   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | Fixed H or V, controlled by solder pads (*V only)   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | Yes   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [000](NROM.xhtml "INES Mapper 000")  
  
**NESCartDB**

[iNES 000](https://nescartdb.com/search/advanced?ines=0)  
---  
[NROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=-NROM)  
[HROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=-HROM)  
[RROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=-RROM)  
[RTROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=-RTROM)  
[SROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=-SROM)  
[STROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=-STROM)  
  
  
The generic designation **NROM** refers to the Nintendo cartridge boards NES-NROM-128, NES-NROM-256, their [HVC](Family_Computer.xhtml "Famicom") counterparts, and clone boards. The [iNES](INES.xhtml "INES") format assigns **mapper 0** to NROM. 

The suffixes 128 and 256 refer to kilobits by Nintendo's own designation; not kilobytes (as is the case with [UNROM 512](UNROM_512.xhtml "UNROM 512"), for example). 

## Contents

  * 1 Overview
  * 2 Banks
  * 3 Solder pad config
  * 4 Registers
  * 5 Variants
  * 6 See also



## Overview

  * PRG [ROM](ROM.xhtml "ROM") size: 16 KiB for NROM-128, 32 KiB for NROM-256 (DIP-28 standard [pinout](Mask_ROM_pinout.xhtml "Mask ROM pinout"))
  * PRG ROM bank size: Not bankswitched
  * PRG RAM: 2 or 4 KiB, not bankswitched, only in Family Basic (but most emulators provide 8)
  * CHR capacity: 8 KiB ROM (DIP-28 standard pinout) but most emulators support RAM
  * CHR bank size: Not bankswitched, see [CNROM](CNROM.xhtml "CNROM")
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Solder pads select vertical or horizontal mirroring
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): Yes, but irrelevant



## Banks

All Banks are fixed, 

  * CPU $6000-$7FFF: Family Basic only: PRG RAM, mirrored as necessary to fill entire 8 KiB window, write protectable with an external switch
  * CPU $8000-$BFFF: First 16 KB of ROM.
  * CPU $C000-$FFFF: Last 16 KB of ROM (NROM-256) or mirror of $8000-$BFFF (NROM-128).



## Solder pad config

  * Horizontal mirroring : 'H' disconnected, 'V' connected.
  * Vertical mirroring : 'H' connected, 'V' disconnected.



## Registers

None. This has normally no mapping capability whatsoever! Nevertheless, tile animation can be done by swapping between pattern tables $0000 and $1000, using [PPUCTRL](PPU_registers.xhtml "PPUCTRL") bits 4-3 as a "poor man's [CNROM](CNROM.xhtml "CNROM")". 

## Variants

[RROM and SROM](Mask_ROM_pinout.xhtml#Variants "Mask ROM pinout") are NROM with different CHR ROM pinouts. RTROM and STROM split PRG into two 8KiB ROMs. HROM is an early NROM variant without the solder pads to select mirroring, only vertical mirroring available (as if the 'H' pad were selected). 

Family Basic, released by Nintendo, Sharp, and Hudson, contains 2 KiB WRAM (or 4 KiB WRAM for v3) accessible at $6000-$7FFF and [decoded with a 74HC20](PRG_RAM_circuit.xhtml "PRG RAM circuit"), that is backed with 2 AA batteries. The cartridge is equipped with a back up switch that, when enabled, disables all access to WRAM in order to prevent possible data corruption when the Famicom is turned on or off. 

[Galaxian](https://nescartdb.com/profile/view/1808/galaxian) uses a smaller 8KiB PRG ROM chip. iNES images are all therefore overdumps. 

Modern homebrew programs sometimes use mapper 0 with 8 KB of CHR RAM, which would not be supported on official NROM PCBs without rewiring. In theory, [BNROM](INES_Mapper_034.xhtml "BNROM") might be a more appropriate choice. But most emulators will do the right thing with a 16K or 32K PRG ROM with mapper 0. 

[NROM-368](NROM_368.xhtml "NROM-368") is a recent invention that allows addressing more memory without bank switching. 

## See also

  * [Programming NROM](Programming_NROM.xhtml "Programming NROM")



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
