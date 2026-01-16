# VT01 Pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VT01_Pinout) | View [other pages](Special_AllPages.xhtml#VT01_Pinout)
    
    
                                            _____
                                           / 103 \ -> CPU A6
                                   GND -- / 1 102 \ <> CPU D7
                                         /     101 \ -> CPU A7
                              CPU D6 <> / 2     100 \ -> CPU A11
                             CPU A5 <- / 3        99 \ -> CPU A8
                            CPU D5 <> / 4          98 \ -> CPU A13
                           CPU A4 <- / 5            97 \ -> CPU A9
                          CPU D4 <> / 6              96 \ -> CPU A12
                         CPU A3 <- / 7                95 \ -> CPU A10
                        CPU D3 <> / 8                  94 \ <- CPU Clock (1.8MHz)
                       CPU A2 <- / 9                    93 \ -> CPU A11
                      CPU D2 <> / 10                     92 \ ?? DIVF
                        GND -- / 11                       91 \ -- VCC
                       VCC -- / 12                         90 \ -> Audio
                      VCC -- / 13                           89 \ <- TFT LCD Select
                  CPU A1 <- / 14                             88 \ -> Toggle Enable for LCD source driver
                 CPU D1 <> / 15                               87 \ -> LCD Blue
                CPU A0 <- / 16                                 86 \ -> LCD Green
               CPU D0 <> / 17                                   85 \ -> LCD Vertical Start Pulse
             CPU R/W <- / 18                                     84 \ -> LCD /OE
            /ROMSEL <- / 19                                       83 \ -> LCD Red
              /IRQ -> / 20                                         82 \ -> STH
          PPU R/W <- / 21                                           81 \ -> CPH1
                    /                                                80 \ -> CPH2
            GND -- / 22                                               79 \ -> CPH3
                  /                                                    78 \ -- VCC
     Video /OE -> \ 23                                                  77 \ -- GND
        CPU A15 <- \ 24                                                  76 \ -- GND
        IVAB A10 -> \ 25                                                     \
            Power <- \ 26                                                 75 / <- JOYAM
            PPU A6 <- \ 27                                               74 / ?? CPU47 
             PPU A7 <- \ 28                                             73 / <- JOYRTA
              PPU A5 <- \ 29                                           72 / -> IO1
               PPU A8 <- \ 30                                         71 / <- JOYDNA
                   GND -- \ 31                                       70 / <- JOYLFA
                 PPU A4 <- \ 32                                     69 / <- JOYUPA
                  PPU A9 <- \ 33                                   68 / <- $4016.1
                   PPU A3 <- \ 34                                 67 / <- JOYST
                   PPU A10 <- \ 35                               66 / ?? CUP46
                     PPU A2 <- \ 36                             65 / <- JOYSE
                     PPU A11 <- \ 37                           64 / <- JOYBM
                       PPU A1 <- \ 38                         63 / -> Clock Crystal Output
                       PPU A12 <- \ 39                       62 / <- Clock Crystal Input
                         PPU A0 <- \ 40                     61 / <- /RST
                             VCC -- \ 41                   60 / <- TEST
                        EXROM /CS -> \ 42                 59 / -- VCC
                            PPU D0 <> \ 43               58 / -> Video
                             PPU D7 <> \ 44             57 / <- REG1
                              PPU D1 <> \ 45           56 / <- REG0
                               PPU D6 <> \ 46         55 / <> PPU D4
                                JOYSEL -> \ 47       54 / <> PPU D3
                                  LCDGD -> \ 48     53 / <> PPU D5
                                    VCOM -> \ 49   52 / <> PPU D2
                                      Q2H -> \ 50 51 / -- GND
                                              -------
    

  * IVAB A10: Internal Video Address A10
  * Power: Power On signal, default high, low if $2001(D6)=1 or no any pushed for 5 minutes
  * LCDGD: Clock input for gate driver of TFT LCD Driver.
  * VCOM: Common electrode voltage control of TFT LCD Driver
  * Q2H: Video input rotation control of TFT LCD Driver
  * REGx: Region Selector. All 0 = NTSC; All 1 = PAL
  * CPHx: Sampling and shift clock for source driver of TFT LCD Driver
  * STH: Start pulse for source driver of TFT LCD Driver



Based on <http://www.vrt.com.tw/old_site/admin/upload/datasheet/VT01%20Data%20Sheet%20RevisionA2_ENG_.pdf>
