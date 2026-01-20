# INES Mapper 071

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_071) | View [other pages](Special_AllPages.xhtml#INES_Mapper_071)

**iNES Mapper 071** is assigned to games developed by Codemasters and published by Camerica. It's largely a clone of [UNROM](UxROM.xhtml "UNROM"), and Camerica games were initially emulated under [iNES Mapper 002](UxROM.xhtml "INES Mapper 002") before 071 was assigned. 

Example Games: 

  * _MiG 29 - Soviet Fighter_
  * _Fire Hawk_
  * _The Fantastic Adventures of Dizzy_
  * _Bee 52_



## Contents

  * 1 Overview
  * 2 Banks
  * 3 Registers
    * 3.1 Outer bank select ($8000-$BFFF) (Only on iNES Mapper 232)
    * 3.2 Mirroring ($8000-$9FFF)
    * 3.3 Bank select ($C000-$FFFF)
    * 3.4 CIC stun control ($E000-$FFFF)
  * 4 Notes
  * 5 See also



## Overview

  * PRG ROM size: 128 or 256 KiB
  * PRG ROM bank size: 16 KiB inner; ( 64 KiB outer in [iNES Mapper 232](INES_Mapper_232.xhtml "INES Mapper 232") )
  * PRG RAM: None
  * CHR capacity: 8 KiB RAM
  * CHR bank size: Not bankswitched
  * Nametable [mirroring](Mirroring.xhtml "Mirroring"): Hardwired H or V (most) or mapper controlled 1-screen (Fire Hawk)
  * Subject to [bus conflicts](Bus_conflict.xhtml "Bus conflict"): No



## Banks

  * $8000-$BFFF: Switchable
  * $C000-$FFFF: Fixed to last bank
  * $8000-$FFFF: In the "Quattro" games, A16 and A17 are controlled by a separate register, allowing 64 KiB super-banks.



## Registers

### Outer bank select ($8000-$BFFF) (Only on [iNES Mapper 232](INES_Mapper_232.xhtml "INES Mapper 232"))

This register is present only on the Quattro carts, which use the [BF9096 IC](Camerica_BF909x_pinout.xhtml "Camerica BF909x pinout"), and on the CCU in the Aladdin adapter (per [KH](http://kevtris.org/mappers/camerica/aladdin.html)) 
    
    
    7  bit  0
    ---- ----
    xxxB Bxxx
       | |
       +-+--- Select 64 KiB PRG ROM bank for CPU $8000-$FFFF
    

### Mirroring ($8000-$9FFF)

This register is present only on _Fire Hawk_ , which uses the [BF9097 IC](Camerica_BF909x_pinout.xhtml "Camerica BF909x pinout"). [A submapper](NES_2_0_submappers.xhtml#071:_1_Fire_Hawk "NES 2.0 submappers") has been allocated for this difference. 

_Fire Hawk_ only writes this register at the address $9000, and other games like _Micro Machines_ and _Ultimate Stuntman_ write $00 to $8000 on startup. For compatibility without using a submapper, FCEUX begins all games with fixed mirroring, and applies single screen mirroring only once $9000-9FFF is written, ignoring writes to $8000-8FFF. 
    
    
    7  bit  0
    ---- ----
    xxxM xxxx
       |
       +----- Select 1 KiB CIRAM bank for PPU $2000-$2FFF
    

### Bank select ($C000-$FFFF)

This register is present on all games: [BF9093](Camerica_BF909x_pinout.xhtml "Camerica BF909x pinout") (most common), BF9096, BF9097, and CCU. The number of bits available vary: 4 for the BF9093, 3 for the BF9097, and 2 for the BF9096. 
    
    
    7  bit  0
    ---- ----
    xxxx PPPP
         ||||
         ++++- Select 16 KiB PRG ROM bank for CPU $8000-$BFFF
    

### CIC stun control ($E000-$FFFF)

This register is present on all games. A0 controls a latch going to the [CIC stun circuit](http://kevtris.org/mappers/lockout/camerica.html). 

## Notes

This mapper covers several Camerica/Codemasters boards. Most boards have fixed vertical or horizontal mirroring, but the _Fire Hawk_ board differs in that it has mapper controlled 1-screen mirroring. These variants can be distinguished by [NES 2.0 submappers](NES_2_0_submappers.xhtml "NES 2.0 submappers"). 

Some of these games are like _Battletoads_ in that they're very [picky](Tricky_to_emulate_games.xhtml "Tricky-to-emulate games") about timing and use some seldom used aspects of the NES, despite the mapper itself being as simple as the discrete logic mappers. 

In particular: 

  * _Bee 52_ uses the sprite overflow flag ($2002.5)
  * _MiG 29_ uses DMC IRQs, and is VERY PICKY about their timing. If your DMC IRQ timing isn't spot on (or at least very close), this game will glitch a lot.



This mapper also involves a custom lockout defeat circuit which is mostly unimportant for emulation purposes. Details will not be mentioned here, but are outlined in Kevtris' Camerica Mappers documentation. 

_Fire Hawk_ does some strange timing code when changing the mirroring mode. It is unknown whether or not any special timing is required. 

## See also

  * [Camerica Mappers by Kevin Horton](http://kevtris.org/mappers/camerica/index.html)
  * [NES Mapper list](http://www.romhacking.net/documents/362/) by Disch.
  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) by \Firebug\\. Information on mapper's initial state is innacurate.



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
