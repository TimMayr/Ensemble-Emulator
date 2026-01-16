# Talk:Nesdev Wiki

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANesdev_Wiki) | View [other pages](Special_AllPages.xhtml#Talk_Nesdev_Wiki)

This page has ad-hoc protection on it to deter spambots. To reply to a thread or start a new thread:  
  
**[Unlock this page](Talk_Nesdev_Wiki_current.xhtml "Talk:Nesdev Wiki/current")**

## Contents

  * 1 Layout test pages
  * 2 Anonymous editing
  * 3 Protocol relative linking?
  * 4 Deletion requests



### Layout test pages

If you have any example of layout for the wiki that you would like to share, please make a link to your examples here. 

  * [User_talk:Blargg](User_talk_Blargg.xhtml "User talk:Blargg") | [User:Blargg/Blargg_Main](User_Blargg_Blargg_Main.xhtml "User:Blargg/Blargg Main") | [User:Blargg/Blargg_PPU](User_Blargg_Blargg_PPU.xhtml "User:Blargg/Blargg PPU")



Would be nice if the <http://nesdev.org/> main page made it clearer that all the up-to-date stuff is on the wiki. It's just two links and fairly easy to miss. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 09:32, 12 April 2013 (MDT) 

## Anonymous editing

Why are non-registered users allowed to edit talk pages ? [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 09:23, 27 March 2014 (MDT) 

    As a means of reporting technical problems with registration to registered users, for one. Almost all automated vandalism gets reverted within twenty-four hours anyway. And because some drive-by edits have in fact been constructive. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 18:29, 27 March 2014 (MDT)
    It also makes spambots less likely to skip immediately to the sign-up page. Instead, they try to post on whatever pages anonymous visitors can post on, where the ~~ABUSE filter~~ picks up on behavior patterns in the bot software. Looking at all the ~~$#!+ that~~ helps me figure out how to stop even those spambots that bother to register. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 19:54, 16 July 2014 (MDT)

I am editing through my router --~~69.20.131.234~~ 17:24, 10 November 2016 (MST) 

    And through my neighbor's router on the same ISP --~~69.20.131.234~~ 17:27, 10 November 2016 (MST) 

    And through 4G LTE on a cellphone. Why do I have the same IP address each time? Reverse proxy much? --~~69.20.131.234~~ 17:30, 10 November 2016 (MST) 

    That must be it. This plus an edit from work are all in the same /30 as the result of `nslookup wiki.nesdev.org`. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 17:32, 10 November 2016 (MST)

It looks like WhoaMan came back after a long absence, upgraded phpBB, and [made a change to the BBS's proxy settings](https://forums.nesdev.org/viewtopic.php?p=244282#p244282). But that change didn't propagate to the wiki. `nslookup wiki.nesdev.org` returns 38.141.52.251, while I appear to be coming from an address in the same /30: ~~38.141.52.250~~ 11:42, 21 November 2019 (MST) 

## Protocol relative linking?

Noticed this edit by lidnariq: [diff](https://wiki.nesdev.org/w/index.php?title=GTROM&curid=1857&diff=16180&oldid=16179)

I understand the value of using "protocol relative" links to the forum, which omit the http: or https: prefix, so that the user's choice remains continuous (though it's more of a roulette situation coming here from the forums). However, I don't understand why we'd want to do this for links to anywhere else. Shouldn't we just link to whichever the "default" form of link is for the external site? AFAIK most websites with https don't really want users to use http anyway? NESDev seems like a very unusual case for treating them as interchangeable. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:19, 19 January 2019 (MST) 

    I figured that for anything that doesn't force the user to just use https, using protocol-relative links preserves Rahsennor's use case (caching proxies necessarily break https). That's the only even vaguely good reason that I was working with. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 22:13, 20 January 2019 (MST)

    Another potentially problem with protocol relative linking is if you download the file into your computer and then access it locally, then it might not work properly. If a client software supports user URL overrides then you can specify your own preference in case of caching proxies or whatever other reasons you might need. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 12:39, 21 January 2019 (MST)

    

    Saying "user error" is a total cop-out, but that is what it is ... By definition wget/curl shouldn't change what they received over the wire without being told to do otherwise, and _all_ relative links – not just protocol-relative – require rewriting (e.g. wget -k). Saving pages using firefox or chrome also does this rewriting. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 20:21, 21 January 2019 (MST)

## Deletion requests

See: 

  * {{[delete](Template_Delete.xhtml "Template:Delete")}}
  * [Category:Deletion requests](Category_Deletion_requests.xhtml "Category:Deletion requests")



I created a {{delete}} template that editors can use to flag pages that should be reviewed for deletion by moderators, which collects them in that category. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 01:07, 17 March 2023 (UTC) 
