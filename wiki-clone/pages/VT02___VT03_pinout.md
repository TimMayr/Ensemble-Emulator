# VT02 / VT03 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VT02_/_VT03_pinout) | View [other pages](Special_AllPages.xhtml#VT02___VT03_pinout)
    
    
                                 _______
                                /    91 \ -- GND
                       /CS0 <- / 1    90 \ -> CPU A5
                      /CS1 <- / 2      89 \ <> CPU D6
                   CPU D5 <> / 3        88 \ -> CPU A6
                 CPU A14 <- / 4          87 \ <> CPU D7
                 CPU D3 <> / 7            86 \ -> CPU A7
                CPU A2 <- / 8              85 \ -> CPU A14
               CPU D2 <> / 9                84 \ -> CPU A8
              CPU A1 <- / 10                 83 \ -> CPU A13
                GND -- / 11                   82 \ -> CPU A9
            CPU D1 <> / 12                     81 \ -> CPU A12
           CPU A0 <- / 13                       80 \ -> CPU A10
          CPU D0 <> / 14                         79 \ <- CPU CLK (1.8MHz)
            R/W <- / 15                           78 \ -- VCC
       /ROMSEL <- / 16                             77 \ -> CPU A11
         /IRQ -> / 17                               76 \ <- $4016.0
     PPU R/W <- / 18                                 75 \ <> IOCLK1
        VDE <- / 19                                   74 \ <> IO2
        VCC -- \ 20                                    73 \ -> IO1
         ERS <- \ 21                                    72 \ <- $4017.4
    CIRAM A10 -> \ 22                                    71 \ -> IO0
        PPU A6 <- \ 23                                    70 \ <- $4017.3
         PPU A7 <- \ 24                                    69 \ <- $4016.1
          PPU A5 <- \ 25                                    68 \ <- $4017.2
           PPU A8 <- \ 26                                    67 \ <> IOCLK0
            PPU A4 <- \ 27                                    66 \ <- $4017.1
             PPU A9 <- \ 28                                    65 \ <- /FH
              PPU A3 <- \ 29                                   64 / -- VCC
              PPU A10 <- \ 30                                 63 / <- /BTS
                PPU A2 <- \ 31                               62 / <- $4017.0
                PPU A11 <- \ 32                             61 / -> Crystal Out
                     VCC -- \ 33                           60 / <- Crystal In
                   PPU A1 <- \ 34                         59 / <- /IJSE
                   PPU A12 <- \ 35                       58 / <- /16B
                     PPU A0 <- \ 36                     57 / <- OBS
                        /ERS -- \ 37                   56 / -- GND
                       /SFCCS -- \ 38                 55 / -> Audio Out 1
                        PPU D0 <> \ 39               54 / -> Audio Out 0
                         PPU D7 <> \ 40             53 / <- /RST
                          PPU D1 <> \ 41           52 / <- Test Pin
                           PPU D6 <> \ 42         51 / -> VOUT
                            PPU D2 <> \ 43       50 / <- REG0
                             PPU D5 <> \ 44     49 / <- REG1
                              PPU D3 <> \ 45   48 / <- /LCDEN
                               PPU D4 <> \ 46 47 / -- GND
                                          \_____/
    

  * /CS0: /CS for $6000 - $7FFF
  * /CS1: /CS for $8000 - $FFFF
  * VDE: Video Data Output Enable; I/O in OneBus Mode
  * ERS: External ROM chip selector / Power on Indicator / I/O in OneBus Mode
  * /SFCCS: Swap the function of /CS0 and /CS1 when low
  * /LCDEN: Enable LCD Output for Testing
  * REGx: Region Selector. All 0 = NTSC, All 1 = PAL
  * OBS: OneBus Mode Enable
  * /16B: 16 bits data bus selector in OneBus Mode. A0 will decide the low byte or high byte of data on CPU D0-7
  * /IJSE: Internal Joystick Enable when XJOYSEL = 0
  * /BTS: Bus tristate enable
  * /FH: Force /CS0 and /CS1 High
  * IOCLKx: Clock of I/O or XCUP47, can be video extention address
  * IOx: I/O interface Output pins or Video extention address



### Sources

  * VT02 Datasheet: <http://www.vrt.com.tw/old_site/admin/upload/datasheet/VT02%20Data%20Sheet%20RevisionA5_ENG__1.pdf>
  * VT03 Datasheet: <http://www.vrt.com.tw/old_site/admin/upload/datasheet/VT03%20Data%20Sheet%20RevisionA6_ENG.pdf>


