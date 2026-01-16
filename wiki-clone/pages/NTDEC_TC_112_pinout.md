# NTDEC TC-112 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NTDEC_TC-112_pinout) | View [other pages](Special_AllPages.xhtml#NTDEC_TC_112_pinout)

NTDEC TC-112: 40-pin 0.6" PDIP. (Canonically [iNES Mapper 193](INES_Mapper_193.xhtml "INES Mapper 193")) 
    
    
                    .---\/---.
     (n)      M2 -> |1     40| -- VCC
     (nr) CPU D7 -> |2     39| -> PRG A16  (r)
     (nr) CPU D6 -> |3     38| -> PRG A15  (r)
     (nr) CPU D5 -> |4     37| -> PRG A14  (r)
     (nr) CPU D4 -> |5     36| -> PRG A13  (r)
     (nr) CPU D3 -> |6     35| -> PRG /CE  (r)
     (nr) CPU D2 -> |7     34| -> CHR A17  (r)
     (nr) CPU D1 -> |8     33| -> CHR A16  (r)
     (nr) CPU A1 -> |9     32| -> CHR A15  (r)
     (nr) CPU A0 -> |10    31| -> CHR A14  (r)
     (n) CPU R/W -> |11    30| -> CHR A13  (r)
     (n) CPU /CE -> |12    29| -> CHR A12  (r)
     (n) CPU A14 -> |13    28| -> CHR A11  (r)
     (n) CPU A13 -> |14    27| -> CHR1 /CE (r) = (PPU /RD) or (PPU A13) or (CHR A17)
     (n) PPU A13 -> |15    26| -> CHR2 /CE (r) = (PPU /RD) or (PPU A13) or (not CHR A17)
     (n) PPU A12 -> |16    25| <- CPU D0  (nr)
     (n) PPU A11 -> |17    24| -> /W6005
     (nr)PPU A10 -> |18    23| -> /W6006
      †  PPU /RD -> |19    22| <- CPU A2  (nr)
             GND -- |20    21| -> CIRAM A10 (n)
                    `--------'
    

† On one board, connected to a seemingly-needlessly-complicated [7400](7400.xhtml "7400") circuit. See the [thread on the forum](http://forums.nesdev.org/viewtopic.php?t=7106). 

  
Sources: 

  * BootGod: <https://forums.nesdev.org/viewtopic.php?p=69275#p69275>
  * Krzysiobal: <https://forums.nesdev.org/viewtopic.php?t=19751>



Categories: [Pinouts](Category_Pinouts.xhtml)
