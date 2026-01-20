# GMNFC 01 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/GMNFC_01_pinout) | View [other pages](Special_AllPages.xhtml#GMNFC_01_pinout)
    
    
                                                      +-------+                    
                                              +5V -- / 001 128 \ -- GND
                                              M2 -> / 002   127 \ -- GND
                                        CPU A11 -> / 003     126 \ <- CIR /CS
                                       CPU A12 -> / 004       125 \ <- CIR A10
                                          GND -- / 005         124 \ <- PPU /RD
                               SLAVE CPU A13 <- / 006           123 \ -> SLAVE CIR A10
                                     CPU A9 -> / 007             122 \ <- PPU A13
                             SLAVE CPU A14 <- / 008               121 \ -> SLAVE PPU A13
                                   CPU A8 -> / 009                 120 \ NC
                                  CPU D7 <> / 010                   119 \ -> EPR /OE
                                 CPU A7 -> / 011                     118 \ -> EPR A13
                                CPU D6 <> / 012                       117 \ <- PPU A10
                               CPU A6 -> / 013                         116 \ <- CPU A13
                              CPU D5 <> / 014                           115 \ <- CPU A14
                             CPU A5 -> / 015                             114 \ ??
                            CPU D4 <> / 016                               113 \ -- +5V
                              +5V -- / 017                                 112 \ ??
                         CPU A14 -> / 018                                   111 \ ??
                            GND -- / 019                                     110 \ ??
                        CPU D3 <> / 020                                       109 \ ??
                       CPU A3 -> / 021                                         108 \ <- /RESET
                      CPU D2 <> / 022                                           107 \ <> RAM D7
                            NC / 023                                             106 \ -- +5V
                    CPU A2 -> / 024                                               105 \ NC
                   CPU D1 <> / 025                                                 104 \ <> RAM D5
                  CPU A1 -> / 026                                                   103 \ <> RAM D6
                 CPU D0 <> / 027                                                     102 \ <> RAM D4
                CPU A0 -> / 028                                                       101 \ -- XTAL
    SLAVE CPU /ROMSEL <- / 029                                                         100 \ -- GND
         CPU /ROMSEL -> / 030                                                           099 \ ??
            CPU R/W -> / 031                                                             098 \ -- GND
               GND -- / 032                           GMNFC 01                            097 \ -- GND
                   ?? \ 033                            OKI                                096 / ??
                    ?? \ 034                                                             095 / ??
       SLAVE PPU /RD <- \ 035                                                           094 / ??
        SLAVE CIR /CS <- \ 036                                                         093 / -- +5V
                       ?? \ 037                                                       092 / ??
                        ?? \ 038                                                     091 / -- GND
                         ?? \ 039                                                   090 / ??
                          ?? \ 040                                                 089 / ??
                           NC \ 041                                               088 / ??
                            ?? \ 042                                             087 / NC
                             ?? \ 043                                           086 / -- +5V
                              ?? \ 044                                         085 / ??
                               ?? \ 045                                       084 / -- GND
                                ?? \ 046                                     083 / ??
                                 ?? \ 047                                   082 / ??
                                  ?? \ 048                                 081 / ??
                               +5V -- \ 049                               080 / <> RAM D2
                                    ?? \ 050                             079 / ??
                                     ?? \ 051                           078 / <> RAM D3
                                      ?? \ 052                         077 / -- GND
                                       ?? \ 053                       076 / <> RAM D0
                                        ?? \ 054                     075 / <> RAM D1
                                         ?? \ 055                   074 / -> LA04 / LA00
                                          NC \ 056                 073 / -> LA07 / LA12
                                           ?? \ 057               072 / -> LA03 / LA06
                                       ALE1 <- \ 058             071 / -> LA10 / LA08
                                     RAM /WE <- \ 059           070 / -> LA14
                                              ?? \ 060         069 / -> LA16 / LA13
                                          ALE2 <- \ 061       068 / -> LA15 / LA09
                                        RAM /OE <- \ 062     067 / -> LA05 / LA02
                                             GND -- \ 063   066 / -> LA01 / LA11
                                              GND -- \ 064 065 / -- +5V
                                                      +------+/
    

Source: [[1]](https://forums.nesdev.org/viewtopic.php?f=9&t=20529)

Categories: [Pinouts](Category_Pinouts.xhtml)
