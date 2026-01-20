# PT8159 Pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/PT8159_Pinout) | View [other pages](Special_AllPages.xhtml#PT8159_Pinout)

The PT8159 exists as a Apoxy Blob Present along with the [PT8154](PT8154_pinout.xhtml "PT8154 pinout") (which generates IRQs and Mappes CHR ROM) in Street Fighter II by Yoko Soft 
    
    
               .--\/--.
    PRG A16 <- |01    |
    PRG A15 <- |02  19| -- VCC
     CPU D4 -> |03  18| <- PPU A13
     CPU D5 -> |04  17| <- PPU /RD
     CPU D0 -> |05  16| -> CHR /CS
     CPU D1 -> |06  15| <- CPU R/W
     CPU D2 -> |07  14| -- GND
     CPU D3 -> |08  13| <- M2
    /ROMSEL -> |09  12| -> CPU A8
    CPU A14 -> |10  11| <- CPU A13
               `------'
    

See also: 

  * <https://forums.nesdev.org/viewtopic.php?t=20419>


