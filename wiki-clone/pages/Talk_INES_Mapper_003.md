# Talk:INES Mapper 003

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_003) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_003)

## KB or KiB

Some people change KiB to KB and some other people change KB to KiB. Personally, I want to punch anyone who thinks KB means anything but 1024 bytes, but they do have a point and its probably for the best. On the whole I have no opinion. But rather than zap everyone's edits repeatedly, we should probably decide what this wiki is gonna use and stick to it. --[Zeromus](User_Zeromus.xhtml "User:Zeromus") 23:47, 24 June 2012 (PDT) 

    IEC specifies "kB=1000 bytes and KiB=1024 bytes"; JEDEC specifies "KB=1024 bytes". There are two contexts: (A) storage capacities of solid-state memories with raw access and (B) everything else. The only platforms that I've seen using a CPU whose speed is specified in MiHz are the Game Boy (4 MiHz), Game Boy Color (8 MiHz), and Game Boy Advance (16 MiHz). Everything else is in MHz (e.g. the 315/176 MHz speed of the NTSC NES CPU). Rates are likewise specified in base-10 units, such as kHz and kbps. Even the sizes of flash memories wrapped in a standard block device interface such as CF or SATA SSDs are specified in MB (10^6 byte) or GB (10^9 byte); I gather that the drive's controller uses the 5 to 7% difference between the stated GB capacity and the raw GiB capacity for wear leveling. Anyway, Wikipedia appears to specify JEDEC usage at [wikipedia:Wikipedia:COMPUNITS](https://en.wikipedia.org/wiki/Wikipedia:COMPUNITS "wikipedia:Wikipedia:COMPUNITS"), but I disagree with for precision's sake. --[Tepples](User_Tepples.xhtml "User:Tepples") 08:01, 25 June 2012 (PDT)

## Submappers

Suggestion: submapper 1 for no bus conflicts, submapper 2 for bus conflicts, submapper 0 to determine by hashes. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 18:41, 10 September 2012 (MDT) 
