# User talk:Zzo38/Mapper 768

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3AZzo38/Mapper_768) | View [other pages](Special_AllPages.xhtml#User_talk_Zzo38_Mapper_768)

## Extra Data

The [NES 2.0](NES_2_0.xhtml "NES 2.0") format doesn't have any provisions for additional data beyond what the original [iNES](INES.xhtml "INES") format mentions (where the PC10 PROM is often missing and the 128-byte/127-byte "title" is completely optional), so how would emulators know to load the "extra data for mapper 768" without having to hardcode mapper-specific behavior into their file loading logic? Perhaps it might be worth updating the NES 2.0 standard (maybe NES 2.1?) to include a means of specifying additional mapper-specific data (since many emulators close the .NES file before ever initializing the mapper)... --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 19:58, 19 March 2013 (MDT) 

    It is probably not a bad idea; you can do this if you want to. Nevertheless, the extra data is always at the end, after everything else. It will probably be ignored otherwise anyways, such as if you are converting a private mapper (3840 and higher) to mapper 768 or vice versa, or into a standard mapper number (such conversions like this might be useful for testing; for actual conversion into standard mapper numbers you will probably truncate the file, too). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 21:15, 19 March 2013 (MDT)
