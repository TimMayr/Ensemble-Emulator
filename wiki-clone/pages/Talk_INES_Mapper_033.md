# Talk:INES Mapper 033

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_033) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_033)

Preliminary research done over Overload seems to suggest that: 

  1. Disch's writeup of mapper 33 is an accurate description of the TC0190 all by itself.
  2. The TC0350 has an IRQ as described by mapper 48 but _LEAVES THE MIRRORING CONTROL BIT_ at $8000.6
  3. The addition of the PAL16R4 to the TC0190 moves the mirroring control bit to $E000.6
  4. A few games (e.g. Captain Saver, Bubble Bobble 2) both rely on the IRQ and the moved mirroring control bit (i.e. what Disch wrote for mapper 48). We have no pictures of these games' PCBs to figure out what's going on.



—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:47, 9 September 2013 (MDT) 

    Overload found a picture of a game of type #4. It appears to be the [TC0690](http://www.crazysmart.net.au/phpBB3/viewtopic.php?f=10&t=3). —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:40, 22 October 2013 (MDT)

## Games listing

Per the information in GoodNES 3.14, Nestopia's database, and NEScartDB, the games for mapper 33 and mapper 48 are: 

TC0190 | Akira (J) [!]   
---|---  
TC0190 | Bakushou!! Jinsei Gekijou (J) [!]   
TC0190 | Bakushou!! Jinsei Gekijou 2 (J) [!]   
TC0190 | Golf Ko Open (J)   
TC0190 | Insector X (J)   
TC0190 | Operation Wolf (J)   
TC0190 | Power Blazer (J)   
TC0190 | Takeshi no Sengoku Fuuunji (J) [!]   
TC0190+PAL16R4 | Bakushou!! Jinsei Gekijou 3 (J) [!]   
TC0190+PAL16R4 | Don Doko Don 2 (J)   
TC0350 | Don Doko Don (J)   
m48 | Bubble Bobble 2 (J)   
m48 | Captain Saver (J)   
m48 | Jetsons, The - Cogswell's Caper! (J)   
TC0690 | Flintstones, The - The Rescue of Dino & Hoppy (J)   
  
  * "m48" means "TC0190+PAL16R4 or TC0690 but not known which"
  * IC names are from photographs from NEScartDB or Overload



—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 21:13, 21 August 2016 (MDT) 

## TC0350

Here's a pinout for the TC0350 I made with Multi-Meter Continuity test: 
    
    
               .--\/--.
        GND -- |01  48| -- VCC
        N/C -- |02  47| <- M2
    PRG /OE <- |03  46| -> PRG A14
        N/C -- |04  45| -> PRG A13
    PRG A15 <- |05  44| -> PRG /CE
    CPU A13 -> |06  43| <- D7
    CPU A14 -> |07  42| <- D6
     CPU A1 -> |08  41| <- D5
     CPU A0 -> |09  40| <- D4
         D0 -> |10  39| <- D3
         D1 -> |11  38| -- N/C
         D2 -> |12  37| -> CHR A17
    /ROMSEL -> |13  36| -> CHR A14
    CPU R/W -> |14  35| -> CHR A13
       /IRQ <- |15  34| <- PPU A8
    PPU /RD -> |16  33| <- PPU A9
    CHR A16 <- |17  32| <- PPU A10
    CHR A15 <- |18  31| -> CHR A11
    CHR A12 <- |19  30| <- PPU A11
     PPU A7 -> |20  29| -> CHR A10/CIRAM A10
     PPU A6 -> |21  28| <- PPU A12
     PPU A5 -> |22  27| -> CHR /OE
     PPU A4 -> |23  26| <- PPU A13
        GND -- |24  25| -- VCC
               .------.
    

Hopefully didn't make any mistakes. Can't edit myself. 
