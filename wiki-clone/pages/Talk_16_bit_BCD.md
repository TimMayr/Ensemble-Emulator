# Talk:16-bit BCD

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3A16-bit_BCD) | View [other pages](Special_AllPages.xhtml#Talk_16_bit_BCD)

These kind of description of arithmetic stuff is useful. There might be useful to have a category for this. I also asking about a signed version, but I suppose such thing wouldn't be too difficult (print a minus sign and then XOR by 65535 and then add one, and then do it with unsigned). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 17:07, 24 August 2013 (MDT) 

    Good idea. I made [Category:Arithmetic](Category_Arithmetic.xhtml "Category:Arithmetic"). You can probably populate it by going through the last week of ~~Special:Contributions/Koitsu~~. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 17:30, 24 August 2013 (MDT)

I seem to remember optimizing this routine way back, at the very least to eliminate that unnecessary sec in the loop. Make it a clc, adjust the low table, and take the clc out of the loop since the bcc ensures it's clear each time through.[Blargg](User_Blargg.xhtml "User:Blargg") ([talk](User_talk_Blargg.xhtml "User talk:Blargg")) 11:06, 25 November 2013 (MST) 
