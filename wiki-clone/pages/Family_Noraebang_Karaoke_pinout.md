# Family Noraebang Karaoke pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Family_Noraebang_Karaoke_pinout) | View [other pages](Special_AllPages.xhtml#Family_Noraebang_Karaoke_pinout)

Family Noraebang Karaoke: 45 pin card expansion ([iNES 2.0 Mapper 515](NES_2_0_Mapper_515.xhtml "NES 2.0 Mapper 515")) 
    
    
                .------.
     CPU R/W <- |23  01| -- GND
         +5V -- |24  02| -- GND
     CPU A7  <- |25  03| -- GND
     CPU A12 <- |26  04|    NC
     CPU A5  <- |27  05| -- +5V
     CPU A6  <- |28  06| -> PRG A18
     CPU A3  <- |29  07| -- GND
     CPU A4  <- |30  08| -> PRG A17
     CPU A1  <- |31  09| -> PRG A20
     CPU A2  <- |32  10| -> PRG A16
     CPU D0  -> |33  11| -> PRG A19
     CPU A0  <- |34  12| -> PRG A15
     CPU D2  -> |35  13| -- VCC
     CPU D1  -> |36  14| -> CPU A13
        GND  -- |37  15| -> PRG A14
        GND  -- |38  16| -> CPU A9
     CPU D3  -> |39  17| -> CPU A8
        GND  -- |40  18| <- /IRQ
     CPU D5  -> |41  19| -> CPU A10
     CPU D4  -> |42  20| -> PRG A21
     CPU D7  -> |43  21| -> M2
     CPU D6  -> |44  22| -> CPU /ROMSEL
    CPU A11  <- |45    |
                '------'
                 ^   ^----- bottom side (this row is shifted by half of pin width)
                 +--------- label side
    

Source: [[1]](https://forums.nesdev.org/viewtopic.php?f=9&t=16124&sid=d372720780756a42eca14f75373fdef9&start=45#p213771)

Categories: [Pinouts](Category_Pinouts.xhtml)
