# Talk:MMC5 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AMMC5_pinout) | View [other pages](Special_AllPages.xhtml#Talk_MMC5_pinout)

Will the colors still scroll in CL mode even if the pattern isn't? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 12:52, 28 April 2013 (MDT) 

CL mode => PPU controls CHR A0, CHR A1, CHR A2. SL mode => MMC5 controls those lines instead (passing the signal through when vertical split is not used, with a delay). 

In both cases (scrolling or not), the vertical split section have the MMC5 trick the PPU by feeding name and attributes fetches with tiles from EXRAM instead of VRAM (while disabling the real VRAM). When scrolling is used as well, it just fetch from a different address for the coarse scrolling (adding a row to the fetched tile every 8 pixels), and shift the address of the CHR-ROM for fine scrolling. 

So, in CL mode, the MMC5 decides which tile to display while the PPU choose which line of the tile is displaying, while in SL mode the MMC5 decides everything. If ones tries to use a different fine scroll for the main area and the vertical split (EXRAM) area, then the tile themselves will scroll smoothly, but their content won't, and will stick to the main background. This will lead to a quite interesting effect ! 

As for attributes, the MMC5 has to "guess" which attribute byte is fetched basing on the fine scrolling. If it scrolls fine in SL mode (let's assume it does) then it means the colour data is fetched form the correct place within EXRAM, so it will scroll the same way in CL mode, because the CHR-ROM is not involved in any way. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 15:35, 8 February 2014 (MST) 

    Yes, this is exactly what I expected it would do. Is the delay ever significant? What exactly do pins 97 and 98 do, then? Are they used for saving power or something like that? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 01:07, 9 February 2014 (MST)

## CHR enable lines...where?

It seems like pins 29-30 would be the logical place to put them, but is that right? Where are those? Surely they're signals that the MMC5 generates? Is a /WE implemented so as to make CHR-RAM easy?[Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 18:52, 12 July 2016 (MDT) 

    I think Rockman 4 Minus Infinity, which uses MMC5 with CHR RAM, just runs /WE from the cart edge directly to the CHR RAM chip. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 20:42, 12 July 2016 (MDT)

    Because the MMC5 can't do anything peculiar with its pattern table (e.g. no ROM nametables), the carts just normally connect PPU A13 to ROM /CE and PPU /RD to ROM /OE. I wouldn't be surprised if one of pin 29 or pin 30 were (PPU A13 OR PPU /RD), though, to accommodate a never-released board with a 28-pin 128 KiB mask ROM. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 22:58, 12 July 2016 (MDT)

  


## Pin 77

Pin 77 is CPU-R/!W, no idea why somebody ommited this. --[Krzysiobal](User_Krzysiobal.xhtml "User:Krzysiobal") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Krzysiobal&action=edit&redlink=1 "User talk:Krzysiobal \(page does not exist\)")) 20:16, 4 November 2017 (MDT) 

## Chip pinout rotated by 45°

No offense, but this way of representing pinout looks awful in my personal opinion.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 05:52, 5 October 2018 (MDT) 

    I disagree. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 10:39, 5 October 2018 (MDT)

    I don't really understand the alternative you're comparing it to. Every pin needs a line to describe it. The diagonal arrangement makes it practical to give each one a horizontal line of text attached. That's impossible if two of the sides are vertical. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 00:52, 6 October 2018 (MDT)

    I don't have any particular alternative in mind, but before the edit everything seemed fine to me, perhaps I'm missing something.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:22, 8 October 2018 (MDT)
