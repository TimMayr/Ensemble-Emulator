# User talk:Zzo38/Famicom Hangman

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3AZzo38/Famicom_Hangman) | View [other pages](Special_AllPages.xhtml#User_talk_Zzo38_Famicom_Hangman)

## Fixing the excuses

I looked at what you thought was keeping the game out of the multicart project. 

  * "Too large ROM, wrong mapper": I can add BNROM games to A53 by modifying the values in the table you called `buscon`. I can also help you convert this game to BNROM if you want. [A commercial Hangman-like game for NES](http://bootgod.dyndns.org:7777/profile.php?id=1383) uses [a mapper identical to BNROM except for scrolling](AxROM.xhtml "AxROM"), and Hangman doesn't really scroll.
  * "Requires keyboard": You could add an on-screen keyboard.
  * "Trademark": If you were calling it "Wheel of Fortune", that'd be one thing. But I don't see any issues with the name "Hangman" at least on [Wikipedia:Hangman (game)](https://en.wikipedia.org/wiki/Hangman_\(game\) "wikipedia:Hangman \(game\)"). Who has a trademark on "Hangman" where you live?



\--[Tepples](User_Tepples.xhtml "User:Tepples") 19:06, 30 September 2012 (MDT) 

    Some of the included phrases ("Nintendo Family Computer", "Dungeons & Dragons", "Wheel of Fortune") might have trademark issues, is what I meant; not the title. Still, if I get enough of other good phrases I may be able to remove the trademark issues. (Even if I did call it "Wheel of Fortune", the trademark issue is not the only thing; it also does not have the wheel and that stuff!) I may add an on-screen keyboard later on too, to make it work with standard controller (although I intend to keep the support for the Famicom keyboard as well). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 20:42, 30 September 2012 (MDT)

## trydata

Where did you get the values used for `trydata`? 
    
    
    	; Number of tries by number of letters
    trydata	.db 0,11,10,9,8,7,6,6,5,5,5,5,5,5,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,3
    

\--[Tepples](User_Tepples.xhtml "User:Tepples") 19:06, 30 September 2012 (MDT) 

    They are based on a BBS door game ("StarTrek Guess" on X-BIT), with some changes made for shorter phrases. But these values may still not be very good so if you have any better ideas then please tell me. (Even if you don't, I may change them myself if I have my own better ideas.) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 20:42, 30 September 2012 (MDT)
