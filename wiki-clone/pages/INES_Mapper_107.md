# INES Mapper 107

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_107) | View [other pages](Special_AllPages.xhtml#INES_Mapper_107)

_[Magic Dragon](http://community.fandom.com/wiki/c:bootleggames:Magic_Dragon "wikia:c:bootleggames:Magic Dragon")_ is a side-scrolling shoot-em-up by Magicseries. It is heavily inspired by Irem's _R-Type_. 

Here are Disch's original notes: 
    
    
     ========================
     =  Mapper 107          =
     ========================
     
     Example Game:
     --------------------------
     Magic Dragon
     
     
     Registers:
     ---------------------------
     I do not know whether or not this mapper suffers from bus conflicts.  Use caution!
     
       $8000-FFFF:  [PPPP PPP.]
                    [CCCC CCCC]
         P = Selects 32k PRG @ $8000
         C = Selects 8k CHR @ $0000
     
     This is very strange.  Bits 1-7 seem to be used by both CHR and PRG.
    

FCEUX implements only the bottom 3 bits of the unified register. 

Categories: [Discrete logic mappers](Category_Discrete_logic_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with overlapping PRG and CHR registers](Category_Mappers_with_overlapping_PRG_and_CHR_registers.xhtml)
