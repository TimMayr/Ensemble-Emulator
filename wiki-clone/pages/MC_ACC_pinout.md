# MC-ACC pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/MC-ACC_pinout) | View [other pages](Special_AllPages.xhtml#MC_ACC_pinout)

[MC-ACC](MMC3.xhtml#IRQ_Specifics "MMC3") chip: (40-pin 0.6" DIP) 
    
    
                   .--\/--.
            GND ?? |01  40| -- 5V
    (r) PRG A16 <- |02  39| -> PRG A18 (r)
    (r) PRG A15 <- |03  38| -> PRG A17 (r)
    (n) CPU A14 -> |04  37| -> PRG A14 (r)
    (n) CPU A13 -> |05  36| -> PRG A13 (r)
            n/c ?? |06  35| <- CPU R/W (n)
            n/c ?? |07  34| <- /ROMSEL (n)
            n/c ?? |08  33| -> PRG /OE (r)
    (nr) CPU A0 -> |09  32| <- CPU D7 (nr)
    (nr) CPU D0 -> |10  31| <- CPU D6 (nr)
    (nr) CPU D1 -> |11  30| <- CPU D5 (nr)
    (nr) CPU D2 -> |12  29| <- CPU D4 (nr)
    (r) CHR A16 <- |13  28| <- CPU D3 (nr)
    (r) CHR A15 <- |14  27| -> CIRAM A10 (n)
    (r) CHR A14 <- |15  26| -> CHR A17 (r)
    (r) CHR A13 <- |16  25| -> /IRQ (n)
    (r) CHR A12 <- |17  24| <- PPU A12 (n)
    (r) CHR A11 <- |18  23| <- PPU A11 (n)
    (r) CHR A10 <- |19  22| <- PPU A10 (n)
            GND -- |20  21| <- M2 (n)
                   '------'
    
    39: confirmed in [[1]](https://forums.nesdev.org/viewtopic.php?p=292722#p292722)
    24: on the [55741 PCB](https://nescartdb.com/search/advanced?pcb=55741), deglitched and shaped as MCACCA12IN = AND(PPUA12,AND(AND(AND(PPUA12)))). [Kevtris's notes](http://kevtris.org/mappers/2ndparty/acclaim_55741.html)
    

Source: <https://forums.nesdev.org/viewtopic.php?p=16795#p16795>

Categories: [Pinouts](Category_Pinouts.xhtml)
