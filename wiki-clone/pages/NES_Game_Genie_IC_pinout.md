# NES Game Genie IC pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_Game_Genie_IC_pinout) | View [other pages](Special_AllPages.xhtml#NES_Game_Genie_IC_pinout)

See also: [Game Genie](Game_Genie.xhtml "Game Genie")
    
    
                      .---\/---.  
        CC PRG /CE <- | 01  48 | <- PPU A13 on NES
     Genie ROM /CE <- | 02  47 | -> CHR A13 on CC
           /ROMSEL -> | 03  46 | <- PPU /RD
           CPU R/W -> | 04  45 | <- PPU  A2
           CPU  A0 -> | 05  44 | <- PPU  A4
           CPU  A1 -> | 06  43 | <- PPU  A5
           CPU  A2 -> | 07  42 | <- PPU  A6
           CPU  A3 -> | 08  41 | <- PPU  A7
           CPU  A4 -> | 09  40 | <> CPU  D0
           CPU  A5 -> | 10  39 | <> CPU  D1
           CPU  A6 -> | 11  38 | <> CPU  D2
               GND -- | 12  37 | <> CPU  D3
           CPU  A7 -> | 13  36 | -- +5V
           CPU  A8 -> | 14  35 | <> CPU  D4
           CPU  A9 -> | 15  34 | <> CPU  D5
           CPU A10 -> | 16  33 | <> CPU  D6
           CPU A11 -> | 17  32 | <> CPU  D7
           CPU A12 -> | 18  31 | -> PPU  D7
           CPU A13 -> | 19  30 | ?? NC
           CPU A14 -> | 20  29 | -> PPU  D6
            /RESET -- | 21  28 | -> PPU  D5
                NC ?? | 22  27 | -> PPU  D4
           PPU  D0 <- | 23  26 | -> PPU  D3
           PPU  D1 <- | 24  25 | -> PPU  D2    
                      '--------'	 
    

Pins 31–35 and 37–40 are connected to the CPU `Dx` lines via 200-ohm resistors. Their function is to prevent a bus conflict. 

`CC M2` is pulled low by resistor-diode logic when `Genie ROM /CE` is low. 

`/RESET` only detects power-on reset, not reset-button reset. 

Pin 30 goes to an unpopulated location for a capacitor. 

## References

  * [NESDev Forums – _Game Genie tracing/diagramming_](http://forums.nesdev.org/viewtopic.php?t=12777)



Categories: [Pinouts](Category_Pinouts.xhtml)
