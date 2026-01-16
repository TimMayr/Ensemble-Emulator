# Nantettatte!! Baseball pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Nantettatte%21%21_Baseball_pinout) | View [other pages](Special_AllPages.xhtml#Nantettatte___Baseball_pinout)
    
    
                            Deck | CART | Deck
                                 /------.
                           M2 -> |02  01| -- GND
    SS4 PRG /CE | SS4 PRG A17 -> |04  03| <- SS4 WRAM /CE | CPU R/W | $F000.4
                       CPU D3 <> |06  05| <> CPU D2
                       CPU D4 <> |08  07| <> CPU D1
                       CPU D5 <> |10  09| <> CPU D0
                       CPU D6 <> |12  11| <- CPU A0
                       CPU D7 <> |14  13| <- CPU A1
                      CPU A10 -> |16  15| <- CPU A2
                  SS4 PRG A16 -> |18  17| <- CPU A3
                      CPU A11 -> |20  19| <- CPU A4
                       CPU A9 -> |22  21| <- CPU A5
                       CPU A8 -> |24  23| <- CPU A6
                      CPU A13 -> |26  25| <- CPU A7
                  SS4 PRG A14 -> |28  27| <- CPU A12 
                          +5V -- |30  29| <- SS4 PRG A15 
                                 \------' 
                          2x15 pin 0.1" edge connector
                  
    

  * Pin numbers correspond to the markings on edge connctor
  * Even pins are on the label side of external cartridge (02=leftmost)



Source: [[1]](https://nescartdb.com/profile/view/2265/nantettatte-baseball), [[2]](https://na6ko.hatenadiary.jp/entry/20130123/p1)

Categories: [Pinouts](Category_Pinouts.xhtml)
