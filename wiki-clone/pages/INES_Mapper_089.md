# INES Mapper 089

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_089) | View [other pages](Special_AllPages.xhtml#INES_Mapper_089)

This is part of a family of Sunsoft mappers used largely by Japanese games: (iNES Mapper 089, [iNES Mapper 093](INES_Mapper_093.xhtml "INES Mapper 093"), [iNES Mapper 184](INES_Mapper_184.xhtml "INES Mapper 184")) 

This mapper is used by Tenka no Goikenban: Mito Koumon (J). This game uses the [Sunsoft-2](Sunsoft_2_pinout.xhtml "Sunsoft 2 pinout") chip on the Sunsoft-3 board, and so works differently than [Mapper 93](INES_Mapper_093.xhtml "INES Mapper 093")

Here is the documentation in [disch's original style](INES_Mapper_DischDocs.xhtml "INES Mapper DischDocs"): 
    
    
     Registers **BUS CONFLICTS**:
     --------------------------
     
       $8000-FFFF:  [CPPP MCCC]
         C = Select 8k CHR @ $0000
         P = Select 16k PRG @ $8000
         M = Mirroring:
           0 = 1ScA
           1 = 1ScB
     
     
     PRG Setup:
     --------------------------
     
           $8000   $A000   $C000   $E000  
         +---------------+---------------+
         |     $8000     |     { -1}     |
         +---------------+---------------+
    

Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
