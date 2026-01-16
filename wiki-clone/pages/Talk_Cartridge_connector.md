# Talk:Cartridge connector

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ACartridge_connector) | View [other pages](Special_AllPages.xhtml#Talk_Cartridge_connector)

## Contents

  * 1 Orientation
  * 2 CIRAM A10
  * 3 Audio In vs Out
  * 4 Label on pins 58 and 65 a typo?
  * 5 /IRQ & M2
  * 6 Logic thresholds



## Orientation

I'm a little confused about this. Why does the pin orientation for the NES appear to be viewed from underneath the connector? Wouldn't it make more sense if it was a top-down view like on the famicom diagram? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 20:45, 26 August 2012 (MDT) 

    Probably to be consistent with standard pinout numbering, which always goes counterclockwise and has pin 1 as top-left. It's also consistent with the edge on the mainboard. Not the best of reasons... —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 20:53, 26 August 2012 (MDT)

    

    It make it very counter-intuitive if you're trying to do something related to Famicom<->NES conversion. I could see it being more useful if you're soldering something to the back of a toploader board, but other than that case I find it really confusing. Would you mind if I reversed it to be consistent with the Famicom version? I could reverse the numbers alongside it, if it's important that they stay the same. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 21:12, 26 August 2012 (MDT)

    

    Ahh, I see that many Nintendo NES carts are numbered in this way, but I still think it'd be better just to have the numbering proceed upside down. As it is right now you would have to be looking at the card-edge upside down to see the same orientation as this diagram (and the numbers on the board would be upside down as well). - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 21:37, 26 August 2012 (MDT)

    

    

    The hesitations I have to reordering it is 1- that the parallelism between the Famicom and NES header isn't as obvious and 2- the presented pin order isn't consistent with the industry convention. But these aren't good enough reasons for me to stop you. (But they're good enough for me to not do the edit myself)—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 23:21, 26 August 2012 (MDT)

    

    

    

    Okay, I have reversed the NES diagram. The numbering in both diagrams is consistent with the numbering on Nintendo PCBs, so I made no changes to that. I don't know what industry convention is for numbering pins of a card-edge connector, but I don't think it's relevant here, since Nintendo provided their own numbering on their PCBs (and if one of them meets the industry standard, the other is backwards). I added some notes about the orientation and how the two systems relate to each other. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 10:47, 27 August 2012 (MDT)

    

    

    

    What is the industry convention for card edges? I'm used to pin orders being counter-clockwise around a chip from the top. Is it different for a card edge? The Nintendo boards are numbered left to right on one side, right to left on the other, rather than clockwise or counterclockwise around the board. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 12:41, 27 August 2012 (MDT)

    

    

    

    

    I meant "same as IC convention", but I think industry convention for card edges is (letter)(pin number) in line, without any rotation. e.g. ISA has A01-A31, B01-B31, C01-C18, D01-D18, arranged as
    
    
    B01 B02 … B30 B31   D01 … D18
    A01 A02 … A30 A31   C01 … C18
    

    

    

    

    

    —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 12:52, 27 August 2012 (MDT)

## CIRAM A10

If you connect CIRAM A10 to PA6 (the adjacent pin), will this work for a fixed single page mirroring (it won't actually be contiguous in RAM but as far as I can tell it could work)? If so, do any known cartridges do this? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 14:44, 23 September 2012 (MDT) 

## Audio In vs Out

Just curious about this change: [http://wiki.nesdev.org/w/index.php?title=Cartridge_connector&diff=5344&oldid=4339](http://wiki.nesdev.org/w/index.php?title=Cartridge_connector&diff=5344&oldid=4339)

The descriptions on this page of Audio Out and Audio In are relative to the Famicom, not the cartridge, so "Audio Out" means out from the 2A03, and "Audio In" means audio in from the cart. Is that in line with the change you made, Lidnariq? (And if not, perhaps we should revise the descriptions, and/or use a more descriptive naming than In/Out to avoid future confusion.) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 21:46, 19 January 2013 (MST) 

    Good point. I changed it because I'd noticed I'd RE'd the SS5B schematic incorrectly due to misunderstanding the names, but I agree that "in" and "out" are insufficiently unambiguous. So now this should be plenty clear.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 23:34, 19 January 2013 (MST)

## Label on pins 58 and 65 a typo?

[[1]](http://www.kevtris.org/nes/nestuff2.txt)Kevtris has pin 58 as CHR A13 and 65 as CHR /A13. Does the Nes Dev Wiki have them wrong or does Kevtris? — Preceding unsigned comment added by [1iquidsnake](https://www.nesdev.org/w/index.php?title=User:1iquidsnake&action=edit&redlink=1 "User:1iquidsnake \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:1iquidsnake&action=edit&redlink=1 "User talk:1iquidsnake \(page does not exist\)") • ~~contribs~~) 03:33, 27 September 2016 (UTC)

    Kevtris's document is in error. You'll note that almost all boards connect PPU /A13 to CIRAM /CE so that nametable RAM can be mapped from PPU $2000-$3FFF; these two pins are adjacent and you can easily see this jumpering of adjacent pins by looking at a board. — Preceding unsigned comment added by [Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq") • ~~contribs~~) 04:50, 27 September 2016 (UTC)

\---I checked out my schematics for Punch Out, which uses a MMC2L (42pin), and it shows pin 22 going to pin 31 of the CHRROM which is /CE which is also tied to edge connector 65 (CHR /A13 per Kevtris). And pin 34 goes to CHRROM pin 28 which is CHR 13. If the documentation is right then that would make CHRROM pin 31 CHR A13. But that would be wrong. Can someone else confirm this info? — Preceding unsigned comment added by [69.20.131.234](User_69_20_131_234.xhtml "User:69.20.131.234") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:69.20.131.234&action=edit&redlink=1 "User talk:69.20.131.234 \(page does not exist\)") • ~~contribs~~) 13:55, 27 September 2016 (UTC)‎

    [MMC2 pinout](MMC2_pinout.xhtml "MMC2 pinout") pin 22 is PPU A13, which is almost always used as CHR-ROM /CE. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 09:12, 27 September 2016 (MDT)

  


## /IRQ & M2

_/IRQ : (...) Can only be connected to an open collector cartridge output_

Not true - some cartridges uses for example 74*74's Q or /Q to control /IRQ line, which is not tri-state. 

    In this case, I guess that trying to trigger a DMC or fame counter IRQs would result in a short-circuit between that chip and the 2A03 ?[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 04:37, 3 November 2017 (MDT)

_M2 : (...)When this signal is high, this means the CPU bus address and data lines are in a stable state and can be latched by external hardware_

Not true for data bus - it is driven by CPU (on writes) approximatelly in middle of M2's high interval \--[Krzysiobal](User_Krzysiobal.xhtml "User:Krzysiobal") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Krzysiobal&action=edit&redlink=1 "User talk:Krzysiobal \(page does not exist\)")) 19:40, 31 October 2017 (MDT) 

## Logic thresholds

Does anyone know the logic thresholds, VIH, VOH, VIL, VOL? Are outputs from the NES/Famicom all CMOS? How about inputs to the NES/Famicom: Are those CMOS? Are they even known to be consistent? I would like to put notes about that here. [Ben Boldt](User_Ben_Boldt.xhtml "User:Ben Boldt") ([talk](User_talk_Ben_Boldt.xhtml "User talk:Ben Boldt")) 21:45, 27 March 2022 (UTC) 

    Based on some off-the-shelf chips used:

     | VOH (min)  | VIH (min)  | VIL (max)  | VOL (max)  | Where Used   
---|---|---|---|---|---  
SN74LS373N  | 2.4 | 2 | 0.8 | 0.4 | PPU A0-A7   
cxk5816ps-15l  | 2.4 | 2.2 | 0.8 | 0.4 | CIRAM A10, CIRAM /CE   
SN74LS139N  | 2.7 | 2 | 0.8 | 0.4 | /ROMSEL   
MN74HC368  | 4.9 | 3.51 | 0.99 | 0.1 | PPU /A13   
  
    But I still do not know where to get info for the CPU and PPU direct connections, which is most of them. Given that PPU +A13, M2, and CPU /INP0 all drive inputs of MN74HC368 chips, it suggest that they both must have CMOS outputs. So, PPU A0-A7 are TTL (because of 74 _LS_ 373), and A8-A13 CMOS?? It is a mixture like that for real? And does this mean that the data buses need CMOS levels when they are an input to the CPU or PPU? [Ben Boldt](User_Ben_Boldt.xhtml "User:Ben Boldt") ([talk](User_talk_Ben_Boldt.xhtml "User talk:Ben Boldt")) 19:31, 29 March 2022 (UTC) 

    It's a mixture, but you just assume it's TTL thresholds. Because the NES is NMOS, not CMOS. To actually quantify the data bus inputs, we'll need to build a specialty cart with transmission gates (e.g. two 4016s) and a voltage reference. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 18:28, 30 March 2022 (UTC)
