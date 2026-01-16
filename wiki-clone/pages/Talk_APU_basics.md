# Talk:APU basics

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AAPU_basics) | View [other pages](Special_AllPages.xhtml#Talk_APU_basics)

I removed some of the comments you added to the initial register values, because the aim of this article is to present a simplified model of the APU, and not require the programmer to bother with anything more. You can add them back if you want, but I'm thinking this should focus on simplicity and getting results, not understanding the APU. 

I also removed the use of the round() function, as I didn't feel it would be clear to the intended audience (I put a comment explaining to round the result). 

Again, I'm fairly new to Wikis so feel free to explain better ways of handling these than I have. :) 

[Blargg](User_Blargg.xhtml "User:Blargg") 15:28, 29 July 2010 (UTC) 

    Some comments are helpful so that advanced programmers can understand why it's written the way it's written. For instance, the init code that's been on the page the last couple years until now had a nasty side effect where $4014 (OAMDMA) was written during APU init. When I fixed this issue, I made a note explaining why $4014 has to be skipped. While that comment will probably mean nothing to a total newbie, it will prevent other people from trying the same simplification that led to this bug in the first place. - [Furrykef](User_Furrykef.xhtml "User:Furrykef") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Furrykef&action=edit&redlink=1 "User talk:Furrykef \(page does not exist\)")) 11:35, 21 August 2016 (MDT)

As I said in the forum, in my personal opinion, this page should be removeed completely from the wiki and any significant about the APU should go to the APU page. I do not see the purpose of this page, and most information is either wrong, incorrect or incomplete. The coding exemples are bad exemples and extremely unrealistic of how a NES game actually uses the APU. I say this as someone who wrote multiple sound engines, and deeply analyzed multiple kinds of sound engines written by other people. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 13:34, 22 August 2016 (MDT) 

    This page doesn't offer _any_ new information about the APU, there's nothing here that would be appropriate to fold back into [APU](APU.xhtml "APU"). That's not what this page is for, though, it's a tutorial. Towards the quality of the tutorial, I might make some suggestions: 

  1. The init code loop obfuscates what it's doing. I suggested an [unrolled version](http://forums.nesdev.org/viewtopic.php?p=177903#p177903) so that details can be explained.
  2. The init code writes $40 to $4017, but then the triangle code writes $C0 to it, which is inconsistent. Should just initialize to $C0?
  3. I would suggest _not_ using $4017 with the triangle code; yes there's a slight delay otherwise, but I think messing with $4017 is an advanced topic. It will still mute during the correct frame without it.
  4. The playing a musical note example should write LO before HI. Not terribly significant to the example, but good practice esp. if you eventually want to use the envelopes, etc.
  5. I think this deserves a note somewhere that is disabling a lot of APU features (envelope, length counter, etc.), doesn't need to describe exactly what, just that the examples are taking a simplified subset approach.
  6. The example code linked at the bottom still has the "unpleasant" $4016 write that [started this discussion](http://forums.nesdev.org/viewtopic.php?p=177864#p177864). A new version should be written and hosted on the wiki.



    Aside from these nitpicks, I think it's actually a good tutorial example program. It works, and it seems like a useful simple way to demonstrate how to use the APU channels. What exactly do you find unrealistic about it, bregalad? I mean, obviously most games are going to be loading data from a music/sound engine and not immediate values to feed to the APU registers, but I think all that's separate from just learning to use the APU, i.e. do one first, then the other. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:18, 22 August 2016 (MDT)

    

    The wiki is not just a reference, but a place for tutorials as well, even if we [haven't got all that many at he moment.](Programming_guide.xhtml "Getting started") Hmm…we don't have a Tutorial category yet. [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 00:24, 23 August 2016 (MDT)

    Then, after renaming the page as "tutorial", and applying all the numerous changes that Rainwarrior suggested in order to make this tutorial stop being so horrible, then yet I do no have anything against keeping the page anymore. But you have to admit, before those changes, this page sucked hard. I still think it's weird to load registers from direct data, but if the intent is just "trying" the APU then this make sense. I'll also suggest adding a link to SNDTEST.NES, which is what I always used to "play with" the APU registers, personally.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 12:53, 23 August 2016 (MDT)

So, why is this horrible initialisation loop _still_ there ?[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 14:02, 25 August 2016 (MDT) 
