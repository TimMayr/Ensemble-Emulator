# INES Mapper 180

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_180) | View [other pages](Special_AllPages.xhtml#INES_Mapper_180)

**UxROM (inverted)**

**Company** | Nichibutsu   
---|---  
**Games** | [1 in NesCartDB](https://nescartdb.com/search/advanced?ines=180)  
**Complexity** | Discrete logic   
**Boards** | UNROM   
**PRG ROM capacity** | 128K/4096K   
**PRG ROM window** | 16K fixed + 16K   
**PRG RAM capacity** | None   
**CHR capacity** | 8K   
**CHR window** | n/a   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | Fixed H or V, controlled by solder pads   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | Yes   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | 180, [002](UxROM.xhtml "INES Mapper 002"), [094](INES_Mapper_094.xhtml "INES Mapper 094")  
  
**NESCartDB**

[iNES 180](https://nescartdb.com/search/advanced?ines=180)  
---  
  
INES Mapper 180 is used to represent an unusual configuration of the [UNROM](UxROM.xhtml "UNROM") board used only for the game _Crazy Climber_. 

## Contents

  * 1 Notes
  * 2 Banks
  * 3 Registers
    * 3.1 Bank select ($8000-$FFFF)
  * 4 See also



## Notes

It is similar in principle to the more common [INES Mapper 002](UxROM.xhtml "INES Mapper 002") configuration but uses AND logic ([74HC08](7408.xhtml "7408")) instead of OR logic ([74HC32](7432.xhtml "7432")), producing a fixed first bank at $8000 rather than a fixed last bank at $C000. 

Crazy Climber had a special "climber stick" add-on that mounted a stick on top of the Famicom's directional pad. 

For a similar mapper with additional mirroring control, see [iNES Mapper 097](INES_Mapper_097.xhtml "INES Mapper 097"). 

## Banks

  * CPU $8000-$BFFF: 16 KB PRG ROM bank, fixed to the first bank
  * CPU $C000-$FFFF: 16 KB switchable PRG ROM bank



## Registers

### Bank select ($8000-$FFFF)
    
    
    7  bit  0
    ---- ----
    xxxx xPPP
          |||
          +++-- Select 16 KB PRG ROM bank for CPU $C000-$FFFF
    

## See also

  * [UxROM](UxROM.xhtml "UxROM")
  * [Programming UNROM](Programming_UNROM.xhtml "Programming UNROM")
  * NES mapper list by Disch [[1]](http://www.romhacking.net/documents/362/)



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
