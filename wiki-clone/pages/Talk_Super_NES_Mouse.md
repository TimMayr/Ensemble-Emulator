# Talk:Super NES Mouse

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ASuper_NES_Mouse) | View [other pages](Special_AllPages.xhtml#Talk_Super_NES_Mouse)

Is there any actual data in the first byte, or is it just all zeroes? --[Quietust](User_Quietust.xhtml "User:Quietust") 17:54, 9 December 2011 (UTC) 

    It appears to be all zeroes in my tests. Kirby's Avalanche for Super NES is known not to check the signature byte; the mouse buttons show up as X and A Buttons. If the first byte were anything but all zeroes, then doing things with the mouse would cause button presses on B, Y, Select, Start, Up, Down, Left, or Right. --[Tepples](User_Tepples.xhtml "User:Tepples") 18:35, 9 December 2011 (UTC)

Since it is done the way it is, it mean, you could have the keyboard and mouse connected at the same time, it seems like. I suppose in some cases it might be useful, but I don't think any such software currently exists. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 14:09, 18 March 2013 (MDT) 

## Sensitivity

I don't see any mention of how the sensitivity affects the output. The nintendulator source seems to treat sensitivity as a multiplier on the returned value (specifically 1.25x, 1.5x, 2.0x). Is this representative of what the original hardware does? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 23:14, 22 April 2016 (MDT) 
