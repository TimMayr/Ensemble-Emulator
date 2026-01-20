# Epoxy package mapper pinouts

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Epoxy_package_mapper_pinouts) | View [other pages](Special_AllPages.xhtml#Epoxy_package_mapper_pinouts)

  
When looking at PCB with unknown epoxy blob mapper, trying to map its pin-out with any existing one can help recognizing the mapper chip. 

  
Notes: 

  * pins are numbered counter-clockwise (like in ordinary QFP and DIL chips)
  * first pin is the one appearing after VCC, so that VCC is last pin (if there are multiple VCCs, the choice is arbitrary)
  * pin-outs shown bellow contain the maximum (full) pin-out that was ever observed; if some pins are not used in a particular board (for example: epoxy MMC3 + 128kB PRG + 128kB CHR, no WRAM), there won't be wires coming into the blob (in that example: PRG A17, PRG A18, CHR A17, WRAM +CE, WRAM /CE, WRAM /WE are omitted);



## Contents

  * 1 Mapper 090
  * 2 Mapper 091
  * 3 Mapper 221



# Mapper 090
    
    
                   /-------\
             M2 -> |  1 56 | -- VCC
        CHR /CE <- |  2 55 | <- CPU A11
    CPU /ROMSEL -> |  3 54 | -> PRG A20
        CPU A12 -> |  4 53 | -> PRG A19
        CPU A13 -> |  5 52 | -> PRG A18
        CPU A14 -> |  6 51 | -> PRG A17
         CPU A0 -> |  7 50 | -> PRG A16
         CPU A1 -> |  8 49 | -> PRG A15
         CPU A2 -> |  9 48 | -> PRG A14
        PRG /CE <- | 10 47 | -> PRG A13
       WRAM /CE <- | 11 46 | -> /IRQ
         CPU D0 <> | 12 45 | -> CIRAM A10
         CPU D1 <> | 13 44 | -> CHR A18
         CPU D2 <> | 14 43 | <- CPU R/W
         CPU D3 <> | 15 42 | <- PPU /RD
         CPU D4 <> | 16 41 | -> CHR A17
         CPU D5 <> | 17 40 | -> CHR A16
         CPU D6 <> | 18 39 | -> CHR A15
         CPU D7 <> | 19 38 | -> CHR A14
        PPU A10 -> | 20 37 | -> CHR A13
        PPU A11 -> | 21 36 | -> CHR A12
        PPU A12 -> | 22 35 | -> CHR A11
        PPU A13 -> | 23 34 | -> CHR A10
        CHR A19 <- | 24 33 | -> CIRAM /CE
         PPU A3 -> | 25 32 | <- PPU A9
         PPU A4 -> | 26 31 | <- PPU A8
         PPU A5 -> | 27 30 | <- PPU A7
            GND -- | 28 29 | <- PPU A6
                   \-------/
    

Sample games: 

  * _Super Mario World (demo)_ \- does not use CPU A11 [[1]](http://krzysiobal.com/carts/?action=view&id=201)
  * _45 in 1_ [[2]](http://krzysiobal.com/carts/?action=view&id=241)
  * _WarioLand_ \- the only known game using WRAM /CE pin



# Mapper 091
    
    
               /-------\
         M2 -> |  1    |  
     CPU D0 -> |  2 35 | -- VCC
     CPU D1 -> |  3 34 | -> PRG /CE
     CPU D2 -> |  4 33 | -> CHR A18
     CPU D3 -> |  5 32 | -> CHR A17
     CPU D4 -> |  6 31 | -> CHR A16
     CPU D5 -> |  7 30 | -> CHR A15
     CPU D6 -> |  8 29 | -> CHR A14
     CPU D7 -> |  9 28 | -> CHR A13
     CPU A0 -> | 10 27 | -> CHR A12
     CPU A1 -> | 11 26 | -> CHR A11
       /IRQ <- | 12 25 | -> PRG A16
     CPU A2 -> | 13 24 | -> PRG A15
    CPU A12 -> | 14 23 | -> PRG A14
    CPU A13 -> | 15 22 | -> PRG A13
    CPU A14 -> | 16 21 | <- CPU /ROMSEL
    PPU A11 <- | 17 20 | <- CPU R/W
    PPU A12 <- | 18 19 | -- GND
               \-------/
    

Sample games: 

  * _Dragon Ball Z - Super Butouden 2_ [[3]](http://krzysiobal.com/carts/?action=view&id=230)
  * _Super Mario Sonik 2_ \- CHR A18/A17 are repurposed to select mirroring [[4]](http://krzysiobal.com/carts/?action=view&id=548)
  * _Street Fighter 3_ \- discrete version [[5]](http://krzysiobal.com/carts/?action=view&id=274)



  


# Mapper 221
    
    
               /-------\
    PRG A19 <- |  1 28 | -- VCC
    PRG A18 <- |  2 27 | <- /RESET
    PRG A17 <- |  3 26 | <- PPU A10
    PRG A16 <- |  4 25 | <- PPU A11
    PRG A15 <- |  5 24 | <- PPU /WE
    PRG A14 <- |  6 23 | <- CPU A14
    CPU A10 -> |  7 22 | <- CPU R/W
     CPU A9 -> |  8 21 | <- CPU /ROMSEL
     CPU A8 -> |  9 20 | -> CIRAM A10
     CPU A7 -> | 10 19 | <- CPU A3
     CPU A6 -> | 11 18 | <- CPU A2
     CPU A5 -> | 12 17 | <- CPU A1
    PRG /CE <- | 13 16 | <- CPU A0
        GND -- | 14 15 | -> CHR /WE
               \-------/
    
    

Sample games: 

  * _400 in 1_ [[6]](http://krzysiobal.com/carts/?action=view&id=33)
  * _400 in 1_ \- discrete version [[7]](http://krzysiobal.com/carts/?action=view&id=32)



Categories: [Pinouts](Category_Pinouts.xhtml)
