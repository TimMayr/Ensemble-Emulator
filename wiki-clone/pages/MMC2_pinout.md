# MMC2 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/MMC2_pinout) | View [other pages](Special_AllPages.xhtml#MMC2_pinout)

[MMC2](MMC2.xhtml "MMC2") Chip: (40/42 pin shrink-DIP) 

Later revisions of the chip are labelled MMC2-L and are 42 pin instead of 40 pin. 
    
    
                     .--\/--.
              GND  - |XX  XX| -  +5V
      (n)      M2 -> |01  40| -  +5V
      (n) CPU A14 -> |02  39| -  GND
      (n) CPU A13 -> |03  38| -> CIRAM A10 (n)
      (r) PRG A15 <- |04  37| -> CHR A15 (r)
      (r) PRG A14 <- |05  36| -> CHR A12 (r)
     (rn) CPU A12 -> |06  35| -> CHR A14 (r)
      (r) PRG A13 <- |07  34| <- PPU A12 (n)
      (r) PRG A16 <- |08  33| -> CHR A13 (r)
      (r) PRG /CE <- |09  32| -> CHR A16 (r)
     (rn) CPU  D4 -> |10  31| <- PPU  A8 (rn)
     (rn) CPU  D3 -> |11  30| <- PPU  A5 (rn)
     (rn) CPU  D0 -> |12  29| <- PPU  A9 (rn)
     (rn) CPU  D1 -> |13  28| <- PPU  A4 (rn)
     (rn) CPU  D2 -> |14  27| <- PPU A11 (rn)
      (n) CPU R/W -> |15  26| <- PPU  A3 (rn)
      (n) /ROMSEL -> |16  25| <- PPU  A7 (rn)
     (rn) PPU /RD -> |17  24| <- PPU  A2 (rn)
     (rn) PPU  A0 -> |18  23| <- PPU A10 (rn)
     (rn) PPU  A6 -> |19  22| <- PPU  A1 (rn)
              GND  - |20  21| <- PPU A13 (rn)
                     `------'
    
    (r) - this pin connects to the ROM chips
    (n) - this pin connects to the NES connector
    

Categories: [Pinouts](Category_Pinouts.xhtml)
