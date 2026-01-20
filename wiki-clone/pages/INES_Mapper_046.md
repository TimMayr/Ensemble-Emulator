# INES Mapper 046

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_046) | View [other pages](Special_AllPages.xhtml#INES_Mapper_046)

This mapper represents the [Rumble Station](http://community.fandom.com/wiki/c:bootleggames:Rumble_Station "wikia:c:bootleggames:Rumble Station"), a combined NES-on-a-Chip and multicart of licensed [Color Dreams](Color_Dreams.xhtml "Color Dreams") games. The outer mapper, selecting 64K CHR ROM banks (bits 7-4) and 64K PRG ROM banks (bits 3-0), is at $6000-$7FFF, and the inner mapper is a size-reduced subset of Color Dreams (#11). Together, this supports up to 1 MB PRG ROM and 1 MB CHR ROM. 
    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 046          =
     ========================
     
     Example Game:
     --------------------------
     Rumblestation 15-in-1
     
     
     Bus Conflicts?:
     ---------------------------
     No idea whether or not this mapper suffers from bus conflicts.  Use caution!
     
     
     Registers:
     ---------------------------
     
     Regs at $6000-7FFF means no PRG-RAM.
     
     
       $6000-7FFF:  [CCCC PPPP]   High CHR, PRG bits
       $8000-FFFF:  [.CCC ...P]   Low CHR, PRG bits
     
     'C' selects 8k CHR @ $0000
     'P' select 32k PRG @ $8000
     
     
     Powerup:
     ---------------------------
     $6000 set to 0 on powerup.
    

Nestopia and FCEUX reset both registers, not just the outer bank. 

Categories: [GNROM-like mappers](Category_GNROM_like_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
