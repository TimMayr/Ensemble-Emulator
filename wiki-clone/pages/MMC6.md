# MMC6

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/MMC6) | View [other pages](Special_AllPages.xhtml#MMC6)

**MMC6**   
**HxROM**

**Company** | Nintendo   
---|---  
**Games** | [2 in NesCartDB](https://nescartdb.com/search/advanced?ines=4)  
**Complexity** | ASIC   
**Boards** | HKROM   
**Pinout** | [MMC6 pinout](MMC6_pinout.xhtml "MMC6 pinout")  
**PRG ROM capacity** | 512K   
**PRG ROM window** | 8K   
**PRG RAM capacity** | 1K   
**PRG RAM window** | 1K   
**CHR capacity** | 256K   
**CHR window** | 1K + 2K   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H or V, switchable   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | Yes   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [004](MMC3.xhtml "INES Mapper 004")  
  
The **Nintendo MMC6** is a [mapper](Mapper.xhtml "MMC") [ASIC](Category_ASIC_mappers.xhtml "Category:ASIC mappers") used in Nintendo's NES-HKROM Game Pak board. This board, along with most common [TxROM](TxROM.xhtml "TxROM") boards (which use the [Nintendo MMC3](MMC3.xhtml "MMC3")) are assigned to [iNES Mapper 004](MMC3.xhtml "INES Mapper 004"). The MMC3C and the MMC6 are alike, except the MMC6 has 1 KB of internal PRG RAM with different write/enable controls. This page only explains the differences, see [MMC3](MMC3.xhtml "MMC3") for full details on the rest of the mapper. 

The [NES 2.0 submapper 004:1](NES_2_0_submappers.xhtml#004:_1_MMC6 "NES 2.0 submappers") was assigned to disambiguate MMC6 from the MMC3 mapper it shares an iNES mapper with. 

This chip first appeared in December 1990. A total of two commercial games were released using the MMC6: 

  * StarTropics
  * Zoda's Revenge: StarTropics II



## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Bank select ($8000-$9FFE, even)
    * 2.2 PRG RAM protect ($A001-$BFFF, odd)
  * 3 Hardware



## Banks

  * CPU $7000-$7FFF: 1 KB PRG RAM, [mirrored](Mirroring.xhtml "Mirroring")
  * CPU $8000-$9FFF (or $C000-$DFFF): 8 KB switchable PRG ROM bank
  * CPU $A000-$BFFF: 8 KB switchable PRG ROM bank
  * CPU $C000-$DFFF (or $8000-$9FFF): 8 KB PRG ROM bank, fixed to the second-last bank
  * CPU $E000-$FFFF: 8 KB PRG ROM bank, fixed to the last bank
  * PPU $0000-$07FF (or $1000-$17FF): 2 KB switchable CHR bank
  * PPU $0800-$0FFF (or $1800-$1FFF): 2 KB switchable CHR bank
  * PPU $1000-$13FF (or $0000-$03FF): 1 KB switchable CHR bank
  * PPU $1400-$17FF (or $0400-$07FF): 1 KB switchable CHR bank
  * PPU $1800-$1BFF (or $0800-$0BFF): 1 KB switchable CHR bank
  * PPU $1C00-$1FFF (or $0C00-$0FFF): 1 KB switchable CHR bank



## Registers

The MMC6 has 4 pairs of registers at $8000-$9FFF, $A000-$BFFF, $C000-$DFFF, and $E000-$FFFF - even addresses ($8000, $8002, etc.) select the low register and odd addresses ($8001, $8003, etc.) select the high register in each pair. Only $8000 and $A001 are covered here. For the rest of the registers, see [MMC3](MMC3.xhtml "MMC3"). 

### Bank select ($8000-$9FFE, even)
    
    
    7  bit  0
    ---- ----
    CPMx xRRR
    |||   |||
    |||   +++- Specify which bank register to update on next write to Bank Data register
    ||+------- PRG RAM enable
    |+-------- PRG ROM bank configuration (0: $8000-$9FFF swappable, $C000-$DFFF fixed to second-last bank;
    |                                      1: $C000-$DFFF swappable, $8000-$9FFF fixed to second-last bank)
    +--------- CHR ROM bank configuration (0: two 2 KB banks at $0000-$0FFF, four 1 KB banks at $1000-$1FFF;
                                           1: four 1 KB banks at $0000-$0FFF, two 2 KB banks at $1000-$1FFF)
    

### PRG RAM protect ($A001-$BFFF, odd)
    
    
    7  bit  0
    ---- ----
    HhLl xxxx
    ||||
    |||+------ Enable writing to RAM at $7000-$71FF
    ||+------- Enable reading RAM at $7000-$71FF
    |+-------- Enable writing to RAM at $7200-$73FF
    +--------- Enable reading RAM at $7200-$73FF
    

## Hardware

The MMC6 exists in a [64-pin TQFP package](MMC6_pinout.xhtml "MMC6 pinout"). At least two revisions exist[**_citation needed_**], though only MMC6B has been observed. 

The PRG RAM protect bits control mapping of two 512B banks of RAM. If neither bank is enabled for reading, the $7000-$7FFF area is [open bus](Open_bus_behavior.xhtml "Open bus"). If only one bank is enabled for reading, the other reads back as zero. The write-enable bits only have effect if that bank is enabled for reading, otherwise the bank is not writable. Both banks may be enabled for reading (and optionally writing) at the same time. 

When PRG RAM is disabled via $8000, the mapper continuously sets $A001 to $00, and so all writes to $A001 are ignored. 

Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
