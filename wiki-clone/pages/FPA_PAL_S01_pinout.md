# FPA-PAL-S01 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/FPA-PAL-S01_pinout) | View [other pages](Special_AllPages.xhtml#FPA_PAL_S01_pinout)

**FPA-PAL-S01** and **FPA-92-S01** : 22-pin 0.4" PDIP ([Four Score](Four_player_adapters.xhtml "Four Score") and Famicom Four-Player Adapter) 

Present in NES FOUR SCORE, both PAL and NTSC; and in HORI 4 PLAYERS ADAPTOR 
    
    
               .---v---.
     P2 CLK <- |01   22| -- +5V
     P2 D0  -> |02   21| <- /4P (LOW) or 2P (HIGH)
     P3 CLK <- |03   20| -> P1 CLK
     P3 D0  -> |04   19| <- P1 D0
     P4 CLK <- |05   18| -> P1,P2,P3,P4 STROBE (BUFFERED)
     P4 D0  -> |06   17| <- STROBE
    TURBO B -> |07   16| -> $4016 D0
    TURBO A -> |08   15| <- $4016 CLK
      XTAL1 -> |09   14| -> $4017 D0
      XTAL2 <- |10   13| <- $4017 CLK
        GND -- |11   12| <- SIGNATURE SELECT (*)
               `-------`
    
    PIN 12 | $4016 reads 16-24  | $4017 reads 16-24
     GND   |       $10          |        $20
     +5V   |       $20          |        $10
    

Source: [[1]](https://forums.nesdev.org/viewtopic.php?p=238272#p239781)

Categories: [Pinouts](Category_Pinouts.xhtml)
