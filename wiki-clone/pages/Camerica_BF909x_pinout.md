# Camerica BF909x pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Camerica_BF909x_pinout) | View [other pages](Special_AllPages.xhtml#Camerica_BF909x_pinout)
    
    
                      .--v--.                   |                   .--v--.                   |                   .--v--.
               +5V -- |01 20| <- CPU A14        |            +5V -- |01 20| <- CPU A14        |            +5V -- |01 20| <- CPU A14    
           CPU R/W -> |02 19| <- CPU A13        |        CPU R/W -> |02 19| <- CPU A13        |        CPU R/W -> |02 19| <- CPU A13
           PRG /CE <- |03 18| -> $C000.1 OR A14 |        PRG /CE <- |03 18| -> $C000.1 OR A14 |        PRG /CE <- |03 18| -> $C000.1 OR A14 
    $C000.0 OR A14 <- |04 17| -> $C000.2 OR A14 | $C000.0 OR A14 <- |04 17| -> $8000.4        | $C000.0 OR A14 <- |04 17| -> $8000.4  
                M2 -> |05 16| <- ??             |             M2 -> |05 16| <- ??             |             M2 -> |05 16| -> $C000.2 OR A14 
            CPU A0 -> |06 15| <- CPU /ROMSEL    |         CPU A0 -> |06 15| <- CPU /ROMSEL    |         CPU A0 -> |06 15| <- ?? 
            CPU D0 -> |07 14| -> $C000.3 OR A14 |         CPU D0 -> |07 14| -> $8000.3        |         CPU D0 -> |07 14| <- CPU /ROMSEL
            CPU D1 -> |08 13| -> CIC stun       |         CPU D1 -> |08 13| -> CIC stun       |         CPU D1 -> |08 13| -> CIC stun               
            CPU D2 -> |09 12| <- CPU D4         |         CPU D2 -> |09 12| <- CPU D4         |         CPU D2 -> |09 12| <- CPU D4
               GND -- |10 11| <- CPU D3         |            GND -- |10 11| <- CPU D3         |            GND -- |10 11| <- CPU D3
                      '-----'                   |                   '-----'                   |                   '-----'
                      BF9093                    |                   BF9096                    |                   BF9097
    
           $C000.3 OR CPU A14 = PRG A17         |        $8000.3            = PRG A17         |       $8000.4            = CIRAM A10 in Firehawk
           $C000.2 OR CPU A14 = PRG A16         |        $8000.4            = PRG A16         |       $C000.2 OR CPU A14 = PRG A16 
           $C000.1 OR CPU A14 = PRG A15         |        $C000.1 OR CPU A14 = PRG A15         |       $C000.1 OR CPU A14 = PRG A15
           $C000.0 OR CPU A14 = PRG A14         |        $C000.0 OR CPU A14 = PRG A14         |       $C000.0 OR CPU A14 = PRG A14
    

Notes: 

  * All chips are 20-pin 0.3" PDIP
  * Register $8000 is at $8000-$BFFF
  * Register $C000 is at $C000-$FFFF
  * ?? is unknown input, tied to GND in all cartridges
  * BF9093 is used in 64 kB / 128 kB / 256 kB NES & 256 kB Famicom singles ([iNES Mapper 071](INES_Mapper_071.xhtml "INES Mapper 071") [submapper 0](NES_2_0_submappers.xhtml#071:_0 "NES 2.0 submappers"))
  * BF9096 is used in NES/Famicom Quattro multicarts ([iNES Mapper 232](INES_Mapper_232.xhtml "INES Mapper 232"))
  * BF9097 is used in Firehawk ([iNES Mapper 071](INES_Mapper_071.xhtml "INES Mapper 071") [submapper 1](NES_2_0_submappers.xhtml#071:_1_Fire_Hawk "NES 2.0 submappers")) and in 64 kB Famicom singles (pins 17, 16 are not wired here)
  * CIC stun latches inverse of CPU A0 when writing at $E000-$FFFF
  * Pinout from [Kevtris](http://kevtris.org/mappers/camerica/) is not accurate



Source: [[1]](https://forums.nesdev.org/viewtopic.php?t=21092)
