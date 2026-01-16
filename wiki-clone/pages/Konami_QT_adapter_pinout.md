# Konami QT adapter pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Konami_QT_adapter_pinout) | View [other pages](Special_AllPages.xhtml#Konami_QT_adapter_pinout)
    
    
                        +-----+
                 GND -- |01 21| -- VCC
              CPU A6 -> |02 22| <- CPU A8
             CPU A11 -> |03 23| <- CPU A9
              CPU A4 -> |04 24| <- CPU A5
              CPU A3 -> |05 25| <- CPU A10
              CPU A2 -> |06 26| <> CPU D7
             CPU A12 -> |07 27| <- CPU A7
              CPU D6 <> |08 28| <- CPU R/W
              CPU D5 <> |09 29| <- CPU A1
              CPU D4 <> |10 30| <- CPU A0
                 GND -- |11 31| -- GND
              CPU D3 <> |12 32| <> CPU D0
              CPU D2 <> |13 33| <> CPU D1
             PRG A13 -> |14 34| <- PRG A14
             PRG A15 -> |15 35| <- PRG A16
    EXT PRG ROM0 /CE -> |16 36| <- PRG A17
    EXT PRG ROM1 /CE -> |17 37| <- EXT PRG ROM2 /CE
                      ? |18 38| <- PRG RAM A12
    EXT PRG RAM /CE  -- |19 39| ?    
                 VCC -- |20 40| -- VCC
                        +-----+
    

**Notes:**

  * Pins 1-20 are on label side



**Source:**

  * [BBS](http://forums.nesdev.org/viewtopic.php?f=2&t=19237&start=15#p242297)



Categories: [Pinouts](Category_Pinouts.xhtml)
