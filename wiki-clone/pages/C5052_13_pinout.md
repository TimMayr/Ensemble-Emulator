# C5052-13 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/C5052-13_pinout) | View [other pages](Special_AllPages.xhtml#C5052_13_pinout)

The **C5052-13** is a mapper chip with [MMC1](MMC1.xhtml "MMC1") and [MMC3](MMC3.xhtml "MMC3") functionality. 
    
    
                .--\/--. 
         +5V -- |01  40| <- CPU A0
        mode -> |02  39| <- CPU A1
     PRG A13 <- |03  38| <- M2
     PRG A14 <- |04  37| <- CPU A12
     PRG A15 <- |05  36| <- CPU A13
     PRG A16 <- |06  35| <- CPU A14
     PRG A17 <- |07  34| <- CPU D4
     PRG A18 <- |08  33| <- CPU D5
     PRG /CE <- |09  32| <- CPU /ROMSEL
    WRAM +CE <- |10  31| <- CPU D6
     CHR A10 <- |11  30| <- CPU D7
     CHR A11 <- |12  29| <- CPU D0
     CHR A12 <- |13  28| <- CPU R/!W
     CHR A13 <- |14  27| -> CIRAM A10
     CHR A14 <- |15  26| <- CPU D1
     CHR A15 <- |16  25| <- CPU D2
     CHR A16 <- |17  24| <- CPU D3
     CHR A17 <- |18  23| -> PPU A12
        /IRQ <- |19  22| -> PPU A11
         GND -- |20  21| -> PPU A10
                `------' 
                C5052-13 
    

## References

  * [BBS - Interesting mapper: C5052-13 (DIP40)](https://forums.nesdev.org/viewtopic.php?f=9&t=19248)



Categories: [Pinouts](Category_Pinouts.xhtml)
