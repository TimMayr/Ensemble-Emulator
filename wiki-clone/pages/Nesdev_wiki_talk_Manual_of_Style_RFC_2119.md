# Nesdev wiki talk:Manual of Style/RFC 2119

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Nesdev_wiki_talk%3AManual_of_Style/RFC_2119) | View [other pages](Special_AllPages.xhtml#Nesdev_wiki_talk_Manual_of_Style_RFC_2119)

## This should be deprecated or deleted

After Bregalad [left a comment](http://wiki.nesdev.org/w/index.php?title=Talk:Standard_controller&curid=849&diff=10916&oldid=7976) about the use of MUST, I ended up taking a look at the use of this template across the wiki. 

This should not be part of our manual of style for this wiki (do we even have one? this seems to be the only page in this namespace). All of these words have perfectly normal English meanings, and those are the meanings that new users come in expecting. Putting them in ALLCAPS and insisting that they have some specific meaning (which is only obscurely different from their normal English meaning) creates confusion and slows comprehension of the article. An infobox wasting 100 words to try to explain that "MUST" means "must, but pedantically" is counter-productive. The capitalization style of RFC 2119 is jargon used only by a few particular groups of people; the vast majority of users are not familiar with it, and it wastes our time. 

I looked through the wiki and found no use of this template that I thought contributed to better understanding of the meaning of the words in question, and I found several cases where MUST was being used incorrectly. I'd recommend deleting this page, though this wiki doesn't seem to have a deletion request process. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:49, 26 May 2015 (MDT) 

    Why in hell does typing RFC 2119 automatically create an external link??? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:51, 26 May 2015 (MDT)

    

    Mediawiki feature(?): <http://www.mediawiki.org/wiki/Manual:RFC> —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:15, 26 May 2015 (MDT)

    See [RfcKeywords on W3C Wiki](http://www.w3.org/wiki/RfcKeywords). If MUST was being misused, it should be fixed. But just because something _can_ be misused doesn't mean it's useless. For example, look at all the other RFCs that cite this RFC in the same way the `{{[RFC 2119](Template_RFC_2119.xhtml "Template:RFC 2119")}}` template does; is it useless there? And if it's useless, why is the delusion that it's useful [so widespread](https://www.google.com/search?q=site%3Astackexchange.com+rfc+2119)? Should I reword all uses of all-caps MUST along the lines "Failure to do so causes [undefined behavior](https://en.wikipedia.org/wiki/undefined_behavior "wikipedia:undefined behavior")"? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 16:55, 26 May 2015 (MDT)

    

    No, absolutely DO NOT redefine what "MUST" is supposed to mean and try to impose this new personal definition of the word on others. That's my primary issue with this! You're creating nonstandard meanings for words and then expecting others to read some documentation to keep up with you. The word "must" is _perfectly fine_ already; just use English words as they already exist. I don't care if some people have a use for RFC 2119 elsewhere, it's not helping here. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 17:30, 26 May 2015 (MDT)

    

    

    "personal definition"\--The IETF is hardly a person creating a personal definition; RFC 2119 is a standard set by the organisation eighteen years ago. (As for tepples creating a "personal definition" he is just suggesting a rephrasing this particular meaning of MUST into the vernacular; not redefining it.) "perfectly normal"? English (well, language in general) is fairly extensively ambiguous. I think it is[/was] useful for distinguishing these particular meanings. The group that uses these...is those creating technical computer document specifications, which we are/do; I find it useful to discriminate advice for complying with the specification from requirements of the specification. Furthermore, not all of our readers are native English speakers. Defining the difference between should and must is a useful thing. (And one doesn't have to click if one doesn't find it necessary to be clarified, meaning no time lost!)(Perhaps better to contend a wide adoption to counter "particular groups of people", than its usefulness? Using it to imply usefulness is an _argumentum ad populum_.)[Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 17:43, 26 May 2015 (MDT)

    

    

    Perhaps more saliently, "If you don't carefully and clearly define the words you use in communications, you will fail to communicate. Standards documents are – first and foremost – about clear communication of an agreement on how to do a thing." <http://www.quora.com/Whats-the-story-behind-RFC-2119> [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 17:47, 26 May 2015 (MDT)

    

    

    

    Sorry, I misread what tepples said. (I thought he was suggesting editing the template to define MUST = otherwise undefined behaviour, which would have been even worse.) I'm saying that we should not encumber our text with unnecessary definitions. I don't disagree with RFC 2119 as a practice on how to personally use these words, but I absolutely disagree with an annoying and intrusive infobox to explain them everywhere they appear, and with UNNECESSARY capitalizations of PARTICULAR words for NO REASON other than they're relevant to some ESSAY you once read about how to USE them. If you think the word "must" is used ambiguosly in an article _**fix the text to resolve the ambiguity**_ , don't slap ugly formatting an an infobox and a link to some 5000 word essay about how to use these words on it! We don't need to [offer](https://en.wiktionary.org/wiki/offer "wiktionary:offer") [definitions](https://en.wiktionary.org/wiki/definition "wiktionary:definition") [for](https://en.wiktionary.org/wiki/for "wiktionary:for") [every](https://en.wiktionary.org/wiki/every "wiktionary:every") [use](https://en.wiktionary.org/wiki/use "wiktionary:use") [of](https://en.wiktionary.org/wiki/of "wiktionary:of") [a](https://en.wiktionary.org/wiki/a "wiktionary:a") [word](https://en.wiktionary.org/wiki/word "wiktionary:word"), we should present the words clearly so they can be understood in context. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:02, 26 May 2015 (MDT) 

    The capitalization is intended to distinguish the RFC 2119 readings from the far less precise dictionary readings. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 18:35, 26 May 2015 (MDT)

    

    

    

    I wouldn't mind at all if you included this template on talk pages, if you wanted to use it as part of a comment suggest to an _editor_ how it may be helpful to _use_ these words (if some editor were consistently misusing them), but I am very strongly opposed to suggesting to the _reader_ how they are supposed to _read_ them by placing it within articles. When editing you should acknowledge the naive reading of your words, and accomodate that by using them well, not by telling the reader to get with the program and read this essay first. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:15, 26 May 2015 (MDT) 

    "The naive reading" of the word _bit_ is "a small portion", not "an individual switch that can be set to a low or high value". --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 18:35, 26 May 2015 (MDT)

    

    

    

    

    

    Just, please do not propose an infobox explaining the word "BIT", capitalizing it wherever it is used with a link to a 10 page essay about what it means. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:51, 26 May 2015 (MDT)

    

    

    

    For what it's worth, I'm in agreement with Rainwarrior on this matter. I've no issue with RFC2119 or its purpose, but adding an infobox (or anything else similar in concept) to pages to add nothing more than clarification of what English words mean is, quite literally, excessively pedantic. As someone who has (obviously) written technical documentation for decades, know that pedantry of this sort ends up making the documentation _more_ confusing to readers. (For sake of comparison, the same goes for excessive linking). The purpose of a piece of technical documentation (re: NES/Famicom) is not to reassert dictionary definitions, it's to convey technical details. Leave the job of a dictionary to a dictionary; if there are thoughts/details which are unclear or vague given their wording, simply fix the wording (or provide multiple phrasing, e.g. what I did with the Attribute Table description in [nestech.txt](http://web.textfiles.com/games/nestech.txt)). TL;DR: I think what's going on here is ridiculously excessive OCD and catering to it does not help make documentation any clearer. - [Koitsu](User_Koitsu.xhtml "User:Koitsu") ([talk](User_talk_Koitsu.xhtml "User talk:Koitsu")) 18:34, 26 May 2015 (MDT)

Rainwarrior wrote in [this edit summary](http://wiki.nesdev.org/w/index.php?title=Standard_controller&curid=70&diff=10957&oldid=10955): _(and then saying it's a "specification"?)_ But what is this wiki if not the specification of the Nintendo Entertainment System virtual machine as understood by homebrew developers and emulator developers? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 18:35, 26 May 2015 (MDT) 

    Please stop treating everything I say as some black and white absolute. I think the choice of words was poor on those two edits. It's not _wrong_ to say it's a specification, but I think it was terrible form to say it that way. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:38, 26 May 2015 (MDT)

There is three reasons to ever use words in all caps: 

1) If lowercase is not available. This is clearly not the case in this wiki 2) For acronyms. 3) To indicate that you are shouting loud. 

Here the capitalization of MUST looks like an accronym but in context its clear its not, instead it is shouting. I believe that shouting like that and saying THIS MUST NOT BE DONE! NEVER! FORBIDDEN! do not suit a technical documentation style and should be forbidden (no joke) on this wiki. Do not assume the reader is a 3 years old kid who can't think for himself, you don't have to say "Do NOT cross the road!" like you'd shout a small kid (for his own safety), instead, say "if you're going to cross the road, be aware that there is also vehicles on it, and many drivers tends not to respect speed limitations.". Any adult person knowns exactly what this means. 

In short, I agree with Rainwarrior. I do not even know what RFC2119 is and I do not care. This is not related to NES development in any way. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 03:28, 27 May 2015 (MDT) 

I do not agree. RFC 2119 is good in many cases. (However, that doesn't mean it should be used everywhere.) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 14:55, 28 May 2015 (MDT) 

    Are you disagreeing with the idea that we shouldn't stick this template all over the articles and capitalize MUST and SHOULD, or are you just stating your approval of RFC-2119's definitions of must and should? (I have no problem with it as a suggested guideline for editors on how to use the words; it's the template and capitalization that I object to.) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 15:43, 28 May 2015 (MDT)

    

    You certainly shouldn't use it in all articles, although in cases where you should use RFC2119 then it probably makes sense to capitalize it which would probably make it clear to people who know RFC2119 even if you don't have the template (although not using the template may make it less clear to those who don't already know RFC2119). I do not mean any as shouting anyways. This is about protocol compatibility (which is why RFC2119 helps, and in turn why using all capitals helps here); it isn't meant to be the law or whatever. Tepples wrote "The capitalization is intended to distinguish the RFC 2119 readings from the far less precise dictionary readings." and this is really how I agree with. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 01:34, 29 May 2015 (MDT)

    

    

    What is RFC2119?! I have absolutely no idea. Please explain me. Also the verbs "to must" and "to shall", along with other verbs such as "to can", "to have to", etc... have very precise meanings in English language. We learned them very early when learning the language, too, because when going to a foreign country it is usually important to know what is forbidden or obligatory, and what is just tips or advice. In call cases, I see no reason to capitalize those particular words, this makes absolutely no sense to shout like that at the poor little reader.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:45, 29 May 2015 (MDT)

    

    

    There is a really easy way to make "must" precise. All you have to do is add a clause explaining what must is predicated on. "You must buy a ticket if you want to get on the train." The RFC 2119 approach seems to be "You MUST buy a ticket." This implies to me that the reader is beneath you and undeserving of explanation. Actually, reading RFC 2119 again, it continually and clearly explains that it is primarily for work specifications; that is: ordering someone to make something for you. When you say MUST in a specification, the point of RFC 2119 is just to pedantically clarify that it's not an option for the implementer, because a lot of people like to take shortcuts and adjust the specification to fit what is convenient for them. (I'm sure everyone who has been involved in this kind of work has experienced this problem, which is why RFC 2119 is a popular clarification for it.) This kind of relationship is _not_ what the Wiki is. We are not ordering someone to make us an Emulator. We are not hiring someone to build us a cartridge. We are not managing the development of open source software (you may be, personally, but the Wiki isn't). People come to the wiki to learn how to make things _for themselves_ , not for _you_ , and in this kind of situation there is no RFC 2119 "MUST", they don't actually have to do _anything_ you say. Taking this kind of "fall in line and do as I say" approach to the wiki is abrasive, insulting, and completely unnecessary. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 10:44, 29 May 2015 (MDT)

    

    

    

    I can only speak for myself in this regard, but RFC 2119 reads in the exact same tone as the [Tiger Oil Memos](http://www.lettersofnote.com/2010/08/tiger-oil-memos.html) to me. It is the tone of a superior addressing a subordinate, as such. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 10:54, 29 May 2015 (MDT) 

    The purpose of a manual (or this wiki) is to instruct people what they MUST do for their program to interact with the NES in well-defined ways. The purpose of the RFC 2119 infobox is to clarify that capital letters is to denote not a shouting tone, but a particular meaning for the modal verbs: that is, to indicate that a MUST is a "you must do this, in order to comply with the specification and produce well-defined, as-documented behavior", but because this meaning is so-often used, being able to drop the "in order to..." modifier-clause and only define it once is useful (Don't Repeat Yourself). The infobox is hardly intrusive. Now, if it floated over everything and followed my scroll, that would be intrusive, and _that behavior_ should be changed, not the inclusion of the infobox. Incidentally, RFC 2199 itself...is indeed a memo for the IETF **instructing** people how to write their specification requirement levels consistently. [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 14:56, 29 May 2015 (MDT)

    

    

    

    I agree with tepples. While I agree that nobody forces you to follow the specification, but if you don't, then it is not going to be correct and it is not guaranteed to work! (Maybe in some cases it might work, but you should follow an agreed-on protocol/API/format/whatever if you want to properly claim support for such things.) (For example: The program MUST wait for the PPU to be ready before expecting it to work; however you can still write some registers before then but they aren't guaranteed to do anything (but still might); some of my own programs do try to disable NMI immediately upon reset; if the register write fails, then it is guaranteed that NMI is already disabled anyways so it doesn't matter. An implementation of the PPU MAY require the game program to wait in this way.) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 12:53, 29 May 2015 (MDT)
