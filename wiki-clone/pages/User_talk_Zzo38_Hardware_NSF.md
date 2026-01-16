# User talk:Zzo38/Hardware NSF

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3AZzo38/Hardware_NSF) | View [other pages](Special_AllPages.xhtml#User_talk_Zzo38_Hardware_NSF)

You can push some space at the top of the stack on startup if you need to reserve some RAM for your own variables. $8000 is probably not a good location to override; lots of NSFs use it for code. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 23:15, 24 August 2012 (MDT) 

    I agree with you, however, that doesn't count because of: The value of $8000 storing the current song number is only when the main routine ROM is turned on. When the main routine ROM is disabled, it becomes other ROM (if FDS is not used) or RAM (if FDS is used). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 00:20, 25 August 2012 (MDT)

    

    I still don't see any reason it should be at $8000. If you put it elsewhere (e.g. on the stack, or some other internal register mapped to $4xxx somewhere) you don't have to build a complicated switching device for this single address. What's the purpose of making this one particular address special, when everything else in $8000-FFFF otherwise acts in only one way? Are you actually planning to build this in hardware? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 09:20, 25 August 2012 (MDT)

    

    

    Well, after writing it I did think of making it switch a larger amount for mirroring (since the main routine ROM does not need to be very large), or have that byte of RAM mapped to $4xxx somewhere. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 10:26, 25 August 2012 (MDT)
