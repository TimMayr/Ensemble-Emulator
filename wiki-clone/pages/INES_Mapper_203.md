# INES Mapper 203

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_203) | View [other pages](Special_AllPages.xhtml#INES_Mapper_203)
    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 203          =
     ========================
     
     Example Games:
     --------------------------
     35-in-1
     
     
     Registers:
     ---------------------------
     
     
       $8000-FFFF:  [PPPP PPCC]
         P = PRG Reg
         C = CHR Reg
     
     
     CHR Setup:
     ---------------------------
     
           $0000   $0400   $0800   $0C00   $1000   $1400   $1800   $1C00 
         +---------------------------------------------------------------+
         |                             $8000                             |
         +---------------------------------------------------------------+
     
     
     PRG Setup:
     ---------------------------
     
          $8000   $A000   $C000   $E000  
         +---------------+---------------+
         |     $8000     |     $8000     |
         +---------------+---------------+
    

The only known instantiation of this mapper has 64 KiB PRG and 32 KiB CHR, i.e. only four bits of the register exist, and the "35" games are the various parts of Duck Hunt, [Hogan's Alley](http://bootgod.dyndns.org:7777/profile.php?id=3726), [Wild Gunman](http://bootgod.dyndns.org:7777/profile.php?id=306), and [~~Battle City~~ “Tank 1990”](http://bootgod.dyndns.org:7777/profile.php?id=3799). 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
