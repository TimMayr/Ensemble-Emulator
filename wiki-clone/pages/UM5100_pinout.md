# UM5100 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/UM5100_pinout) | View [other pages](Special_AllPages.xhtml#UM5100_pinout)

UMC UM5100: 40-pin 0.6" PDIP (Used in an unlicensed reproduction of the only [mapper 86](INES_Mapper_086.xhtml "INES Mapper 086") [game](http://bootgod.dyndns.org:7777/profile.php?id=1549)) 
    
    
                    .--\/--.
    /WRITE PULSE <- |01  40| -- +5V
             A12 <- |02  39| -> A14
              A7 <- |03  38| -> A13
              A6 <- |04  37| -> A8
              A5 <- |05  36| -> A9
              A5 <- |06  35| -> A11
              A3 <- |07  34| <- /RECORD
              A2 <- |08  33| -> A10
              A1 <- |09  32| -> /READ
              A0 <- |10  31| <> D7
              D0 <> |11  30| <> D6
              D1 <> |12  29| <> D5
              D2 <> |13  28| <> D4
              C1 -- |14  27| <> D3
              R1 -- |15  26| -> TD
          +RESET -> |16  25| -> ANG
           /PLAY -> |17  24| -> /TD
        COMPDATA -> |18  23| -> /ANG
    CLOCK DRIVER <- |19  22| <- ENVELOPE
             GND -- |20  21| -> FILTER
                    '------'
                         
    

Source: [BBS](https://forums.nesdev.org/viewtopic.php?p=203755#p203755)

Categories: [Audio](Category_Audio.xhtml), [Pinouts](Category_Pinouts.xhtml)
