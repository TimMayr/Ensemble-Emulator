# Talk:PowerPak Menu

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3APowerPak_Menu) | View [other pages](Special_AllPages.xhtml#Talk_PowerPak_Menu)

## Other useful questions

Does it reload this mapper automatically after /RESET? by invocation? [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 10:46, 10 July 2017 (MDT) 

    Based on there being [four chips on the board](http://www.vintagecomputing.com/wp-content/images/powerpak/powerpak_inside2_large.jpg) alongside the memory and FPGA, and MAPPERWR being at $8000 (probably actually $8000-$FFFF), I think the UNROM part is discrete logic (and gets enabled by a long reset) and the rest is manually reloaded as almost the first thing the boot ROM does. I don't know if the UNROM chips stay in use during the menu or if the FPGA takes over. [NovaSquirrel](User_NovaSquirrel.xhtml "User:NovaSquirrel") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:NovaSquirrel&action=edit&redlink=1 "User talk:NovaSquirrel \(page does not exist\)")) 01:39, 11 July 2017 (MDT)
