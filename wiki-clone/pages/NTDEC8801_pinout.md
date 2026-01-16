# NTDEC8801 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NTDEC8801_pinout) | View [other pages](Special_AllPages.xhtml#NTDEC8801_pinout)
    
    
                .---\/---.
         VCC -- | 01  16 | -- GND
     /WRA000 -> | 02  15 | ?? NC
          NC ?? | 03  14 | ?? NC
      /RESET -> | 04  13 | -> A13
     /WR8000 -> | 05  12 | <- CPU-/ROMSEL
     /WR8000 -> | 06  11 | <- CPU-A13
    CNTRESET <- | 07  10 | ?? NC
          NC ?? | 08  09 | -- +5V
                '--------'
                 NTDEC8801
    

This chip was used in at least one of SMB2J pirate ports (mapper 40)[[1]](https://forums.nesdev.org/viewtopic.php?f=9&t=17280&start=0#p217200). There exist exactly the same SMB2J port without this chip but with additional 74's IC [[2]](https://forums.nesdev.org/viewtopic.php?f=9&t=19744#p246867), so NTDEC8801 is aggregate the function of: 

  * flip flop with set/reset: CNTRESET <= 0 when /WRA000=0 else '1' when /WR8000=0
  * multiplexer with negation: A13 <= CPU-A13 when CPU-!ROMSEL! = 0 else not CPU-A13
  * and something more that is not used (not wired)



Categories: [Pinouts](Category_Pinouts.xhtml)
