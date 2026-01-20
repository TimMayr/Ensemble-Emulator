# Talk:APU Triangle

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AAPU_Triangle) | View [other pages](Special_AllPages.xhtml#Talk_APU_Triangle)

I'm fairly sure triangle can be silenced by writing $00 to $4008, too. Any value with the low 7 bits at 0 will silence the channel, regardless of the value of bit 7.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 02:18, 20 March 2019 (MDT) 

    Yes you can write $00. The consequence though is that the length counter goes active and will eventually expire, requiring an additional reload of $400B to un-silence the channel later. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 07:24, 20 March 2019 (MDT)

As per my comment in this thread: [[1]](http://forums.nesdev.org/viewtopic.php?f=3&t=19191), I believe the $4008 "Side effects" to be wrong - As best I can figure, if you actually code $4008 to set the reload flag, it causes the triangle to behave incorrectly (see thread). But since it seems to have been very explicitly added recently, I figured I'd ask about it here instead of just changing it. It seems to be incorrect? [Drilian](https://www.nesdev.org/w/index.php?title=User:Drilian&action=edit&redlink=1 "User:Drilian \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Drilian&action=edit&redlink=1 "User talk:Drilian \(page does not exist\)")) 00:09, 12 August 2019 (PDT) 

    Responded [in that thread](https://forums.nesdev.org/viewtopic.php?p=241700#p241700). - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 03:30, 12 August 2019 (MDT)
