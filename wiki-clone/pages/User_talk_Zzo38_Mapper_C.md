# User talk:Zzo38/Mapper C

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3AZzo38/Mapper_C) | View [other pages](Special_AllPages.xhtml#User_talk_Zzo38_Mapper_C)

## Hardware required

  * Two NAND gates making an SR latch using PPU A11 and A10, producing CHR A12 or A13
  * One NAND gate decoding PPU A12 & A13 to make PRG A15
  * One NAND gate decoding two of the three of (/ROMSEL, A12, M2) going to 6264 RAM -CE 
    * The last of those three signals goes to 6264 +CE



A few complicating factors: 

  * the PPU always fetches 34 tiles per scanline, so even with a scroll of (256,240) it will fetch from the nametable at $2800 before it fetches the sprites.
  * At this precise moment, Visual2C02 is simulating the unused sprite fetches as coming from tile 0, not tile $FF as documented elsewhere.



—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:21, 4 January 2014 (MST) 

    This is precisely the hardware I intended to be required. I was aware of the first "complicating factor" but not the second; this probably just means the section about CHR bankswitching should be written more clearly; it should still work as long as the correct assumptions are being made. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 20:11, 4 January 2014 (MST)
