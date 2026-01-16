# Talk:MMC1 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AMMC1_pinout) | View [other pages](Special_AllPages.xhtml#Talk_MMC1_pinout)

## Contents

  * 1 Tying PA12 low
  * 2 SNROM, etc... loses CHR banking ?
  * 3 SXROM wram
  * 4 PPU A12 / CHR A12



## Tying PA12 low

Based on the description of [NES-EVENT](NES_EVENT.xhtml "NES-EVENT") (#105), it appears to tie the MMC1's PA12 input low and connect the cart's PA12 to A12 of the CHR RAM. --[Tepples](User_Tepples.xhtml "User:Tepples") 08:31, 12 October 2012 (MDT) 

    Given the pictures I can find ([kevtris](http://kevtris.org/mappers/nes_custom/NES_EVENT.html), [kevtris-via-bootgod](http://bootgod.dyndns.org:7777/profile.php?id=4627); both are component side only), it looks like the PA12 input is likely still connected to the cartridge edge. I can't tell anything about whether MMC1 CHR A12 OUT goes to CHR RAM A12. And when I load NWC in FCEUX and trap all mapper writes, it looks like the game just uses 8KB mode anyway. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 11:41, 12 October 2012 (MDT)

## SNROM, etc... loses CHR banking ?

I'm pretty sure they don't, they just bank only 8k as they were always supposed to, and it's possible to have two banks of 4k and bankswitch them independently. I might be wrong, I'll have to double check. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 04:26, 15 May 2014 (MDT) 

## SXROM wram

32K ram does not have an A15 line if the address lines start at the conventional 0, right? pins 9 and 10 should probably be WRAM A13 and A14 respectively? —~~68.107.78.15~~ ([talk](https://www.nesdev.org/w/index.php?title=User_talk:206.174.175.22&action=edit&redlink=1 "User talk:206.174.175.22 \(page does not exist\)")) 19:37, 24 February 2015 (MST) 

    Nice catch! Fixed. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 21:49, 24 February 2015 (MST)

## PPU A12 / CHR A12

Holodnak's last edit implies a board where PPU A12 does not go via the MMC1 to the CHR memory. But to the best of our current knowledge, all boards connect PPU A12 to MMC1 PPU A12 IN and MMC1 CHR A12 OUT to CHR A12. What board did Holodnak find? Are (some of) the special boards (e.g. SNROM, SXROM) that always used 8 KiB of CHR RAM wired in a way that's arbitrarily different without the benefits of freeing up an output? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 21:15, 14 March 2016 (MDT) 
