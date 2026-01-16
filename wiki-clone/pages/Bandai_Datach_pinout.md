# Bandai Datach pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Bandai_Datach_pinout) | View [other pages](Special_AllPages.xhtml#Bandai_Datach_pinout)

Bandai Datach: 32-pin 0.1" card edge ([iNES Mapper 157](INES_Mapper_157.xhtml "INES Mapper 157")) 
    
    
      **CHR A13 <- |  1 32 | -- +5V
        PRG A16 <- |  2 31 | -> ??CHR A12??
        PRG A15 <- |  3 30 | -> PRG A17
        CPU A12 <- |  4 29 | -> PRG A14
         CPU A7 <- |  5 28 | -> CPU A13
         CPU A6 <- |  6 27 | -> CPU A8
         CPU A5 <- |  7 26 | -> CPU A9
         CPU A4 <- |  8 25 | -> CPU A11
         CPU A3 <- |  9 24 | -> /ExternalROMRead
         CPU A2 <- | 10 23 | -> CPU A10
         CPU A1 <- | 11 22 | <> I2C SDA
         CPU A0 <- | 12 21 | <> CPU D7
         CPU D0 <> | 13 20 | <> CPU D6
         CPU D1 <> | 14 19 | <> CPU D5
         CPU D2 <> | 15 18 | <> CPU D4
         GND    -- | 16 17 | <> CPU D3
    

Notes: 

  * Pin 1 is used for the external I²C EEPROM clock.
  * Pin 31 is Naruko's educated guess



This is almost the standard JEDEC ROM pin order. 

Source: [Naruko](https://seesaawiki.jp/famicomcartridge/d/Bandai%20Datach)

Categories: [Pinouts](Category_Pinouts.xhtml)
