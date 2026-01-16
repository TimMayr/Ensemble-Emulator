# Talk:INES Mapper 183

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_183) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_183)

Suikan Pipe is supposedly a port of Gimmick! to the VRC4. q.v. <http://www.emu-land.net/forum/index.php?topic=67514.0> (in russian). What's really going on here? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 05:15, 25 April 2015 (MDT) 

This description should be removed as it is inaccurate and misleading. Correct description is just "VRC4e + additional latch (latches A3..A0) at $6000-$7fff controlling PRG bank at $6000-$7fff", as stated here: [https://www.nesdev.org/wiki/Gimmick_VRC4_Hack](Gimmick_VRC4_Hack.xhtml)

[Krzysiobal](User_Krzysiobal.xhtml "User:Krzysiobal") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Krzysiobal&action=edit&redlink=1 "User talk:Krzysiobal \(page does not exist\)")) 23:43, 30 April 2023 (UTC) 

    The page you linked to specifically says that it is not about mapper 183. The linked page's version does not use the x800 registers for PRG control. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 10:59, 1 May 2023 (UTC)

    

    Dump of the "VRC4e" catridge is identical to the dump of 183 mapper. Game indeed writes to $x800 registers, but the mask is $F00C, not $F80C, so the 11th bit should be ignored. There is no extra register at $a000 for controlling $c000-$dfff PRG bank, this bank is wired to -2 (why anyone ever thought of existence of this reg? game never writes to $a000). Furthermore, looks like game also never read/executes any code from $c000-$dfff so that's why current mapper implementation of m183 does not break the gameplay) [Krzysiobal](User_Krzysiobal.xhtml "User:Krzysiobal") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Krzysiobal&action=edit&redlink=1 "User talk:Krzysiobal \(page does not exist\)")) 13:57, 1 May 2023 (UTC) 

    Then the claim "Oddly enough, this is not iNES Mapper 183. " must be removed from the linked page as well, otherwise everybody will be confused. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 16:08, 1 May 2023 (UTC)
