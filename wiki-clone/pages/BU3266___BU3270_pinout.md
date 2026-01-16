# BU3266 / BU3270 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/BU3266_/_BU3270_pinout) | View [other pages](Special_AllPages.xhtml#BU3266___BU3270_pinout)

Custom Nintendo PIO chip found in the New Famicom (HVC-101). This replaces the U3 (139) and the U7/U8 (368). 
    
    
              .---v---.  
    INV 3I -> |01   32| -- +5V
    INV 3O <- |02   31| ?? NC
        M2 -> |03   30| ?? NC
       A15 -> |04   29| -> /ROMSEL
       A14 -> |05   28| -> PPU /CE
       A13 -> |06   27| -> CPU RAM /CE
     P1 D0 -> |07   26| <- INV 2I
     P0 D0 -> |08   25| -> INV 2O
     P1 D1 -> |09   24| -> CPU D0
     P0 D1 -> |10   23| -> CPU D1
     P1 D2 -> |11   22| <- /INP0 ($4016 /RD)
     P0 D2 -> |12   21| <- /INP1 ($4017 /RD)
     P1 D3 -> |13   20| -> CPU D2
     P1 D4 -> |14   19| -> CPU D3
       GND -- |15   18| -> CPU D4
    INV 1I -> |16   17| -> INV 1O
              '-------'  
    

Notes: 

  * INV 1I/O (16-17) = Audio Inverter
  * INV 2I/O (25-26) = PA13 Inverter
  * INV 3I/O (01-02) = unused



Sources: 

  * [nes_pio_pinout.txt](http://nesdev.org/nes_pio_pinout.txt)
  * [BBS - AV Famicom JIO Chip Replacement Idea](https://forums.nesdev.org/viewtopic.php?t=16764)



Categories: [Pinouts](Category_Pinouts.xhtml)
