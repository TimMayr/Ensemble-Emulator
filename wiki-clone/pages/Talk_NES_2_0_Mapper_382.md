# Talk:NES 2.0 Mapper 382

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANES_2.0_Mapper_382) | View [other pages](Special_AllPages.xhtml#Talk_NES_2_0_Mapper_382)

## Inner Bank Register

The bits in the inner bank register change from meaning A16 vs nothing, A15 vs A16, and A14 vs A15 depending on operational mode? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 18:54, 17 November 2019 (MST) 

    Yes, the purpose apparently being that both UNROM and BNROM games can be used without any modification to their bankswitching code. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 23:15, 17 November 2019 (MST)
