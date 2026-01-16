# PT8154 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/PT8154_pinout) | View [other pages](Special_AllPages.xhtml#PT8154_pinout)
    
    
                .--\/--.
     CHR A10 <- |01  28| -- VCC
     PPU A12 -> |02  27| -> CHR A16
     PPU A11 -> |03  26| -> CHR A11     
     PPU A10 -> |04  25| <- CPU D3
     CHR A13 <- |05  24| <- CPU D2
     CHR A14 <- |06  23| <- CPU D4
     CHR A12 <- |07  22| <- CPU D1
     CIR A10 <- |08  21| <- CPU D5
     CHR A15 <- |09  20| <- CPU D0
    CPU /RMS -> |10  19| <- CPU D6
     CPU R/W -> |11  18| <- CPU A0
     CPU A14 -> |12  17| <- CPU D7
         GND -- |13  16| <- M2
     CPU A13 -> |14  15| -> PRG /CE
                `------'
                 PT8154
    

Source: [[1]](http://forums.nesdev.org/viewtopic.php?f=9&t=20419&p=252538#p252538)

Categories: [Pinouts](Category_Pinouts.xhtml)
