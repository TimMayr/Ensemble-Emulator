# INES Mapper 225

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_225) | View [other pages](Special_AllPages.xhtml#INES_Mapper_225)

INES Mapper 225 denotes the **ET-4310** (60-pin) and **K-1010** (72-pin) multicart circuit boards. 
    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 225          =
     ========================
     
     Example Games:
     --------------------------
     52 Games
     58-in-1
     64-in-1
     
     
     Registers:
     ---------------------------
     
       $5800-5803:  [.... RRRR]  RAM  (readable/writable)
                     (16 bits of RAM -- 4 bits in each of the 4 regs)
       $5804-5FFF:    mirrors $5800-5803
     
       $8000-FFFF:  A~[.HMO PPPP PPCC CCCC]
         H = High bit (acts as bit 7 for PRG and CHR regs)
         M = Mirroring (0=Vert, 1=Horz)
         O = PRG Mode
         P = PRG Reg
         C = CHR Reg
     
     
     CHR Setup:
     ---------------------------
     
                    $0000   $0400   $0800   $0C00   $1000   $1400   $1800   $1C00 
                  +---------------------------------------------------------------+
     CHR Mode 0:  |                             $8000                             |
                  +---------------------------------------------------------------+
     
     
     PRG Setup:
     ---------------------------
     
                    $8000   $A000   $C000   $E000  
                  +-------------------------------+
     PRG Mode 0:  |            <$8000>            |
                  +-------------------------------+
     PRG Mode 1:  |     $8000     |     $8000     |
                  +---------------+---------------+
    

Nestopia does not implement the 4 × 4 bit RAM, and FCEUX ceased to support it between 2.1.5 and 2.2.1 

This looks like a duplicate of [iNES Mapper 255](INES_Mapper_255.xhtml "INES Mapper 255"). It's not clear why Nestopia implemented it twice. 

Pictures of the PCB show physical releases both [with](https://forums.nesdev.org/viewtopic.php?p=210128#p210128) and [without](https://forums.nesdev.org/viewtopic.php?p=209732#p209732) the 74'670 that make up the extra RAM. Carts are slightly buggy without it. 

## See also

  * [NES Mapper List](http://www.romhacking.net/documents/362/) by Disch
  * [iNES mapper 225](http://nesdev.org/225.txt), unknown author



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
