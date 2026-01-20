# INES Mapper 062

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_062) | View [other pages](Special_AllPages.xhtml#INES_Mapper_062)
    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 062          =
     ========================
     
     Example Game:
     --------------------------
     Super 700-in-1
     
     
     Registers:
     ---------------------------
     
       $8000-FFFF:  A~[..pp pppp MPOC CCCC]
                      [.... ..cc]
         p = Low bits of PRG Reg
         P = High bit of PRG Reg
         c = Low bits of CHR Reg
         C = High bits of CHR Reg
         O = PRG Mode
         M = Mirroring (0=Vert, 1=Horz)
     
     
     PRG Setup:
     ---------------------------
     
     
                    $8000   $A000   $C000   $E000  
                  +-------------------------------+
     PRG Mode 0:  |            <$8000>            |
                  +-------------------------------+
     PRG Mode 1:  |     $8000     |     $8000     |
                  +---------------+---------------+
     
     
     CHR Setup:
     ----------------------------
       'C' and 'c' select an 8k page @ $0000
    

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
