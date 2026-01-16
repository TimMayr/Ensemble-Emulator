# Category talk:Mappers with IRQs

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Category_talk%3AMappers_with_IRQs) | View [other pages](Special_AllPages.xhtml#Category_talk_Mappers_with_IRQs)

## Where to file fixed-timing IRQ-sources

I don't think it makes sense to say that "if it's clocked by M2, regardless of how general it is, it belongs in [Category:Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml "Category:Mappers with cycle IRQs")". 1- With such a constraint, the only thing that could possibly make sense in the overarching category would be things that used analog effects for timing ... or the way IRQs are used on "real" computers, where they signal asynchronous events. 2- Because these are not general, it _can't_ be used for arbitrary timing. At best, they're far more crippled than Tepples's demonstration of using the DPCM for mid-screen splits. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:10, 18 May 2015 (MDT) 

    Ah, I didn't understand why those 3 were left out of the cycle category. That sounds like a [new category](Category_Mappers_with_fixed_timing_IRQs.xhtml "Category:Mappers with fixed-timing IRQs") to me, then. Since they're all still cycle counters (even though the timing isn't configurable) I made that a subcategory of cycle IRQs for now, but we can move it back to the parent category if there are other kinds (you mention it on the scanline IRQ category description, but do any exist?). Better to just keep all the fixed ones in their own category, since that's a far more important property than either cycle or scanline counting. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:06, 18 May 2015 (MDT)

    

    For fixed PPU timing, there's [iNES Mapper 091](INES_Mapper_091.xhtml "INES Mapper 091"), and whatever board [zxbdragon found](http://forums.nesdev.org/viewtopic.php?t=12554) here... —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:28, 18 May 2015 (MDT)

    

    

    Okay, I'll move the category up to the parent, then. The reason I thought it would be best to exclude them is that if you want list of "good" cycle/scanline IRQs you probably don't want them in there. Actually... maybe multiple-inheritence would solve this. I'll make two new subcategories, both additionally parented to the fixed-timing one. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:44, 18 May 2015 (MDT)

    

    

    Okay, that's done. I think this should make it easy to find "useful" IRQs, but also easy to find the various subcategories one might be interested in. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:53, 18 May 2015 (MDT)
