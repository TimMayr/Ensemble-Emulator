# Talk:Enhancement

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AEnhancement) | View [other pages](Special_AllPages.xhtml#Talk_Enhancement)

## Enhanced graphics via rendering in layers

Hey there, I noticed people are starting up a topic in Enhancement. 

First of all, One thing everyone missed that can be possible, there is a way to make a cart with a extra PPU with different regs to make up new graphics, This method is called _Graphical Layering_. 

It can be simpler than MMC5. But minding the limits, It can be done. 

\--[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") 16:38, 13 November 2011 (UTC) 

    Yeah, Wide Boy and Super Game Boy use that. But someone on the forum told me a long time ago that the NES PPU is more complex than an MMC5, and adding one to a cart would be hideously expensive. Look at [how much bunnyboy charges for a Retrovision](http://www.retrousb.com/product_info.php?cPath=30&products_id=87), for example. --[Tepples](User_Tepples.xhtml "User:Tepples") 17:19, 13 November 2011 (UTC)

    

    Also, neither the PPU EXT pins nor the video passthrough are exposed to the cartridge, which means it cannot be implemented on NES and Famicom cartridges. You could make the emulator to emulate a system that does have these features, but I don't know what's the point. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 15:07, 21 July 2013 (MDT) 

    Technically you don't need the EXT pins or the video passthrough if you're just sending bits 1-0 of each color. You can just stream attributes and CHR data in 8x1 pixel slivers, with one attribute for each sliver. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 17:00, 21 July 2013 (MDT)

    

    

    

    You are talking about a MSX1-style 1bpp graphics mode, Right? That'd be pretty awesome! --[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") ([talk](User_talk_Hamtaro126.xhtml "User talk:Hamtaro126")) 22:29, 21 July 2013 (MDT) 

    Sort of. Instead of specifying two colors for each 8x1 sliver, the mapper would specify 2 bits of "attribute" for whether a sliver uses colors 0-3, 0 and 5-7, 0 and 9-11, or 0 and 13-15. Then the actual pixels in the sliver would have 2bpp. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 22:38, 21 July 2013 (MDT)
    If you are just sending bits 1-0 of each color like you describe (two bits per pixel), then yes it will work, of course; it is clear how the mapper can do that. Also, such a thing would probably be less complicated than the PPU. This is sort of like QBASIC SCREEN 1 but with a different resolution. And yes with more complication you can combine it with 8x1 attribute areas. These are not bad ideas. (This does not give you the features that can be done with EXT pins; if you have the system with two synchronized PPUs and the EXT pins connected in reverse order, then you can get sixteen colors on the screen at once (not including sprites, which adds twelve more), with seven colors per tile!) To implement the 8x1 areas in a mapper, the most obvioiusly way that seems to me, however, would make it very slow to reprogram the graphics during the game. It will have some latches set by the PPU address, and those output to the address of a RAM in the cartridge. To write to it, the program would need to change the PPU address several times in order to set the latches correctly. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 03:29, 22 July 2013 (MDT)

## Metadata INI block for emulation enhancement

I have added a INI block for enhancement to [User:Zzo38/Metadata INI](User_Zzo38_Metadata_INI.xhtml "User:Zzo38/Metadata INI"). It is a draft so there may be a few mistakes in it; if you know the correct way you might fix it. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 02:13, 6 April 2013 (MDT) 

## "Map Logging"

Along these lines are ["atlas encodes".](http://tasvideos.org/Movies-Atlas-Obs.html) I thought more were on TASVideos, but apparently there is a [channel](https://www.youtube.com/user/nesatlas) of them. Here's a topic where they [discuss(ed)](http://tasvideos.org/forum/viewtopic.php?t=10120) them. [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 16:21, 18 February 2016 (MST) 
