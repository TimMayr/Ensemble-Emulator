# INES Mapper 168

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_168) | View [other pages](Special_AllPages.xhtml#INES_Mapper_168)

iNES Mapper 168 represents the board used for Racermate Challenge 2. It could be thought of as a hybrid of [UNROM](UxROM.xhtml "UNROM") and [CPROM](CPROM.xhtml "CPROM"). 

Note that some Chinese ROM websites have both the [NES 2.0 Mapper 514](NES_2_0_Mapper_514.xhtml "NES 2.0 Mapper 514") and the [NES 2.0 Mapper 518](NES_2_0_Mapper_518.xhtml "NES 2.0 Mapper 518") Subor games set to Mapper 168, although no known emulator (Chinese or other) emulates them at that number. 

The software is more-or-less useless without also emulating its [exercise bike](https://www.nesdev.org/w/index.php?title=RacerMate_Bicycle&action=edit&redlink=1 "RacerMate Bicycle \(page does not exist\)"). 

## Contents

  * 1 Overview
  * 2 Banks
  * 3 Jumpers
  * 4 Registers
    * 4.1 Bank Select ($8000-$BFFF)
    * 4.2 RAM Protection and IRQ acknowledge ($C000-$FFFF)
  * 5 Interrupt
  * 6 Hardware
  * 7 References



## Overview

  * PRG ROM size: 64 KiB
  * PRG ROM bank size: 16 KiB
  * PRG RAM: No
  * CHR capacity: 64 KiB RAM, half (or all) battery-backed.
  * CHR bank size: 4 KiB
  * Nametable mirroring: Hardwired vertical mirroring
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): no



## Banks

  * CPU $8000-$BFFF: 16 KiB switchable PRG ROM bank
  * CPU $C000-$FFFF: 16 KiB PRG ROM bank, fixed to the last bank
  * PPU $0000-$0FFF: 4 KiB CHR RAM bank, fixed to the first bank, but see notes in Hardware.
  * PPU $1000-$1FFF: 4 KiB switchable CHR RAM bank



## Jumpers

All boards contain 32KB SRAM chips at U1 and U2. SRAM U2, holding CHR RAM banks 8-15, is always protected and battery-backed. Whether SRAM U1, holding CHR RAM banks 0-7, is battery backed is determined by the following jumpers on the board : 

  * J1: force deselect SRAM U1 when +5V fails
  * J2: allow SRAM U1 to remain selected after +5V fails
  * J3: Power SRAM U1 from +5V line
  * J4: Power SRAM U1 from battery-backed supply



J1+J4 vs J2+J3 should be selected as a pair. 

To the best of our knowledge (5 samples), all delivered boards had J2+J3 hardwired, so the default factory configuration for RacerMate is that only U2 SRAM is battery backed. 

J2+J3 can be disconnected and J1+J4 can be connected. This modification allows 64kB of battery-backed memory as U1 SRAM and U2 SRAM will be battery backed with the J1+J4 jumper combination. 

## Registers

### Bank Select ($8000-$BFFF)
    
    
    7  bit  0
    ---- ----
    PPxx CCCC
    ||   ||||
    ||   ++++- Select 4 KB CHR RAM bank for PPU $1000-$1FFF
    ++-------- Select 16 KB PRG ROM bank for CPU $8000-$BFFF
    

### RAM Protection and IRQ acknowledge ($C000-$FFFF)
    
    
    15   11 address   0  7 data  0
    ---- ---- ---- ----  ---- ----
    11.. .... Y... ....  XXXX XZXX
              |          |||| ||||
              +----------++++-++++- See below
    

Only one of the 9 above bits is used, but the software has no _a priori_ reason to know which. The firmware thus writes $FF to $F080 and $00 to $F000. The PCBs were originally laid out to use the bit above marked "Y" (A7). To the best of our knowledge (5 samples), all delivered boards were reworked to instead use the bit labeled "Z" (D2) 

When this bit is high, the IRQ is acknowledged and the IRQ counter is frozen at 0. When this bit is low, the IRQ counter counts up every M2 cycle. 

When this bit changes from 1 to 0, the RAM protection bit is cleared. While RAM is protected, battery-backed CHR RAMs can neither be read nor written (reads will return open bus, writes will be ignored). Non-battery-backed CHR RAMs can always be read and written. 

This IRQ disable bit is cleared and the RAM protection bit is set when power is removed from the cartridge. 

The 2A03's IRQ line is asserted while the counter's 1024s bit is set. 

## Interrupt

The interrupt is implemented with a large binary counter which drives the /IRQ line low after 1024 M2 cycles. Because the counter continues counting, it will automatically acknowledge itself after another 1024 M2 cycles (for a net frequency of no lower than 874 Hz). In practice, nocash says that the software requires at least 27 IRQs per vblank (for a spacing of not more than 1104 M2 cycles), and the software reliably clears the counter within 30 cycles. 

A board could have the resistor placed in the adjacent position, which would choose a delay of 2048 M2 cycles instead, but to the best of our knowledge (5 samples), none were. 

## Hardware

This cartridge uses two [74LS00s](7400.xhtml "7400"), two [74HCT32s](7432.xhtml "7432"), a 74HCT74, a 74LS174, a 74HCT4040, and a PNP transistor as a low-voltage detection circuit. [1]

Unlike [CPROM](CPROM.xhtml "CPROM") or [iNES Mapper 180](INES_Mapper_180.xhtml "INES Mapper 180"), which use AND logic to fix the first bank to 0 and switch the upper bank, the CHR-RAM here instead uses NAND logic, where the CHR banks are the 2's complement of those specified above. Because nothing else can read or write the RAMs except through this interface, the only question is whether the fixed bank is normally battery backed. It is NOT. The difference between NAND and AND logic is invisible to the software, and so emulators could use either convention if exchange of save files between emulators is not anticipated. 

It is of casual interest, but also completely irrelevant, that the lower bits of the CHR bank register are out of order: 3012. Once again, because nothing else can read or write the RAMs except through this bank register, this only could matter if one were to desolder the RAMs to read them. 

There is a space on the PCB to accept a donor [CIC](CIC_lockout_chip.xhtml "CIC"), labelled U11. In editions released before the top-loading NES-101, this space was populated with a Tengen CIC. 

## References

  1. ↑ [http://forums.nesdev.org/viewtopic.php?f=9&t=9136#p258112](http://forums.nesdev.org/viewtopic.php?f=9&t=9136#p258112) NES BBS



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with battery-backed CHR-RAM](Category_Mappers_with_battery_backed_CHR_RAM.xhtml), [Mappers with fixed-timing cycle IRQs](Category_Mappers_with_fixed_timing_cycle_IRQs.xhtml)
