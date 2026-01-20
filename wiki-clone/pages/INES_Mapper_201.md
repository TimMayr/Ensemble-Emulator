# INES Mapper 201

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_201) | View [other pages](Special_AllPages.xhtml#INES_Mapper_201)

**iNES Mapper 201** has been assigned to a multicart mapper designed to contain multiple NROM-256 games. It acts like [BNROM](INES_Mapper_034.xhtml "BNROM") and [CNROM](CNROM.xhtml "CNROM") overlaid, where one bank number controls both 32 KiB PRG banks and 8 KiB CHR banks, except it uses the address bus instead of the data bus. 
    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 201          =
     ========================
     
     Example Games:
     --------------------------
     8-in-1
     21-in-1 (2006-CA) (Unl)
     
     
     Registers:
     ---------------------------
     
     
       $8000-FFFF:  A~[.... ....  RRRR RRRR]
         R = PRG/CHR Reg
     
     
     CHR Setup:
     ---------------------------
     
           $0000   $0400   $0800   $0C00   $1000   $1400   $1800   $1C00 
         +---------------------------------------------------------------+
         |                             $8000                             |
         +---------------------------------------------------------------+
     
     
     PRG Setup:
     ---------------------------
     
          $8000   $A000   $C000   $E000  
         +-------------------------------+
         |             $8000             |
         +-------------------------------+
    

All known games on this mapper use only the two least significant bits (128KiB PRG / 32KiB CHR). 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
