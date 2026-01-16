# Talk:Detect TV system

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ADetect_TV_system) | View [other pages](Special_AllPages.xhtml#Talk_Detect_TV_system)

Can RGB detect be added? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 12:16, 8 March 2013 (MST) 

    Detecting the difference between the 2C02 and 2C03 comes down to detecting 1 missing pixel out of every 178684 pixels. Thefox tried to do this in the forum - [http://forums.nesdev.org/viewtopic.php?f=2&t=9210](http://forums.nesdev.org/viewtopic.php?f=2&t=9210) \- but it didn't work very well. Detecting the 2C04 or 2C05 is easy, but they're incompatible with existing software, so it's not clear why one would bother. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:51, 8 March 2013 (MST) 

    Then what is the best way for the software to deal with the difference of RGB and NTSC modes? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 13:53, 11 March 2013 (MDT) 

    Compile time, or asking the user. You cannot plug a Famicom or NES game pak into a Vs. System or PC10 cabinet, and aftermarket modifications (2C03s in NESes/Famicoms) impose no obligation to support. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:29, 11 March 2013 (MDT) 

    A Game Pak plugs into a [Famicom Titler](https://en.wikipedia.org/wiki/Famicom_Titler "wikipedia:Famicom Titler") or a [C1 NES TV](https://en.wikipedia.org/wiki/C1_NES_TV "wikipedia:C1 NES TV"), both of which use the same RGB PPU as the PlayChoice. However, it appears developers settled for "compile time", as the box of _Just Breed_ carries a warning not to use the game with Sharp Famicom TVs. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 17:08, 11 March 2013 (MDT)

## source formatting errors

I'm noticing bne :+ is being displayed as bne ;:+ and some of the comments have the ; doubled for some reason. I think there's something wrong with the source tag. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 22:31, 5 March 2014 (MST) 

    Mentioned on the forum- <http://forums.nesdev.org/viewtopic.php?p=124294#p124294> . Conclusion seems to be "Argh" —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 02:02, 6 March 2014 (MST)
