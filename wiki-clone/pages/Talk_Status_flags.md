# Talk:Status flags

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AStatus_flags) | View [other pages](Special_AllPages.xhtml#Talk_Status_flags)

The other description on [CPU ALL](CPU_ALL.xhtml "CPU ALL") is pretty good, but I wanted to present the flags in a way that conceptually makes clear that in the processor, they are just 6 separate flags with no ordering in a byte, and that it's when they're _copied_ to a byte in memory that the other two bits of the byte are given values. I tried to emphasize that the memory version is a copy, which conceptually avoids one thinking of the copy as the actual flags. --[Blargg](User_Blargg.xhtml "User:Blargg") 08:37, 5 November 2010 

    So can you think of some way to divide the description between [Status flags](Status_flags.xhtml "Status flags") and [CPU status flag behavior](Status_flags.xhtml "CPU status flag behavior") so that things aren't unnecessarily covered twice? --[Tepples](User_Tepples.xhtml "User:Tepples") 16:44, 5 November 2010 (UTC)
