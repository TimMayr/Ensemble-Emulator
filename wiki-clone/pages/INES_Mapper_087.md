# INES Mapper 087

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_087) | View [other pages](Special_AllPages.xhtml#INES_Mapper_087)

**J87**

**Company** | Konami, Jaleco, Taito   
---|---  
**Boards** | JF-0x, JF-10 (Jaleco), FC-009 (Taito), MF4034/30xxA (Konami)   
**PRG ROM capacity** | 32K   
**PRG ROM window** | n/a   
**PRG RAM capacity** | None   
**CHR capacity** | 16k (most games), 32K (JF-09/JF-10 only)   
**CHR window** | 8K   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | Fixed H or V, controlled by solder pads   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | 087, [101](INES_Mapper_101.xhtml "INES Mapper 101")  
  
**NESCartDB**

[iNES 087](https://nescartdb.com/search/advanced?ines=87)  
---  
  
  
**J87** (Also known as Mapper 87) refers to the first "mapper" created for the Famicom. Despite this, no games with this mapper ever made it overseas to USA NES owners. [iNES Mapper 101](INES_Mapper_101.xhtml "INES Mapper 101") is the same board, but for a dump that naïvely reversed the CHR bit-order. [CNROM](CNROM.xhtml "CNROM") is a later cost-reduced version of this mapper, by cutting down on one logic chip, at the cost of having bus conflicts, and also has the CHR bank order reversed as well. Most games are capped at 16k CHR, however, the JF-09 and JF-10 boards contain an additional trace that allows the design to address 32k. All games with this mapper are emulated as the 32k variant. 

The name "J87" was given to this mapper due to its historical significance, and creation by Jaleco. Otherwise, no "canonical" name was given to this mapper. Jaleco would later upgrade this mapper to become [iNES Mapper 140](INES_Mapper_140.xhtml "INES Mapper 140"). 

## Banks

  * CPU $8000-$FFFF: 16 KB PRG ROM, fixed (if 16 KB PRG ROM used, then this is the same as $C000-$FFFF)
  * CPU $C000-$FFFF: 16 KB PRG ROM, fixed
  * PPU $0000-$1FFF: 8 KB switchable CHR ROM bank



## Registers

### Bank Select ($6000-$7FFF)

This will select an 8 KB CHR ROM bank for PPU $0000-$1FFF. 
    
    
    7  bit  0
    ---- ----
    xxxx xxLH
           ||
           |+- High CHR bit
           +-- Low CHR bit
    

Categories: [CNROM-like mappers](Category_CNROM_like_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
