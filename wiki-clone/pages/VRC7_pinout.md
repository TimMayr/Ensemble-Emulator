# VRC7 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VRC7_pinout) | View [other pages](Special_AllPages.xhtml#VRC7_pinout)

Konami [VRC7](VRC7.xhtml "VRC7"): 48-pin 0.6" PDIP marked: "VRC VII 053982" (canonically **iNES Mapper 085**) 
    
    
                      .---\/---.
    (PPU /RD) OR A -> | 01  48 | -> OR Y (NC)
    (PPU A13) OR B -> | 02  47 | <- M2
               GND -- | 03  46 | -> PRG RAM /CE
               R/W -> | 04  45 | <- /ROMSEL
              /IRQ <+ | 05  44 | -> PRG ROM /CE
         CIRAM A10 <- | 06  43 | -> Audio Out
            CPU D0 <> | 07  42 | -- +5V
            CPU D1 <> | 08  41 | -> CHR A17
            CPU D2 <> | 09  40 | -> CHR A16
            CPU D3 <> | 10  39 | -> CHR A15
            CPU D4 <> | 11  38 | -> CHR A14
            CPU D5 <> | 12  37 | -> CHR A13
            CPU D6 <> | 13  36 | -> CHR A12
            CPU D7 <> | 14  35 | -> CHR A11
            /DEBUG -> | 15  34 | -> CHR A10
            CPU A5 -> | 16  33 | <- PPU A12
        Crystal X2 -> | 17  32 | <- PPU A11
        Crystal X1 <- | 18  31 | <- PPU A10
            CPU An -> | 19  30 | -- +5V
           PRG A13 <- | 20  29 | <- CPU A14
           PRG A14 <- | 21  28 | <- CPU A13
           PRG A15 <- | 22  27 | <- CPU A12
           PRG A16 <- | 23  26 | -> PRG A18
               GND -- | 24  25 | -> PRG A17
                      `--------'
     
     01,02: The OR gate on pins 1, 2, and 48 was intended for a 28-pin 
             128KiB CHR ROM, but TTA2 doesn't use it. Perhaps it's too slow?
             This can be considered a general-purpose OR gate when not in debug mode.
     17,18: missing on TTA2, 3.58MHz [ceramic resonator](https://en.wikipedia.org/wiki/Ceramic_resonator "wikipedia:Ceramic resonator") on LP
             Note: LP uses a 3-pin resonator, presumably with built-in loading caps.
             An equivalent 2-pin quartz resonator will require loading caps approx. 18pF.
                              _
                            || || 3.579545 MHz
             (VRC7.17) --+--|| ||--+-- (VRC7.18)
                         |  ||_||  |
                        ---       ---
                   18pF ---       --- 18pF
                         |         |
                         +----+----+
                              |
                             GND
             (Your resonator's datasheet may make a suggestion for this value.)
     19: A3 on TTA2, A4 on LP
    
    Expansion audio mixing circuit:[[1]](https://forums.nesdev.org/viewtopic.php?p=41523#p41523)
       The audio is mixed through a ceramic daughterboard with a 9 pin SIP connector. Pins 4-6 are unconnected,
        since they were primarily used for laser trimming the resistors during production.
    
               From 2A03---o--+-----------+
               (Cart.45)      |           |
               To RF-------o--|           |
               (Cart.46)      |           |
               GND---------o--|           |
                              |           |
               NC          o--|           |
               (Mix.4)        |           |
               NC          o--|           |
               (Mix.5)        |           |
               NC          o--|           |
               (Mix.6)        |           |
               GND---------o--|           |
                              |           |
               From VRC7---o--|           |
               (VRC7.43)      |           |
               VCC---------o--+-----------+
     
    Equivalent circuit schematic:
       The daughterboard uses a dual op-amp for amplification. Only one amplifier is used.
    
               From 2A03----------------------------------------------------/\/\/---+
               (Cart.45)                                          _         3.9K    |
                                                                 | \                |
               From VRC7---+---/\/\/---+--------+----------------|+  \              |
               (VRC7.43)   /   27K     |        /                |IC.A>-+---/\/\/---+---To RF
                           \          ---       \             +--|-  /  |   2.2K        (Cart.46)
                           / 2.2K     --- 4.7n  / 33K         |  |_/    |
                           |           |        |             |  240K   |
                           +-----------+--------+     NC------+--/\/\/--+
                           |                          (Mix.5) /         |
                           | VCC---+-------+                  \         |
                           |  |  _ |       |                  / 4.3K    |
                           |  | | \|       |          NC------+         +---NC
                           |  +-|+  \     ---         (Mix.6) |         /   (Mix.4)
                           |  | |IC.B>--x --- 0.1u           ---        \
                           |  +-|-  /      |                 --- 0.22u  / 910
                           |    |_/|       |                  |         |
               GND---------+-------+-------+------------------+---------+
    

When the DEBUG pin is tied low, several pins gain function: 
    
    
                      .---\/---.
    (PPU /RD) OR A -> | 01  48 | -> OR Y (NC)
              **A(w)** -> | 02  47 | <- **A(y)**
               GND -- | 03  46 | -> WRAM /CS
              **A(x)** -> | 04  45 | <- /ROMSEL
              /IRQ <+ | 05  44 | -> PRG /CS
                      :        :
           PRG A13 <- | 20  29 | <- CPU A14
           PRG A14 <- | 21  28 | <- **A(z)**
           PRG A15 <- | 22  27 | <- **/IC**
           PRG A16 <- | 23  26 | -> PRG A18
               GND -- | 24  25 | -> PRG A17
                      `--------'
    

For more information on debug mode, see [VRC7 audio: Debug Mode](VRC7_audio.xhtml#Debug_Mode "VRC7 audio")

  
See also: 

  * Decap'ed VRC7: <https://siliconpr0n.org/archive/doku.php?id=digshadow:konami:vrc_vii_053982>
  * Quietust's annotations for the above decap: <https://www.qmtpro.com/~nes/chipimages/#vrc7>
  * More commentary on VRC7 DEBUG mode: <https://siliconpr0n.org/archive/doku.php?id=vendor:yamaha:opl2#vrc7_debug_mode>



Categories: [Pinouts](Category_Pinouts.xhtml)
