# INES Mapper 232

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_232) | View [other pages](Special_AllPages.xhtml#INES_Mapper_232)

  

    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 232          =
     ========================
     
     Example Games:
     --------------------------
     Quattro Adventure
     Quattro Sports
     Quattro Arcade
     
     
     Notes:
     --------------------------
     This is another Camerica/Codemasters mapper like [071](INES_Mapper_071.xhtml "INES Mapper 071").  Like 071, this mapper also involves a custom lockout
     defeat circuit which is mostly unimportant for emulation purposes.  Details will not be mentioned here, but
     are outlined in Kevtris' Camerica Mappers documentation.
     
     
     Registers:
     ---------------------------
     
      $8000-BFFF:   [...B B...]   PRG Block Select
     
      $C000-FFFF:   [.... ..PP]   PRG Page Select
         
     
     PRG Setup:
     ---------------------------
     
     Pages are taken from the 64k block currently selected by $8000.
     
           $8000   $A000   $C000   $E000  
         +---------------+---------------+
         |     $C000     |     { 3 }     |
         +---------------+---------------+
    

Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
