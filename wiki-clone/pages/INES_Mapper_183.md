# INES Mapper 183

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_183) | View [other pages](Special_AllPages.xhtml#INES_Mapper_183)

INES Mapper 183 was supposedly assigned to represent the pirate port of Gimmick! known as [Suikan Pipe](https://www.youtube.com/watch?v=_aFZ52pZh2E). 

According to Nestopia's source, this is very similar to the [VRC4e](VRC2_and_VRC4.xhtml "VRC4") variant, but the PRG banking registers are a little different: 
    
    
     Mask: $F80C
     
     $8800: PRG at $8000-$9FFF
     $A800: PRG at $A000-$BFFF
     $A000: PRG at $C000-$DFFF
     $9800: standard VRC4 mirroring control
    

CHR and IRQ are the same as VRC4e. 

Categories: [INES Mappers](Category_INES_Mappers.xhtml)
