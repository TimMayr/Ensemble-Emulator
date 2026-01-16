# VRC6 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VRC6_pinout) | View [other pages](Special_AllPages.xhtml#VRC6_pinout)

Konami [VRC6](VRC6.xhtml "VRC6"): 48-pin 0.6" PDIP marked "053328 VRC VI", "053329 VRC VI", or "053330 VRC VI" (iNES Mappers [24](VRC6.xhtml "INES Mapper 024") and [26](VRC6.xhtml "INES Mapper 026")) 

The Die of "053330" contains the markings "SLA7340 C3T0", a 1.5µ 2-metal-layer CMOS high speed gate array made by S-MOS Systems, a Seiko-Epson affiliate. 
    
    
                     .---\/---.  
              GND -> | 01  48 | -- +5V
           AUD D1 <- | 02  47 | -> AUD D0   
           AUD D3 <- | 03  46 | -> AUD D2
           AUD D5 <- | 04  45 | -> AUD D4
     (fr) CPU A12 -> | 05  44 | -> PRG A16 (r)
      (f) CPU A14 -> | 06  43 | <- CPU A13 (f)
           (f) M2 -> | 07  42 | -> PRG A17 (r)
      (r) PRG A14 <- | 08  41 | -> PRG A15 (r)
     (fr) CPU  A1 -> | 09  40 | -> PRG A13 (r)
     (fr) CPU  A0 -> | 10  39 | <- CPU D7 (fr)
     (fr) CPU  D0 -> | 11  38 | <- CPU D6 (fr)
     (fr) CPU  D1 -> | 12  37 | <- CPU D5 (fr)
     (fr) CPU  D2 -> | 13  36 | <- CPU D4 (fr)
      (r) PRG /CE <- | 14  35 | <- CPU D3 (fr)
          (f) R/W -> | 15  34 | <- /ROMSEL (f)
         WRAM /CE <- | 16  33 | -> /IRQ (f)
      (r) CHR /CE <- | 17  32 | -> CIRAM /CE (f)
      (f) PPU /RD -> | 18  31 | <- PPU A10 (f)
      (f) PPU A13 -> | 19  30 | <- PPU A11 (f)
      (r) CHR A16 <- | 20  29 | <- PPU A12 (f)
      (r) CHR A15 <- | 21  28 | -> CHR A17 (r)
      (r) CHR A12 <- | 22  27 | -> CHR A14 (r)
      (r) CHR A11 <- | 23  26 | -> CHR A13 (r)
              GND -- | 24  25 | -> CHR/CIRAM A10 (r)
                     `--------'
     
     1: On packages 053328 and 053330, ground supply. On 053329, an unknown input
     2-4, 45-47: goes to a 6 bit DAC, and then mixed with system audio
       The DAC is a 6-bit R-2R SIP resistor array with an additional internal 2K resistor in series to the output.[[1]](https://www.nesdev.org/vrcvi.txt)
                     3K     3K     3K     3K     3K            2K
           +------+-\/\/-+-\/\/-+-\/\/-+-\/\/-+-\/\/-+------+-\/\/-+
           |      |      |      |      |      |      |      |      |
           \      \      \      \      \      \      \      |      |
           /      /      /      /      /      /      /      |      |
           \ 6K   \ 6K   \ 6K   \ 6K   \ 6K   \ 6K   \ 6K   |      |
           /      /      /      /      /      /      /      |      |
           |      |      |      |      |      |      |      |      |
           O      O      O      O      O      O      O      O      O
          GND     D0     D1     D2     D3     D4     D5   To RF  From 2A03
                                                       (Cart.46) (Cart.45)
    
     9,10: VRC6a (Mapper 24: Akumajou Densetsu) is shown.  For VRC6b (Mapper 26: Madara, Esper Dream 2), these pins are swapped.
     16: passes through a small pulse-shaping network consisting of a resistor, diode, and capacitor.
     32: The VRC6 supports ROM nametables.
    

Categories: [Pinouts](Category_Pinouts.xhtml)
