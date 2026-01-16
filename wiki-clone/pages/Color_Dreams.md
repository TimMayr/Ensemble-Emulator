# Color Dreams

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Color_Dreams) | View [other pages](Special_AllPages.xhtml#Color_Dreams)

**Color Dreams**

**Company** | Color Dreams/Wisdom Tree   
---|---  
**Boards** | COLORDREAMS   
**PRG ROM capacity** | 128K   
**PRG ROM window** | 32K   
**PRG RAM capacity** | None   
**CHR capacity** | 128K   
**CHR window** | 8k   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | Fixed H or V, controlled by solder pads   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | Yes   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [011](Color_Dreams.xhtml "INES Mapper 011")  
  
**NESCartDB**

[iNES 011](https://nescartdb.com/search/advanced?ines=11)  
---  
  
The **Color Dreams Mapper** was a mapper used by the **Color Dreams** company. The [iNES](INES.xhtml "INES") format assigns **mapper 11** to these boards. 

The mapper is a board with [discrete logic](Category_Discrete_logic_mappers.xhtml "Category:Discrete logic mappers") that provides up to four 32 KB banks of PRG ROM, up to sixteen 8 KB banks of CHR ROM, and controls a charge pump to defeat the [CIC](CIC_lockout_chip.xhtml "CIC") lockout chip. 

**Color Dreams** was a company that developed and published unlicensed NES games. They also operated with the alternate labels **Wisdom Tree** which produced games with Christianity themes (primarily sold in Christian bookstores), and **Bunch Games** which produced lower budget games. American Game Cartridges also used the Color Dreams mapper for their port of Exidy's _Chiller_. 

Example games: 

  * _Bible Adventures_
  * _Chiller_
  * _Crystal Mines_
  * _Menace Beach_
  * _Metal Fighter_



## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Bank select ($8000-$FFFF)
  * 3 Hardware
  * 4 Variants
  * 5 References



## Banks

  * CPU $8000-$FFFF: 32 KB switchable PRG ROM bank
  * PPU $0000-$1FFF: 8 KB switchable CHR ROM bank



## Registers

### Bank select ($8000-$FFFF)
    
    
    7  bit  0
    ---- ----
    CCCC LLPP
    |||| ||||
    |||| ||++- Select 32 KB PRG ROM bank for CPU $8000-$FFFF
    |||| ++--- Used for lockout defeat
    ++++------ Select 8 KB CHR ROM bank for PPU $0000-$1FFF
    

## Hardware

The Color Dreams board contains a [74LS377](74377.xhtml "74377") octal D latch (8-bit register) to select the current PRG and CHR banks. 

The lockout defeat is not necessary for emulator implementation. See references below for more information. 

## Variants

Nintendo's own [GxROM](GxROM.xhtml "GxROM") family of boards is nearly identical in function to the Color Dreams board. 

A multicart mapper based on this mapper is [iNES Mapper 046](INES_Mapper_046.xhtml "INES Mapper 046"), with an outer bank register at $6000-$7FFF. 

Some variations of this board/mapper (e.g the one used in the prototype game "Free Fall") appear to be free of [bus conflicts](Bus_conflict.xhtml "Bus conflict") and will not work properly if bus conflicts are emulated. 

## References

  * [Color Dreams](http://kevtris.org/mappers/cdreams/) \- documentation of mapper by Kevin Horton
  * [Lockout](http://kevtris.org/mappers/lockout/) \- documentation of lockout mechanisms by Kevtris
  * [Games](http://bootgod.dyndns.org:7777/search.php?ines=11) \- list of games using the Color Dreams mapper at NesCartDB
  * [Comprehensive NES Mapper Document](http://nesdev.org/mappers.zip) by \Firebug\, information about mapper's initial state is inaccurate.



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [GNROM-like mappers](Category_GNROM_like_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [Mappers with bus conflicts](Category_Mappers_with_bus_conflicts.xhtml)
