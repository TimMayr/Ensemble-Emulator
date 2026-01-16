# INES Mapper 081

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_081) | View [other pages](Special_AllPages.xhtml#INES_Mapper_081)

**N715021**

**Company** | NTDEC   
---|---  
**Complexity** | Discrete logic   
**Boards** | N715021   
**PRG ROM capacity** | 64K   
**PRG ROM window** | 16K + 16K fixed   
**PRG RAM capacity** | None   
**CHR capacity** | 32K   
**CHR window** | 8K   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | Fixed H or V, controlled by solder pads   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | 081  
  
This mapper is used on exactly one known game: Super Gun from NTDEC. Although the game's bankswitching code seems to suggest two separate data latches at $6000 and $8000-$FFFF (the latter with bus conflicts), the board actually latches the four lowest address bits, and ignores the write to $6000. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG/CHR Bank Select ($8000-$FFFF)
  * 3 Related



## Banks

  * CPU $8000-$BFFF: 16 KB switchable PRG ROM bank
  * CPU $C000-$FFFF: 16 KB PRG ROM bank, fixed to the last bank
  * PPU $0000-$1FFF: 8 KB switchable CHR ROM bank



## Registers

### PRG/CHR Bank Select ($8000-$FFFF)
    
    
     CPU address bit#
    1111 1100 0000 0000
    5432 1098 7654 3210
    ---- ---- ---- ----
    1... .... .... PPCC
                   ||||
                   ||++------ Select 8 KB CHR ROM bank for PPU $0000-$1FFF
                   ++-------- Select 16 KB PRG ROM bank for CPU $8000-$BFFF
    

## Related

[Discussion with PCB images and analysis as well as Kazzo dumping script](http://forums.nesdev.org/viewtopic.php?f=3&t=16412)

Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
