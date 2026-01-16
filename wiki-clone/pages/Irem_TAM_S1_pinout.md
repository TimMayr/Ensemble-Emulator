# Irem TAM-S1 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Irem_TAM-S1_pinout) | View [other pages](Special_AllPages.xhtml#Irem_TAM_S1_pinout)

Irem TAM-S1: 28-pin 0.3" PDIP (Canonically [mapper 97](INES_Mapper_097.xhtml "INES Mapper 097")) 
    
    
                         Irem's TAM-S1 IC
                            .---v---.
                 PRG A18 <- |01   28| <- CPU A14 (f)
             (r) PRG A16 <- |02   27| -> PRG A17 (r)
             (r) PRG A15 <- |03   26| -> PRG A14 (r)
            (rf) PPU A10 -> |04   25| -> PRG /A17
            (rf) PPU A11 -> |05   24| -> CIRAM A10 (f)
        (Gnd) prgce mode -> |06   23| <- n/c
                     GND -- |07   22| <- n/c
                 xor a14 -> |08   21| -- +5V
                 (f) R/W -> |09   20| -> PRG /CE (r)
           (Gnd) data in -> |10   19| <- /ROMSEL (f)
                data out <Z |11   18| <- CPU D7 (rf)
             (rf) CPU D0 -> |12   17| <- CPU Dx  (+5v)
             (rf) CPU D1 -> |13   16| <- CPU D4  (Gnd)
             (rf) CPU D2 -> |14   15| <- CPU D3 (rf)
                            '-------'
    

Pins 6, 8, 22, and 23 all permit configuration of how the mapper works, but are inaccessible by default. 

When the value latched by pin 17 is low, pin 11 is a copy of pin 10. When high, pin 11 is high-impedance. Together, these three pins could be modified to either extend PRG support to 1MB, or to allow one kind of 1-screen mirroring. 

Tying pin 8 low instead yields the more standard UNROM-like memory layout with the fixed bank at higher addresses. 

ROM is enabled during writes to the region where the bankswitching register **isn't** , much like the self-flashable versions of [UNROM 512](UNROM_512.xhtml "UNROM 512"). The existing game, however, uses mask ROM and so cannot use this functionality. 

  * Source: [Krzysiobal](https://forums.nesdev.org/viewtopic.php?t=19769)



Categories: [Pinouts](Category_Pinouts.xhtml)
