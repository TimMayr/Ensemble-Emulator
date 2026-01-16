# INES Mapper 076

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_076) | View [other pages](Special_AllPages.xhtml#INES_Mapper_076)

iNES Mapper 076 represents **NAMCOT-3446** , a board used only for the game [Megami Tensei: Digital Devil Story](https://nescartdb.com/profile/view/2308/megami-tensei-digital-devil-story). 

It rewires the [Namcot 108](INES_Mapper_206.xhtml "INES Mapper 206") mapper IC to be able to address 128 KiB of CHR, in exchange for coarser CHR banking. 

The PCB also has a [7432](7432.xhtml "7432") which allows the use of a 28-pin CHR-ROM. 
    
    
     Registers:
     ---------------------------
     Mask: $E001
     
       $8000:  [.... .AAA]
         A = Address for use with $8001
     
       $8001:  [..DD DDDD]    Data port:
           R:2 ->  CHR reg 0  (2k @ $0000)
           R:3 ->  CHR reg 1  (2k @ $0800)
           R:4 ->  CHR reg 2  (2k @ $1000)
           R:5 ->  CHR reg 3  (2k @ $1800)
           R:6 ->  PRG reg 0  (8k @ $8000)
           R:7 ->  PRG reg 1  (8k @ $a000)
     
     CHR Setup:
     ---------------------------
           $0000   $0400   $0800   $0C00   $1000   $1400   $1800   $1C00 
         +---------------+---------------+---------------+---------------+
         |      R:2      |      R:3      |      R:4      |      R:5      |
         +---------------+---------------+---------------+---------------+
     
     PRG Setup:
     ---------------------------
           $8000   $A000   $C000   $E000  
         +-------+-------+-------+-------+
         |  R:6  |  R:7  | { -2} | { -1} |
         +-------+-------+-------+-------+
    

Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
