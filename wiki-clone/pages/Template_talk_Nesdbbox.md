# Template talk:Nesdbbox

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Template_talk%3ANesdbbox) | View [other pages](Special_AllPages.xhtml#Template_talk_Nesdbbox)

If you can set unif's search type to be "unif_op=LIKE+`%25%40%25`" then we can use SQL GLOBs in the template rather than having to enumerate all the variants, q.v. "[http://bootgod.dyndns.org:7777/search.php?unif_op=LIKE+%60%25%40%25%60&unif=A%25ROM](http://bootgod.dyndns.org:7777/search.php?unif_op=LIKE+%60%25%40%25%60&unif=A%25ROM)" —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 04:01, 23 April 2015 (MDT) 

    Well, technically you could make any search string you like, the form is just "PARAM1"="PARAM2" so you could just cram "unif_op"="LIKE+%60%25%40%25%60&unif=A%25ROM" into the string, but if you can explain how that URL you just made work I could try and make an easy-to-use form of this for the template. What does unif_op do? What does "LIKE+`%@%`" mean? Is the % in A%ROM a wildcard character that the LIKE operation expands? (Is this documented somewhere?)
    What would you like the template code to look like? Something like this?
    
    
    {{nesdblink|unif_wildcard|A%ROM|AxROM}}

    \- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 10:56, 23 April 2015 (MDT)

    

    In SQL-ese, "LIKE" gets you globs like filename ? and *, and "_" and "%" are equivalent to ? and *. @ here is substituted with the value of the unif parameter. (But "LIKE" is case-insensitive, while "GLOB" is case-insensitive.)
    But, yes, something like your example would be lovely. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:48, 23 April 2015 (MDT)

    

    

    Added. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:40, 23 April 2015 (MDT)

    

    

    

    Something that might make things easier for making queries sufficiently restrictive would be to change it to "LIKE `@`" and the query to "%-A%ROM", i.e. move the globs into the query and out of the "op"eration. On the other hand, searching for "LIKE `%A%ROM%`" produces 128 hits, for "LIKE `%A%ROM`" produces 112, and for mapper 7 produces 76 hits, so it's still rather inadequate. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:25, 23 April 2015 (MDT)
