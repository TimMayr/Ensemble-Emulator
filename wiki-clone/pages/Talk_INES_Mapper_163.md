# Talk:INES Mapper 163

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_163) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_163)

## VRAM Scanline switching

Despite its annoying pirate features, this mapper is on the whole rather simple. A complex scanline counter seems unlikely. The games I've seen all use this feature only for a static title screen, which got me to thinking: What if it isn't scanline based at all? I implemented the following setup in Bizhawk and it appeared to work for all of the games that I could try with no graphical glitching: 

**Whenever PPU A13 transitions from 0 to 1, latch the current value of PPU A9. When the 'c' bit is on, use that latched value instead of PPU A12 as the input to CHR RAM A12.**

(Note that Bizhawk implements an accurate PPU fetch pattern, including all dummy fetches). This method is reminiscent of Mapper 096 and can be implemented with discrete hardware or with a very small number of gates in a ASIC or CPLD. [Natt](User_Natt.xhtml "User:Natt") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Natt&action=edit&redlink=1 "User talk:Natt \(page does not exist\)")) 10:10, 27 January 2014 (MST) 

    Looks plausible... If there are any differences, they'll pop up if any games ever set the Y scroll to something non-zero. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:34, 27 January 2014 (MST)
    That said, a PPUA12-clocked 13-bit counter isn't that complicated either. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:06, 27 January 2014 (MST)

    

    If anyone has such cartridge, then see if you are able to put a program to set Y scroll to nonzero to see if there is a difference which you describe. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 13:47, 27 January 2014 (MST)

    

    

    I am in possesion of cartridge using this mapper (Harvest Moon). Mapper is blob-chip and it indeed utilizes PPU-A13, -A12, -A9, -/RD lines [Krzysiobal](User_Krzysiobal.xhtml "User:Krzysiobal") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Krzysiobal&action=edit&redlink=1 "User talk:Krzysiobal \(page does not exist\)")) 18:23, 27 February 2019 (MST)
    
    
     <https://obrazki.elektroda.pl/5360312600_1551316790.png>
     <https://obrazki.elektroda.pl/7536472600_1551316794.png>
     <https://obrazki.elektroda.pl/8470236300_1551316911.png>
    

    

    

    

    [http://forums.nesdev.org/viewtopic.php?f=3&t=18000](http://forums.nesdev.org/viewtopic.php?f=3&t=18000) [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 22:32, 27 February 2019 (MST)
