# Talk:UNIF

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AUNIF) | View [other pages](Special_AllPages.xhtml#Talk_UNIF)

## Add UNIF board documentation?

What do people think of adding documentation on the various UNIF MAPRs that don't have iNES encapsulations? I randomly stumbled across [Quietust's Drip game](http://forums.nesdev.org/viewtopic.php?t=1976) and it might be nice to have documentation that's not just part of (e.g.) Nintedulator's source. 

Should they go in subpages under UNIF? e.g. UNIF/UNL-DRIPGAME ? If so, do we want to refile all the iNES mappers to subpages also? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:55, 29 April 2014 (MDT) 

## Dead UNIF, dirty CaH4e3

With the deprecation of UNIF, how do we denote the custom mapper used for Quietust's port of _[Drip](http://www.qmtpro.com/~nes/drip/)_ (UNL-DRIPGAME) and CaH4e3's dumps of pirate games? Does this mean the East Asian plane of NES 2.0 will finally see some action? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 07:25, 11 April 2015 (MDT) 

    I have already wanted to assign a mapper number to UNL-DripGame (as well as Game Genie, but that's a completely different issue). None has been assigned yet. They didn't want to assign a iNES mapper number, therefore I recommend to put into plane 1 instead of plane 0, for this reason. (I also recommend not using plane 0 for any new mappers that require NES 2.0 features.) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 11:09, 11 April 2015 (MDT)

    Personally, I think Koitsu's deprecation of UNIF—and let's be clear, this is his action, not anything else—isn't in good faith. If he wanted to mark it deprecated, that's news to the current maintainer of the standard, and clearly in reaction to zxbdragon's reverse-engineering work. To be perfectly blunt: making a complete GoodNES-like tool and database is a non-negotiable prerequisite to being able to consider UNIF deprecated. Anything else is just posturing.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:46, 11 April 2015 (MDT)

    

    This seems like it's more à propos of the[/a] forum, but I agree with Lidnariq: this seems unilateral. If the **current** maintainer is someone else, and it's still in use in a meaningful sector (like these dumpers) then it doesn't really make sense for the original guy to be permitted to torpedo it, any more than it would make sense for Marat Fayzullin to be allowed to declare iNES format deprecated. UNIF seems like it has at least one point over [i]NES[_2.0]: there's less need for a central authority to decide where to place mappers, as one just uses the board-name. [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 16:16, 3 May 2015 (MDT)
