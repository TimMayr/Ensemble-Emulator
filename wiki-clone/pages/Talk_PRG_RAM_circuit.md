# Talk:PRG RAM circuit

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3APRG_RAM_circuit) | View [other pages](Special_AllPages.xhtml#Talk_PRG_RAM_circuit)

## Emulation

Although I have not actually tried it, looking at the FCEUX source codes seems to not emulate PRG RAM in UNROM. There are some games that require it to be absent and some might require it present, this is a mess... you should fix FCEUX; if there are some games that require the absence of PRG RAM in some mappers, then for those mappers you could make it enable PRG RAM only if the battery bit is turned on, even if the game isn't actually using the battery; that seems the clearest way to do it without using NES 2.0. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 16:05, 25 June 2013 (MDT) 

    No commercial UNROM game came with PRGRAM, so it is not reasonable to expect emulators to support it by default. Some games even rely on that area being open bus. Use NES2.0. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 17:52, 25 June 2013 (MDT) 

    Is it reasonable to expect emulators that _don't_ support it by default to support NES 2.0? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 18:03, 25 June 2013 (MDT) 

    Well, it would be reasonable for it to either support NES 2.0 or to make it add 8K PRG RAM when the battery bit is set if $6000-$7FFF isn't already used by that mapper and the battery bit also isn't already used by that mapper; in such a case it would create a save file for the PRG RAM even if the game doesn't use it. Supporting NES 2.0 is probably better (therefore you can add 8K PRG RAM, or 4K battery and 8K non-battery, or vice-versa; the specification ought to be made for the "generic" purpose of the ROM and RAM amount headers if the mapper doesn't normally use them) but not all emulators do. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 18:49, 25 June 2013 (MDT)
