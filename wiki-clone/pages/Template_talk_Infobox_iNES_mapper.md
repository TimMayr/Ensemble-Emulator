# Template talk:Infobox iNES mapper

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Template_talk%3AInfobox_iNES_mapper) | View [other pages](Special_AllPages.xhtml#Template_talk_Infobox_iNES_mapper)

## Contents

  * 1 Don't Repeat Yourself?
  * 2 NesDB linking, non-1:1 issues
  * 3 General improvements
  * 4 Multi-infoboxes and suggestion about Overview purpose
  * 5 CHR-RAM
  * 6 Proposed field: Multicart mapper of...



## Don't Repeat Yourself?

"If you provide nescartdbgames, it automatically inserts..." Is there a way to likewise automate/fold into the box [Mappers with bus conflicts](Category_Mappers_with_bus_conflicts.xhtml "Category:Mappers with bus conflicts")? [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 16:52, 21 April 2015 (MDT) 

    Yes, once I figure out a bit more string manipulation to distinguish "yes" and "Yes" and "Y" from "no" and "No". --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 19:26, 21 April 2015 (MDT)

## NesDB linking, non-1:1 issues

I think it would be better if the template was just for "mapper" not "iNES mapper", simply because a lot of articles are organized around something other than iNES number (Nintendo board, multiple iNES, no iNES, etc.) 

I think we need more comprehensive NesDB linking, as well. For each thing there are several relevant ways to search NesDB, and we should probably have all of them in the infobox. Also, there are much cleaner and succinct URLs you could be using, e.g.: 

  * <http://bootgod.dyndns.org:7777/search.php?unif=NES-CNROM>
  * <http://bootgod.dyndns.org:7777/search.php?ines=3>



If an artice is about 3 different iNES mappers, it would ideally have a seachlink to each relevant iNES, should probably have a search link for each relevant board too. I dunno if the "number of games" is very useful either; not sure how often that changes, but if it's just copy pasted from the search result, it's easy to find it there. Seems like an annoying bit of extra maintenance; why do I need a number of games to create an NesDB link? I feel like the link text should have something to do with the mapper/board used for search, not some number you have to look up manually, maybe something like: NesDB [CNROM] -br- NesDB [iNES 001] -br- NesDB [iNES 185], I dunno. 

Also, how come only the first iNES mapper number has leading 0s? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 17:03, 21 April 2015 (MDT) 

    I dunno, just "number of games" seems a really strange thing to put in. I get that we want to have an idea of the relative frequency of the mapper, but the NesDB is full of regional duplicates, etc. it's really hairy to try and come up with a number that means what you might intuitively think it does. Actually following the link and using the database directly is a much better way to figure out what you need. The stats page is especially good for this: <http://bootgod.dyndns.org:7777/stats.php> \- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 17:10, 21 April 2015 (MDT) 

    You technically don't need a number of games; you just need a non-empty string like "Few" or "Many". The query I'm using already removes region dupes. I'd put in support for multiple NesCartDB queries, but I'm still unsure of my skill in making variadic templates. And UNIF is officially dead. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 19:25, 21 April 2015 (MDT)

    

    

    I don't think "few" or "many" is useful information. It's not information at all. It's useless. If you can't or won't meaningfully fill in a field, we shouldn't force nonsense into it. I also don't care at all what the status of "UNIF" is to you, but it's the search parameter for PCB names in NesDB, and it's a useful search parameter. Rather than making the template variadic, you could always just let the user type whatever stuff they want in that field and let them manually make links. This is one of the reasons I preferred the existing Overview section to an infobox: editing them for special cases is easy to understand. Adding all the special cases to the template is complicated and error-prone (i.e. anybody can edit wiki text, but probably only you should touch the complex template code you created). The other big reason is that we often need to make notes and qualifications about entries in the overview which are quite inappropriate for an infobox. I'm okay with the infobox, though, we just have to be careful not to sacrifice quality of information for the sake of the template. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 20:07, 21 April 2015 (MDT) 

    "Few" or "many" is ideally something to put in when clicking "Show preview". And the special cases are why, for example, I left the free-text half of [MMC5#Overview](MMC5.xhtml#Overview "MMC5") alone.
    An alternative might be just to create a separate NesDB box, if that's easier than trying to wrangle it all into the infobox. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 20:10, 21 April 2015 (MDT)

    

    

    One more note, to search as 1-per region you can add "&group=groupid" to the search link, though I don't know if that's a good default. Different people would want different information, it's probably best to use the simplest (and most inclusive) form of the search. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 20:17, 21 April 2015 (MDT)

I created a simple box for making multiple NesDB links. I have placed it on [AxROM](AxROM.xhtml "AxROM") as an initial test with [this edit](http://wiki.nesdev.org/w/index.php?title=AxROM&oldid=10302). Please let me know if you have suggestions about it. Doing it this way might be easier than trying to work it into the infobox. I don't think the infobox can really handle multiple cases very well, most mappers have several relevant NesDB cases (various NES board types, one or more iNES mappers associated with the page, etc.) so it might be better to just have a simple "NesDB" box. It also automatically tags the [In NesCartDB](Category_In_NesCartDB.xhtml "Category:In NesCartDB") category as well. (Sorry if this is a duplicate of discussion at [Category talk:In NesCartDB](Category_talk_In_NesCartDB.xhtml "Category talk:In NesCartDB"), not sure where the best place to ask is.) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:11, 22 April 2015 (MDT) 

## General improvements

I fixed a bug in the template (prgmax check was looking at chrmax) - [Koitsu](User_Koitsu.xhtml "User:Koitsu") ([talk](User_talk_Koitsu.xhtml "User talk:Koitsu")) 16:00, 22 April 2015 (PDT) 

I disagree with some "assumptions" in the current logic in the template: 1) if prgmax is defined to blindly assume there's a prgpage default of unknown (recommended: don't bother printing the line at all if prgpage isn't explicitly set), 2) if chrmax isn't set to blindly assume 8K (I believe there are mappers which have no CHR) (recommended: don't bother printing the line at all if chrmax isn't explicitly set), 3) if chrmax is defined to blindly assume there's a chrpage default of 8K (recommended: don't bother printing the line at all if chrpage isn't explicitly set). Good examples of a couple of these are on the [CNROM](CNROM.xhtml "CNROM") page which I've updated - [Koitsu](User_Koitsu.xhtml "User:Koitsu") ([talk](User_talk_Koitsu.xhtml "User talk:Koitsu")) 16:00, 22 April 2015 (PDT) 

    I have made `prgpage` independent from `prgmax` and `chrpage` independent from `chrmax`. But mappers designed for use without CHR (Magic Floor, Game Genie) are such an edge case that `chrmax=None` ought to satisfy the template. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 16:23, 30 April 2015 (MDT)

## Multi-infoboxes and suggestion about Overview purpose

Lidnariq is experimenting with multiple infoboxes at Namco 163: [[1]](http://wiki.nesdev.org/w/index.php?title=Namco_163&diff=10408&oldid=10392)

IMO this looks nightmarish, though it might not be so bad if the infoboxes could stack horizontally. However, I have a different suggestion. 

It might be best to just keep the old "Overview" sections in addition to the infobox? PGCX864 made a series of Overview-to-infobox edits earlier that ended up erasing a bunch of subtle but important information, and a lot of this stuff it's hard to find a good place for when putting it back. 

If we kept the Overview, sticking multiple mappers into a single infobox might not be such a problem. The Infobox would be just for a "quick overview" of what you're going to find in the article, and the Overview section itself can have the proper details. 

For an example of what I mean, if we had a mapper article that has 3 mappers with different PRG-ROM capacities, the infobox would be inclusive with the information, and the overview section would resolve it in detail. The point of the infobox being a summary of the article, and the point of the overview being a detailed summary of all the mappers included. 

Infobox: 

**PRG ROM capacity:** | 64K / 128K / 512K   
---|---  
  
Overview: 

    Mapper A:
    PRG ROM capacity: 64K
    
    Mapper B:
    PRG ROM capacity: 128K
    
    Mapper C:
    PRG ROM capacity: 512K

Alternative: 

    PRG ROM capacity: 64K (A), 128K (B), or 256K (C)

Whichever form seems to make more sense. Is this a sensible suggestion? Maybe the infobox isn't really a good alternative to the Overview sections, but rather a good supplement/addition to it. You know what I mean? At the same time, I think this would make it appropriate to move category tags back into the overview, rather than attempting to resolve all the cases with complex infobox code. \- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:14, 23 April 2015 (MDT) 

    I'd be inclined to agree with leaving unusual Overview stuff in Overview. Notice that I left the Overview section in the MMC5 article because there was so much that would not fit in the infobox. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 14:31, 23 April 2015 (MDT)

## CHR-RAM

I realize CHR-RAM is almost always an option via the iNES format, but on physical boards it's often not quite as trivial to go between ROM and RAM implementation. Shouldn't it be a line on this infobox? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 19:18, 29 April 2015 (MDT) 

    Whether the PPU /WR line is available depends more on the board than on the mapper. Some boards have this pin on the cart edge, not connected to anything. This apparently includes all Famicom boards. As far as I can tell, modding these to take a [62256](6264_static_RAM.xhtml "6264 static RAM") in the CHR slot isn't harder than modding a game with Nintendo [mask ROM pinout](Mask_ROM_pinout.xhtml "Mask ROM pinout") to use JEDEC flash in the CHR slot. Others, especially later NES boards, omit it to save some piddly amount of money per board. Hence the search for ["full pin SMB2"](http://forums.nesdev.org/viewtopic.php?f=28&t=6224) (that is, early revisions of NES-TSROM) to mod into a TNROM-compatible donor for reproductions of translated _Final Fantasy III_. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 21:58, 29 April 2015 (MDT)

    

    Okay, but my question was whether it should be included in the infobox. Some board-classes may have variants with and without CHR-RAM, but a lot of them were just one way or the other. It's a useful thing to know about a mapper (whether it is a single-board mapper, or a class-of-boards mapper) that it had CHR-RAM always, sometimes, or never. We've had [Category:Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml "Category:Mappers with CHR RAM") for a long time, and I've thought it was good knowledge to have. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:52, 30 April 2015 (MDT) 

    My fear is that emulator authors might misinterpret such a declaration in the infobox as meaning that a particular class of boards can never have CHR RAM, and any homebrew program that uses CHR RAM necessarily exhibits [what the C programming language standard calls "undefined behavior"](https://en.wikipedia.org/wiki/Undefined_behavior "wikipedia:Undefined behavior"). It'd have to be carefully worded to avoid this. I find this important because I'm working with another programmer on an NES project that uses CHR RAM with a mapper that had used only CHR ROM during the NES's commercial era. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 16:02, 30 April 2015 (MDT)

    

    

    

    If you're afraid that emulator authors are going to do something stupid, you should advise them against it in a much more direct way, not with some passive organization of infobox article summaries. The iNES format makes CHR-RAM optional for every mapper, unless there is some structural conflict (maybe this should be stated explicitly on the iNES article, and possibly in the "mappers with CHR RAM" category text). This wiki is for more than emulator authors, it is for anyone involved in NES development, anyone who wants to do research on boards and mappers, and all sorts of other uses. Knowing which boards were used for CHR-RAM is important. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 17:07, 30 April 2015 (MDT)

    

    Also I might as well recommend a move to: [Template:Infobox mapper](https://www.nesdev.org/w/index.php?title=Template:Infobox_mapper&action=edit&redlink=1 "Template:Infobox mapper \(page does not exist\)") since we don't (and shouldn't) organize the mapper articles per-iNES. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 15:08, 30 April 2015 (MDT) 

    Such a naming would imply that this infobox shall be made no longer specific to mappers that have had an iNES number assigned. What would go in the mapper number entry for a mapper that lacks a number or has an NES 2.0 number greater than 255? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 16:02, 30 April 2015 (MDT)

    

    

    

    Well, all of the infoboxes created so far are about a board-class rather than an iNES mapper specifically. Why shouldn't mappers that aren't iNES have infoboxes too? I don't really understand this restriction. It should be optional. Also, the decision to exclude "NES 2.0" from being an "iNES mapper" is tremendously pedantic and silly at face value. iNES mappers are easily categorized already by the mapper grid, the iNES mapper category, and the redirect pages for iNES mappers that are better described under some other category. I don't think the infobox actually helps this organization; the infobox is to make it easier to survey mappers at a glance. It's a page-summary, not a categorization or organization of pages. The content of the mapper articles should dictate the needs of the infobox, not the other way around. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 17:02, 30 April 2015 (MDT)

  
In any case, it seems odd that there is no way to include this staple-of-Overview-section datum...and if a new variant is made with a different configuration, that's a new "board variant" (even if it's only on emulator), and the information can be updated. Until then, it seems prudent to stick to extant/used variants in our descriptions, just like we do with mapper numbers. [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 15:41, 3 May 2015 (MDT) 

## Proposed field: Multicart mapper of...

An optional field to show which mappers a multicart mapper can encompass would be handy, I feel. (As would a category for multicart mappers.) [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 16:12, 28 May 2015 (MDT) 
