# Mapper 208 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Mapper_208_pinout) | View [other pages](Special_AllPages.xhtml#Mapper_208_pinout)

[iNES Mapper 208](INES_Mapper_208.xhtml "INES Mapper 208")
    
    
               +---v---+
    CPU A14 -> | 1   28| -- VCC
    CPU A13 -> | 2   27| <- M2
    CPU A12 -> | 3   26| <- CPU /ROMSEL
    CPU A11 -> | 4   25| <- CPU R/!W
     CPU A1 -> | 5   24| <- CPU D0
     CPU A0 -> | 6   23| <- CPU D1
    PPU A10 -> | 7   22| <- CPU D2
    PPU A11 -> | 8   21| -- GND
       OR A -> | 9   20| <- CPU D3
       OR B -> |10   19| <- CPU D4
    PRG A16 <- |11   18| <- CPU D5
    PRG A15 <- |12   17| <- CPU D6
       OR Y <- |13   16| <- CPU D7
        GND -- |14   15| -> CIR A10
               '-------'
                UNNAMED
            0.6" 40-pin PDIP
    		
    

Source: [[[1]](https://forums.nesdev.org/viewtopic.php?p=282406#p282406)] 

Categories: [Pinouts](Category_Pinouts.xhtml)
