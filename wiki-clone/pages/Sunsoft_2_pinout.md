# Sunsoft 2 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Sunsoft_2_pinout) | View [other pages](Special_AllPages.xhtml#Sunsoft_2_pinout)

Sunsoft-2: 24 pin shrink PDIP (mappers [89](INES_Mapper_089.xhtml "INES Mapper 089") and [93](INES_Mapper_093.xhtml "INES Mapper 093")) 
    
    
                    .---\/---.       
     (r) PRG A15 <- | 01  24 | -- +5V
     (r) PRG A14 <- | 02  23 | <- CPU A14 (f)
     (r) PRG A16 <- | 03  22 | -> OR Y (CHR nCS) (r)
     (fr) CPU D7 -> | 04  21 | -> CHR A16 (r)
     (fr) CPU D6 -> | 05  20 | <- OR B (PPU nRD) (f, r?)
     (fr) CPU D5 -> | 06  19 | -> CHR A13 (r)
     (fr) CPU D4 -> | 07  18 | -> CHR A14 (r)
     (fr) CPU D3 -> | 08  17 | -> CHR A15 (r)
     (fr) CPU D2 -> | 09  16 | <- /ROMSEL (fr)
     (fr) CPU D1 -> | 10  15 | <- CPU RnW (f)
     (fr) CPU D0 -> | 11  14 | <- OR A (PPU A13) (f)
             GND -- | 12  13 | -> CIRAM A10 (f)
                    `--------'
    
    22 CHR nCS is the logical OR of PPU nRD and PPU A13, allowing them to use a 28-pin 128kB ROM
    19 CHR A13 is connected to CHR RAM's positive chip enable, so games that use CHR RAM must write 1 to it
    

The Sunsoft-2 mapper was found on the [Sunsoft-3](INES_Mapper_089.xhtml "INES Mapper 089") and [Sunsoft-3R](INES_Mapper_093.xhtml "INES Mapper 093") boards, which are identical besides the default setting for the 9 configuration jumpers. 

The Sunsoft-3R board was by default jumpered for 8kB of CHR RAM; the Sunsoft-3 board for 128kB of CHR ROM. 

  * J1, J2 - CHR's nWR/A14 input: J1=PPU nWR (support CHR-RAM), J2=SS2 A14 (CHR bank bits are contiguous)
  * J3, J4 - SS2's pin 20 input: J3=PPU nRD (support 28 pin 128KiB CHR ROM), J4=ground (support CHR-RAM)
  * J5, J6 - CHR's nOE/A16 input: J5=SS2 A16 (support 28 pin 128KiB CHR ROM), J6=PPU nRD (support CHR-RAM)
  * J7, J8, J9 - mirroring: J7=horizontal, J8=vertical, J9=mapper-controlled one-screen



This IC could be replaced with a 74377 and a 7432: 
    
    
                           74-377
                      .------\/------.       
      (fr) /ROMSEL -> | /E 01  24 +5 | -- +5V
       (r) CHR A13 <- | Q0 02  23 Q7 | -> CHR A16 (r)
       (fr) CPU D0 -> | D0 03  22 D7 | <- CPU D7 (fr)
       (fr) CPU D1 -> | D1 04  21 D6 | <- CPU D6 (fr)
       (r) CHR A14 <- | Q1 05  20 Q6 | -> Q6
       (r) CHR A15 <- | Q2 06  19 Q5 | -> Q5
       (fr) CPU D2 -> | D2 07  18 D5 | <- CPU D5 (fr)
       (fr) CPU D3 -> | D3 08  17 D4 | <- CPU D4 (fr)
     (f) CIRAM A10 <- | Q3 09  16 Q4 | -> Q4
               GND -- | gn 10  15 CP | <- CPU RnW (f)
                      `--------------'
     
                           74--32
                      .------\/------.       
                Q4 -> | A1 01  24 +5 | -- +5V
       (f) CPU A14 -> | B1 02  23 A4 | <- PPU nRD (f)
       (r) PRG A14 <- | Y1 03  22 B4 | <- PPU A13 (f)
                Q5 -> | A2 04  21 Y4 | -> CHR nCS (r)
       (f) CPU A14 -> | B2 05  20 A3 | <- Q6
       (r) PRG A15 <- | Y2 06  19 B3 | <- CPU A14 (f)
               GND -- | gn 07  18 Y3 | -> PRG A16 (r)
                      `--------------'
    

Categories: [Pinouts](Category_Pinouts.xhtml)
