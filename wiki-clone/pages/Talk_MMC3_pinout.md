# Talk:MMC3 pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AMMC3_pinout) | View [other pages](Special_AllPages.xhtml#Talk_MMC3_pinout)

## Pinout with chip rotated 45°

No offense but the pinout with the chip rotated 45° by Lidnariq in july 2012 looks awful in my personal opinion. (How could it go unnoticed by me for 6 years ?!) [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 05:55, 5 October 2018 (MDT) 

    The alternatives all look worse to me.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 10:46, 5 October 2018 (MDT)

## PRG RAM /CE and PRG RAM +CE

How PRG RAM +CE works? Is it just inverted PRG RAM /CE? Or it's controlled via the $A001 register while PRG RAM /CE is low when $6000-$7FFF addressed? [Cluster](https://www.nesdev.org/w/index.php?title=User:Cluster&action=edit&redlink=1 "User:Cluster \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Cluster&action=edit&redlink=1 "User talk:Cluster \(page does not exist\)")) 19:03, 8 August 2021 (UTC) 

    I am pretty sure (but not confirmed) +CE is being controlled by $A001 bits 6 and 7, and /CE is being controlled by the CPU address bus range $6000-7FFF. (/ROMSEL, A14, A13 = 1,1,1) [Ben Boldt](User_Ben_Boldt.xhtml "User:Ben Boldt") ([talk](User_talk_Ben_Boldt.xhtml "User talk:Ben Boldt")) 19:08, 8 August 2021 (UTC) 

    Furrtek's reverse-engineered schematic from the die shots of the ULA [[1]](https://github.com/furrtek/VGChips/tree/master/Nintendo/MMC3C) show that pin is just /RD... but I checked on an MMC3A and MMC3B and pin 30 connects to RAM pin 20 - -CE; and RAM pin 22 is grounded. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 04:43, 22 August 2021 (UTC)
