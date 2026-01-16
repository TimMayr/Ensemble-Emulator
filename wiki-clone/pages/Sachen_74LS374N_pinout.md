# Sachen 74LS374N pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Sachen_74LS374N_pinout) | View [other pages](Special_AllPages.xhtml#Sachen_74LS374N_pinout)

Sachen fake-marked “74LS374N”: 20-pin 0.3" DIP or epoxy blob on daughterboard. 
    
    
                 .---V---.
         R5.0 <- | 01 20 | -- Vcc
    CIRAM A10 <- | 02 19 | <- CPU A8
      PPU A11 -> | 03 18 | <- R/W
         R5.1 <- | 04 17 | <- M2
      CPU A14 -> | 05 16 | <- PPU A10
      CPU  A0 -> | 06 15 | <- /ROMSEL
      CPU  D0 <> | 07 14 | <> CPU  D2
      CPU  D1 <> | 08 13 | -> R6.1
         R2.0 <- | 09 12 | -> R6.0
          GND -- | 10 11 | -> R4.0
                 '-------'
    

## Sachen SA-015 PCB ([INES Mapper 150](INES_Mapper_150.xhtml "INES Mapper 150"))
    
    
                 .---V---.
      PRG A15 <- | 01 20 | -- Vcc
    CIRAM A10 <- | 02 19 | <- CPU A8
      PPU A11 -> | 03 18 | <- R/W
      PRG A16 <- | 04 17 | <- M2
      CPU A14 -> | 05 16 | <- PPU A10
      CPU  A0 -> | 06 15 | <- /ROMSEL
      CPU  D0 <> | 07 14 | <> CPU  D2 or Vcc, depending on solder pad setting
      CPU  D1 <> | 08 13 | -> CHR A14
          N/C <- | 09 12 | -> CHR A13
          GND -- | 10 11 | -> CHR A15
                 '-------'
    

## Sachen SA-020A PCB ([INES Mapper 243](INES_Mapper_243.xhtml "INES Mapper 243"))
    
    
                 .---V---.
      PRG A15 <- | 01 20 | -- Vcc
    CIRAM A10 <- | 02 19 | <- CPU A8
      PPU A11 -> | 03 18 | <- R/W
      PRG A16 <- | 04 17 | <- M2
      CPU A14 -> | 05 16 | <- PPU A10
      CPU  A0 -> | 06 15 | <- /ROMSEL
      CPU  D0 <> | 07 14 | <> CPU  D2
      CPU  D1 <> | 08 13 | -> CHR A16
      CHR A13 <- | 09 12 | -> CHR A15
          GND -- | 10 11 | -> CHR A14
                 '-------'
    

Categories: [Pinouts](Category_Pinouts.xhtml)
