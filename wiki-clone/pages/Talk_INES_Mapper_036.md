# Talk:INES Mapper 036

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_036) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_036)

What does "write $8000-$FFFF: copy RR to PRG banking pins" mean? Does that mean "copy RR to PP", or is it copying RR to some other internal register that's also updated during writes to $4102 (but which does _not_ affect writes to $4100)? --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 10:51, 6 August 2017 (MDT) 

    I've attempted to clarify. Please let me know if I need to try again! —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:05, 6 August 2017 (MDT) 

    I guess that answers my question - it is, indeed, the latter scenario I described. --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 18:35, 6 August 2017 (MDT)
