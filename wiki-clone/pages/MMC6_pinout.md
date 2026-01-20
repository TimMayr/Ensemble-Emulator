# MMC6 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/MMC6_pinout) | View [other pages](Special_AllPages.xhtml#MMC6_pinout)
    
    
                                            ___
                                           /   \
                                          /     \
                          (n) CPU A13 -> / 1  64 \ -> PRG A17 (r)
                                  M2 -> / 2    63 \ <- CPU A14 (n)
                     (unknown, GND) -> / 3   O  62 \ -> PRG A18 (r)
                               n/c -- / 4        61 \ -> PRG A14 (r)
                              n/c -- / 5          60 \ -> PRG A15 (r)
                             /M2 <- / 6            59 \ -> PRG A13 (r)
                 (unknown, GND) -> / 7              58 \ <- CPU A12 (nr)
                 (unknown, 5V) -> / 8                57 \ -- GND
                (unknown, 5V) -> / 9                  56 \ -- VCC/batt
                    VCC/batt -- / 10                   55 \ <- CPU A8 (nr)
                        GND -- / 11              ( )    54 \ <- CPU A7 (nr)
             (unknown, 5V) -> / 12                       53 \ <- CPU A9 (nr)
           3.3V Core Bias -- / 13                         52 \ <- CPU A6 (nr)
             (n) PPU A10 -> / 14                           51 \ <- CPU A5 (nr)
            (n) PPU A11 -> / 15                             50 \ -> PRG A16 (r)
    PPU /RD || PPU A13 <- / 16                               49 \ <- CPU A4 (nr)    Orientation:
                         /             Nintendo MMC6             \                  --------------------
                         \      Package QFP-64, 0.8mm pitch      /                     48          33
           (r) CHR A10 <- \ 17                               48 / <- CPU A3 (nr)        |          |
            (r) CHR A16 <- \ 18                             47 / -> PRG /CE (r)        .------------.
             (r) CHR A11 <- \ 19                           46 / <- CPU A2 (nr)      49-|            |-32
              (n) PPU A12 -> \ 20                         45 / <> CPU D7 (nr)          |  Nintendo  |
               (r) CHR A13 <- \ 21    ( )                44 / <- CPU A1 (nr)           |O  MMC6B   O|
                (r) CHR A12 <- \ 22                     43 / <> CPU D6 (nr)            |            |
                 (r) CHR A14 <- \ 23                   42 / <- CPU A0 (nr)          64-|o           |-17
                          GND -- \ 24                 41 / <> CPU D5 (nr)              \------------'
                      VCC/batt -- \ 25               40 / <> CPU D0 (nr)                |          |
                    (r) CHR A15 <- \ 26             39 / <> CPU D4 (nr)                 1          16
                   (n) CIRAM A10 <- \ 27           38 / -- VCC/batt
            (nr) CHR /OE, PPU /RD -> \ 28         37 / -- GND             Legend:
             (nr) CHR /CE, PPU A13 -> \ 29       36 / <> CPU D1 (nr)      ------------------------------
                        (r) CHR A17 <- \ 30     35 / <> CPU D3 (nr)       --[MMC6]-- Power
                            (n) /IRQ <- \ 31   34 / <> CPU D2 (nr)        ->[MMC6]<- MMC6 input
                          (n) /ROMSEL -> \ 32 33 / <- CPU R/W (n)         <-[MMC6]-> MMC6 output
                                          \     /                         <>[MMC6]<> Bidirectional
                                           \   /                          ??[MMC6]?? Unknown
                                            \ /                               n      NES connection
                                             V                                r      ROM chip connection
    

  * GND pins (11,24,37,57) are internally connected to each other.
  * VCC/batt pins (10,25,38,56) are internally connected to each other.
  * All pins have internal protection diodes from GND and to VCC/batt except pins 4 and 5.
  * Pins 4 and 5 measure infinite resistance to all other pins.
  * Unknown input pins (3,7,8,9,12) are labeled per connections on NES-HKROM PCB.



  
Battery Circuit from NES-HKROM PCB: 
    
    
       +------|>|----/\/\/------+------|<|-----O  NES 5V
    +  |             1 kohm     |
      ---  3V                   |
     ----- Lithium              +--------------O  MMC6 VCC/Batt
       |   2032
    -  |
       +---------------------------------------O  NES GND, MMC6 GND
    

  
3.3V Core Bias Circuit from NES-HKROM PCB: 
    
    
               NES 5V  O----/\/\/----+
                           181 ohm   |
                                     |
                                     +-----O  MMC6 3.3V Core Bias
                                     |
                           470 ohm   |
    NES GND, MMC6 GND  O----/\/\/----+
    

  
NES-HKROM (Startropics 1) BOM: 
    
    
    R1  1 kohm
    R2  181 ohm
    R3  470 ohm
    C1  22uF 6.3V Electrolytic
    C2  10nF Ceramic
    C3  10nF Ceramic
    C8  22uF 6.3V Electrolytic
    D1  Diode (0.6V Forward)
    D2  Diode (0.6V Forward)
    U1  PRG-ROM
    U2  CHR-ROM
    U3  CIC
    U4  MMC6
    Batt 2032 3V Lithium
    

Categories: [Pinouts](Category_Pinouts.xhtml)
