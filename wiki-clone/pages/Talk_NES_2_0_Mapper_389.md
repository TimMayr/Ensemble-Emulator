# Talk:NES 2.0 Mapper 389

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANES_2.0_Mapper_389) | View [other pages](Special_AllPages.xhtml#Talk_NES_2_0_Mapper_389)

The given order of PRG/CHR bank bits is not correct: is: 
    
    
     PRG A15-A18, CHR A15-A17, CHR A13-A14, PRG A14-A15
    

should be: 
    
    
     PRG A18-A15, CHR A17-A15, CHR A14-A13, PRG A15-A14
    

The description does not says what drives PRG-A15 when in UNROM-064 mode (Inner PRG? Outer PRG? OR of them? - for SuperRun, "Inner PRG" seems to be the good choice) 
