# Talk:Namcot 108 family pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANamcot_108_family_pinout) | View [other pages](Special_AllPages.xhtml#Talk_Namcot_108_family_pinout)

## Inefficiencies and other wiring possibilities

There's absolutely no reason for this discussion, but I wanted to just put my thoughts _somewhere_. 

The N108 uses too many pins for what it delivers. A0's unnecessary (the two registers could have been separated using A13 or A14), and because the IC does not provide WRAM, M2 is also unnecessary. These two pins could then have been used for a bunch of things: PRG A17 and A18, increasing PRG to 512KiB; CPU D6 and CHR A16, increasing CHR to 128KiB; adding CIRAM A10 OUT, adding mirroring control; an interrupt; or if you kept M2, you could add WRAM. 

In a slightly more real-world-constrained version: if you swap N108 A13 IN and N108 A14 IN, you change the banking style to 8+8F+8+8F, allowing for DPCM banking. PRG could be inflated to 256KiB with either 16+16F or 16+16 banking by respectively tying either N108 A13 IN or N108 A14 IN low and then connecting CPU A13->PRG A13, N108 A13 OUT->PRG A14 &c. 

In any case, this is 25 years too late, so whatever. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:53, 8 May 2013 (MDT) 
