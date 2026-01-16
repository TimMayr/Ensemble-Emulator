# KC5373B-010 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/KC5373B-010_pinout) | View [other pages](Special_AllPages.xhtml#KC5373B_010_pinout)
    
    
                  .--v--.
    CIC-TO-PAK -> |01 16| (*)
        PPU-A0 -> |02 15| --[C]-> CIC-TO-MB
           VCC -- |03 14| <- \___ one of those pins connects to CIC-CLK
           VCC -- |04 13| <- /
           GND -- |05 12| -- VCC
              +-- |06 11| --+
              +-- |07 10| --+
              +-- |08 09| --+
              |   '-----'   |
              +-------------+--[charge pump] -> CIC-RST
    
    
    
    *Pin 16 is either tied to pin 15 or left floating, depending on the board
    

This chip was found in many NES NTDEC games and acted like CIC stun [[1]](https://forums.nesdev.org/viewtopic.php?p=246918#p246918), [[2]](https://forums.nesdev.org/viewtopic.php?p=204138#p204138)

It's exact behviour is unknown, but PPU-A0 is probably a source of high frequency signal, whose current is amplified and available on pins 6-11. Next, going through charge pump, a negative voltage of around -4V is produced which disables NES' internal CIC. 

The role of other signals (CIC-TO-PAK, CIC-TO-MB, CIC-CLK) is open question. 

Categories: [Pinouts](Category_Pinouts.xhtml)
