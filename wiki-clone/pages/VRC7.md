# VRC7

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VRC7) | View [other pages](Special_AllPages.xhtml#VRC7)

**VRC7**

**Company** | Konami   
---|---  
**Games** | [2 in NesCartDB](https://nescartdb.com/search/advanced?ines=85)  
**Complexity** | ASIC   
**Boards** | 352402, 353429   
**Pinout** | [VRC7 pinout](VRC7_pinout.xhtml "VRC7 pinout")  
**PRG ROM capacity** | 512K   
**PRG ROM window** | 8Kx3 + 8K fixed   
**PRG RAM capacity** | 8K   
**PRG RAM window** | 8K fixed   
**CHR capacity** | 256K   
**CHR window** | 1Kx8   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H, V, or 1, switchable   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | Yes   
**Audio** | [VRC7a only](VRC7_audio.xhtml "VRC7 audio")  
**iNES[mappers](Mapper.xhtml "Mapper")** | [085](VRC7.xhtml "INES Mapper 085")  
  
The **Konami VRC7** is an [ASIC](Category_ASIC_mappers.xhtml "Category:ASIC mappers") [mapper](Mapper.xhtml "MMC"). It is assigned to **iNES Mapper 085**. 

Example games: 

  * _Lagrange Point_
  * _Tiny Toon Adventures 2_



See [VRC7 pinout](VRC7_pinout.xhtml "VRC7 pinout") for chip pinout. 

## Contents

  * 1 Banks
    * 1.1 CPU
    * 1.2 PPU
  * 2 Registers
    * 2.1 PRG Select 0 ($8000)
    * 2.2 PRG Select 1 ($8010, $8008)
    * 2.3 PRG Select 2 ($9000)
    * 2.4 CHR Select 0…7 ($A000…$DFFF)
    * 2.5 Mirroring Control ($E000)
    * 2.6 IRQ Control ($E008 - $F010)
  * 3 Sound ($9010, $9030)
  * 4 See also



## Banks

### CPU

  * $6000-$7FFF: 8 KB PRG-RAM bank, fixed
  * $8000-$9FFF: 8 KB switchable PRG-ROM bank
  * $A000-$BFFF: 8 KB switchable PRG-ROM bank
  * $C000-$DFFF: 8 KB switchable PRG-ROM bank
  * $E000-$FFFF: 8 KB PRG-ROM bank, fixed to the last bank



### PPU

  * $0000-$03FF: 1 KB switchable CHR-ROM bank
  * $0400-$07FF: 1 KB switchable CHR-ROM bank
  * $0800-$0BFF: 1 KB switchable CHR-ROM bank
  * $0C00-$0FFF: 1 KB switchable CHR-ROM bank
  * $1000-$13FF: 1 KB switchable CHR-ROM bank
  * $1400-$17FF: 1 KB switchable CHR-ROM bank
  * $1800-$1BFF: 1 KB switchable CHR-ROM bank
  * $1C00-$1FFF: 1 KB switchable CHR-ROM bank



If CHR-RAM is used instead of CHR-ROM, the banking feature is still functional. 

## Registers

There are two board variations: 

  * VRC7b uses A3 to select registers ($x008), used in _Tiny Toon Adventures_ (**Submapper 1**).
  * VRC7a uses A4 to select registers ($x010), used in _Lagrange Point_ (**Submapper 2**).



Although A5 is wired for sound registers on both ($x028, $x030), in the former board the ceramic resonator necessary for the sound hardware to work is missing. 

### PRG Select 0 ($8000)
    
    
    7  bit  0
    ---------
    ..PP PPPP
      || ||||
      ++-++++- Select 8 KB PRG ROM at $8000
    

  


### PRG Select 1 ($8010, $8008)
    
    
    7  bit  0
    ---------
    ..PP PPPP
      || ||||
      ++-++++- Select 8 KB PRG ROM at $A000
    

### PRG Select 2 ($9000)
    
    
    7  bit  0
    ---------
    ..PP PPPP
      || ||||
      ++-++++- Select 8 KB PRG ROM at $C000
    

### CHR Select 0…7 ($A000…$DFFF)

Write to CPU address | 1KB CHR bank affected   
---|---  
$A000 | $0000-$03FF   
$A008 or $A010 | $0400-$07FF   
$B000 | $0800-$0BFF   
$B008 or $B010 | $0C00-$0FFF   
$C000 | $1000-$13FF   
$C008 or $C010 | $1400-$17FF   
$D000 | $1800-$1BFF   
$D008 or $D010 | $1C00-$1FFF   
  
### Mirroring Control ($E000)
    
    
    7  bit  0
    ---------
    RS.. ..MM
    ||     ||
    ||     ++- [Mirroring](Mirroring.xhtml "Mirroring") (0: vertical; 1: horizontal;
    ||                        2: one-screen, lower bank; 3: one-screen, upper bank)
    |+-------- Silence expansion sound if set
    +--------- WRAM enable (1: enable WRAM, 0: protect)
    

### IRQ Control ($E008 - $F010)
    
    
    $E008, $E010:  IRQ Latch
           $F000:  IRQ Control
    $F008, $F010:  IRQ Acknowledge
    

Many VRC mappers use the same IRQ system. For details on IRQ operation, see [VRC IRQs](VRC_IRQ.xhtml "VRC IRQ"). 

## Sound ($9010, $9030)

For details on sound information, see [VRC7 audio](VRC7_audio.xhtml "VRC7 audio"). 

## See also

  * [Konami VRC-VII Chip Info](http://nesdev.org/vrcvii.txt) by Kevin Horton.
  * [NES Mapper List](http://www.romhacking.net/documents/362/) by Disch
  * [Submapper test ROMs](https://forums.nesdev.org/viewtopic.php?f=3&t=17573#p223145)
  * [siliconpr0n VRC7 decapsulation photographs](https://siliconpr0n.org/archive/doku.php?id=digshadow:konami:vrc_vii_053982)



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
