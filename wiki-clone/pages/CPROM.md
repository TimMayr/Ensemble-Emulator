# CPROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/CPROM) | View [other pages](Special_AllPages.xhtml#CPROM)

NES-CPROM (no [HVC](Family_Computer.xhtml "Famicom")-CPROM known) is a particular Nintendo cartridge board which uses discrete logic to provide up to four 4 KB banks of CHR RAM. The [iNES](INES.xhtml "INES") format assigns **mapper 13** to this board. Only _[Videomation](https://en.wikipedia.org/wiki/Videomation "wikipedia:Videomation")_ , a paint program for NES, is known to use it. 

## Contents

  * 1 Overview
  * 2 Banks
  * 3 Registers
    * 3.1 Bank select ($8000-$FFFF)
  * 4 Hardware
  * 5 See also



## Overview

  * PRG ROM size: 32 KiB (DIP-28 standard pinout)
  * PRG ROM bank size: Not bankswitched
  * PRG RAM: None
  * CHR capacity: 16 KiB RAM (in two [8KiB RAMs](6264_static_RAM.xhtml "6264 static RAM"))
  * CHR bank size: 4 KiB
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Vertical
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): Yes



## Banks

  * PPU $0000-$0FFF: 4 KiB fixed CHR RAM bank (first page)
  * PPU $1000-$1FFF: 4 KiB swappable CHR RAM page



## Registers

### Bank select ($8000-$FFFF)
    
    
    7  bit  0
    ---- ----
    xxxx xxCC
           ||
           ++- Select 4 KiB CHR RAM bank for PPU $1000-$1FFF
    

## Hardware

The CPROM board uses a [74HC161](74161.xhtml "74161") (4-bit latch) and a [74HC08](7408.xhtml "7408") (quad 2-input AND gate), with a [74HC04](7404.xhtml "7404") (hex inverter) to unify the two RAMs. 

## See also

  * [Kevtris's notes](http://kevtris.org/mappers/nes_discrete/NES_CPROM.html)
  * The [NROM-368](NROM_368.xhtml "NROM-368") proposal is also compatible with this board.



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers with bus conflicts](Category_Mappers_with_bus_conflicts.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
