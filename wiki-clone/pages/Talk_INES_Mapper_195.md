# Talk:INES Mapper 195

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_195) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_195)

"A NES 2.0-compliant image and emulator will treat mappers 194 and 195 the same; they both specify CHR-RAM pages start at 0 and contain no mirrors." <\-- That's only true for the commonly-available ROMs, which are hacked. The unhacked images of the Chinese Waixing editions of Columbus and Captain Tsubasa II do expect CHR-RAM pages $00-$03 to be mirrored at $28-$2B. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 15:16, 3 March 2018 (MST) 

    What on earth is up with that memory map? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:19, 3 March 2018 (MST) 

    Oh, and in Captain Tsubasa 2, the $28-$2B mirror is only CHR-RAM when assigned to PPU $0000-$0FFF, not to PPU $1000-$1FFF. Also, the game specifically checks that this region is CHR-RAM by reading $2007 and halting when it's not. I think this should could as another lame attempt at protection. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 15:45, 3 March 2018 (MST) 

    Do we have any pictures of the PCBs of any of the various Waixing MMC3 CHR RAM+ROM games? Are they all chip-on-board? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 17:15, 3 March 2018 (MST) 

    Not yet, but I agree they should be obtained. I would assume one common board with a place for a game-specific PAL that assigns specific CHR addresses to CHR-RAM. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 23:50, 3 March 2018 (MST)
