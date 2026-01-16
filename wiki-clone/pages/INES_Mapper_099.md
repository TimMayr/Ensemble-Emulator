# INES Mapper 099

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_099) | View [other pages](Special_AllPages.xhtml#INES_Mapper_099)

**Mapper 99** is a simple mapper used by [Vs. System](Vs__System.xhtml "Vs. System") games such as _Vs. Super Mario Bros._ It is comparable to [CNROM](CNROM.xhtml "CNROM"), but without bus conflicts. 

The Vs. System has other ports related to coin insertion and DIP switches that must be emulated to get games to advance past [attract mode](https://en.wikipedia.org/wiki/Attract_mode "wikipedia:Attract mode"), and most games' palettes differ from the standard RGB NES palette used by _Duck Hunt_ and PlayChoice games. See [Vs. System](Vs__System.xhtml "Vs. System") for more information. 

## Contents

  * 1 Overview
  * 2 Banks
  * 3 Registers
    * 3.1 Bank select ($4016)
  * 4 Hardware



## Overview

  * PRG ROM size: 8 to 40 KiB, in 8 KiB steps (as one to four 8 KiB ROMs, or one 16 KiB ROM and three 8 KiB ROMs)
  * PRG ROM bank size: 8 KiB
  * PRG RAM: 2 KiB, at least in Vs DualSystem, switchable between CPUs
  * CHR capacity: 8 or 16 KiB (as one or two 8 KiB ROMs)
  * CHR bank size: 8 KiB
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Fixed as four screen
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): No



## Banks

  * CPU $6000-$7FFF: 2 KiB RAM, swappable between CPUs (open bus when not available)
  * CPU $8000-$9FFF: 8 KiB switchable PRG ROM bank
  * CPU $A000-$FFFF: 24 KiB fixed PRG ROM bank
  * PPU $0000-$1FFF: 8 KiB switchable CHR ROM bank



## Registers

### Bank select ($4016)
    
    
    7  bit  0
    ---- ----
    xxxx xBRC
          |
          +--- Select both PRG ROM and CHR ROM bank
    

Note that the two LSBs are used for other things (see [Vs. System](Vs__System.xhtml "Vs. System")), so you'll probably need to keep a copy of the value stored. 

## Hardware

The banking configuration described by mapper 99 connects the 2A03's OUT2 pin to a simple 1-to-2 demultiplexer to select between two CHR ROMs. 

Only _Vs. Gumshoe_ uses the 40KiB PRG variant; in the [iNES](INES.xhtml "INES") encapsulation, the 8KiB banks are arranged as 0, 1, 2, 3, 0alternate, empty. (The hardware adds a wire to EPROM socket 2D or 6D, connecting OUT2 to A13) 

Note: unlike all other mappers, an undersize mapper 99 image implies open bus instead of mirroring. The original hardware has six sockets, and if any of them are not populated, the corresponding area isn't driven. Most pertinently, this means that for games with only 8 KiB of CHR, when the bank select bit is high the PPU will have no tile data to render! 

Although this hardware has never been used officially outside the Vs. System, its behavior is well-defined in the absence of the [for Vs. System](INES.xhtml#Flags_7 "INES") or [four-screen VRAM](INES.xhtml#Flags_6 "INES") bits. 

An [NES-compatible bootleg of _Vs. Super Mario Bros._](http://www.retrousb.com/product_info.php?cPath=29&products_id=94) was produced, with DIP switches. They must be [implemented differently](https://satoshimatrix.wordpress.com/2010/11/16/vs-super-mario-bros-review-nes/) than the Vs. switches, because there's no way to prevent a bus conflict on reads from the lower five bits of the controller ports. 

Categories: [CNROM-like mappers](Category_CNROM_like_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with overlapping PRG and CHR registers](Category_Mappers_with_overlapping_PRG_and_CHR_registers.xhtml)
