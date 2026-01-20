# Talk:PPU pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3APPU_pinout) | View [other pages](Special_AllPages.xhtml#Talk_PPU_pinout)

## EXT port description

Can the function of the EXT pins be described more precisely? I do not understand very well, there are four EXT pins, and the colors are six bits (selecting the palette entry is only two bits), so how can it decide the color from this? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 00:27, 21 January 2013 (MST) 

    I am having a doozy of a time figuring out how to rephrase it on the main page in an intelligible manner, so I'll just start from scratch: Assume the NES's palette has no gaps and thus is a {32 entry} × {6 bit} array. log₂(32)=5, so indexing this array requires 5 bits. The EXT port takes as input or output the bottom 4 bits of this 5 bit index. If the EXT port is used as an input, it replaces the "transparent" color in index 0 with any of colors 0-15, i.e. it's an extra layer of background and background-colored sprites underneath. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 01:59, 21 January 2013 (MST) 

    OK, thanks, that makes sense, now I can understand. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 03:37, 25 January 2013 (MST)

## SYNC -> RST

Through analysis of Visual2C02 and cross-referencing of the [Family Computer schematic](http://nesdev.org/Ntd_8bit.jpg), I've decided to rename pin 22 from "/SYNC" to "/RST" because that's clearly what it does (and I also renamed /VBL to /INT). However, there is still the matter of this comment: "(In a dual-PPU arrangement, the master /INT is also connected to the slave's /RST input)". This _cannot_ be the case, because it would result in the chips becoming **desynchronized** \- /INT goes high at the beginning of scanline 261 (pre-render), but /RST places the chip at the beginning of scanline 0 **and** forces rendering to be disabled for a complete frame. --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 09:33, 31 March 2013 (MDT) 

    I imagine that the two chips' /RST inputs would be tied together. There'd also have to be glue logic to keep some semblance of synchronization between writes to $2001 on both PPUs so that neither PPU is put in a state where one skips the dot between pre-render and the first line of picture and the other does not. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 12:32, 31 March 2013 (MDT)
    Yeah, the more dwe refined the functions of the pins, the less I trusted that comment.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:23, 31 March 2013 (MDT)

## Rename some pins?

[This wiring diagram](https://www.nesdev.org/w/images/default/f/f3/Neswires.jpg "Neswires.jpg") calls the A2-A0 pins on the PPU RS2-RS0, which I'm guessing stands for Register Select. I think that might be a less confusing name for them. The /CS line is also called /DBE (prolly Data Bus Enable), which might be better than /CS since (I think) /CS might imply that the entire PPU would be disabled rather than just the CPU-to-PPU bit. Mind if I rename those pins? -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 12:04, 22 May 2013 (MDT) 

    What is more useful, to describe what the pins are known as in isolation, or what they connect to? I personally prefer the latter, especially for inputs.
    I object to using /DBE. The "/CS" terminology is well established industry practice used by peripheral ICs (e.g. MC6845, YM2149), not just memories: it requires less explanation to someone who already has background.
    If we are simply going to use Nintendo's terminology, we should also rename CPU pins 1, 2, 29, 35, and 36; furthermore they used different terms for CPU pins 3, 29, 31, and 35-39 in the Vs System documentation (<http://nesdev.org/VSSCHEM.pdf>) than the Famicom (<http://nesdev.org/Ntd_8bit.jpg>) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:07, 22 May 2013 (MDT) 

    What makes it confusing in this case is that some of the VRAM address pins are also called Ax. It's easy to think that e.g. A0 and A8 are related. I'd prefer to label the pins according to what they do and also explain what they connect to. (Well, except in cases where it's immediately obvious from what they connect to, like CLK, etc. :)
    Looks like you're right about /CS. Maybe not as confusing/unexpected as it seemed coming at it as a beginner then. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 14:18, 22 May 2013 (MDT)
