# MMC4

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/MMC4) | View [other pages](Special_AllPages.xhtml#MMC4)

**MMC4**   
**FxROM**

**Company** | Nintendo   
---|---  
**Games** | [3 in NesCartDB](https://nescartdb.com/search/advanced?ines=10)  
**Complexity** | ASIC   
**Boards** | FJROM, FKROM   
**Pinout** | [MMC4 pinout](MMC4_pinout.xhtml "MMC4 pinout")  
**PRG ROM capacity** | 256K   
**PRG ROM window** | 16K + 16K fixed   
**PRG RAM capacity** | 8K   
**PRG RAM window** | 8K   
**CHR capacity** | 128K   
**CHR window** | 4K + 4K (triggered)   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H or V, switchable   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [010](MMC4.xhtml "INES Mapper 010")  
  
The **Nintendo MMC4** is an [ASIC](Category_ASIC_mappers.xhtml "Category:ASIC mappers") [mapper](Mapper.xhtml "MMC"), used on the [FxROM](FxROM.xhtml "FxROM") board set. The [iNES](INES.xhtml "INES") format assigns **mapper 10** to these boards. The chip first appeared in August 1988. 

Nintendo's MMC2, used in PxROM boards, is a similar mapper with 8 KB switchable PRG ROM banks, a 24 KB fixed PRG ROM bank, no PRG RAM, and a slightly different behaviour in auto-switching on the left (low) pattern table. This page only explains the differences, see [MMC2](MMC2.xhtml "MMC2") for full details on the rest of the mapper. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG ROM bank select ($A000-$AFFF)
  * 3 Hardware
  * 4 See also



## Banks

  * CPU $6000-$7FFF: 8 KB fixed PRG RAM bank
  * CPU $8000-$BFFF: 16 KB switchable PRG ROM bank
  * CPU $C000-$FFFF: 16 KB PRG ROM bank, fixed to the last bank
  * PPU $0000-$0FFF: Two 4 KB switchable CHR ROM banks
  * PPU $1000-$1FFF: Two 4 KB switchable CHR ROM banks



When the PPU reads from specific tiles in the pattern table during rendering, the MMC4 sets a latch that tells it to use a different 4 KB bank number. On the background layer, this has the effect of setting a different bank for all tiles to the right of a given tile, virtually increasing the tile count limit from 256 to 512 without monopolising the CPU. 

  * PPU reads $0FD8 through $0FDF: latch 0 is set to $FD
  * PPU reads $0FE8 through $0FEF: latch 0 is set to $FE
  * PPU reads $1FD8 through $1FDF: latch 1 is set to $FD
  * PPU reads $1FE8 through $1FEF: latch 1 is set to $FE



## Registers

The MMC4 has 6 registers at $A000-$AFFF, $B000-$BFFF, $C000-$CFFF, $D000-$DFFF, $E000-$EFFF and $F000-$FFFF. Only $A000-$AFFF is covered here. For the rest of the registers, see [MMC2](MMC2.xhtml "MMC2"). 

### PRG ROM bank select ($A000-$AFFF)
    
    
    7  bit  0
    ---- ----
    xxxx PPPP
         ||||
         ++++- Select 16 KB PRG ROM bank for CPU $8000-$BFFF
    

## Hardware

The MMC4 is implemented in a 44-pin TQFP package: [MMC4 pinout](MMC4_pinout.xhtml "MMC4 pinout")

Only one revision is known to exist. 

## See also

  * [NES Mapper List](http://www.romhacking.net/documents/362/) by Disch
  * [Nintendo MMC4](http://nesdev.org/mmc4.txt) (author unknown)
  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) by \Firebug\, information about mapper's initial state is inaccurate.



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers triggering on reads](Category_Mappers_triggering_on_reads.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
