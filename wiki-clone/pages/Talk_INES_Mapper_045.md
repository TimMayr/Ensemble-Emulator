# Talk:INES Mapper 045

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_045) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_045)

The described behaviour of DIP switch read is doubtable. Analyzing the dip readback routine reveals that it first reads 5010 and check for bit 0, if this is nonzero it reads 5020 and check again for bit 0, then it reads 5040, etc. Ony if bit 0 is zero it stops checking, so it should be "0=If DIP switch is in the position selected by the corresponding A" [Krzysiobal](User_Krzysiobal.xhtml "User:Krzysiobal") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Krzysiobal&action=edit&redlink=1 "User talk:Krzysiobal \(page does not exist\)")) 17:46, 7 December 2020 (MST) 
