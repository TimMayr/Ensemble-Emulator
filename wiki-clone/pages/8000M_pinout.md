# 8000M pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/8000M_pinout) | View [other pages](Special_AllPages.xhtml#8000M_pinout)

8000M or 8000M-1: 28-pin 0.6" PDIP. (Canonically [iNES Mapper 227](INES_Mapper_227.xhtml "INES Mapper 227")) 
    
    
                    .---\/---.
         PPU /WE -> |1     28| -- VCC
        CPU M2(*)-> |2     27| <- CPU A14     
          CPU A7 -> |3     26| <- CPU R/W     
          CPU A6 -> |4     25| <- CPU A8      
          CPU A5 -> |5     24| <- CPU A9      
          CPU A4 -> |6     23| <- PPU A11     
          CPU A3 -> |7     22| -> (**)        
          CPU A2 -> |8     21| <- PPU A10     
          CPU A1 -> |9     20| <- CPU /ROMSEL 
          CPU A0 -> |10    19| -> CHR /WE     
         PRG A14 <- |11    18| -> CIRAM A10   
         PRG A15 <- |12    17| -> PRG A19 (***)	
         PRG A16 <- |13    16| -> PRG A18
             GND -- |14    15| -> PRG A17     
                    `--------'
    

This chip appears either in PDIP or epoxy blob package. Cartridges with blob package have often GG1 markings near the mapper blob. 

_Notes:_

  * Pin 2 is in fact positive reset. Cartridges connect CPU M2 signal here via 2.2k resistor. There must be large capacitance inside as probing this pin during cartidge operation shows triangle wave instead of square.
  * Pin 17 for cartridges with < 1MB drives 74157 which switches what connects A3..A0 to PRG ROM (CPU A via some fixed solder pad)
  * Pin 22 is output that changes its value after some M2 cycles (probably output of restored reset signal, not used in any of analyzed cartridges)



Sources: 

  * BBS: <https://forums.nesdev.org/viewtopic.php?t=24115>



Categories: [Pinouts](Category_Pinouts.xhtml)
