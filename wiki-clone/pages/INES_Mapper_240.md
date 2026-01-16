# INES Mapper 240

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_240) | View [other pages](Special_AllPages.xhtml#INES_Mapper_240)
    
    
     Here are Disch's original notes:  
     ========================
     =  Mapper 240          =
     ========================
     
     
     Example Games:
     --------------------------
     Jing Ke Xin Zhuan
     Sheng Huo Lie Zhuan
     
     
     Registers:
     --------------------------
       $4020-5FFF:  [PPPP CCCC]
         P = Selects 32k PRG @ $8000
         C = Selects 8k CHR @ $0000
    

This is two similar PCBs with subtly different addresses for the register. 

Jing Ke Xin Zhuan (荊軻新傳) decodes the register as: (ADDRESS & $E800) == $4800 

Sheng Huo Lie Zhuan (聖火列傳) decodes the register as: (ADDRESS & $E100) == $4100 

Both boards add PRG RAM at the usual $6000 

Categories: [GNROM-like mappers](Category_GNROM_like_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml)
