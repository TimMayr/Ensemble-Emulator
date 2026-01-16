# Taito TC0350 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Taito_TC0350_pinout) | View [other pages](Special_AllPages.xhtml#Taito_TC0350_pinout)

Taito TC0350: 0.6" 48-pin PDIP (Canonically [iNES Mapper 033](INES_Mapper_033.xhtml "INES Mapper 033")) 
    
    
               .--\/--.
        GND -- |01  48| -- VCC
    ?PRG A18?? |02  47| <- M2
    PRG A16 <- |03  46| -> PRG A14
    ?PRG A17?? |04  45| -> PRG A13
    PRG A15 <- |05  44| -> PRG /CE
    CPU A13 -> |06  43| <- D7
    CPU A14 -> |07  42| <- D6
     CPU A1 -> |08  41| <- D5
     CPU A0 -> |09  40| <- D4
         D0 -> |10  39| <- D3
         D1 -> |11  38| ?> ?CHR A18?
         D2 -> |12  37| -> CHR A17
    /ROMSEL -> |13  36| -> CHR A14
    CPU R/W -> |14  35| -> CHR A13
       /IRQ <- |15  34| <- PPU A8
    PPU /RD -> |16  33| <- PPU A9
    CHR A16 <- |17  32| <- PPU A10
    CHR A15 <- |18  31| -> CHR A11
    CHR A12 <- |19  30| <- PPU A11
     PPU A7 -> |20  29| -> CHR A10/CIRAM A10
     PPU A6 -> |21  28| <- PPU A12
     PPU A5 -> |22  27| -> CHR /OE
     PPU A4 -> |23  26| <- PPU A13
        GND -- |24  25| -- VCC
               .------.
    

Categories: [Pinouts](Category_Pinouts.xhtml)
