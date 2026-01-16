# INES Mapper 140

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_140) | View [other pages](Special_AllPages.xhtml#INES_Mapper_140)

Mapper 140 represents Jaleco's [JF-11](https://nescartdb.com/search/advanced?pcb=JF-11) and [JF-14](https://nescartdb.com/search/advanced?pcb=JF-14) boards, which act much like [GNROM](GxROM.xhtml "GNROM") (**66**) except that the writable port is moved down to $6000-$7FFF. 
    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 140          =
     ========================
     
     
     Example Game:
     --------------------------
     Bio Senshi Dan - Increaser Tono Tatakai
     
     
     Notes:
     ---------------------------
     Regs lie at $6000-7FFF, so there's no SRAM
     
     
     Registers:
     --------------------------
       $6000-7FFF:  [..PP CCCC]
         P = Selects 32k PRG @ $8000
         C = Selects 8k CHR @ $0000
    

Categories: [GNROM-like mappers](Category_GNROM_like_mappers.xhtml), [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
