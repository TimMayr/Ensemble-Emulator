# INES Mapper 047

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_047) | View [other pages](Special_AllPages.xhtml#INES_Mapper_047)
    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 047          =
     ========================
     
     Example Game:
     --------------------------
     Super Spike V'Ball + Nintendo World Cup
     
     
     Notes:
     ---------------------------
     
     Yet another [MMC3](MMC3.xhtml "MMC3") multicart.  See [mapper 004](MMC3.xhtml "INES Mapper 004") for info on MMC3.
     
     There is no PRG-RAM.  The multicart reg lies at $6000-7FFF, but is only writable when MMC3 PRG-RAM is enabled
     and writable (see $A001)
     
     
     Registers:
     ---------------------------
     
       $6000-7FFF:  [.... ...B]  Block select
       $8000-FFFF:  Same as MMC3 for selected block
     
     
     Each block has 128k PRG and 128k CHR.
    

Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
