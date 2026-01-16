# User talk:Zzo38/Metadata INI

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3AZzo38/Metadata_INI) | View [other pages](Special_AllPages.xhtml#User_talk_Zzo38_Metadata_INI)

## Effectively public domain

By "effectively public domain", did you mean to include all-permissive licenses requiring attribution, such as the MIT License, New BSD License, and GNU Permissive License? The only practical difference between these and public domain status is that all copies of a work under one of these licenses must retain the copyright notice. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 19:00, 18 March 2013 (MDT) 

    No, but I do mean CC0 and Unlicense (as far as I know, these are "effectively public domain"). However, it might be useful to include those too, such as with something like the Creative Commons permissions tags. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 18:39, 19 March 2013 (MDT)

## Move

If anyone else uses it and after discussion in forum or whatever makes it consider to be official, it might be moved (or perhaps copied, if experimental additions don't want to be official?) to the main namespace. For now it is kept here, though. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 01:59, 6 April 2013 (MDT) 

## Mapper times sixteen

I did some refactoring of the controller section. Then for kicks, I tried making an INI for [Thwaite](User_Tepples_Thwaite.xhtml "User:Tepples/Thwaite"), until I came across this in the mapper section: "Number: The iNES mapper number, multiplied by sixteen, plus the submapper number." Why? Mapper numbers are so well known that it'd seem silly to require that they be multiplied. I'd prefer to separate the mapper and submapper number with a dot, such as "71.0" for most Codemasters games or "71.1" for Fire Hawk. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 06:56, 6 April 2013 (MDT) 

    I was expecting hexadecimal to be used (with `0x`) prefix, but yes you are probably right, where using a dot is better. I will change that right now. Thanks for notifying me! --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 21:26, 6 April 2013 (MDT)
