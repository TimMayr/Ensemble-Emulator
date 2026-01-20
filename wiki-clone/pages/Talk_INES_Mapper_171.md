# Talk:INES Mapper 171

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_171) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_171)

## Differences in documentation

Both [Nestopia](https://github.com/0ldsk00l/nestopia/blob/master/source/core/board/NstBoardKaiser.cpp#L45)'s and [MAME](https://github.com/mamedev/mame/blob/master/src/devices/bus/nes/kaiser.cpp#L290)'s sources claim that mapper 171 = KS7058 is something similar to Sunsoft-1, not MMC1. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:03, 13 April 2018 (MDT) 

    I'm basing my description on [FCEUX' source code](https://github.com/asfdfdfd/fceux/blob/master/src/boards/mmc1.cpp). The ROM image writes the same bank data both to MMC1-compatible addresses and those odd F000/F080 addresses. Since the FCEUX source code names a chip ("KS203"), I assume that FCEUX is closer to what the board hardware actually does. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 16:58, 13 April 2018 (MDT)
