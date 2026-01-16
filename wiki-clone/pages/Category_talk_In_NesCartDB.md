# Category talk:In NesCartDB

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Category_talk%3AIn_NesCartDB) | View [other pages](Special_AllPages.xhtml#Category_talk_In_NesCartDB)

## But why?

I'm curious what the goal/use of this category is? 

I think an external link in the lead to the DB search entry might be very useful. You could click on it and go to that information. e.g.: <http://bootgod.dyndns.org:7777/search.php?ines=3>

This category, though, doesn't seem to offer any convenience or useful information. If you want to know NesDB information about the mapper, you still have to manually go to the site and search for it. What's the point of the category, exactly? 

It's also woefully incomplete, but that's easy to address. I'm just thinking, if I were to go to the effort of tagging them all, wouldn't it be a better effort to remove the category entirely, and just add links? \-- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 15:37, 18 April 2015 (MDT) 

    The reason I added it was it 1- counts as a loose score of verifiability, and 2- means I don't have to independently ask NesCartDB whether it has a given thing.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 09:47, 20 April 2015 (MDT) 

    I wonder if an "infobox" approach could improve things. A new template `{{[infobox iNES mapper](Template_Infobox_iNES_mapper.xhtml "Template:Infobox iNES mapper")}}` with arguments like `year`, `max_prg_rom`, and `in_nescartdb` would put a mapper in the appropriate categories and generate a "Summary" that floats at the top right of a page. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 13:31, 20 April 2015 (MDT)

    

    

    A lot of the mapper pages have an "overview" section that contains that kind of information. We could start adding an entry about NesCartDB to them (would also be a good place for the category tag, if you do find it useful, though I must admit I don't understand the use just described by Lidnariq). I don't know if an infobox template would work well, because there are so many special cases with mapper descriptions. Whether or not you want to try to use templates to help, having consistent overview information across the mapper pages would be great. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 20:27, 20 April 2015 (MDT)

    

    

    

    I'm reworking the overview into an infobox called `{{[Infobox iNES mapper](Template_Infobox_iNES_mapper.xhtml "Template:Infobox iNES mapper")}}`. If you specify a game count, it'll add this category and make a link to search NesCartDB. See it at the top of [Sunsoft FME-7](Sunsoft_FME_7.xhtml "Sunsoft FME-7"). --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 16:51, 21 April 2015 (MDT)

    

    

    

    

    It's a good start. I left some notes on its talk page. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 17:18, 21 April 2015 (MDT)

    

    

    Another thing probably worth adding to the overview of each mapper page, the iNES mapper number. Where the page is the article about that iNES mapper, the number could just be bold, for pages like [UxROM](UxROM.xhtml "UxROM") where it's a 1 to many relationship, link all the mappers (+ probably a clarifying note). The reason I'm a little iffy about templates for this is that it's difficult to write and modify template code when you need to handle a special case, and there will be lots. Writing wiki markup for an overview section anybody can do easily. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 20:54, 20 April 2015 (MDT)

I created a template to easily make a box of NesDB links. [Template:nesdbbox](Template_Nesdbbox.xhtml "Template:Nesdbbox") I guess this could appear directly beneath an infobox? I've placed the trial run at [AxROM](AxROM.xhtml "AxROM") with [this edit](http://wiki.nesdev.org/w/index.php?title=AxROM&oldid=10302) for now, please let me know if you have suggestions. (Might be better to answer at [Template talk:Infobox iNES mapper](Template_talk_Infobox_iNES_mapper.xhtml#NesDB_linking.2C_non-1:1_issues "Template talk:Infobox iNES mapper").) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:05, 22 April 2015 (MDT) 
