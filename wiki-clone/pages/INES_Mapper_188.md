# INES Mapper 188

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_188) | View [other pages](Special_AllPages.xhtml#INES_Mapper_188)

iNES Mapper 188 describes the board used for Bandai's Karaoke Studio. This game, like [Nantettatte!! Baseball](INES_Mapper_068.xhtml#.24F000-.24FFFF:_PRG_bank_at_.248000 "INES Mapper 068") additionally supports an external ROM that allows augmenting the original game. 

## Contents

  * 1 Overview
  * 2 Registers
    * 2.1 $6000-$7FFF: Microphone input
    * 2.2 $8000-$FFFF: Banking control
  * 3 Warning
  * 4 References



## Overview

  * PRG ROM: Up to 256 KiB + Up to 256 KiB expansion cartridge
  * PRG ROM bank size: 16 KiB
  * PRG RAM: None
  * CHR capacity: 8 KiB RAM
  * CHR bank size: Not bankswitched
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Controlled by mapper
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): Yes



The [mapper IC](Bandai_M60001_0115P_pinout.xhtml "Bandai M60001-0115P pinout") used here has Mitsubishi's part number for custom orders: M60001. 

## Registers

### $6000-$7FFF: Microphone input

The microphone for this game is permanently tethered to the cartridge, and contains two buttons labeled "A" (closer to the microphone itself) and "B" (closer to the cord). 

Reads from this address provide the status of the microphone and its buttons: 
    
    
    7  bit  0
    .... .MBA
    ---- ----
    |||| ||||
    |||| |||+- 0: A button is pressed
    |||| ||+-- 0: B button is pressed
    |||| |+--- 1-bit ADC microphone input
    ++++-+---- open bus
    

### $8000-$FFFF: Banking control
    
    
    7  bit  0
    .LXR BBBB
    ---- ----
     ||| ||||
     ||| ++++- Select ROM bank mapped from $8000-$BFFF.
     ||+------ 0: Select external ROM, 1: Select internal ROM
     |+------- CIRAM A10 is connected to  0: PPU A10 ("vertical mirroring"), 1: PPU A11 ("horizontal mirroring")
     +-------- 1 bit latch, present but unused
    

The bank from $C000-$FFFF is fixed to the last bank of the internal ROM. 

## Warning

The 256 KiB iNES dumps under this mapper are actually the 128KiB internal ROM followed by a 128KiB expansion ROM. 

For the 128 KiB iNES dump (solely the internal ROM), several emulators implement logic that selects the last bank of the internal ROM when the game tries to select the first of the external ROM. There may be a fingerprint the game requires on being absent. Properly implementing open bus is more accurate. 

## References

  * Enri's schematic: <http://cmpslv3.stars.ne.jp/Famic/Famic.htm> (scroll down to " カラオケスタジオの回路図 ")
  * Naruko's wiki: [カラオケスタジオ](http://seesaawiki.jp/famicomcartridge/d/%a5%ab%a5%e9%a5%aa%a5%b1%a5%b9%a5%bf%a5%b8%a5%aa)
  * Nestopia and FCEUX source



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with bus conflicts](Category_Mappers_with_bus_conflicts.xhtml)
