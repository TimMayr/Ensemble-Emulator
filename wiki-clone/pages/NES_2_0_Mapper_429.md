# NES 2.0 Mapper 429

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_429) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_429)

NES 2.0 Mapper 429 is for Milowork FCFC1 flash cartridge (in A mode). There is some doujin game in progress using this mapper. 

## Contents

  * 1 Overview
  * 2 Registers
    * 2.1 Bank select ($8000-$FFFF)
      * 2.1.1 Regular
      * 2.1.2 Submapper #1
  * 3 FCFC1 Details



## Overview

  * PRG ROM size: 512 KB (or 1024 KB, FCFC1 DIY)
  * PRG ROM bank size: 32 KB
  * PRG RAM: None
  * CHR capacity: 32 KB RAM, optional battery-backup or FeRAM
  * CHR bank size: 8 KB
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): vertical or horizontal selected by switch, or fixed (or Single-screen, FCFC1 DIY)
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): No



## Registers

### Bank select ($8000-$FFFF)

#### Regular
    
    
    7  bit  0
    ---- ----
    xpPP PPCC
     ||| ||||
     ||| ||++- Select 8 KB CHR RAM bank for PPU $0000-$1FFF, initial bank 0
     +++-++--- Select 32 KB PRG ROM bank for CPU $8000-$FFFF, initial bank 1
    

#### Submapper #1
    
    
    7  bit  0
    ---- ----
    MpPP PPCC
    |||| ||||
    |||| ||++- Select 8 KB CHR RAM bank for PPU $0000-$1FFF, initial bank 0
    |+++-++--- Select 32 KB PRG ROM bank for CPU $8000-$FFFF, initial bank 1
    +--------- Same as [AxROM](AxROM.xhtml "AxROM")
    

## FCFC1 Details

  * The A mode of FCFC1 has compatibility with [CNROM](CNROM.xhtml "CNROM")/[GxROM](GxROM.xhtml "GxROM"), but the compatibility is fragile, some games will switch back to the page where the firmware is located, causing problems such as crashes.


  * The B mode of FCFC1 is a [BNROM](INES_Mapper_034.xhtml "BNROM")/[AxROM](AxROM.xhtml "AxROM") with initial bank 4 and no [bus conflict](Bus_conflict.xhtml "Bus conflict"), and has compatibility with [NROM](NROM.xhtml "NROM")/[UNROM](UxROM.xhtml "UNROM").


  * To simplify the circuit implementation, the actual location of the page in ROM is shuffled.


  * It is recommended to save the preserved data to a page other than 0 of CHR RAM, as this avoids being overwritten by the kazzo programming process, and it can also make the bank selection play a role similar to the WRAM lock bit.


  * Some FCFC1 sold as game carriers do not have switches, or the switches do not have the W position. These FCFC1 are not compatible with kazzo.

Register value causes error | CNROM (A) | GxROM (A) | NROM (B) (Except using CHR RAM)   
---|---|---|---  
Firmware 2.1 and before | xxx1xx | xxx1xx | xxx1xx   
Firmware 2.2 and after | xx01xx | xx01xx | xx01xx 
