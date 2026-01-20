# INES Mapper 184

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_184) | View [other pages](Special_AllPages.xhtml#INES_Mapper_184)

This is part of a family of Sunsoft mappers used largely by Japanese games: ([iNES Mapper 089](INES_Mapper_089.xhtml "INES Mapper 089"), [iNES Mapper 093](INES_Mapper_093.xhtml "INES Mapper 093"), iNES Mapper 184) 

This mapper is for a few games which use the [Sunsoft-1 mapper](Sunsoft_1_pinout.xhtml "Sunsoft 1 pinout"). 

Fantasy Zone (J), despite using the Sunsoft-1 chip, is more easily emulated by [Mapper 093](INES_Mapper_093.xhtml "INES Mapper 093") because it was written to support execution using either the Sunsoft-2 IC and a variant board that used the Sunsoft-1 IC. 

Despite the mapper existing as a single IC, its functionality is describable using a [74139](74139.xhtml "74139"), a [74174](74174.xhtml "74174"), and a [74157](74157.xhtml "74157"), and so should probably be considered discrete logic. 

Here is the documentation in [disch's original style](INES_Mapper_DischDocs.xhtml "INES Mapper DischDocs"): 
    
    
     Registers:
     --------------------------
       $6000-7FFF:  [.HHH .LLL]
         H = Selects 4k CHR @ $1000
         L = Selects 4k CHR @ $0000
     
     The most significant bit of H is always set in hardware. (i.e. its range is 4 to 7)
     (There can be no SRAM because the register is mapped into $6000-7FFF)
    

fwNES, a NES emulator for DOS, originally assigned mapper **122** for this.[1]

  1. ↑ FWNES98E.TXT, fwNES 0.302b, 1998/11/16, by FanWen Yang



Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
