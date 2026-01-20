# V7021 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/V7021_pinout) | View [other pages](Special_AllPages.xhtml#V7021_pinout)

Sony/ASCII V7021: 28-pin 0.4" PDIP (Used in the French NES) 
    
    
                  .--\/--.
           Gnd -- |01  28| <- Feedback clamp time constant
          Sync <- |02  27| <- Video
         Burst <- |03  26| <> Pedestal clamp time constant
        ACK TC <> |04  25| <> Chroma normalization time constant
        TP adj -> |05  24| <- Chroma
           Vcc -- |06  23| <- Chroma adj
       V phase <- |07  22| -- Vcc
           Vcc -- |08  21| -> Chroma
        APC TC <> |09  20| <- Delay Line Amplifier bias
       Hue Adj -> |10  19| <- Delay Line Amplifier
            X2 -> |11  18| -> Blue
            X1 <- |12  17| -> Green
           Gnd -- |13  16| -> Red
    subcarrier <- |14  15| -- Gnd
                  '------'
    

  * ACK TC: "Auto Color Killer Time Constant"
  * V phase: indicates whether PAL colorburst V component is inverted
  * APC TC: "Auto Phase Control Time Constant"



See also: 

  * [Reverse-engineered schematic as used in French NES](https://forums.nesdev.org/viewtopic.php?t=18861)



Categories: [Pinouts](Category_Pinouts.xhtml)
