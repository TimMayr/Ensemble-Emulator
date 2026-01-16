# Sachen SA8259A pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Sachen_SA8259A_pinout) | View [other pages](Special_AllPages.xhtml#Sachen_SA8259A_pinout)

[Sachen SA8259A](Sachen_8259.xhtml "Sachen 8259"): 28-pin 0.6" DIP 

Most of Sachen's mapper ICs have names that collide with other well-known ICs, presumably as an initial round of indirection to confuse would-be mass copiers. The 8259A is canonically the original IBM PC's [Programmable Interrupt Controller](https://en.wikipedia.org/wiki/Programmable_Interrupt_Controller "wikipedia:Programmable Interrupt Controller"). 
    
    
                  .---\/---.              
       PRG A15 <- | 01  28 | -- Vcc
     CIRAM A10 <- | 02  27 | -> r42
       PPU A11 -> | 03  26 | <- CPU A8 (+CE)
       PPU A13 -> | 04  25 | -> rX0
       PPU /RD -> | 05  24 | <- R/W
             ? ?? | 06  23 | <- M2
       PRG A16 <- | 07  22 | <- PPU A12
       CPU A14 -> | 08  21 | <- PPU A10
       CPU  A0 -> | 09  20 | <- /ROMSEL
           rX1 <- | 10  19 | -> r41
       CPU  D0 -> | 11  18 | <- CPU D2
       CPU  D1 -> | 12  17 | -> r60
       PRG A17 <- | 13  16 | -> rX2
           GND -- | 14  15 | -> r40
                  '--------'
    

The exact connectivity of the pins labelled r?? depends on the corresponding mapper number or name allocated. 

pin \ mapper | [137](INES_Mapper_137.xhtml "INES Mapper 137") | 138 | 139 | 141   
---|---|---|---|---  
rX0 | CHR A10 | CHR A11 | CHR A13 | CHR A12   
rX1 | CHR A11 | CHR A12 | CHR A14 | CHR A13   
rX2 | CHR A12 | CHR A13 | CHR A15 | CHR A14   
r40 | CHR A14¹ | CHR A14 | CHR A16 | CHR A15   
r41 | CHR A14¹ | CHR A15 | CHR A17 | CHR A16   
r42 | CHR A14¹ | CHR A16 | CHR A18 | CHR A17   
r60 | CHR A13¹ | n/c | n/c | n/c   
  
¹: r40, r41, r42, and r60 go through a 74LS253 dual 4 to 1 multiplexer and a 74LS257 quad 2 to 1 multiplexer to produce 4x1+4F CHR banking 

Reference: Cah4e3 ( <http://cah4e3.shedevr.org.ru/%5Blst%5D-sachen-mappers.txt> ) 

  * [Picture of box, cart, and PCB of _Super Pang II_](https://www.flickr.com/photos/153392699@N08/sets/72157679901951752)



Categories: [Pinouts](Category_Pinouts.xhtml)
