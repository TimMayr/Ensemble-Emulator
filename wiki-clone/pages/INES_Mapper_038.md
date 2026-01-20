# INES Mapper 038

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_038) | View [other pages](Special_AllPages.xhtml#INES_Mapper_038)

**iNES Mapper 038**

**Company** | Bit Corp.   
---|---  
**Games** | [1 in NesCartDB](https://nescartdb.com/search/advanced?ines=38)  
**Boards** | UNL-PCI556   
**PRG ROM capacity** | 128K   
**PRG ROM window** | 32K   
**PRG RAM capacity** | None   
**CHR capacity** | 32K   
**CHR window** | 8k   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | V(hardwired)   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | 038  
  
**NESCartDB**

[iNES 038](https://nescartdb.com/search/advanced?ines=38)  
---  
  
iNES Mapper 038 represents the board used for [Bit Corp.](http://community.fandom.com/wiki/c:bootleggames:Bit_Corp. "wikia:c:bootleggames:Bit Corp.")'s [Crime Busters](https://nescartdb.com/profile/view/4103/crime-busters). It is another mapper almost identical to [GNROM](GxROM.xhtml "GxROM") (**66**) except the bits and writeable port moved around. 

## Contents

  * 1 Registers
    * 1.1 Bank Select ($7000-$7FFF)
  * 2 Hardware
  * 3 Emulator oversize support



## Registers

### Bank Select ($7000-$7FFF)
    
    
    7  bit  0
    ---- ----
    xxxx CCPP
         ||||
         ||++- Select 32 KB PRG ROM bank for CPU $8000-$FFFF
         ++--- Select 8 KB CHR ROM bank for PPU $0000-$1FFF
    

## Hardware

Writes to $F000-$FFFF are very likely to also trigger a bankswitch, but well behaved code has no reason to. The [74138](74138.xhtml "74138") used by this board uses /ROMSEL and R/W on its enable inputs, but /ROMSEL is 1 while M2 is low, not just while M2 is high and and A15 is low. As a result, before M2 is high, if the 2A03's A14, A13, and A12 lines are 1 and R/W is 0, the [74161](74161.xhtml "74161")'s /LOAD line will still load from the data bus. 

## Emulator oversize support
    
    
     FCEUX (2.1.5):
      $7000-$7FFF: [CCCC CCPP]
    
     Nestopia (1.4.0h):
      $6000-$7FFF: [CCCC CC..]
                   [PPPP PPPP]
    

_See also:_ [Oversize](Oversize.xhtml "Oversize")

Categories: [GNROM-like mappers](Category_GNROM_like_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
