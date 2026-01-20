# Aladdin deck enhancer pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Aladdin_deck_enhancer_pinout) | View [other pages](Special_AllPages.xhtml#Aladdin_deck_enhancer_pinout)
    
    
                     Deck | CART | Deck
                           ------ 
                    NC    |01  36| -- +5V
                    NC    |02  35| <- $C000.2 OR CPU A14
    $C000.3 OR CPU A14 -> |03  34| <- $C000.1 OR CPU A14
                    NC    |04  33| <- ROM /OE
                    NC    |05  32| <- ROM /CE
                CPU A0 -> |06  31| -> CPU D0
                CPU A1 -> |07  30| -> CPU D1
                CPU A2 -> |08  29| -> CPU D2
                CPU A3 -> |09  28| -> CPU D3
                CPU A4 -> |10  27| -> CPU D4
                CPU A5 -> |11  26| -> CPU D5
                CPU A6 -> |12  25| -> CPU D6
                CPU A7 -> |13  24| -> CPU D7
                CPU A8 -> |14  23| <- $C000.0 OR CPU A14
                CPU A9 -> |15  22| <- CPU A13
               CPU A10 -> |16  21| <- CPU A12
               CPU A11 -> |17  20| <- $8000.4
                   GND -- |18  19| <- $8000.3
                           ------ 
                   2x18 pin 0.1" edge connector
    

Notes: 

  * Pins 01-18 are on the label side (01 = leftmost)
  * ROM /OE is logically `(not CPU R/W) or CPU /ROMSEL`
  * ROM /CE is grounded in Aladdin Deck Enhancer 1.1 and driven by PIC16C54 in 2.0
  * Single game cartridges wire ROM as: PRG A17 = $C000.3, PRG A16 = $C000.2, PRG A15 = $C000.1, PRG A14 = $C000.0
  * Quattro cartridges wire ROM as: PRG A17 = $8000.3, PRG A16 = $8000.4, PRG A15 = $C000.1, PRG A14 = $C000.0
  * All Aladdin Deck Enhancer games use vertical mirroring, as there is no mirroring control on this port



Source: 

  * [BBS - Aladdin Deck Enhancer / CCU_v2.00 CF30288 pinouts](http://forums.nesdev.org/viewtopic.php?f=9&t=19781&p=273471#p273471)



Categories: [Pinouts](Category_Pinouts.xhtml)
