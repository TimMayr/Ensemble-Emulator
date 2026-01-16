# Talk:MMC5 audio

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AMMC5_audio) | View [other pages](Special_AllPages.xhtml#Talk_MMC5_audio)

does the MMC5 DAC accept signed or unsigned samples? --Mukunda 

    They're zero-terminated, so I'd strongly guess unsigned. --[Tepples](User_Tepples.xhtml "User:Tepples") 23:08, 20 February 2010 (UTC)

    It's unsigned. Works exactly like $4011 except it has 8 used bits instead of 7, and writing all bits 0 does not change the counter, though. -- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 19:56, 3 May 2012 (PDT)
