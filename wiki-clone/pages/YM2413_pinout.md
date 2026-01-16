# YM2413 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/YM2413_pinout) | View [other pages](Special_AllPages.xhtml#YM2413_pinout)

Yamaha YM2413 or ??? K-663A: 18-pin 0.3" PDIP (used in [Mapper 515](NES_2_0_Mapper_515.xhtml "NES 2.0 Mapper 515")). Also used as a cheaper and similar-sounding substitute for the [VRC7](VRC7_audio.xhtml "VRC7 audio") in the [TNS-HFX4](http://www2s.biglobe.ne.jp/~tns/nr26730801.html). 
    
    
            .--\/--.
     GND -- |01  18| <> D1
      D2 <> |02  17| <> D0
      D3 <> |03  16| -- +5V
      D4 <> |04  15| -> RHYTHM OUT
      D5 <> |05  14| -> MELODY OUT
      D6 <> |06  13| <- /RESET
      D7 <> |07  12| <- /CE
     XIN -> |08  11| <- R/W
    XOUT <- |09  10| <- A0
            '------'
    

Sources: [[1]](https://forums.nesdev.org/viewtopic.php?t=16124), [[2]](http://www.alldatasheet.pl/datasheet-pdf/pdf/1135013/YAMAHA/YM2413.html)

Categories: [Audio](Category_Audio.xhtml), [Pinouts](Category_Pinouts.xhtml)
