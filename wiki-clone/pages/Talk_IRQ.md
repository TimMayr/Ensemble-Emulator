# Talk:IRQ

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AIRQ) | View [other pages](Special_AllPages.xhtml#Talk_IRQ)

Is it really impossible to acknownledge a DMC IRQ by reading $4015 ? It's weird this register has a DMC IRQ flag, but that reading it would not ack the interrupt. Also it sounds fishy you'd have to stop the sample and restart another one to acknownledge.Bregalad 14:00, 24 April 2012 (PDT) 

    Whatever [my DPCM Split and DPCM Letterbox demos](Projects.xhtml#stuff_by_Damian_Yerrick "Projects") are doing works on an NES. --[Tepples](User_Tepples.xhtml "User:Tepples") 16:15, 24 April 2012 (PDT)

## Table

Just how comprehensive do we want this list to be? Obviously including every single mapper with IRQs is overkill (q.v. [my list, sort it by IRQ presence](User_Lidnariq_MMC3_Variants.xhtml#ASIC_mappers_with_simple_banking "User:Lidnariq/MMC3 Variants")), but does it make sense to include non-US mappers here? If so, why these ones? (e.g. there were more games released on the Namco 163 (20) than the VRC4+6+7 combined (16)) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 18:28, 17 May 2015 (MDT) 

    Yeah, actually the whole table is probably useless. Knowing some value to enable/disable/ack an IRQ is too little information to do anything useful with it (except the disable, I guess). Really that information is best left at the article page. What we really need is [Category:Mappers with IRQs](Category_Mappers_with_IRQs.xhtml "Category:Mappers with IRQs"), and your list is a great TODO list for building that category. I guess this article needs a rewrite with a better focus, probably. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:50, 17 May 2015 (MDT)

    

    Before I go and start adding that Category to everything ... do we want to distinguish at the category level between "quantized to scanlines" and "cycle-based" ? I'd vaguely be inclined to, although there's some fuzziness for the ones that can be switched between cycle-based and scanline-quantized. (Obviously both of those categories would be subcategories of "Mappers with IRQ")... —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:56, 17 May 2015 (MDT)

    

    

    Sure, I don't see why not. You could just put something in both categories if it can do both. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 20:13, 17 May 2015 (MDT)
    Sure. Whether it uses the PPU or CPU as a time base is especially important for whether a program would need to be adapted for the longer vblank and slower clock divider of the PAL NES. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 21:01, 17 May 2015 (MDT) 

    Hm. Where should we file [VRC IRQs](VRC_IRQ.xhtml "VRC IRQ") then? I was originally going to say with the scanline IRQs, but if the argument here is 2C02-and-Dendy-vs-2C07, it might then instead belong in cycle-based...—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 21:37, 17 May 2015 (MDT)

    

    

    

    

    VRC is cycle based. It's just cycle based with an interface that's convenient for working with scanlines. Any cycle based counter can do what VRC does, but no cycle based counter can do what a true scanline counter does. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 22:23, 17 May 2015 (MDT)
