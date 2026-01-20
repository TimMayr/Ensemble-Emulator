# Talk:6264 static RAM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3A6264_static_RAM) | View [other pages](Special_AllPages.xhtml#Talk_6264_static_RAM)

## More pinouts for different sizes of 62* SRAM's

Some extra pinouts for SRAMs: 62256 is identical except for the !CS2 input and NC becoming address lines. This is to make address expansion of a board design almost trivial using inverters as half of an address decoder (other half is !CS input).  
  
CY62256 SRAM (28 pins, 1-14,28-15) Note that the Ax and Dx line order doesn't really matter.  
A5 +5V  
A6 !WE  
A7 A4  
A8 A3  
A9 A2  
A10 A1  
A11 !OE  
A12 A0  
A13 !CS  
A14 D7  
D0 D6  
D1 D5  
D2 D4  
Gnd D3  
  
62512 SRAM (32 pins, 1-16, 32-17)  
A18 +5V  
A16 A15  
A14 A17  
A12 /WE  
A7 A13  
A6 A8  
A5 A9  
A4 A11  
A3 /OE  
A2 A10  
A1 /CS  
A0 D7  
D0 D6  
D1 D5  
D2 D4  
Gnd D3  
  
621024 SRAM (32 pins, 1-16, 32-17)  
NC +5V  
A16 A15  
A14 CS2  
A12 /WE  
A7 A13  
A6 A8  
A5 A9  
A4 A11  
A3 /OE  
A2 A10  
A1 /CS1  
A0 D7  
D0 D6  
D1 D5  
D2 D4  
Gnd D3  
  
LY6220488 TSOP-package SRAM (44 pins, 1-22, 23-44)  
A4 A5  
A3 A6  
A2 A7  
A1 !OE  
A0 CS2  
!CS A8  
NC NC  
NC NC  
D0 D7  
D1 D6  
+5V Gnd  
Gnd +5V  
D2 D5  
D3 D4  
NC NC  
A20 NC  
!WE A9  
A19 A10  
A18 A11  
A17 A12  
A16 A13  
A15 A14  
  
DS1249Y/AB SRAM (32 pins, 1-16, 32-17) Note that this is equivalent to 622048 but AFAICT, no one makes those.  
NC +5V  
A16 A15  
A14 A17  
A12 !WE  
A7 A13  
A6 A8  
A5 A9  
A4 A11  
A3 !OE  
A2 A10  
A1 !CS  
A0 D7  
D0 D6  
D1 D5  
D2 D4  
Gnd D3  
  
DS1250Y/AB SRAM (32 pins, 1-16, 32-17) Note that this is equivalent to 624096 but AFAICT, no one makes those. These are $47 at Maxim's site as of time I last checked! The 1251 is also a 4Mb part, BTW.  
A18 +5V  
A16 A15  
A14 A17  
A12 !WE  
A7 A13  
A6 A8  
A5 A9  
A4 A11  
A3 !OE  
A2 A10  
A1 !CS  
A0 D7  
D0 D6  
D1 D5  
D2 D4  
Gnd D3  
  
These would need to be edited to match formatting of the existing Wiki article.  


    Forgot to sign, and it seems I may have made some mistakes? Pretty sure I got those off of the original datasheets, but maybe a typo snuck in. Can someone please check these? It doesn't make sense that a 512Kb device has more pins than a 1024. I think that's what the article means by "breaking the pattern" since it's apparently 512K**B** which is 4096 K**b**. Maybe I didn't make a mistake at all! _Nope, I did. One of my PDFs was mislabeled. UM62512 is definitely only 64KB._

[alphamule](User_Alphamule.xhtml "User:Alphamule") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Alphamule&action=edit&redlink=1 "User talk:Alphamule \(page does not exist\)")) 13:42, 28 January 2015 (MST) I found this site which has a nice explanation of the various differences on the bigger chips. <http://sblive.narod.ru/ZX-Spectrum/ZX-SEGA/ZX-SEGA.htm> Seems also that JEDEC has references. Search for "3_07_05R12.pdf" and you'll find it pn page 3.7.5-11. [alphamule](User_Alphamule.xhtml "User:Alphamule") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Alphamule&action=edit&redlink=1 "User talk:Alphamule \(page does not exist\)")) 13:52, 28 January 2015 (MST) 
