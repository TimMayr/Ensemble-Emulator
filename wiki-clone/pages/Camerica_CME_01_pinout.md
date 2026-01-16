# Camerica CME-01 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Camerica_CME-01_pinout) | View [other pages](Special_AllPages.xhtml#Camerica_CME_01_pinout)

Camerica CME-01: 0.6" 28-pin PDIP ([Mapper 71](INES_Mapper_071.xhtml "INES Mapper 071") or [Mapper 232](INES_Mapper_232.xhtml "INES Mapper 232") depending on PCB) 
    
    
                                         .---v---.
    (R)  $C000.2 OR CPU-A14 (PRG A16) <- |01   28| -- VCC
    (R)  $C000.1 OR CPU-A14 (PRG A15) <- |02   27| -> $C000.3 OR CPU-A14 (PRG A17) (R)
                       (R,N)  CPU-A12 -> |03   26| -> $C000.0 OR CPU-A14 (PRG A14) (R)
                         (N)  CPU-A14 -> |04   25| <- CPU A13 (R,N)
                         (N) CPU /RMS -> |05   24| -> PRG /CE (R)
                         (N)  CIC+RST -> |06   23| -> PRG /OE (R)
                         (N)  CPU R/W -> |07   22| <> CPU D7  (R,N)
                       (n/c)  $8000.4 <- |08   21| <> CPU D0  (R,N)
                       (n/c)  $8000.2 <- |09   20| <> CPU D6  (R,N)
                       (n/c)  $8000.3 <- |10   19| <> CPU D1  (R,N)
                               MOSFET <- |11   18| <> CPU D5  (R,N)
                         10MHz CLK IN -> |12   17| <> CPU D2  (R,N)
                        10MHz CLK OUT <- |13   16| <> CPU D4  (R,N)
                                  GND -- |14   15| <> CPU D3  (R,N)
                                         '-------'
    

The data bus injects a NOP slide for a fixed amount of time, relying on using the MOSFET connected to the MOSFET output to brown out the NES's power supply and crash the CIC. 

  * Appears in _Cosmic Spacehead_[[1]](http://bootgod.dyndns.org:7777/profile.php?id=4588) and _Super Adventure Quests_ [[2]](https://www.jammarcade.net/tag/codemasters/)
  * Source: [Krzysiobal](https://forums.nesdev.org/viewtopic.php?f=9&t=19808)



Categories: [Pinouts](Category_Pinouts.xhtml)
