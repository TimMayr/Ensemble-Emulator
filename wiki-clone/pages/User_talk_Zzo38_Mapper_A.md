# User talk:Zzo38/Mapper A

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3AZzo38/Mapper_A) | View [other pages](Special_AllPages.xhtml#User_talk_Zzo38_Mapper_A)

## Boot to last bank

"The mapper will not work properly unless both ports are in output mode, so after reset this should be done at first. All 32K PRG ROM banks should contain the reset code due to this, since otherwise the I/O ports are in input mode and won't assert any logic signals."

Would pull-up resistors on the PRG bank lines allow predictable booting to the last bank? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 08:16, 29 June 2013 (MDT) 

    Probably it would; thank you. The emulator could assume the pull-up resistors are there, and the .NES.INI could specify whether or not the game requires them (a more sophisticated emulator could read the .NES.INI and warn you or something if the game requires them but it says it doesn't). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 11:21, 29 June 2013 (MDT)
