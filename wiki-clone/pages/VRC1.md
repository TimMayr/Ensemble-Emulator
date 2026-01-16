# VRC1

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VRC1) | View [other pages](Special_AllPages.xhtml#VRC1)

**VRC1**

**Company** | Konami   
---|---  
**Games** | [6 in NesCartDB](https://nescartdb.com/search/advanced?ines=75)  
**Complexity** | ASIC   
**Boards** | JF-20, JF-22, 4036, 302114A, 350459   
**Pinout** | [VRC1 pinout](VRC1_pinout.xhtml "VRC1 pinout")  
**PRG ROM capacity** | 128K   
**PRG ROM window** | 8K   
**PRG RAM capacity** | none   
**PRG RAM window** | n/a   
**CHR capacity** | 128K   
**CHR window** | 4K   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | H or V, switchable   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [075](VRC1.xhtml "INES Mapper 075")  
  
The Konami VRC1 is an [ASIC](Category_ASIC_mappers.xhtml "Category:ASIC mappers") [mapper](Mapper.xhtml "MMC"). The [iNES](INES.xhtml "INES") format assigns **mapper 75** to VRC1. 

The VRC1 was used in several Famicom games released by Konami and Jaleco: 

  * _Exciting Boxing_
  * _Ganbare Goemon! Karakuri Douchuu_
  * _Jajamaru Ninpouchou_
  * _King Kong 2: Ikari no Megaton Punch_
  * _Moero!! Junior Basket: Two on Two_
  * _Tetsuwan Atom_



## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG Select 0 ($8000-$8FFF)
    * 2.2 PRG Select 1 ($A000-$AFFF)
    * 2.3 PRG Select 2 ($C000-$CFFF)
    * 2.4 Mirroring Control, CHR bits ($9000-$9FFF)
    * 2.5 CHR Select 0 ($E000-$EFFF)
    * 2.6 CHR Select 1 ($F000-$FFFF)
  * 3 References



  


## Banks

  * CPU $8000-$9FFF: 8 KiB switchable PRG ROM bank
  * CPU $A000-$BFFF: 8 KiB switchable PRG ROM bank
  * CPU $C000-$DFFF: 8 KiB switchable PRG ROM bank
  * CPU $E000-$FFFF: 8 KiB PRG ROM bank, fixed to the last bank
  * PPU $0000-$0FFF: 4 KiB switchable CHR bank
  * PPU $1000-$1FFF: 4 KiB switchable CHR bank



## Registers

### PRG Select 0 ($8000-$8FFF)
    
    
    7  bit  0
    ---------
    .... PPPP
         ||||
         ++++- Select 8 KB PRG ROM at $8000
    

### PRG Select 1 ($A000-$AFFF)
    
    
    7  bit  0
    ---------
    .... PPPP
         ||||
         ++++- Select 8 KB PRG ROM at $A000
    

  


### PRG Select 2 ($C000-$CFFF)
    
    
    7  bit  0
    ---------
    .... PPPP
         ||||
         ++++- Select 8 KB PRG ROM at $C000
    

  


### Mirroring Control, CHR bits ($9000-$9FFF)
    
    
    7  bit  0
    ---------
    .... .BAM
          |||
          ||+- [Mirroring](Mirroring.xhtml "Mirroring")  (0: Vertical; 1: Horizontal)
          |+-- High Bit of 4 KB CHR bank at PPU $0000
          +--- High Bit of 4 KB CHR bank at PPU $1000
    

The mirroring bit is ignored if the cartridge is wired for 4-screen VRAM, as is typical for [Vs. System](Vs__System.xhtml "Vs. System") games using the VRC1. 

### CHR Select 0 ($E000-$EFFF)
    
    
    7  bit  0
    ---------
    .... CCCC
         ||||
         ++++- Low 4 bits of 4 KB CHR bank at PPU $0000
    

These bits combined with bit 1 of $9000 make a 5-bit CHR selection. 

  


### CHR Select 1 ($F000-$FFFF)
    
    
    7  bit  0
    ---------
    .... CCCC
         ||||
         ++++- Low 4 bits of 4 KB CHR bank at PPU $1000
    

These bits combined with bit 2 of $9000 make a 5-bit CHR selection. 

## References

  * [Disch's Mapper Notes](http://www.romhacking.net/documents/362/)



Categories: [ASIC mappers](Category_ASIC_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml)
