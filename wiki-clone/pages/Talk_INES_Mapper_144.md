# Talk:INES Mapper 144

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_144) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_144)

## Alternative interpretation

This mapper could also be thought of as "ROM wins all bus conflicts", instead of only the LSB. The only place a bus conflict is elicited is where the game writes $00 to a place in ROM that contains $A9, while requiring that PRG bank 1 remain mapped. (The CHR bankswitch specified by $A in the upper nybble is irrelevant: the game switches back to the correct bank afterwards)—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 21:35, 9 September 2015 (MDT) 
