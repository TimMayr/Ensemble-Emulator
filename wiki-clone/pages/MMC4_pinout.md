# MMC4 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/MMC4_pinout) | View [other pages](Special_AllPages.xhtml#MMC4_pinout)

Nintendo [MMC4](MMC4.xhtml "MMC4"): 44-pin TQFP 
    
    
                          / \
                         / O \
                   ? ?? /01 44\ <- CPU R/W
            /ROMSEL -> /02   43\ <- CPU D0
           PPU /RD -> /03     42\ <- CPU D1
           PPU A3 -> /04       41\ <- CPU D2
               ? ?? /05         40\ <- CPU D3
            GND -- /06         \ 39\ <- CPU D4
        PPU A4 -> /07           \ 38\ -> PRG /CE
       PPU A5 -> /08         \  _\ 37\ -> PRG A17
      PPU A6 -> /09         \ \|    36\ -> PRG A16
     PPU A7 -> /10           \ \     35\ -> PRG A15
         ? ?? /11         \  _\       34\ -> PRG A14
    PPU A8 -> \12       /\ \|         33/ -- Tied to Pin 32
     PPU A9 -> \13        \ \        32/ <- CPU A12
     PPU A10 -> \14        \        31/ <- CPU A13
      PPU A11 -> \15  \/\  /       30/ <- CPU A14
       PPU A12 -> \16  \ \        29/ <- M2
        PPU A13 -> \17  \        28/ -- GND
             GND -- \18         27/ -- VCC
          CHR A12 <- \19       26/ -> VRAM A10              Legend:
           CHR A13 <- \20     25/ -> PRG RAM +CE             Input: -> [MMC4] <-
            CHR A14 <- \21   24/ -> CHR A16                 Output: <- [MMC4] ->
             CHR A15 <- \22 23/ -> PRG RAM /CE               Power: -- [MMC4] --
                         \   /	                       Unknown: ?? [MMC4] ??
                          \ /
    

  * Source: [INL on the forum](http://forums.nesdev.org/viewtopic.php?p=100951#p100951)



Categories: [Pinouts](Category_Pinouts.xhtml)
