# Tengen RAMBO-1 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Tengen_RAMBO-1_pinout) | View [other pages](Special_AllPages.xhtml#Tengen_RAMBO_1_pinout)

Tengen 337006 or [RAMBO-1](RAMBO_1.xhtml "RAMBO-1"): 40-pin 0.6" PDIP (Canonically **mapper 64**) 
    
    
                      .--\/--.
            (n) M2 -> |01  40| -- +5V
       (r) PRG A16 <- |02  39| -> PRG A17 (r)
       (r) PRG A15 <- |03  38| <- CPU A13 (n)
       (r) PRG A13 <- |04  37| -> PRG A14 (r)
       (r) PRG /CE <- |05  36| <- CPU A14 (n)
       (rn) CPU D6 -> |06  35| <- CPU D7 (rn)
       (rn) CPU A0 -> |07  34| <- CPU D5 (rn)
       (rn) CPU D4 -> |08  33| <- CPU D0 (rn)
       (rn) CPU D2 -> |09  32| <- CPU D1 (rn)
           (n) R/W -> |10  31| <- CPU D3 (rn)
       (n) /ROMSEL -> |11  30| -> /IRQ (n)
       (r) CHR A10 <- |12  29| -> CIRAM A10 (n)
       (n) PPU A10 -> |13  28| -> CHR A11 (r)
       (n) PPU A12 -> |14  27| <- PPU A11 (n)
     (n) CIC toPak -> |15  26| -> CHR A13 (r)
      (n) CIC toMB <- |16  25| -> CHR A12 (r)
      (n) CIC +RST -> |17  24| -> CHR A14 (r)
       (n) CIC CLK -> |18  23| -> CHR A15 (r)
               GND ?? |19  22| -> CHR A17 (r)
               GND -- |20  21| -> CHR A16 (r)
                      '------'
    

Variant pinout for [iNES Mapper 158](INES_Mapper_158.xhtml "INES Mapper 158"): 
    
    
       (n) /ROMSEL -> |11  30| -> /IRQ (n)
       (r) CHR A10 <- |12  29| -> **n/c**
       (n) PPU A10 -> |13  28| -> CHR A11 (r)
       (n) PPU A12 -> |14  27| <- PPU A11 (n)
     (n) CIC toPak -> |15  26| -> CHR A13 (r)
      (n) CIC toMB <- |16  25| -> CHR A12 (r)
      (n) CIC +RST -> |17  24| -> CHR A14 (r)
       (n) CIC CLK -> |18  23| -> CHR A15 (r)
               GND ?? |19  22| -> **CIRAM A10 (n)**
               GND -- |20  21| -> CHR A16 (r)
                      '------'
    

  
On the PCB 800032, there are seven jumpers: 

  * R1 connects PRG ROM pin 22(28 pin IC)/24(32 pin IC) to ground, for use with ROMs that follow the standard EPROM pinout.
  * R2 connects the same pin to RAMBO-1 PRG A16, for use with 28-pin 128 KiB JEDEC mask ROMs.



Exactly one of R1 and R2 should be populated. 

  * R3 connects CHR ROM pin 22(28 pin IC)/24(32 pin IC) to PPU /RD, for use with ROMs that follow the standard EPROM pinout.
  * R4 connects the same pin to RAMBO-1 CHR A16, for use with 28-pin 128 KiB JEDEC mask ROMs.
  * R5 (unlabeled, under the 74'32 socket) connects CHR ROM pin 20(28 pin IC)/22(32 pin IC) to PPU A13, for use with ROMs that follow the standard EPROM pinout.
  * Populating the 74'32 in the upper right corner of the board connects CHR ROM pin 20(28 pin IC)/22(32 pin IC) to _PPU /RD OR PPU A13_ , for use with 28-pin 128 KiB JEDEC mask ROMs.



If R4 is populated, the 74'32 should be populated. Exactly one of R3 and R4 should be populated. Exactly one of R5 and the 74'32 should be populated. 

  * R6 and R7 tie the RAMBO-1's pins 15 and 17 to ground, respectively.



Additionally, there are two cut & solder jumpers: 

  * P28 (on the underside of the board) controls whether PRG pin 28(28 pin IC)/30(32 pin IC) is connected to +5V or to RAMBO-1 PRG A17.
  * C28 controls whether CHR pin 28(28 pin IC)/30(32 pin IC) is connected to +5V or to RAMBO-1 CHR A17.



  
Source: 

  * Kevtris [[1]](http://kevtris.org/mappers/tengen/800032.html) [[2]](http://kevtris.org/mappers/tengen/800037.html)
  * [FrankWDoom's Hard Drivin' reproduction](http://forums.nesdev.org/viewtopic.php?p=125092#p125092)



Categories: [Pinouts](Category_Pinouts.xhtml)
