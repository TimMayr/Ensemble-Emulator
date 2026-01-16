# Sunsoft 4 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Sunsoft_4_pinout) | View [other pages](Special_AllPages.xhtml#Sunsoft_4_pinout)

[Sunsoft 4](INES_Mapper_068.xhtml "INES Mapper 068"): 40-pin 0.6" PDIP 
    
    
                 .---\/---.
           M2 -> | 01  40 | -- +5V
      /ROMSEL -> | 02  39 | <- R/W
      CPU A14 -> | 03  38 | <- CPU D6
      CPU A13 -> | 04  37 | <- CPU D5
      CPU A12 -> | 05  36 | <- CPU D4
     PPU /A13 -> | 06  35 | <- CPU D3
      PPU A12 -> | 07  34 | <- CPU D2
      PPU A11 -> | 08  33 | <- CPU D1
      PPU A10 -> | 09  32 | <- CPU D0
         ?GND -> | 10  31 | <- OR A (PPU /RD)
      PRG /CE <- | 11  30 | <- OR B (SS4 /CHRSEL)
      CHR A17 <- | 12  29 | -> OR Y (CHR /CS)
      CHR A16 <- | 13  28 | -> CIRAM /CS
      CHR A15 <- | 14  27 | -> SS4 /CHRSEL
      CHR A14 <- | 15  26 | -> WRAM /CE
      CHR A13 <- | 16  25 | -> PRG A14
      CHR A12 <- | 17  24 | -> PRG A15
      CHR A11 <- | 18  23 | -> PRG A16
      CHR A10 <- | 19  22 | -> PRG A17
          GND -- | 20  21 | -> WRAM +CE
                 `--------'
    
    10 probably actually some control, not a supply pin
    19 also connects to CIRAM A10
    29,30,31 actually just an OR gate; for use with a 28-pin 128KB CHR-ROM
    

Reference: [Naruko's notes](http://forums.nesdev.org/viewtopic.php?p=106818#p106818)

  


Tengen's Sunsoft-4 seems to have a little modified pinout: [[1]](https://nescartdb.com/profile/view/326/after-burner)
    
    
    .. |
    31 | <- OR A (PPU /RD)
    30 | <- OR B (PPU A13)
    29 | -> (NC?)
    28 | -> CIRAM /CS
    27 | -> (NC?)
    26 | -> goes to NOR gate CHR roms decoder
    .. |
    

Categories: [Pinouts](Category_Pinouts.xhtml)
