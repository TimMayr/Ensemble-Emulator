# 6578 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/6578_pinout) | View [other pages](Special_AllPages.xhtml#6578_pinout)
    
    
                                            _______
                                   IRQ1 -> / 1 100 \ <- /IRQ4
                                  IRQ2 -> / 2  _ 99 \ <- IRQ3
                                   NC -- / 3  (_) 98 \ -> $4016.1
                             JOY1CLK <- / 4        97 \ -> $4016.0
                            KEY_CLK -> / 5          96 \ <- $4016.1
                           KEY_DAT -> / 6            95 \ -> JOY2CLK
                          PRG A20 <- / 7              94 \ <- $4017.0
                         PRG A19 <- / 8                93 \ <- $4016.0
                        PRG A18 <- / 9                  92 \ -> AEN
                       PRG A17 <- / 10                   91 \ <- /RST
                      PRG A16 <- / 11                     90 \ -- GND
                      /EXRAM <- / 12                       89 \ -> AUDIO
                      I/O 7 <> / 13                         88 \ -> VIDEO
                     I/O 6 <> / 14                           87 \ -- VCC
                      VCC -- / 15                             86 \ <> Crystal 2
                 CPU A11 <- / 16                               85 \ <> Crystal 1
                     M2 <- / 17                                 84 \ <> I/O 0
               CPU A10 <- / 18                                   83 \ <> I/O 1
              CPU A12 <- / 19                                     82 \ <> I/O 2
              CPU A9 <- / 20                                       81 \ <> I/O 3
            CPU A13 <- / 21                                        80 / <> PPU D4
            CPU A8 <- / 22                                        79 / <> PPU D3
          CPU A14 <- / 23                                        78 / <> PPU D5
          CPU A7 <- / 24                                        77 / <> PPU D2
         CPU D7 <> / 25                                        76 / <> PPU D6
        CPU A6 <- / 26                                        75 / <> PPU D1
       CPU D6 <> / 27                                        74 / <> PPU D7
      CPU A5 <- / 28                                        73 / <> PPU D0
     CPU D5 <> / 29                                        72 / -> /PTR_ST
    CPU A4 <- / 30                                        71 / <- REGION
    CPU D4 <> \ 31                                       70 / <- TEST2
     CPU A3 <- \ 32                                     69 / <- TEST 1
      CPU D3 <> \ 33                                   68 / <> I/O 4
       CPU A2 <- \ 34                                 67 / <> I/O 5
        CPU D2 <> \ 35                               66 / -> PPU A14
         CPU A1 <- \ 36                             65 / -> PPU A13
          CPU D1 <> \ 37                           64 / -> PPU A0
           CPU A0 <- \ 38                         63 / -> PPU A12
            CPU D0 <> \ 39                       62 / -> PPU A1
            CPU R/W <- \ 40                     61 / -> PPU A11
             /ROMSEL <- \ 41                   60 / -> PPU A2
                   NC -- \ 42                 59 / -> PPU A10
                   GND -- \ 43               58 / -> PPU A3
                    VCC -- \ 44             57 / -> PPU A9
                  /RTCAS <- \ 45           56 / -> PPU A4
                      /PW <- \ 46         55 / -> PPU A8
                       /PR <- \ 47       54 / -> PPU A5
                        GND -- \ 48     53 / -> PPU A7
                     PPU /WR <- \ 49   52 / -> PPU A6
                      PPU /RD <- \ 50 51 / -> /XVRAM
                                  -------
    

Legend: Input: -> [6578] <\- Output: <\- [6578] -> Multidirect: <> [6578] <> Power/Other: -- [6578] -- Unknown: ?? [6578] ?? 

  * NC: No Connection; Leave Floating
  * /EXRAM: CS for External RAM ($6000 - $7FFF); Active Low
  * /RTCAS: Signal of Address Strobe for RTC
  * /PW: Write Signal for peripheral at $4080 - $43xx
  * /PR: Read Signal for $4080 - $43xx
  * /XVRAM: /CS of External VRAM (PPU $8000 - $FFFF)
  * TESTx: Test Pins, Leave Floating
  * REGION: 0 = PAL; 1 = NTSC
  * /PTR_ST: Signal for Printer to Strobe Data
  * AEN: For extending peripheral; like "AEN" signal of ISA bus of PC ($41xx - $43xx)
  * IRQx: IRQ Input. Active High, except for /IRQ4, which is active low



Source: <http://playpower.pbworks.com/w/file/fetch/58323638/SH6578%208-bit%20Educational%20Computer%20Chipset.pdf>
