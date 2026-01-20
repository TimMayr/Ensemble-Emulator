# Taito X1-017 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Taito_X1-017_pinout) | View [other pages](Special_AllPages.xhtml#Taito_X1_017_pinout)

Taito X1-017: 64-pin 0.6" shrink DIP (Canonically [mapper 82](Taito_X1_017.xhtml "INES Mapper 082")) 
    
    
                       .---\/---.
    $7EFA.5 (PRG A) <- |01    64| -- VCC
       (fw) CPU A13 -> |02    63| <- M2 (f)
       (frw) CPU A8 -> |03    62| -> $7EFA.0 (PRG A)
       (frw) CPU A7 -> |04    61| -> $7EFA.2 (PRG A)
       (frw) CPU A9 -> |05    60| -> $7EFA.1 (PRG A)
       (fw) CPU A14 -> |06    59| -> $7EFA.3 (PRG A)
      (frw) CPU A11 -> |07    58| -> $7EFA.4 (PRG A)
       (frw) CPU A6 -> |08    57| <- CPU A12 (frw)
       (frw) CPU A5 -> |09    56| -> delayed M2 by ~80ns, open collector
      (frw) CPU A10 -> |10    55| -> delayed M2 by ~80ns 
       (frw) CPU A4 -> |11    54| -> delayed M2 by ~40ns
       (frw) CPU A3 -> |12    53| <- PRG RAM +CE (jumpered to pin 55 by default)
       (frw) CPU D7 <> |13    52| -- VCC
       (frw) CPU A2 -> |14    51| -- GND
       (frw) CPU D6 <> |15    50| -- PAD2 (NC)
       (frw) CPU A1 -> |16    49| -- VBAT (direct connect to positive battery terminal)
       (frw) CPU D5 <> |17    48| -- RAM VCC 
       (frw) CPU A0 -> |18    47| -- GND
       (frw) CPU D4 <> |19    46| <- PRG RAM +WE (jumpered to pin 56 with C3+R2)
       (frw) CPU D0 <> |20    45| -> CHR0 /CE (lower 128KB ROM if two 128KB CHR ROMs)
       (frw) CPU D3 <> |21    44| -> CHR1 /CE (upper 128KB ROM if two 128KB CHR ROMs)
       (frw) CPU D1 <> |22    43| -- GND
       (frw) CPU D2 <> |23    42| -> CHR A11 (r)
        (f) CPU R/W -> |24    41| -> CIRAM A10 (f)
       (fr) /ROMSEL -> |25    40| -> CHR A17 (r) 
           (f) /IRQ <- |26    39| <- PPU A10 (f)
                GND -- |27    38| -> CHR A16 (r)
     (PPU /RD) OR A -> |28    37| <- PPU A11 (f)
        (r) CHR A15 <- |29    36| -> CHR A10 (r)
        (r) CHR A14 <- |30    35| <- PPU A12 (f)
        (r) CHR A13 <- |31    34| -> OR Y (CHR /CE)
        (r) CHR A12 <- |32    33| <- OR B (PPU A13)
                       '--------'
    

Wiring of PRG address pins: 

  * OEM PCBs wires: $7EFA.5 as PRG-A13, $7EFA.4 as PRG-A14, $7EFA.3 as PRG-A15, $7EFA.2 as PRG-A16 and skips $7EFA.1 and $7EFA.0. Mapper 552 should be used to descibe it.


  * The inaccurate definition of mapper 82 and all ROM dumps that are assigned to it assumes: $7EFA.5 as PRG-A16, $7EFA.4 as PRG-A15, $7EFA.3 as PRG-A14 and $7EFA.2 as PRG-A13 [[1]](https://forums.nesdev.org/viewtopic.php?p=246418#p246418)



Sources: 

  * [Ice Man's forum post](https://forums.nesdev.org/viewtopic.php?p=246369#p246369)
  * [Krzysiobal's forum post](https://forums.nesdev.org/viewtopic.php?f=9&t=19724)



Categories: [Pinouts](Category_Pinouts.xhtml)
