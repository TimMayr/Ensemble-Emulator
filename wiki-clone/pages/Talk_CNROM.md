# Talk:CNROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ACNROM) | View [other pages](Special_AllPages.xhtml#Talk_CNROM)

## Bankswitching and $2007 reads

I wonder if this info is any useful, especially the second sentence : 

    Many CNROM games such as Milon's Secret Castle store data tables in otherwise unused portions of CHR ROM and access them through $2007 reads. If an emulator can show the title screen of the NROM game Super Mario Bros., but CNROM games don't work, the emulator's $2007 readback is likely failing to consider CHR ROM bankswitching.

The technique to read non-graphics data from CHR ROM is of course worth mentioning, there's no doubt about that, but I don't feel like this is the right page to mention it, as it has nothing to do with CNROM / mapper 3 itself, in fact games using all kind of mappers does that (Super Mario Bros (NROM), Tiny Toons Adventure 2 (MMC3), Batman - Return of the Joker (FME-7), etc....) This should be mentioned on a page which deals directly with CHR-ROM or something. 

As for emulators where the title screen of SMB works, and that Milon's Secret Castle doesn't work, well how many of them there are ? I really see zero pertinence in this sentence. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:40, 5 May 2014 (MDT) 

    People tend to implement the simpler mappers first, so they're more likely to collide with this failure mode while implementing NROM and CNROM long before they could notice it with MMC3 or FME7. I can see an argument that the information belongs only in [Tricky-to-emulate games](Tricky_to_emulate_games.xhtml "Tricky-to-emulate games"), but it fits very nicely in the lede here, and [Tricky-to-emulate games](Tricky_to_emulate_games.xhtml "Tricky-to-emulate games") is a poorly-organized info dump... —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 03:20, 5 May 2014 (MDT)

    

    I understand, but what is emu-dev stuff doing just below the headline of a mapper/hardware page ? I'm not saying the info is useless, just that it is out of place. The fact another page is a poorly-organized info dump is not an excuse to leave info that doesn't belong to here here.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 04:03, 5 May 2014 (MDT)

    

    

    1- A mention of this problem in some form belongs on this page anyway, because emulator authors will implement CNROM before the more complicated games that combine backswitching with 2007 reads.
    2- Removing this germane, clear, and unambiguous piece of information is of negative utility without a better place to put it. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:55, 5 May 2014 (MDT)

## Security Diodes

About the security diodes, I noticed that a [Japanese Gradius board](http://bootgod.dyndns.org:7777/profile.php?id=1569) has slots for 4 diodes instead of 2 (but only 2 are used). Surprisingly, many non-Nintendo made boards cloning the CNROM mapper (Konami, Taito, Bandai) also have those same diodes. Considering how few sense those makes, it's very surprising they would all use them, isn't it ? [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:40, 5 May 2014 (MDT) 

I've moved the NROM diode discussion to [Talk:NROM](Talk_NROM.xhtml "Talk:NROM"); it doesn't belong here. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:55, 5 May 2014 (MDT) 
