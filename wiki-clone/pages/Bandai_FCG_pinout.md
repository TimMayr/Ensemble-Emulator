# Bandai FCG pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Bandai_FCG_pinout) | View [other pages](Special_AllPages.xhtml#Bandai_FCG_pinout)

[Bandai FCG-1/LZ93D36 and FCG-2](Bandai_FCG_board.xhtml "Bandai FCG board"): 42-pin shrink PDIP ([iNES Mapper 016](INES_Mapper_016.xhtml "INES Mapper 016")) 
    
    
                .---\/---.
           ? ?? |  1  42 | -- +5V
          M2 -> |  2  41 | ?? ?
     CPU A13 -> |  3  40 | -> PRG A17
     CPU A14 -> |  4  39 | -> PRG A15
      CPU A3 -> |  5  38 | -> PRG A14
      CPU A2 -> |  6  37 | -> PRG A16
      CPU A1 -> |  7  36 | <- CPU D7
      CPU A0 -> |  8  35 | <- CPU D6
     /ROMSEL -> |  9  34 | <- CPU D5
      CPU D0 -> | 10  33 | <- CPU D4
      CPU D1 -> | 11  32 | <- CPU D3
      CPU D2 -> | 12  31 | -> /IRQ
         R/W -> | 13  30 | -> CIRAM A10
     PPU /RD -> | 14  29 | -> CHR A17
     CHR A15 <- | 15  28 | -> CHR A14
     CHR A12 <- | 16  27 | -> CHR A13
     PPU A10 -> | 17  26 | -> CHR A11
     PPU A11 -> | 18  25 | -> CHR A16
     PPU A12 -> | 19  24 | -> CHR A10
     PPU A13 -> | 20  23 | -> CHR /CE
         GND -- | 21  22 | <- ?
                '--------'
    

Pin 22 is grounded on at least one FCG-1 board and floating on at least one FCG-2. 

Categories: [Pinouts](Category_Pinouts.xhtml)
