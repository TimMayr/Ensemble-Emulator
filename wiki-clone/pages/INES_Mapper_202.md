# INES Mapper 202

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_202) | View [other pages](Special_AllPages.xhtml#INES_Mapper_202)

  
Implemented in BizHawk as of commit f72875b584abd79201ad9d3d0131fa5afcf0c429 

Used for the 150-in-1 pirate cart (which was shipped with dual sloat SP60 famiclone console). 

[![](../wiki-images/150in1_top.jpg)](File_150in1_top_jpg.xhtml)

[](File_150in1_top_jpg.xhtml "Enlarge")

150-in-1 PCB top side

[![](../wiki-images/150in1_bottom.jpg)](File_150in1_bottom_jpg.xhtml)

[](File_150in1_bottom_jpg.xhtml "Enlarge")

150-in-1 PCB bottom side

[![](../wiki-images/150in1_schematics.png)](File_150in1_schematics_png.xhtml)

[](File_150in1_schematics_png.xhtml "Enlarge")

150-in-1 schematics
    
    
     Registers:
     $8000-FFFF: A~[.... .... .... O..O]
                   [.... .... .... RRRM]
     
     
     O = PRG Mode, if 3, will use 32 PRG mode, else 16
     R = Page registers, used for both CHR 8k banks, and PRG banks (in both 16k and 32k modes)
     M = Mirroring, 0 = Vertical, 1 = Horizontal
     
     CHR Setup:
     
                        $0000   $0400   $0800   $0C00   $1000   $1400   $1800   $1C00 
                       +-------------------------------+-------------------------------+
     CHR-ROM:          |                             $8000                             |
                       +-------------------------------+-------------------------------+
     
     
     
     PRG Setup:
     ------------------------------
     32k Mode:
     
     $8000     $A000     $C000     $E000
     +-------------------------------------+
     |               <$8000>               |
     +-------------------------------------+
     
     
     16k Mode:
     
     In 16k mode both regions are mapped to the same PRG bank
     
     $8000     $A000     $C000     $E000     
     +--------------------------------------+
     |       $8000       |      $8000       |
     +--------------------------------------+
    

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
