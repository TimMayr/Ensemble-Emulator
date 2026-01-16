# INES Mapper 049

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_049) | View [other pages](Special_AllPages.xhtml#INES_Mapper_049)
    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 049          =
     ========================
     
     Example Game:
     --------------------------
     Super HIK 4-in-1
     
     
     Notes:
     ---------------------------
     
     Yet another [MMC3](MMC3.xhtml "MMC3") multicart.  For info on MMC3, see [mapper 004](MMC3.xhtml "INES Mapper 004").
     
     There is no PRG-RAM.  The multicart reg lies at $6000-7FFF, but is only writable when MMC3 PRG-RAM is enabled
     and writable (see $A001)
     
     
     Registers:
     ---------------------------
     
       $6000-7FFF:  [BBPP ...O]  Multicart reg
            B = Block
            P = 32k PRG Reg
            O = PRG Mode (0=32k mode)
     
       $8000-FFFF:  Same as MMC3 for selected block
     
     
     Each block is 128k PRG and 128k CHR
     
     
     
     PRG Setup:
     ---------------------------
     
       When the 'O' mode bit is clear, ordinary MMC3 PRG regs are ignored, and instead, 32k PRG page 'P' is swapped
     in at $8000.  When 'O' is set, 'P' is ignored, and MMC3 PRG regs work normally for the current block.
     
     
     Powerup:
     ---------------------------
     
     $6000 set to 0 on powerup.
    

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
