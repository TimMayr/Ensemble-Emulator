# Talk:VRC2 and VRC4

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AVRC2_and_VRC4) | View [other pages](Special_AllPages.xhtml#Talk_VRC2_and_VRC4)

## § VRC2 vs VRC4

At least [one](https://forums.nesdev.org/viewtopic.php?p=160553#p160553) VRC2-using pirate game actually uses the microwire interface in the VRC2. I can't decide whether this warrants any mention at all... —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:23, 28 April 2017 (MDT) 

    Heh, that's interesting. I'm not opposed to mentioning pirate stuff on the wiki, just I'm not interested much in it myself so I'd never add something like that. In this case it's a hack of River City Ransom? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:58, 28 April 2017 (MDT)

PRG Swap Mode control register is initialized '0' on power up state. Because the reset vector is assigned $e000-$ffff area in all VRC4 games. But some VRC2 games reset vectors uses $c000-$dfff area. If you wants run VRC2 games on VRC4 without reset vector hacking and initialize swap control, games does not run. I think Konami wants remove compatibilities in similar hardwares with a little change.--[Naruko](https://www.nesdev.org/w/index.php?title=User:Naruko&action=edit&redlink=1 "User:Naruko \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Naruko&action=edit&redlink=1 "User talk:Naruko \(page does not exist\)")) 19:42, 2 May 2017 (MDT) 

* * *

A2/A1 - what does that suppose to mean in VRC2/VRC4 table? Shouldn't be A0/A1? It is misleading (Krzysiobal) 

    Thanks, yes that was a mistake. I've fixed it, though please feel free to make edits yourself. (Also the "automatic" way to sign a talk post with your username and date is to type ~~~~ where the signature should go.) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:09, 20 June 2017 (MDT)

* * *

The scanline IRQ category is inappropriate for the VRC4 IMO. The VRC4 does not detect actual scanlines the way the MMC3 or MMC5 does. Rather, its "scanline" mode is just a prescaler for the cycle-based IRQ counter. That makes it a _pseudo_ -scanline IRQ counter at best. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 16:45, 22 December 2021 (UTC) 

    I see what you mean. Here is a perspective where it made sense to me. I was looking at a hack of Super Mario Bros that allows simultaneous 2-players. They used VRC6 for the mapper. I investigated why they use such a rare mapper for this and I found that they are using only bankswitching of CPU $8000-BFFF and scanline IRQ, so they basically had no reason, it just needed to support scanline IRQ. I then came to the nesdev wiki in search of simpler mappers that can do these 2 things (i.e. MMC3). So I want to find all mappers that can generate IRQ based on scanline #. I think the category is useful to search for all mappers that have the ability to generate IRQ based on scanline #. Maybe we can do 2 categories, mappers with scanline IRQs, and mappers with scanline detection, the latter being what you are interested in. What do you think? [Ben Boldt](User_Ben_Boldt.xhtml "User:Ben Boldt") ([talk](User_talk_Ben_Boldt.xhtml "User talk:Ben Boldt")) 20:13, 22 December 2021 (UTC) 

    The distinction between a pseudo- and a real scanline IRQ comes into play in two ways: (1) a true scanline IRQ counter functions on PAL consoles just as well as on NTSC consoles, without having to adjust the value; (2) a true scanline IRQ does not count during vertical blanking. The VRC IRQ requires a different target value for PAL games as for NTSC games -- which is not immediately obvious as there are no PAL games with a VRC chip inside --; just start Kid Dracula in NTSC mode and switch the emulator to PAL mode without resetting and watch what happens to the status bar. The VRC IRQ also counts during vertical blanking, so if you start the counter at the beginning of your NMI handler, you have to add a value of about 20 to the desired scanline value. Those two limitations preclude it from being called a scanline counter at all; it's a scanline-length-prescaled CPU cycle counter. Having "scanline IRQ counters" and "scanline-detecting IRQ counters" categories sounds rather confusing to me. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 22:09, 22 December 2021 (UTC) 

    You have my blessing to change it however you want, including reverting all my changes. I do not feel anywhere near as strongly about it as you do. [Ben Boldt](User_Ben_Boldt.xhtml "User:Ben Boldt") ([talk](User_talk_Ben_Boldt.xhtml "User talk:Ben Boldt")) 23:45, 22 December 2021 (UTC)
