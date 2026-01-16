# VRC2 and VRC4

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VRC2_and_VRC4) | View [other pages](Special_AllPages.xhtml#VRC2_and_VRC4)

**VRC2, VRC4**

**Company** | Konami   
---|---  
**Complexity** | ASIC   
**Boards** | VRC2a-c VRC4a-f   
**Pinout** | [VRC2 pinout](VRC2_pinout.xhtml "VRC2 pinout")  
**PRG ROM capacity** | 256K   
**PRG ROM window** | 8K   
**PRG RAM capacity** | 8K   
**PRG RAM window** | 8K   
**CHR capacity** | 256K 512K   
**CHR window** | 1K   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H, V switchable H, V, 1 switchable   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | Yes   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [021](VRC2_and_VRC4.xhtml "INES Mapper 021"), [022](VRC2_and_VRC4.xhtml "INES Mapper 022"), [023](VRC2_and_VRC4.xhtml "INES Mapper 023"), [025](VRC2_and_VRC4.xhtml "INES Mapper 025")  
  
**NESCartDB**

[iNES 021](https://nescartdb.com/search/advanced?ines=21)  
---  
[iNES 022](https://nescartdb.com/search/advanced?ines=22)  
[iNES 023](https://nescartdb.com/search/advanced?ines=23)  
[iNES 025](https://nescartdb.com/search/advanced?ines=25)  
  
The **Konami VRC2** and **Konami VRC4** are two related [ASIC](Category_ASIC_mappers.xhtml "Category:ASIC mappers") [mappers](Mapper.xhtml "Mapper") in the VRC[1] family. 

Because the VRC2 is mostly a subset of the VRC4, relevant games are often emulated as VRC4 only. iNES mappers **21** , **22** , **23** and **25** implement various board permutations, and [NES 2.0 submappers](NES_2_0_submappers.xhtml "NES 2.0 submappers") may be used to disambiguate further. 

[Mapper 27](INES_Mapper_027.xhtml "INES Mapper 027") represents a related pirate mapper. 

Example games: 

     Game | Variant | iNES mapper   
---|---|---  
_Ganbare Pennant Race_ | VRC2a | 22   
_TwinBee 3_ | VRC2a | 22   
_Wai Wai World_ | VRC2b | 23   
_Ganbare Goemon Gaiden_ | VRC2c | 25   
_Wai Wai World 2_ | VRC4a | 21   
_Bio Miracle Bokutte Upa_ | VRC4b | 25   
_Gradius 2 (J)_ | VRC4b | 25   
_Ganbare Goemon Gaiden 2_ | VRC4c | 21   
_Teenage Mutant Ninja Turtles (J)_ | VRC4d | 25   
_Boku Dracula-kun_ | VRC4e | 23   
_Tiny Toon Adventures (J)_ | VRC4e | 23   
  
## Contents

  * 1 Variations
    * 1.1 VRC2 vs VRC4
    * 1.2 iNES mappers
  * 2 Banks
    * 2.1 Common
    * 2.2 VRC2
    * 2.3 VRC4
  * 3 Registers
    * 3.1 PRG Swap Mode/WRAM control ($9002) VRC4
    * 3.2 PRG Select 0 ($8000, $8001, $8002, $8003)
    * 3.3 PRG Select 1 ($A000, $A001, $A002, $A003)
    * 3.4 Mirroring Control ($9000, $9001, $9002, $9003)
    * 3.5 VRC4 External Select ($9003)
    * 3.6 CHR Select 0 low($B000), high($B001)
    * 3.7 CHR Select 1 low($B002), high($B003)
    * 3.8 CHR Select 2…7 ($C000-$EFFF)
    * 3.9 IRQ Control ($F00x) VRC4
    * 3.10 Microwire interface ($6000-$6FFF) VRC2
  * 4 Hardware
  * 5 References



## Variations

Variations VRC2a/b/c and VRC4a/b/c/d/e/f are referring to different _PCB layouts_. Considering the mapper chip itself, all VRC2 behave the same way and all VRC4 behave the same way. This section describes how each variation presents itself to the NES CPU considering the different _PCBs_. 

The VRC2 and VRC4 were used with several similar but incompatible boards. The primary difference between them was having the mapper address lines connected in different ways. In particular, two lines chosen from A0-A7 will be used to select registers. 

VRC2   
---  
Nickname | PCB | A0 | A1 | Registers | iNES mapper | submapper   
VRC2a | 351618 | A1 | A0 | $x000, $x002, $x001, $x003 | 22 | 0   
VRC2b | many† | A0 | A1 | $x000, $x001, $x002, $x003 | 23 | 3   
VRC2c | 351948 | A1 | A0 | $x000, $x002, $x001, $x003 | 25 | 3   
VRC4   
---  
Nickname | PCB | A0 | A1 | Registers | iNES mapper | submapper   
VRC4a | 352398 | A1 | A2 | $x000, $x002, $x004, $x006 | 21 | 1   
VRC4b | 351406 | A1 | A0 | $x000, $x002, $x001, $x003 | 25 | 1   
VRC4c | 352889 | A6 | A7 | $x000, $x040, $x080, $x0C0 | 21 | 2   
VRC4d | 352400 | A3 | A2 | $x000, $x008, $x004, $x00C | 25 | 2   
VRC4e | 352396 | A2 | A3 | $x000, $x004, $x008, $x00C | 23 | 2   
VRC4f | - | A0 | A1 | $x000, $x001, $x002, $x003 | 23 | 1   
  
    †  The VRC2b PCBs include: 350603, 350636, 350926, 351179

### VRC2 vs VRC4

The VRC2 has: 

  * CHR limit of 256k
  * 1 bit latch at $6000-6FFF
  * Only horizontal or vertical mirroring.



The VRC4 has: 

  * CHR limit of 512k
  * Additional 8k PRG bank at $C000
  * Horizontal, vertical and 1-screen mirroring.
  * IRQ device



The VRC2a (mapper 22) additionally wires the CHR banking lines differently (see below). 

The RAM decoding circuit that's part of the VRC4 itself only decodes RAM from $6000-$6FFF. For the [one game](http://bootgod.dyndns.org:7777/profile.php?id=1573) with 8 KiB of RAM, an external [circuit was added](PRG_RAM_circuit.xhtml "PRG RAM circuit"). 

### iNES mappers

iNES mappers 21, 23 and 25 each implements _two_ address mappings at the same time: 

  * **21:** VRC4a, VRC4c
  * **23:** VRC2b + VRC4f, VRC4e
  * **25:** VRC2c + VRC4b, VRC4d



Because the address pairings do not overlap, and the games appear to use these registers in a well behaved manner, it is presumed sufficient for compatibility in most cases. Additionally, the VRC4 is always presumed for these three mappers. 

With an [NES 2.0](NES_2_0.xhtml "NES 2.0") header, a [submapper](NES_2_0_submappers.xhtml "NES 2.0 submappers") may be used to specify which address mapping to use, and either VRC2 or VRC4. 

## Banks

### Common

  * PPU $0000-$03FF: 1 KiB switchable CHR bank
  * PPU $0400-$07FF: 1 KiB switchable CHR bank
  * PPU $0800-$0BFF: 1 KiB switchable CHR bank
  * PPU $0C00-$0FFF: 1 KiB switchable CHR bank
  * PPU $1000-$13FF: 1 KiB switchable CHR bank
  * PPU $1400-$17FF: 1 KiB switchable CHR bank
  * PPU $1800-$1BFF: 1 KiB switchable CHR bank
  * PPU $1C00-$1FFF: 1 KiB switchable CHR bank



### VRC2

  * CPU $6000-$6FFF: 1 bit latch, or
  * CPU $6000-$7FFF: optional 8 KiB RAM
  * CPU $8000-$9FFF: 8 KiB switchable PRG ROM bank
  * CPU $A000-$BFFF: 8 KiB switchable PRG ROM bank
  * CPU $C000-$FFFF: 16 KiB PRG ROM bank, fixed to the last 16 KiB



### VRC4

  * CPU $6000-$6FFF: optional 2 KiB PRG RAM bank (mirrored once), or
  * CPU $6000-$7FFF: optional 8 KiB PRG RAM bank
  * CPU $8000-$9FFF (or $C000-$DFFF): 8 KiB switchable PRG ROM bank
  * CPU $A000-$BFFF: 8 KiB switchable PRG ROM bank
  * CPU $C000-$DFFF (or $8000-$9FFF): 8 KiB PRG ROM bank, fixed to the second-last bank
  * CPU $E000-$FFFF: 8 KiB PRG ROM bank, fixed to the last bank



## Registers

This page lists registers as they are in the VRC2b and VRC4f variants (iNES mapper 23). 

### PRG Swap Mode/WRAM control ($9002) VRC4
    
    
    7  bit  0
    ---------
    .... ..MW
           |+- WRAM Control
           +-- Swap Mode
    

This register is VRC4 only. 

When 'W' is clear: 

  * the 8 KiB page at $6000 is open bus, and WRAM content cannot be read nor written



When 'W' is set: 

  * the 8 KiB page at $6000 is WRAM, and WRAM content can be read and written



When 'M' is clear: 

  * the 8 KiB page at $8000 is controlled by the $800x register
  * the 8 KiB page at $C000 is fixed to the second last 8 KiB in the ROM



When 'M' is set: 

  * the 8 KiB page at $8000 is fixed to the second last 8 KiB in the ROM
  * the 8 KiB page at $C000 is controlled by the $800x register



### PRG Select 0 ($8000, $8001, $8002, $8003)
    
    
    7  bit  0
    ---------
    ...P PPPP
       | ||||
       +-++++- Select 8 KiB PRG bank at $8000 or $C000 depending on Swap Mode
    

VRC2 does not have a Swap Mode. The bank is always at $8000. 

### PRG Select 1 ($A000, $A001, $A002, $A003)
    
    
    7  bit  0
    ---------
    ...P PPPP
       | ||||
       +-++++- Select 8 KiB PRG bank at $A000
    

### Mirroring Control ($9000, $9001, $9002, $9003)
    
    
    7  bit  0
    ---------
    .... ..MM
           ||
           ++- [Mirroring](Mirroring.xhtml "Mirroring") (0: vertical; 1: horizontal; 2: one-screen, lower bank; 3: one-screen, upper bank)
    

VRC2 supports only vertical or horizontal mirroring. Bit 1 is ignored. 

VRC4 only has mirroring control at $9000 only. $9002 is used to select PRG swap mode instead (see above). 

VRC2-using games are usually well-behaved and only write 0 or 1 to this register, but [Wai Wai World](http://bootgod.dyndns.org:7777/profile.php?id=2274) in one instance writes $FF instead[2]. 

### VRC4 External Select ($9003)

The VRC4 decodes writes to $9003 and emits an active low signal. No Konami games made use of this, but some unlicensed games, assigned to other mappers, added extra hardware behind this address. 

### CHR Select 0 low($B000), high($B001)
    
    
      $B000        $B001
    7  bit  0    7  bit  0
    ---------    ---------
    .... LLLL    ...H HHHH
         ||||       | ||||
         ||||       +-++++- High 5 bits of 1 KiB CHR bank at PPU $0000
         ++++-------------- Low 4 bits
    

VRC2 only has 4 high bits of CHR select. $B001 bit 4 is ignored. 

On VRC2a (mapper 22), the low bit is ignored (right shift value by 1). 

### CHR Select 1 low($B002), high($B003)
    
    
      $B002        $B003
    7  bit  0    7  bit  0
    ---------    ---------
    .... LLLL    ...H HHHH
         ||||       | ||||
         ||||       +-++++- High 5 bits of 1 KiB CHR bank at PPU $0400
         ++++-------------- Low 4 bits
    

VRC2 only has 4 high bits of CHR select. $B003 bit 4 is ignored. 

On VRC2a (mapper 22), the low bit is ignored (right shift value by 1). 

### CHR Select 2…7 ($C000-$EFFF)

The other six CHR bank selects continue the pattern: 

Write to CPU address  | 1KB CHR bank affected   
---|---  
(low 4 bits) | (high 5 bits)   
$C000 | $C001 | $0800-$0BFF   
$C002 | $C003 | $0C00-$0FFF   
$D000 | $D001 | $1000-$13FF   
$D002 | $D003 | $1400-$17FF   
$E000 | $E001 | $1800-$1BFF   
$E002 | $E003 | $1C00-$1FFF   
  
### IRQ Control ($F00x) VRC4
    
    
    $F000:  IRQ Latch, low 4 bits
    $F001:  IRQ Latch, high 4 bits
    $F002:  IRQ Control
    $F003:  IRQ Acknowledge
    

Many VRC mappers use the same IRQ system. For details on IRQ operation, see [VRC IRQs](VRC_IRQ.xhtml "VRC IRQ"). 

### Microwire interface ($6000-$6FFF) VRC2

This only exists on VRC2. 

How it was supposed to work: 
    
    
    7  bit  0
    ---------
    .... .SCD
          |||
          ||+- Data to EEPROM (write) or from EEPROM (read)
          |+-- Clock to EEPROM (write only)
          +--- Chip Select (write only)
    

How it works in practice: 
    
    
    7  bit  0
    ---------
    .... ...L
            |
            +- 1-bit latch value (r/w)
    

Reads from $6000-6FFF return open bus for the top 7 bits. Reads from $7000-7FFF only ever return open bus. 

The VRC2 was supposed to have shipped with a [Microwire](https://en.wikipedia.org/wiki/Microwire "wikipedia:Microwire") interface for save games. However, Konami never used it, seemingly due to a defect in the VRC2. 

Across all the VRC2-using boards, the _Data from EEPROM_ pin has been connected to many different things. On **351618** ([22](VRC2_and_VRC4.xhtml "INES Mapper 022")) it's connected to ground. On **350603** , **350636** , and **351179** it floats. On **350926** it's connected to _Data to EEPROM_. On **351948** ([25](VRC2_and_VRC4.xhtml "INES Mapper 025")) it's connected to ground but extra hardware keeps the VRC2 from seeing reads from $6000 so as to not conflict with PRG RAM. 

Several games, including Contra (J) and Ganbare Goemon 2 (J), rely on the two _Data_ pins being connected to each other, and so expect to be able to read the written value back. In these cases, the LSB agrees with the last value written and the upper seven bits are open bus, e.g. both LDA $6100 and LDA $6000 will return $60|latch. Returning neither open bus nor 0x00 will work, and these games will lock almost immediately after execution begins. Fortunately, no games ever rely on any other behavior. 

Emulators that use the same VRC4 core (and its PRG RAM) for VRC2 emulation will have the effect simulated for them. However, only **351948** contains any RAM. 

## Hardware

The VRC2 and VRC4 exist in a [40-pin PDIP package](VRC2_pinout.xhtml "VRC2 pinout"). 

## References

  1. ↑ [Forum thread](http://forums.nesdev.org/viewtopic.php?f=2&t=10611): Several Famicom Nes Documents in Japanese indicate that VRC stands for "Virtual ROM Controller"
  2. ↑ [Forum thread](http://forums.nesdev.org/viewtopic.php?f=3&t=13473): incorrect mirroring in Wai Wai World, and VRC2 testing.



  * [NES Mapper List](http://www.romhacking.net/documents/362/) by Disch
  * [Konami Mapper](http://nesdev.org/konami-e.txt) by goroh, translated by Sgt. Bowhack
  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) by \Firebug\, information about mapper's initial state is inaccurate.
  * The CHR registers really are 9 bits: <http://forums.nesdev.org/viewtopic.php?t=8569>



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with single-screen mirroring](Category_Mappers_with_single_screen_mirroring.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
