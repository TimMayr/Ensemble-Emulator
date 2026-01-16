# Namcot 108 family pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Namcot_108_family_pinout) | View [other pages](Special_AllPages.xhtml#Namcot_108_family_pinout)

Namcot 108, 109, 118, 119: 28-pin shrink PDIP, also Tengen 337001 or MIMIC-1: 28-pin 0.6" PDIP (Canonically [mapper 206](INES_Mapper_206.xhtml "INES Mapper 206")). 
    
    
                   .--\/--.
    (f) CPU A14 -> |01  28| -> PRG A15 (r)
    (fr) CPU A0 -> |02  27| -> PRG A14 (r)
    (fr) CPU D5 -> |03  26| <- M2 (f)      
    (fr) CPU D0 -> |04  25| -> PRG A13 (r)
    (fr) CPU D4 -> |05  24| <- CPU A13 (f)
    (fr) CPU D1 -> |06  23| -> PRG A16 (r)
            Gnd -- |07  22| -> PRG /CE (r)
    (fr) CPU D3 -> |08  21| -- +5V
    (fr) CPU D2 -> |09  20| -> CHR A13 (r)
    (f) /ROMSEL*-> |10  19| -> CHR A11 (r)
        (f) R/W -> |11  18| -> CHR A10 (r)
    (r) CHR A15 <- |12  17| <- PPU A10 (f)
    (r) CHR A14 <- |13  16| <- PPU A11 (f)
    (r) CHR A12 <- |14  15| <- PPU A12 (f)
                   `------'
    
    10: on the Vs. System daughterboards, this is instead CPU /A15
    

No behavioral differences are known between the five different part numbers; [several](http://bootgod.dyndns.org:7777/profile.php?id=3247) [games](http://bootgod.dyndns.org:7777/profile.php?id=3185) were released on the same board with identical code and different mapper ICs. 

## Contents

  * 1 Functional Variations
    * 1.1 3446
    * 1.2 3433, 3443, 3453
    * 1.3 3425
  * 2 Pirate clones
    * 2.1 Functional variations
      * 2.1.1 KS7037
      * 2.1.2 Sachen 3009
  * 3 Sources



## Functional Variations

Many boards (and thus iNES mappers) redefined parts of the pinout for various extensions, mostly to increase the amount of addressable CHR ROM. 

#### 3446

[Mapper 76](INES_Mapper_076.xhtml "INES Mapper 076"): Rewires CHR banking to be able to address more total CHR with coarser CHR banks. 
    
    
    (fr) CPU D2 -> |09  20| -> CHR **A14** (r)
    (f) /ROMSEL -> |10  19| -> CHR **A12** (r)
        (f) R/W -> |11  18| -> CHR **A11** (r)
    (r) CHR **A16** <- |12  17| <- PPU **A11** (f)
    (r) CHR **A15** <- |13  16| <- PPU **A12** (f)
    (r) CHR **A13** <- |14  15| <- **+5V**
                   `------'
    
          **(f) PPU A10 - > CHR A10 (r)**
    

#### 3433, 3443, 3453

Mappers [88](INES_Mapper_088.xhtml "INES Mapper 088") and [154](INES_Mapper_154.xhtml "INES Mapper 154") connect (f) PPU A12 -> CHR A16 (r), skipping the mapper IC altogether. 

#### 3425

[Mapper 95](INES_Mapper_095.xhtml "INES Mapper 095"): Repurposes CHR A15 to control nametable banking 
    
    
    **(f) CIRAM A10** <- |12  17| <- PPU A10 (f)
      (r) CHR A14 <- |13  16| <- PPU A11 (f)
      (r) CHR A12 <- |14  15| <- PPU A12 (f)
                     `------'
    

## Pirate clones

AX-24G or NTDEC8701: 28-pin 0.6" PDIP 
    
    
                   .--\/--.                                 .--\/--.           
    (r) PRG A13 <- |01  28| <- CPU A14 (f)            M2 -> |01  28| -- +5V    
    (r) PRG A14 <- |02  27| <- CPU A13 (f)           R/W -> |02  27| <- RESET  
    (r) PRG A16 <- |03  26| <- CPU A0 (fr) MODE/CPU A14* -> |03  26| <- CPU D3*
    (r) PRG A15 <- |04  25| -- VCC              CPU A13* -> |04  25| -> CHR A15
    (r) CHR A14 <- |05  24| <- /ROMSEL (f)       CPU D1* -> |05  24| -> CHR A14
    (r) CHR A11 <- |06  23| -> PRG /OE (r)       CPU D0* -> |06  23| -> CHR A13
    (r) CHR A13 <- |07  22| <- M2 (f)            /ROMSEL -> |07  22| -> CHR A12
    (r) CHR A12 <- |08  21| <- R/W (f)           CPU D4* -> |08  21| -> CHR A11
    (fr) CPU D2 -> |09  20| <- CPU-D0 (fr)        CPU A0 -> |09  20| -> CHR A10
    (fr) CPU D3 -> |10  19| <- CPU-D5 (fr)       PRG A16 <- |10  19| <- CPU D5*
    (fr) CPU D1 -> |11  18| <- CPU-D4 (fr)       PRG A15 <- |11  18| <- PPU A12
            Gnd -- |12  17| <- PPU-A12 (f)       PRG A14 <- |12  17| <- PPU A11
    (r) CHR A15 <- |13  16| <- PPU-A11 (f)       PRG A13 <- |13  16| <- PPU A10
    (r) CHR A10 <- |14  15| <- PPU-A10 (f)           GND -- |14  15| <- CPU D2*
                   `------'                                 `------'           
           AX-24G, NTDEC8701, AX5604P                        KS 204		
    

KS204: 

  * MODE: If pin 3 is low on a falling edge of RESET, it operates in N108 mode. If it's high, or before the first falling edge of RESET, it operates in VRC2 mode. [[1]](https://forums.nesdev.org/viewtopic.php?p=243166#p243166)
  * Pins marked with * change behavior in VRC2 mode



### Functional variations

#### KS7037

[Mapper 307](NES_2_0_Mapper_307.xhtml "NES 2.0 Mapper 307") (UNIF "KS7037"): repurpose CHR banking for nametable control. Moves second switchable bank. 
    
    
                 .--\/--.           
           M2 -> |01  28| -- +5V    
          R/W -> |02  27| <- RESET  
      CPU **A13** -> |03  26| <- CPU D3
      CPU **A14** -> |04  25| -> **n/c**
       CPU D1 -> |05  24| -> **n/c**
       CPU D0 -> |06  23| -> **n/c**
      /ROMSEL -> |07  22| -> **n/c**
       CPU D4 -> |08  21| -> **n/c**
       CPU A0 -> |09  20| -> **CIRAM A10**
     PRG A16* <- |10  19| <- CPU D5
      PRG A15 <- |11  18| <- **+5V**
      PRG A14 <- |12  17| <- PPU A11
      PRG A13 <- |13  16| <- PPU A10
          GND -- |14  15| <- CPU D2
                 `------'           
                  KS 204
    

External hardware further modifies the exact memory map. 

#### Sachen 3009

Redundant variation of [mapper 133](INES_Mapper_133.xhtml "INES Mapper 133"): obfuscatory GNROM-style banking. 
    
    
                   .--\/--.                
            **n/c** <- |01  28| <- CPU A14 (f) 
            **n/c** <- |02  27| <- CPU A13 (f) 
            **n/c** <- |03  26| <- CPU A0 (fr) 
            **n/c** <- |04  25| -- VCC         
     **feedback#1** <- |05  24| <- /ROMSEL (f) 
    (r) **PRG A15** <- |06  23| -> PRG /OE (r) 
    (r) CHR **A14** <- |07  22| <- M2 (f)      
    (r) CHR **A13** <- |08  21| <- R/W (f)     
    (fr) CPU **D0** -> |09  20| <- CPU **D3** (fr) 
    (fr) CPU **D1** -> |10  19| <- CPU **D4** (fr) 
    (fr) CPU **D2** -> |11  18| <- CPU **D5** (fr) 
            Gnd -- |12  17| <- **Vcc**
     **feedback#2** <- |13  16| <- **feedback#2**
            **n/c** <- |14  15| <- **feedback#1**
                   `------'                
                    AX-24G
    

## Sources

  * AX-24G or NTDEC8701 [[BBS](https://forums.nesdev.org/viewtopic.php?p=210278#p210278)]
  * KS 204 [[BBS](http://forums.nesdev.org/viewtopic.php?f=9&t=19251)]



Categories: [Pinouts](Category_Pinouts.xhtml)
