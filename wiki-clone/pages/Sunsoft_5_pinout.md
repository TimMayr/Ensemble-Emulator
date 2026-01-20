# Sunsoft 5 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Sunsoft_5_pinout) | View [other pages](Special_AllPages.xhtml#Sunsoft_5_pinout)

[Sunsoft FME-7](Sunsoft_FME_7.xhtml "Sunsoft FME-7") (and [Sunsoft 5B](Sunsoft_5B_audio.xhtml "Sunsoft 5B audio")): 44-pin 0.8mm PQFP 
    
    
                               / \
                              / O \
             (nrw) CPU D3 -> /01 44\ <- CPU D2 (nrw)
                     varies /02   43\ <- CPU D4 (nrw)
                 varies <- /03     42\ <- CPU D1 (nrw)
                (n) M2 -> /04       41\ <- CPU D5 (nrw)
          (n) CPU A13 -> /05         40\ <- CPU D0 (nrw)
                 GND -- /06           39\ -- +5V
        (n) CPU A14 -> /07             38\ <- CPU D6 (nrw)
       (n) /ROMSEL -> /08               37\ <- CPU D7 (nrw)
    (nrw) CPU R/W -> /09                 36\ -> PRG A13 (r)
        (n) /IRQ <- /10                   35\ -> PRG A16 (r)
    (n) PPU /RD -> /11                     34\ -> PRG A14 (r)
    (n) PPU A10 -> \12                     33/ -> PRG A15 (r)
     (n) PPU A11 -> \13                   32/ -> PRG A17 (r)
      (n) PPU A12 -> \14                 31/ -> PRG /CE (r)
       (n) PPU A13 -> \15               30/ -> PRG RAM /CE (w)
        (r) CHR /CE <- \16             29/ -> PRG RAM +CE (w)
                 +5V -- \17           28/ -- GND
                      ?? \18         27/ varies
    (nr) CHR/CIRAM A10 <- \19       26/ -> CHR A17 (r)
            (r) CHR A16 <- \20     25/ -> CHR A15 (r)
             (r) CHR A11 <- \21   24/ -> CHR A14 (r)
              (r) CHR A13 <- \22 23/ -> CHR A12 (r)
                              \   /
                               \ /
    pin 2: Amplifier in (5B) or ?PRG A18 out (FME-7)?
    pin 3: Amplifier out (5B)
    pin 18: Audio /disable (5B) [[1]](https://forums.nesdev.org/viewtopic.php?t=23138) - has internal pullup
    pin 27: Audio out (5B) or +5V (FME-7)
    
    n: connects to NES (CPU or PPU)
    r: connects to ROM (PRG or CHR)
    w: connects to WRAM
    
    Expansion audio mixing circuit:[[2]](https://forums.nesdev.org/viewtopic.php?p=225763#p225763)
                                         +                 +
               From S5B----/\/\/---+---||---+---/\/\/---+---||---To RF
               (S5B.27)    1K      |   1u   |   100K    |   1u   (Cart.46)
                                   |        |           |
               From 2A03---/\/\/---+        |           |
               (Cart.45)   10K              |           |
                                            |           |
               Amp. In----------------------+           |
               (S5B.2)                                  |
                                                        |
               Amp. Out---------------------------------+
               (S5B.3)
    

On FME-7, the two ground supplies and three +5V supplies are shorted internally. Boards (like GRM-E301) that were used with the 5A or 5B did not connect pin 27 to +5V. 

On FME-7, pin 3 has an overvoltage protection diode only; pin 18 has both over- and under- voltage protection diodes. Functionality is still not known. 

Categories: [Pinouts](Category_Pinouts.xhtml)
