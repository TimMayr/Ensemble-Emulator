# Sunsoft 3 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Sunsoft_3_pinout) | View [other pages](Special_AllPages.xhtml#Sunsoft_3_pinout)

Sunsoft-3: 42-pin 0.6" shrink DIP (Canonically [mapper 67](INES_Mapper_067.xhtml "INES Mapper 067")) 
    
    
                    .---\/---.
      delayed M2 -> | 01  42 | -- VCC                
    (n)       M2 -> | 02  41 | <- EXT /IRQ (n/c)
    (nr) CPU A11 -> | 03  40 | <- CPU D7  (nr)        
    (nr) CPU A12 -> | 04  39 | <- CPU D6  (nr)        
    (nr) CPU A13 -> | 05  38 | <- CPU D5  (nr)        
    (n)  CPU A14 -> | 06  37 | <- CPU D4  (nr)        
    (n)  /ROMSEL -> | 07  36 | <- CPU D3  (nr)        
    (n)  CPU R/W -> | 08  35 | <- CPU D2  (nr)        
    (r)  PRG /CE <- | 09  34 | <- CPU D1  (nr)        
    (n)CIRAM A10 <- | 10  33 | <- CPU D0  (nr)            
    (r)  CHR /CE <- | 11  32 | -> /RAM-CE (n/c)
    (nr) PPU A10 -> | 12  31 | -> RAM+CE  (n/c)
    (n)  PPU A11 -> | 13  30 | <- PPU A13 (n)       
    (n)  PPU A12 -> | 14  29 | <- PPU /RD (n)          
    (r)  CHR A16 <- | 15  28 | -> /IRQ    (n)    
    (r)  CHR A15 <- | 16  27 | -> PRG A14 (r) 
    (r)  CHR A14 <- | 17  26 | -> PRG A15 (r) 
    (r)  CHR A13 <- | 18  25 | -> PRG A16 (r) 
    (r)  CHR A12 <- | 19  24 | -> PRG A17 (n/c) 
    (r)  CHR A11 <- | 20  23 | -> /PRG A17 (n/c)      
         GND     -- | 21  22 | -> $f800.4 (n/c)
                    '--------'
    

Notes: 

  * 1 is delayed by roughly 80ns by an external RC
  * 1 and 2 must both be high to access registers or drive the RAM enables, and a rising edge of this combined signal clocks the IRQ counter.
  * 7 is CPU /A15 in the Vs. System



Source: [[Krzysiobal](https://forums.nesdev.org/viewtopic.php?t=19759)] 

Categories: [Pinouts](Category_Pinouts.xhtml)
