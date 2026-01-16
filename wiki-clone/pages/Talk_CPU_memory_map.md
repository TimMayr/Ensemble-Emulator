# Talk:CPU memory map

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ACPU_memory_map) | View [other pages](Special_AllPages.xhtml#Talk_CPU_memory_map)

## A mapper could switch just the vectors, but it would be very unusual. Do any?

In response to Myask's question in the [11011 edit summary](http://wiki.nesdev.org/w/index.php?title=CPU_memory_map&diff=11011&oldid=11003), the PowerPak's NSF mapper does this. I don't know any other examples offhand, though. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 15:45, 28 May 2015 (MDT) 

    Arguably the PowerPak is not a mapper but a metamapper. On the other hand, I wonder how deeply (if at all) the Powerpak can emulate itself. On that note, metamapper seems like a useful term...or just another word for multicart mapper.[Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 16:09, 28 May 2015 (MDT)

    

    Well, I'm not talking about the whole PowerPak. I'm only talking about one of the mappers used for it. The NSF player mapper. Actually, I think Kevtris' hardware NSF player does the same thing, and there may be others. The TNS line of NSF playing cartridges do not; they simply replace the table in the initially mapped high bank of the ROM, and expects it to remain fixed (or else it crashes). NSF players are the only application I can think of that really require this. Game ROMs always have their own bank-appropriate vector tables. NSFs are unique in that the data doesn't already have vectors (the player is expected to provide them, if needed by the implementation), and the specification permits paging that has to share space with the vectors. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 22:11, 28 May 2015 (MDT)

## DMC DMA accessible region

In the most recent edit by Pubby, this was added: 

If using DMC audio: 

  * $C000-$FFF1 = DPCM samples



Where does $FFF1 come from? According to the page [APU DMC](APU_DMC.xhtml "APU DMC"), it says these 2 things: 

  * Sample address = %11AAAAAA.AA000000 = $C000 + (A * 64)
  * The address is incremented; if it exceeds $FFFF, it is wrapped around to $8000.



Although the minimum sample address possible to write to $4012 appears to be $C000, it would appear by the second point that doing a wraparound could in fact access more DPCM samples stored at $8000. Is this correct? [Ben Boldt](User_Ben_Boldt.xhtml "User:Ben Boldt") ([talk](User_talk_Ben_Boldt.xhtml "User talk:Ben Boldt")) 20:25, 15 February 2022 (UTC) 

    It is correct that a sample will wrap from $FFFF to $8000, though because the highest sample address is $FFC0 and the maximum length is $FF1 bytes, only $8000-8FB0 in this lower area can be accessed for DPCM playback. Even without this wrapping, I feel the latest update is a little misleading because the $FFF1-FFFF region is playable, even if doing so is generally undesirable because it should normally contain the vectors. (Another, minor nit is that these regions should use inclusive indexing on both ends, so the $FFF1 upper limit should be $FFF0.) I think the DPCM mapping is worth including on this page, but I'd like it to be more complete. Despite the vectors, the wrapping does potentially have practical applications; zeta0134 has suggested using DMC DMA to set MMC5's PCM level via its read mode, which is only available in the $8000-BFFF region. [Fiskbit](User_Fiskbit.xhtml "User:Fiskbit") ([talk](User_talk_Fiskbit.xhtml "User talk:Fiskbit")) 05:28, 17 February 2022 (UTC)
