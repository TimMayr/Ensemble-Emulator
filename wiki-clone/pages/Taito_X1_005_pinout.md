# Taito X1-005 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Taito_X1-005_pinout) | View [other pages](Special_AllPages.xhtml#Taito_X1_005_pinout)

Taito X1-005: 48-pin 0.6" PDIP (Canonically [mapper 80](INES_Mapper_080.xhtml "INES Mapper 080")). 
    
    
                    .---\/---.
     (r) PRG A18 <- |01    48| -- VCC
     (f)      M2 -> |02    47| -> PRG A17 (r)
    (fr) CPU A12 -> |03    46| -> PRG A15 (r)
     (f) CPU A13 -> |04    45| -> PRG A14 (r)
     (f) CPU A14 -> |05    44| -> PRG A13 (r)
     (fr) CPU A6 -> |06    43| <- CPU A8 (fr)
     (fr) CPU A5 -> |07    42| <- CPU A9 (fr)
     (fr) CPU A4 -> |08    41| <- CPU A11 (fr)
     (fr) CPU A3 -> |09    40| -> PRG A16 (r)
     (fr) CPU A2 -> |10    39| <- CPU A10 (fr)
     (fr) CPU A1 -> |11    38| <- /ROMSEL (fr)
     (fr) CPU A0 -> |12    37| <> PRG D7 (fr)
     (fr) CPU D0 <> |13    36| <> PRG D6 (fr)
     (fr) CPU D1 <> |14    35| <> PRG D5 (fr)
     (fr) CPU D2 <> |15    34| <> PRG D4 (fr)
     (f) CPU R/W -> |16    33| <> PRG D3 (fr)
     (r) CHR A17 <- |17    32| -> /RAMREGION
             GND -- |18    31| -> CIRAM A10 (f)
     (r) CHR A15 <- |19    30| -> CHR A14 (r)
     (r) CHR A12 <- |20    29| -> CHR A13 (r)
     (f) PPU A10 -> |21    28| -> CHR A11 (r)
     (f) PPU A11 -> |22    27| -> CHR A16 (r)
     (f) PPU A12 -> |23    26| -> CHR A10 (r)
            Vbat -- |24    25| -- internal RAM Vcc
                    `--------'
    

Games with a battery add a diode from pin 24 to pin 25. It's not clear what pin 24 is for. Games never connect pin 25 straight to 5V. 

Variant pinout for [mapper 207](INES_Mapper_207.xhtml "INES Mapper 207"): 
    
    
    **(f) CIRAM A10** <- |17    32| -- NC
              GND -- |18    31| -> **NC**
    

pin 32 "goes low for the whole duration of read/write cycle when address in 0x6000 .. 0x7FFF", or if the X1-005's internal RAM is enabled, only the region of 0x6000-0x7EFF. 

Sources: 

  * [BootGod](https://forums.nesdev.org/viewtopic.php?t=3834)
  * [Krzysiobal](https://forums.nesdev.org/viewtopic.php?t=19760)



Categories: [Pinouts](Category_Pinouts.xhtml)
