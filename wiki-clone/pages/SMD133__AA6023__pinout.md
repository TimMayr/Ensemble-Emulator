# SMD133 (AA6023) pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/SMD133_%28AA6023%29_pinout) | View [other pages](Special_AllPages.xhtml#SMD133__AA6023__pinout)

SMD133 (AA6023B): LQFP-48 0.5mm pitch (canonically [NES 2.0 Mapper 268](NES_2_0_Mapper_268.xhtml "NES 2.0 Mapper 268") submapper 2/3) 
    
    
                                                ________
                                    PPU A11 -> / 1   48 \ <- PPU A12
                                   PPU A10 -> / 2     47 \ <- CPU A12
                                      VCC -- / 3       46 \ -> CHR A10
                                     GND -- / 4         45 \ -> CHR A16
                                 CPU A1 -> / 5           44 \ -> CHR A11
    (PRG A22, may vary) "PX0" $5001.1  <- / 6             43 \ -> $5000.5 (CHR A19 in submappers 2/3, varies in others)
                              CHR A13 <- / 7               42 \ -> $5000.4 (CHR A18, varies)
                             CHR A14 <- / 8                 41 \ <- CPU D3
                            CHR A12 <- / 9     SMD133        40 \ <- CPU D2
                 (CIRAM A10?) "HV" <- / 10     (AA6023)       39 \ <- CPU D4
                          CHR A15 <- / 11                      38 \ <- CPU D1
                         CHR A17 <- / 12                        37 \ <- CPU D5
                         PRG A20 <- \ 13                        36 / <- CPU D0
                             /IRQ <- \ 14                      35 / -> $5001.2 "PX1" (PRG A21, may vary)
                       CPU /ROMSEL -> \ 15                    34 / <- CPU D6
                            CPU R/W -> \ 16                  33 / <- CPU A0
                             PRG A15 <- \ 17                32 / <- CPU D7
                              PRG A13 <- \ 18              31 / -> PRG RAM /CE ("EXT CSB")
                               CPU A14 -> \ 19            30 / <- M2 ("CK18")
                           K5K7 or K5K6 -> \ 20          29 / -> VFL
                                 PRG A16 <- \ 21        28 / -- VDD
                                  PRG A14 <- \ 22      27 / -> PRG A19
                                   PRG A18 <- \ 23    26 / -> PRG /CE ("POEB")
                                    CPU A13 -> \ 24  25 / -> PRG A17
                                                --------
    Legend:
    ->[SMD]<- Input
    <-[SMD]-> Output
    --[SMD]-- Power
    

  


  
"VFL" is reportedly a diode from VDD, originally intended for powering 3V flash. (Don't do that, it's not a good enough voltage regulator). 

Source: [[1]](https://aliexpress.com/item/1005004245220736.html)

Categories: [Pinouts](Category_Pinouts.xhtml)
