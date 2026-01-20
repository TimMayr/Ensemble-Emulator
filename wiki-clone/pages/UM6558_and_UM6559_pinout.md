# UM6558 and UM6559 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/UM6558_and_UM6559_pinout) | View [other pages](Special_AllPages.xhtml#UM6558_and_UM6559_pinout)

UM6558 is PPU with additional 8 pins (CD0..CD7) that are used to output currently rendered pixel color. 
    
    
                .--\/--.
         R/W -> |01  48| -- +5V
      CPU D0 <> |02  47| -> ALE
      CPU D1 <> |03  46| <> PPU AD0
      CPU D2 <> |04  45| <> PPU AD1
      CPU D3 <> |05  44| <> PPU AD2
      CPU D4 <> |06  43| <> PPU AD3
      CPU D5 <> |07  42| <> PPU AD4
      CPU D6 <> |08  41| <> PPU AD5
      CPU D7 <> |09  40| <> PPU AD6
    *    /CS -> |10  39| <> PPU AD7
    * CPU A2 -> |11  38| -> PPU A8
    * CPU A1 -> |12  37| -> PPU A9
    * CPU A0 -> |13  36| -> PPU A10
         GND ?? |14  35| -> PPU A11
         GND ?? |15  34| -> PPU A12
         GND ?? |16  33| -> PPU A13
         GND ?? |17  32| -> /RD
    *    VCC ?? |18  31| -> /WR
    *    CLK -> |19  30| -> CD0   *
    *   /INT <- |20  29| -> CD1   *
    *   /RST -> |21  28| -> CD2   *
    *    CD7 <- |22  27| -> CD3   *
    *    CD6 <- |23  26| -> CD4   *
    *    GND -- |24  25| -> CD5   *
                '------'
    

  * = difference with respect to the common 40-pin PPU pinout



UM6559 is DAC that is capable of generating RED/GREEN/BLUE/SYNC/CHROMA/LUMA signals from of the CD0..CD7 values 
    
    
                .--\/--.
         CD4 -> |01  24| <- CD3
         CD5 -> |02  23| <- CD2
         +5V -- |03  22| <- CD1
          A1 -> |04  21| <- CD0
          A2 -> |05  20| -- GND
        LUMA <- |06  19| <- BIAS
       ?? NC -> |07  18| -> BLUE
        SYNC <- |08  17| -> GREEN
         CD6 -> |09  16| -> RED
         CD7 -> |10  15| -- +5V
         CLK -> |11  14| -> CHROMA
         GND -- |12  13| -> NC ??
                '------'
    

Sources: 

  * [Emu-Land forum post with diagram showing difference between UM6558 vs UM6538](http://www.emu-land.net/forum/index.php/topic,27910.msg1252488.html#msg1252488)
  * [Leaked short datasheet for UM6559](https://forums.nesdev.org/viewtopic.php?p=190185#p190185)
  * [Krzysiobal's reverse-engineering of UM6559 in isolation](https://forums.nesdev.org/viewtopic.php?p=248118#p248118)



Categories: [Pinouts](Category_Pinouts.xhtml)
