# SP-80 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/SP-80_pinout) | View [other pages](Special_AllPages.xhtml#SP_80_pinout)
    
    
                                         _____________
                                        /             \
              PPU A0 <-     PPU A13 <- / 79 80   02 01 \ <> PPU D0      -- GND
             PPU A1 <-     PPU A12 <- / 77 78     04 03 \ <> PPU D1      <> PPU D7
            PPU A2 <-     PPU A11 <- / 75 76       06 05 \ <> PPU D2      <> PPU D6
           PPU A3 <-     PPU A10 <- / 73 74         08 07 \ <> PPU D3      <> PPU D5
             GND --      PPU A9 <- / 71 72           10 09 \ <- CLK         <> PPU D4
         PPU A8 <-      PPU A4 <- / 69 70             12 11 \ -> VIDEO       ?? ?
        PPU A7 <-      PPU A5 <- / 67 68               14 13 \ <- /RESET      -- VCC
     PPU /A13 <-      PPU A6 <- / 65 66                 16 15 \ <- $4017 D0    <- $4016 D2
     PPU /WE <-     PPU /RD <- / 63 64                   18 17 \ <- $4016 D0    <- AMP IN
       /IRQ <-          NC -- / 61 62                     20 19 \ -> $4016 CLK   <- $4017 D1
    CPU R/W <-    CPU /RMS <- \ 59 60                     22 21 / -> AUDIO OUT   <- $4017 D2
      CPU A0 <-      CPU D0 <> \ 57 58                   24 23 / <- $4017 D3    <- $4016 D1
       CPU A1 <-      CPU D1 <> \ 55 56                 26 25 / <- $4017 D4    -> OUT0
        CPU A2 <-      CPU D2 <> \ 53 54               28 27 / -> OUT2        -> OUT1
         CPU D3 <>         VCC -- \ 51 52             30 29 / -- GND         -> AMP OUT
          CPU D4 <>      CPU A3 <- \ 49 50           32 31 / -- NC          -> $4017 CLK
           CPU D5 <>      CPU A4 <- \ 47 48         34 32 / -> CPU A11     -> CPU /RAM
            CPU D6 <>      CPU A5 <- \ 45 46       36 35 / -> CPU A10     -> CPU M2
             CPU D7 <>      CPU A6 <- \ 43 44     38 37 / -> CPU A9      -> CPU A12
             CPU A14 <-      CPU A7 <- \ 41 42   40 39 / -> CPU A8      -> CPU A13
                                        \_____________/
    

Applies also to TQFP80 version named 1818P [[1]](http://forums.nesdev.org/viewtopic.php?f=9&t=19805&p=256437#p256507)

Source: [[2]](http://forums.nesdev.org/viewtopic.php?f=9&t=19805)

Categories: [Pinouts](Category_Pinouts.xhtml)
