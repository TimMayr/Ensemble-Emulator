# UxROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/UxROM) | View [other pages](Special_AllPages.xhtml#UxROM)

**UxROM**

**Company** | Nintendo, others   
---|---  
**Games** | [155 in NesCartDB](https://nescartdb.com/search/advanced?ines=2)  
**Complexity** | Discrete logic   
**Boards** | UNROM, UOROM   
**PRG ROM capacity** | 256K/4096K   
**PRG ROM window** | 16K + 16K fixed   
**PRG RAM capacity** | None   
**CHR capacity** | 8K   
**CHR window** | n/a   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | Fixed H or V, controlled by solder pads   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | Yes/No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [002](UxROM.xhtml "INES Mapper 002"), [094](INES_Mapper_094.xhtml "INES Mapper 094"), [180](INES_Mapper_180.xhtml "INES Mapper 180")  
  
**NESCartDB**

[iNES 002](https://nescartdb.com/search/advanced?ines=2)  
---  
[UxROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=-U%25ROM)  
[UNROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=UNROM)  
[UOROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=UOROM)  
  
The generic designation _UxROM_ refers to the Nintendo cartridge boards _NES-UNROM_ , _NES-UOROM_ , [HVC-UN1ROM](INES_Mapper_094.xhtml "INES Mapper 094") their [HVC](Family_Computer.xhtml "Famicom") counterparts, and clone boards. 

  * **[iNES](INES.xhtml "INES") Mapper 002** is the implementation of the most common usage of UxROM compatible boards, described in this article.
  * [iNES Mapper 094](INES_Mapper_094.xhtml "INES Mapper 094") describes UN1ROM, used only in _Senjou no Ookami_.
  * [iNES Mapper 180](INES_Mapper_180.xhtml "INES Mapper 180") describes a reconfiguration of UNROM used only in _Crazy Climber_.



Example games: 

  * _Mega Man_
  * _Castlevania_
  * _Contra_
  * _Duck Tales_
  * _Metal Gear_



## Contents

  * 1 Banks
  * 2 Solder pad config
  * 3 Registers
    * 3.1 Bank select ($8000-$FFFF)
  * 4 Hardware
  * 5 Variants
  * 6 See also
  * 7 External links



## Banks

  * CPU $8000-$BFFF: 16 KB switchable PRG ROM bank
  * CPU $C000-$FFFF: 16 KB PRG ROM bank, fixed to the last bank



## Solder pad config

  * Horizontal mirroring : 'H' disconnected, 'V' connected.
  * Vertical mirroring : 'H' connected, 'V' disconnected.



## Registers

### Bank select ($8000-$FFFF)
    
    
    7  bit  0
    ---- ----
    xxxx pPPP
         ||||
         ++++- Select 16 KB PRG ROM bank for CPU $8000-$BFFF
              (UNROM uses bits 2-0; UOROM uses bits 3-0)
    

Emulator implementations of **iNES mapper 2** treat this as a full 8-bit bank select register, without bus conflicts. This allows the mapper to be used for similar boards that are compatible. 

To make use of all 8-bits for a 4 MB PRG ROM, an [NES 2.0](NES_2_0.xhtml "NES 2.0") header must be used ([iNES](INES.xhtml "INES") can only effectively go to 2 MB). 

The original UxROM boards used by Nintendo were subject to bus conflicts, and the relevant games all work around this in software. Some emulators (notably FCEUX) will have bus conflicts by default, but others have none. [NES 2.0 submappers](NES_2_0_submappers.xhtml#002.2C_003.2C_007:_UxROM.2C_CNROM.2C_AxROM "NES 2.0 submappers") were assigned to accurately specify whether the game should be emulated with bus conflicts. 

## Hardware

The UNROM, UN1ROM, and UOROM boards contain a [74HC161](74161.xhtml "74161") binary counter used as a quad D latch (4-bit register) and a [74HC32](7432.xhtml "7432") quad 2-input OR gate to make one bank always visible. 

The circuit behaves as if it were as follows: 
    
    
          /PRGSEL               A14  A13-A0
           |                      |      |
           |                      |      |
           | D2-D0-.       ,------'      |
           |       |     . |             |
           | ,-----+--.  |`+.            |
           | |Register+--+0  `.          |
           | `--------'  |    |__.       |
           |       |     |    |  |       |
           | R/W --'   7-+1  ,'  |       |
           |             | ,'    |       |
           |             |'      |       |
           |                     |       |
          ,+---------------------+-------+-----.
          |/CE              A16-A14  A13-A0    |
          |         128K by 8 bit ROM     D7-D0+-- to 2A03 data bus
          |                                    |
          `------------------------------------'
    

(This diagram is for UNROM. UOROM has one more bit in the register, multiplexer output, and address input A17, and the multiplexer's other input is $F. Homebrew boards capable of 512 KiB have one more of each, and the multiplexer's input is $1F.) 

The quad OR gate here acts as a multiplexer. A [74HC02](7402.xhtml "7402") quad NOR gate can be used instead if the banks are stored in reverse order in the ROM. If the program is 128 KiB or smaller, the 7402 way leaves one NOR gate free to invert R/W into /OE to avoid [bus conflicts](Bus_conflict.xhtml "Bus conflict").[[1]](http://forums.nesdev.org/viewtopic.php?p=111686#p111686)

If an actual multiplexer ([74HC157](74157.xhtml "74157") quad 2:1) is cheaper than an OR gate, a third-party UxROM-compatible board can use that instead of the 74HC32, as [kyuusaku suggested](http://forums.nesdev.org/viewtopic.php?p=93516#p93516). 

## Variants

The [mapper](INES_Mapper_071.xhtml "INES Mapper 071") used in Codemasters games published by Camerica extends UxROM with [CIC](CIC_lockout_chip.xhtml "CIC") defeat circuitry. 

Nintendo's [HVC-UN1ROM](INES_Mapper_094.xhtml "INES Mapper 094") board moves the bankswitching bits within the byte. 

_Crazy Climber_ replaces the [74HC32](7432.xhtml "7432") quad-OR gate by a [74HC08](7408.xhtml "7408") quad-AND gate, so that the first bank is fixed at $8000-$bfff and the switchable bank is present at $c000-$ffff. This configuration is assigned to [iNES Mapper 180](INES_Mapper_180.xhtml "INES Mapper 180"), which uses the same UNROM PCB. 

With an 8-bit latch ([74HC377](74377.xhtml "74377") or an additional 74HC161) and an additional 74HC32 to control A18-A21, a third-party board implementing this mapper can switch 4 MiB of PRG ROM. 

[Battle Kid 2: Mountain of Torment](User_Sivak_Battle_Kid_2__Mountain_of_Torment.xhtml "User:Sivak/Battle Kid 2: Mountain of Torment") implements a 512kB UxROM mapper. 

[UNROM 512](UNROM_512.xhtml "UNROM 512") implements a superset of 512kB UxROM, with additional CHR-RAM banking and the possibility of flash save. 

The homebrew game _Alwa's Awakening_ adds 8 KiB of battery-backed PRG-RAM in the usual $6000-$7FFF address range. A few Subor educational cartridges have non-battery-backed PRG-RAM in the same range as well. Emulators should support this PRG-RAM at least in the presence of a NES 2.0 header that explicitly specifies it. 

## See also

  * [Programming UNROM](Programming_UNROM.xhtml "Programming UNROM")



## External links

  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) by \Firebug\, information about mapper's initial state is inaccurate.
  * NES mapper list by Disch [[2]](http://www.romhacking.net/documents/362/)
  * [Converting UNROM to UOROM](https://forums.nesdev.org/viewtopic.php?p=179581#p179581)



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers with bus conflicts](Category_Mappers_with_bus_conflicts.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
