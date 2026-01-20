# GxROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/GxROM) | View [other pages](Special_AllPages.xhtml#GxROM)

**GxROM**

**Company** | Nintendo, others   
---|---  
**Boards** | GNROM,MHROM   
**PRG ROM capacity** | 128KiB (512KiB oversize)   
**PRG ROM window** | 32KiB   
**PRG RAM capacity** | None   
**CHR capacity** | 32KiB (128KiB oversize)   
**CHR window** | 8KiB   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | Fixed H or V, controlled by solder pads   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | Yes   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [066](GxROM.xhtml "INES Mapper 066")  
  
**NESCartDB**

[iNES 066](https://nescartdb.com/search/advanced?ines=66)  
---  
[GNROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=GNROM)  
[MHROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=MHROM)  
  
The designation **GxROM** refers to Nintendo cartridge boards labeled NES-GNROM and NES-MHROM (and their [HVC](Family_Computer.xhtml "Famicom") counterparts), which use discrete logic to provide up to four 32 KB banks of PRG ROM and up to four 8 KB banks of CHR ROM. The [iNES](INES.xhtml "INES") format assigns **mapper 66** to these boards. 

The Jaleco board assigned to [iNES Mapper 140](INES_Mapper_140.xhtml "INES Mapper 140") is sometimes confused with GNROM, as they are very similar but with the bankswitch register in a different location. 

Example games: 

  * _Doraemon_
  * _Dragon Power_
  * _Gumshoe_
  * _Thunder & Lightning_
  * _Super Mario Bros. + Duck Hunt_ (MHROM)



## Contents

  * 1 Board Types
  * 2 Banks
  * 3 Registers
    * 3.1 Bank select ($8000-$FFFF)
  * 4 Solder pad config
  * 5 Hardware
  * 6 Variants
  * 7 See also



## Board Types

The following GxROM boards are known to exist: 

Board | PRG ROM | CHR   
---|---|---  
GNROM | 128 KB | 32 KB ROM   
MHROM | 64 KB | 16 / 32 KB ROM   
  
## Banks

  * CPU $8000-$FFFF: 32 KB switchable PRG ROM bank
  * PPU $0000-$1FFF: 8 KB switchable CHR ROM bank



## Registers

### Bank select ($8000-$FFFF)
    
    
    7  bit  0
    ---- ----
    xxPP xxCC
      ||   ||
      ||   ++- Select 8 KB CHR ROM bank for PPU $0000-$1FFF
      ++------ Select 32 KB PRG ROM bank for CPU $8000-$FFFF
    

Bit 5 is not used on MHROM, which supports only 64 KB PRG. 

## Solder pad config

  * Horizontal mirroring : 'H' disconnected, 'V' connected.
  * Vertical mirroring : 'H' connected, 'V' disconnected.



## Hardware

The GNROM board contains a [74HC161](74161.xhtml "74161") binary counter used as a quad D latch (4-bit register) to select the current PRG and CHR banks. MHROM, on the other hand, was often a [glop-top](https://www.google.com/search?q=Chip-on-board "google:Chip-on-board"), as it was used for pack-in games, such as the _Super Mario Bros./Duck Hunt_ [multicart](Multicart.xhtml "Multicart"), and needed to be very inexpensive to produce in huge quantities. 

## Variants

Placing the bank register in $6000-$7FFF instead of $8000-$FFFF gives [mapper 140](INES_Mapper_140.xhtml "INES Mapper 140"). The [Color Dreams](Color_Dreams.xhtml "Color Dreams") board leaves the port at $8000-$FFFF, swaps the nibbles, expands CHR by two bits, and adds two bits for charge pump control. 

Theoretically the bank select register could be implemented with a [74HC377](74377.xhtml "74377") octal D latch, allowing up to 512 KB of PRG ROM and 128 KB of CHR ROM. There are a [large number](Category_GNROM_like_mappers.xhtml "Category:GNROM-like mappers") of other variants on GNROM, where the bits or the writeable address were moved around. 

## See also

  * [NES Mapper List](http://www.romhacking.net/documents/362/) by Disch
  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) by \Firebug\, information about mapper's initial state is inaccurate.



Categories: [GNROM-like mappers](Category_GNROM_like_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers with bus conflicts](Category_Mappers_with_bus_conflicts.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
