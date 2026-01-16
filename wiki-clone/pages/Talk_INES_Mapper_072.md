# Talk:INES Mapper 072

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_072) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_072)

"Pinball Quest will require bus conflicts to be emulated if this mapper is emulated this way." <\-- What is "this way", and what would be the "other" way? The game seems to run fine when not emulating any bus conflicts. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 01:05, 28 May 2020 (MDT) 

    There are a few places where Pinball Quest generates bus conflicts despite having bus conflict prevention tables, but as far as I can tell they're in places where it doesn't matter - by which I mean, the bus conflict happens in the lower bits when restoring the upper bits to 0. [Looking at the edit history](https://wiki.nesdev.org/w/index.php?title=INES_Mapper_072&diff=3813&oldid=3213), "this way" appears to be the belief that the latches were looking for _falling_ edges, instead of the rising edges that they do. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 21:34, 30 May 2020 (MDT)
