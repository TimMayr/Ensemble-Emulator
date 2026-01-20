# Talk:Action 53 mapper

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AAction_53_mapper) | View [other pages](Special_AllPages.xhtml#Talk_Action_53_mapper)

## Pin count

Do few-enough-pin PALs/CPLDs still exist such that pointing out "there's no reason to connect D6 at all and D7 in mode selection could be changed to D5 and free up another input pin" is even remotely useful?—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 19:25, 13 October 2012 (MDT) 

    It might free up pins in the current design, but I'm thinking ahead to how oversize expansions would work should the time come. And I don't see how you're getting 25 pins; it looks like 28 to me, especially with the equivalent of a [7420 for PRG RAM decoding and bus conflict avoidance](PRG_RAM_circuit.xhtml "PRG RAM circuit") included. --[Tepples](User_Tepples.xhtml "User:Tepples") 20:30, 13 October 2012 (MDT) 

    I don't think I said 25 pins anywhere? Unrelatedly, my having suggested D5 might not be the best because of this future-proofing you said; using D4 or D1 might be cleaner. But regardless, it looks like there aren't any PALs or CPLDs both cheaper and fewer pins than the XC9536XL infiniteneslives had specified in the [forum](http://forums.nesdev.org/viewtopic.php?p=101165#p101165), so I think my initial "hey, maybe..." is irrelevant. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 01:19, 14 October 2012 (MDT)

### 512-KiB Mode - Is it Possible?

* * *

I don't think you can fit a 512-KiB game for use in that mapper right now, I think you can replace a mode, or use a new mode if it fits? I possibly doubt it though due to constraints... 

Even then, I like the idea of a multicart like this! --[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") 01:56, 14 October 2012 (MDT) 

    An oversize BNROM game will fit, though only one such game at a time unless each game's bank number table is modified to correspond to its position in the cart. This is the same technique that I had planned for BNROM games in Action 53 before I learned that homebrew developers prefer UNROM over BNROM. --[Tepples](User_Tepples.xhtml "User:Tepples") 22:15, 3 November 2012 (MDT)

## Writable flash

There are rumors on the BBS that INL will make boards that allow in-circuit programming of the beginning of the PRG flash in 4096 byte sectors as a method for saving the game. An extra 8 KiB of SRAM will be mapped at $6000 for managing a log-structured file system. I think I know how to represent this in NES 2.0: if non-backed and backed PRG RAM are present, the non-backed memory is SRAM at $6000 and the backed memory replaces the first pages of the ROM and is writable through [Common Flash Memory Interface](https://en.wikipedia.org/wiki/Common_Flash_Memory_Interface "wikipedia:Common Flash Memory Interface") sequences. I'm waiting on INL for details before I add it. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 20:08, 31 January 2014 (MST) 
