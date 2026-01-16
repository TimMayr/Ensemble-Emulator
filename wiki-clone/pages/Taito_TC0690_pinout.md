# Taito TC0690 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Taito_TC0690_pinout) | View [other pages](Special_AllPages.xhtml#Taito_TC0690_pinout)

Taito TC0690: 64-pin 1.0mm pitch QFP (Canonically [iNES Mapper 048](INES_Mapper_048.xhtml "INES Mapper 048")) 
    
    
                                 _____
                         n/c -- /01 64\ -- n/c
                        n/c -- /02   63\ -- n/c
                   CHR A15 <- /03 (o) 62\ -> CHR A16
                  PPU /RD -> /04       61\ -> CHR A11
                 CHR A18 <- /05         60\ -- n/c
                CHR A10 <- /06           59\ -- GND
              PPU A12* -> /07             58\ <- CPU D3
              PPU A11 -> /08               57\ -- VCC
                 VCC -- /09                 56\ <- CPU D2
            PPU A10 -> /10                   55\ <- CPU D4
               GND -- /11                     54\ <- CPU D1
          CHR A13 <- /12                       53\ <- CPU D5
        PPU A13* -> /13    TAITO TC0690FMI      52\ -- n/c
        CHR A14 <- /14                          51/ -- n/c
       CHR A12 <- /15                          50/ -- n/c
    CIRAM A10 <- /16                          49/ -- n/c
     CHR A17 <- /17                          48/ <- CPU D0
        n/c -- /18                          47/ <- CPU A1
       n/c -- /19                          46/ <- CPU D6
       n/c -- \20                         45/ <- CPU A0
       /IRQ <- \21                       44/ <- CPU D7
     /ROMSEL -> \22                     43/ -- GND
      PRG /CE <- \23                   42/ -> CHR /CE
       CPU R/W -> \24                 41/ -- VCC
            VCC -- \25               40/ <- M2
         PRG A15 <- \26             39/ -- n/c
              GND -- \27           38/ -> PRG A17
           PRG A13 <- \28         37/ <- CPU A13
            CPU A14 -> \29       36/ -> PRG A18
             PRG A16 <- \30     35/ -> PRG A14
                  n/c -- \31   34/ -- n/c
                   n/c -- \32 33/ -- n/c
                           \   /
                            \ / 
    

Pins marked n/c appear to have no internal connection. 

PPU A12 and A13 go through a multiplexer serving as a transparent latch, only permitting changes while PPU /RD is high. 

Source: [krzysiobal on the forum](https://forums.nesdev.org/viewtopic.php?p=241376#p241376)

Categories: [Pinouts](Category_Pinouts.xhtml)
