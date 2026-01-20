# DIS23C01 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/DIS23C01_pinout) | View [other pages](Special_AllPages.xhtml#DIS23C01_pinout)

DAOU ROM CONTROLLER DIS23C01 Daou 245 (Mapper 156) 
    
    
                                       _____
                                      /     \                             Orientation:
                         NC (cut) -- / 1  64 \ -- NC (cut)                --------------------
                         GND (*) -- / 2    63 \ <- CPU A1                     51         33
                         CPU A2 -> / 3  (o) 62 \ <- CPU A0                     |         |
                        CPU A3 -> / 4        61 \ ?? always low               .-----------.
                       CPU A4 -> / 5          60 \ ?? always low           52-|o DAOU    o|-32
                     CPU R/W -> / 6            59 \ -- VCC                    |  DIS23C01 |
                        GND -- / 7              58 \ ?? alway high (cut)   64-|o DaOU 245 |-20
                   CPU A14 -> / 8                57 \ ?? always high          \-----------'
                      GND -- / 9                  56 \ ?? always high          |         |
                 CIR A10 <- / 10                   55 \ ?? always high       01         19
                PRG /CE <- / 11                     54 \ ?? always high
     /IRQ (always low) <- / 12                       53 \ ?? always high
              PRG A14 <- / 13                         52 \ ?? always high
             PRG A15 <- / 14      DAOU DIS23C01       51 / ?? always high
            PRG A16 <- / 15      Package QFP-64      50 / <- CPU D7
           PRG A17 <- / 16        1.00mm pitch      49 / <- CPU D6
          PRG A18 <- / 17         (20mm × 14mm)    48 / <- CPU D5
         PRG A19 <- / 18                          47 / <- CPU D4
        PRG A20 <- / 19                          46 / <- CPU D3
        CHR A10 <- \ 20                         45 / <- CPU D2
             GND -- \ 21                       44 / <- CPU D1
          CHR A11 <- \ 22                     43 / <- CPU D0
           CHR A12 <- \ 23                   42 / -- GND       
            CHR A13 <- \ 24                 41 / <- M2         
                 VCC -- \ 25               40 / <- PPU A12     
    always high (cut) ?? \ 26             39 / <- PPU A11      
               CHR A14 <- \ 27           38 / <- PPU A10       
                CHR A15 <- \ 28         37 / -- GND            
                 CHR A16 <- \ 29       36 / <- CPU /ROMSEL     
                  CHR A17 <- \ 30     35 / -- GND            
                   CHR A18 <- \ 31   34 / -- VCC             
                    CHR A19 <- \ 32 33 / ?? always high                 
                                \     /                      
                                 \   /                       
                                  \ /                        
                                   V     
    

  
notes: 

  * pins 1, 64 - no internal connection; externally cut from the package (NC)
  * pin 2 - no external con, internally connected to other ground pins
  * pins 26, 58 - externally cut
  * PRG A pins reflect the bank being set only when M2 is high; when M2 is low, they always go high
  * CIRAM A10 is not connected to mapper (premanently wired to GND) on this board (Koko Adventure)
  * IRQ pin seems to be non-functional (it is always low)
  * This board uses weird pinouts for PRG/CHR eproms (32 holes, pin 2 and 24 both shorted and connected to PRG/CHR A16 respectively), but eproms have pins 24 cut and not soldered to anything (floating)
  * A lot of not used pins (but showing signs of internal connection) suggests that this chip maybe some kind of PLD instead of ASIC (just like BF909x from codemasters)



Source: [[1]](https://forums.nesdev.org/viewtopic.php?t=24076)] 

Categories: [Pinouts](Category_Pinouts.xhtml)
