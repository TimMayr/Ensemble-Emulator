# Taito TC0190 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Taito_TC0190_pinout) | View [other pages](Special_AllPages.xhtml#Taito_TC0190_pinout)

Taito TC0190: 0.6" 40-pin high-density PDIP (Canonically [iNES Mapper 033](INES_Mapper_033.xhtml "INES Mapper 033")) 
    
    
                     .--\/--.
      (fr) CPU D0 -> |01  40| -- Vcc
      (fr) CPU D1 -> |02  39| <- R/W (f)
      (fr) CPU D2 -> |03  38| <- /ROMSEL (f)
      (fr) CPU D3 -> |04  37| <- M2 (f)
      (fr) CPU D4 -> |05  36| -- n/c
      (fr) CPU D5 -> |06  35| -- n/c
      (fr) CPU D6 -> |07  34| <- PPU A10 (f)
      (fr) CPU D7 -> |08  33| <- PPU A11 (f)
      (r) PRG A13 <- |09  32| <- PPU A12 (f)
      (r) PRG A14 <- |10  31| -- GND
      (r) PRG A15 <- |11  30| -> CIRAM A10 (f)
      (r) PRG A16 <- |12  29| -> CHR A10 (r)
      (r) PRG A17 <- |13  28| -> CHR A11 (r)
      (r) PRG A18 <- |14  27| -> CHR A12 (r)
      (fr) CPU A0 -> |15  26| -> CHR A13 (r)
      (fr) CPU A1 -> |16  25| -> CHR A14 (r)
      (f) CPU A13 -> |17  24| -> CHR A15 (r)
      (f) CPU A14 -> |18  23| -> CHR A16 (r)
      (r) PRG /CE <- |19  22| -> CHR A17 (r)
              GND -- |20  21| -> CHR A18 (r)
                     '------'
    

Some boards added a PAL to the TC0190. As far as we can tell, the only purpose of the PAL was to move the mirroring control bit. This becomes a subset of [iNES Mapper 048](INES_Mapper_048.xhtml "INES Mapper 048") instead: 
    
    
    TC0190 pin 30: (was CIRAM A10) now NC
    TC0190 pin 38: (was /ROMSEL) now from PAL pin 12
    
                     PAL16R4
                      --\/--
      (f) /ROMSEL -> |01  20| -- Vcc
      (f) /ROMSEL -> |02  19| -> CIRAM A10 (f)
      (f)     R/W -> |03  18| ?? NC
      (f) CPU A14 -> |04  17| -> NC (latched value written to D6)
      (f) CPU A13 -> |05  16| -> NC
      (fr) CPU A1 -> |06  15| -> NC
      (fr) CPU A0 -> |07  14| -> NC
      (f) PPU A11 -> |08  13| <- CPU D6 (fr)
      (f) PPU A10 -> |09  12| -> to TC0190 pin 38
              GND -- |10  11| <- GND (PAL /OE signal)
                      ------
    

[PALs](https://en.wikipedia.org/wiki/Programmable_Array_Logic "wikipedia:Programmable Array Logic") allow the programming of arbitrary logic functions, but in the PAL16R4, pins 1 and 11 have fixed functionality. This is why /ROMSEL is connected to both pins 1 and pin 2: pin 1 can _only_ be used as a clock input to latch the value of D6, but /ROMSEL is also connected to pin 2 so that the PAL can fail to pass through the write to the TC0190 when the CPU writes to $E000. 

Categories: [Pinouts](Category_Pinouts.xhtml)
