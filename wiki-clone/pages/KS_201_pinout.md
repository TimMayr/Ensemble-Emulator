# KS-201 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/KS-201_pinout) | View [other pages](Special_AllPages.xhtml#KS_201_pinout)

National Semiconductor KS-201: 28-pin DIP (Canonically [NES 2.0 Mapper 554](NES_2_0_Mapper_554.xhtml "NES 2.0 Mapper 554")) 
    
    
               .-\_/-.
     CPU M2 -> |01 28| -- +5V              
     CPU A1 -> |02 27| -> PRG A13
     CPU A2 -> |03 26| -> PRG A14
     CPU A3 -> |04 25| -> PRG A15
     CPU A4 -> |05 24| -> PRG A16
     CPU A5 -> |06 23| -> CHR A13
     CPU A6 -> |07 22| -> CHR A14
     CPU A7 -> |08 21| -> CHR A15
     CPU A8 -> |09 20| -> CHR A16
     CPU A9 -> |10 19| -> PRG /CE
    CPU A10 -> |11 18| <- unknown input (VCC)
    CPU A11 -> |12 17| <- CPU /ROMSEL
    CPU A12 -> |13 16| <- CPU A14
        GND -- |14 15| <- CPU A13
               '-----'
    

This was also used on [NES 2.0 Mapper 312](NES_2_0_Mapper_312.xhtml "NES 2.0 Mapper 312"): 
    
    
               .-\_/-.
     CPU M2 -> |01 28| -- +5V              
     CPU A1 -> |02 27| -> **PRG /A14**
     CPU A2 -> |03 26| -> **PRG A15**
     CPU A3 -> |04 25| -> **PRG A16**
     CPU A4 -> |05 24| -> **n/c**
     CPU A5 -> |06 23| -> **n/c**
     CPU A6 -> |07 22| -> **n/c**
     CPU A7 -> |08 21| -> **n/c**
     CPU A8 -> |09 20| -> **n/c**
     CPU A9 -> |10 19| -> **n/c**
    CPU A10 -> |11 18| <- unknown input (VCC)
    CPU A11 -> |12 17| <- **CPU /A14**
        **n/c** -> |13 16| <- **CPU /ROMSEL**
        GND -- |14 15| <- CPU A13
               '-----'
    

Categories: [Pinouts](Category_Pinouts.xhtml)
