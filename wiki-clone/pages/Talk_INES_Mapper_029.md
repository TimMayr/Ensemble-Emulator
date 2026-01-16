# Talk:INES Mapper 029

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_029) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_029)

Without a picture of the board, we can't tell, but we know we have the following functionality: 

  1. 5 bits of state
  2. 3 bits of OR-style PRG banking
  3. No bus conflicts
  4. PRG RAM
  5. The ability to control whether the mapper hardware XOR the flash ROM sees any given write



#3 and #5 could be half of a '139 (EXP0, R/W, /ROMSEL; producing /ROMOE, /ROMWE, and /MAPPERWE; /ROMSEL would connect straight to ROM/CE ). #4 could be anything equivalent to a 4-input NAND gate. #1 and #2 could be any 5+ bit latch and any IC with 3 or more OR or NOR gates. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:47, 16 November 2013 (MST) 
