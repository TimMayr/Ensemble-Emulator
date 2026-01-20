# Irem H3001 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Irem_H3001_pinout) | View [other pages](Special_AllPages.xhtml#Irem_H3001_pinout)

Irem IF-H3001: 48-pin 0.6" DIP (canonically [mapper 65](INES_Mapper_065.xhtml "INES Mapper 065")) 
    
    
                   .--\/--.
            n/c -- |01  48| -> +5V
            n/c -- |02  47| -> CHR A17
        PRG A18 <- |03  46| -> CHR-A16
        PRG A17 <- |04  45| -> CHR-A15
        PRG-A16 <- |05  44| -> CHR-A14
        PRG-A15 <- |06  43| -> CHR-A13
        PRG-A14 <- |07  42| -> CHR-A12
        PRG-A13 <- |08  41| -> CHR-A11
            GND -- |09  40| -> CHR-A10
             M2 -> |10  39| -- GND
        CPU-A14 -> |11  38| -> PRG RAM /CE
        CPU-A13 -> |12  37| -> /IRQ     
        CPU-A12 -> |13  36| -> CIR-A10  
        CPU-R/W -> |14  35| -> CHR-/CE  
    CPU-/ROMSEL -> |15  34| -> PRG-/CE 
         CPU-A2 -> |16  33| -> delayed M2
         CPU-A1 -> |17  32| <- PPU-A12  
         CPU-A0 -> |18  31| <- PPU-A11  
        PPU-A13 -> |19  30| <- PPU-A10  
        PPU-/RD -> |20  29| <- CPU-D7  
         CPU-D2 -> |21  28| <- CPU-D6  
         CPU-D1 -> |22  27| <- CPU-D5  
         CPU-D0 -> |23  26| <- CPU-D4  
            GND -- |24  25| <- CPU-D3  
                   '------'
    

References: [[1]](http://forums.nesdev.org/viewtopic.php?f=9&t=19778)

Categories: [Pinouts](Category_Pinouts.xhtml)
