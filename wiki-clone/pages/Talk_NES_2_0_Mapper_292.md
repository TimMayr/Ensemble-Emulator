# Talk:NES 2.0 Mapper 292

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANES_2.0_Mapper_292) | View [other pages](Special_AllPages.xhtml#Talk_NES_2_0_Mapper_292)

If this is based on FCEUX source then shouldn't it be rather like that: 

  * CPU $8000-$9FFF: 8 KiB switchable PRG-ROM bank selected by writing to $6000.
  * CPU $A000-$BFFF: 8 KiB switchable PRG-ROM bank selected by MMC3 register $07~~, or fixed to last bank if PRG A14 inversion is active~~
  * CPU $C000-$DFFF: 8 KiB PRG-ROM bank, fixed to the second-last bank, or switchable PRG-ROM bank selected by MMC3 register $06 if PRG A14 inversion is active
  * CPU $E000-$FFFF: 8 KiB PRG-ROM bank, fixed to the last bank~~, or switchable PRG-ROM bank selected by MMC3 register $07 if PRG A14 inversion is active~~


  * PRG Bank Register ($6000, write) - Mask: Probably ~~$E001~~ $F001


  * CHR Bank Register ($6000, read) - Mask: Probably ~~$E001~~ $F001



[Krzysiobal](User_Krzysiobal.xhtml "User:Krzysiobal") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Krzysiobal&action=edit&redlink=1 "User talk:Krzysiobal \(page does not exist\)")) 15:12, 23 November 2020 (MST) 
