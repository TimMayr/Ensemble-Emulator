# MMC5 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/MMC5_pinout) | View [other pages](Special_AllPages.xhtml#MMC5_pinout)
    
    
                                                       _____
                                                      /     \
                            Audio Amplifier Input -> / 1 100 \ -> Audio Amplifier Output
                                       Audio DAC <- / 2    99 \ -- AGnd
                              Audio Pulse Waves <- / 3  (*) 98 \ <> SL3
                                      +5V AVcc -- / 4        97 \ <> CL3
                                  (fr) PPU A0 -> / 5          96 \ -> CHR A2 †
                                 (fr) PPU A1 -> / 6            95 \ -> CHR A1 †
                                (fr) PPU A2 -> / 7              94 \ -> CHR A0 †
                               (fr) PPU A3 -> / 8                93 \ -> PRG RAM A15 (R)
                              (fr) PPU A4 -> / 9                  92 \ <- /In-Frame Detection Disable
                             (fr) PPU A5 -> / 10                   91 \ <> PPU D7 (fr)
                            (fr) PPU A6 -> / 11                     90 \ <> PPU D6 (fr)
                           (fr) PPU A7 -> / 12                       89 \ <> PPU D5 (fr)
                          (fr) PPU A8 -> / 13                         88 \ <> PPU D4 (fr)
                         (fr) PPU A9 -> / 14                           87 \ <> PPU D3 (fr)
                        (r) CHR A10 <- / 15                             86 \ <> PPU D2 (fr)
                       (r) CHR A11 <- / 16                               85 \ <> PPU D1 (fr)
                      (r) CHR A12 <- / 17                                 84 \ <> PPU D0 (fr)
                     (r) CHR A13 <- / 18                                   83 \ -> PRG RAM +CE (R)
                    (r) CHR A14 <- / 19                                     82 \ ?? (unknown input)
                   (r) CHR A15 <- / 20                                       81 \ ?? (unknown input)
                  (r) CHR A16 <- / 21                                            \
                 (r) CHR A17 <- / 22                                             /
                (r) CHR A18 <- / 23                                          80 / -- DGnd
               (r) CHR A19 <- / 24                                          79 / <- M2 (f)
              (f) PPU A10 -> / 25                                          78 / <- /ROMSEL (f)
             (f) PPU A11 -> / 26              Nintendo MMC5               77 / <- CPU R/W (f)
            (f) PPU A12 -> / 27      Package QFP-100, 0.65mm pitch       76 / -> PRG RAM /WE (R)
          (f) PPU /A13 -> / 28                (20mm × 14mm)             75 / -> PRG RAM /CE (RAM0/CE & RAM1/CE) (R)
      (unknown input) ?? / 29                                          74 / -> PRG /CE (r)
     (unknown input) ?? / 30                                          73 / -> PRG RAM A16 (R)
                       /                                             72 / -> PRG RAM 1 /CE (R)
                       \                                            71 / -> PRG RAM 0 /CE (R)
       (f) CIRAM /CE <- \ 31                                       70 / -> PRG RAM A14 (R)
        (f) CIRAM A10 <- \ 32                                     69 / -> PRG RAM A13 (R)
           (f) PPU /WR -> \ 33                                   68 / <- CPU A14 (f)        Orientation:
            (f) PPU /RD -> \ 34                                 67 / <- CPU A13 (f)         --------------------
                (f) /IRQ <- \ 35                               66 / -> PRG A19 (r)              80         51
             (frR) CPU D0 <> \ 36                             65 / -> PRG A18 (r)                |         |
              (frR) CPU D1 <> \ 37                           64 / -> PRG A17 (r)                .-----------.
               (frR) CPU D2 <> \ 38                         63 / -> PRG A16 (r)              81-| Nintendo o|-50
                (frR) CPU D3 <> \ 39                       62 / -> PRG A15 (r)                  | MMC5      |
                 (frR) CPU D4 <> \ 40                     61 / -> PRG A14 (r)               100-|@          |-31
                  (frR) CPU D5 <> \ 41                   60 / -> PRG A13 (r)                    \-----------'
                   (frR) CPU D6 <> \ 42                 59 / <- CPU A12 (frR)                    |         |
                    (frR) CPU D7 <> \ 43               58 / <- CPU A11 (frR)                   01         30
                         +5V DVcc -- \ 44             57 / -- + Backup battery
                      (frR) CPU A0 -> \ 45           56 / -> PRG RAM Vcc Output   Legend:
                       (frR) CPU A1 -> \ 46         55 / <- CPU A10 (frR)         ------------------------------
                        (frR) CPU A2 -> \ 47       54 / <- CPU A9 (frR)           --[MMC5]-- Power
                         (frR) CPU A3 -> \ 48  O  53 / <- CPU A8 (frR)            ->[MMC5]<- MMC5 input
                          (frR) CPU A4 -> \ 49   52 / <- CPU A7 (frR)             <-[MMC5]-> MMC5 output
                           (frR) CPU A5 -> \ 50 51 / <- CPU A6 (frR)              <>[MMC5]<> Bidirectional
                                            \     /                               ??[MMC5]?? Unknown
                                             \   /                                    f      Famicom connection
                                              \ /                                     r      ROM chip connection
                                               V                                      R      RAM chip connection
    

† Default connections ("CL mode"): 

  * Connect CHR-ROM A0 directly to PPU A0.
  * Connect CHR-ROM A1 directly to PPU A1.
  * Connect CHR-ROM A2 directly to PPU A2.
  * Leave output pins 94, 95 & 96 disconnected on the MMC5.



For smooth vertical scrolling in vertical split mode ("SL mode"): 

  * Connect CHR-ROM A0 only to pin 94 of the MMC5.
  * Connect CHR-ROM A1 only to pin 95 of the MMC5.
  * Connect CHR-ROM A2 only to pin 96 of the MMC5.



"CL mode" passes the lowest PPU address bits straight to CHR ROM, while "SL mode" runs them through MMC5. SL mode allows the MMC5 to perform independent, scanline-precise vertical scrolling in vertical split mode on the side specified by register $5200. CL mode does not. All known MMC5 cartridges use CL mode. It is not known why SL mode was not used instead; possibly ROM speed issues. 

  
Audio circuit topology for HVC-ExROM boards: 

[![MMC5 audio.png](../wiki-images/MMC5_audio.png)](File_MMC5_audio_png.xhtml)

Categories: [Pinouts](Category_Pinouts.xhtml)
