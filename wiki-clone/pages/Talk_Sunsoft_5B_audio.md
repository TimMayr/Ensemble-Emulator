# Talk:Sunsoft 5B audio

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ASunsoft_5B_audio) | View [other pages](Special_AllPages.xhtml#Talk_Sunsoft_5B_audio)

## Disch's Notes

Why was all of this added here? The noise generator is possibly useful (though I will need to verify it), but everything else is entirely redundant, and with a lot of speculative/weasel wording for things are now properly understood. The frequency calculation is also incorrect, and so are some statements that presume it to be an AY and not a YM. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 01:00, 6 May 2013 (MDT) 

    I've removed all of the redundant or incorrect information, and moved the remaining new information into the article. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 01:10, 6 May 2013 (MDT)

    

    I moved it so that there was only one page we had to remove wrong information from. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:46, 6 May 2013 (MDT)

## LFSR noise

By the way, someone made a program that will calculate the taps of a LFSR, given a recording: <http://listengine.tuxfamily.org/lists.tuxfamily.org/hatari-devel/2012/09/msg00003.html> —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:29, 7 May 2013 (MDT) 

    Hmm, that program doesn't look -quit- that simple. I'd first have to decode the recording to a binary stream, but it looks like it could do the work from there. Most of the emulations I've checked are using the 17/14 tap configuration mentioned in that thread. Thanks for the link, though, I'll try to make use of it when I get around to testing my 5B. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:09, 8 May 2013 (MDT)

## YMZ284 vs YM2149

The datasheet for the YMZ284 claims that the top four bits of the data written to register $F must be 0 or else ... something. It'd be idly interesting find out what that something is, and if the 5B does that same something. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:54, 14 November 2014 (MST) 

    Is the "or else" a consequence that involves the I/O port? If so, how would you verify it with the 5B? If there's a test you'd like me to run on the 5B let me know, but I have no idea what I'd be looking for with this suggestion. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 17:33, 17 November 2014 (MST)

    

    The YMZ284 doesn't have any I/O ports, so I have no idea what the top four bits are. Just that the datasheet says "those bits must be 0" and that it "controls power", so it's clearly hooked up to something. So I guess the obvious test is writing non-zero to those bits and see if it obviously changes the sound? If it does, then it seems likely that we know the exact heritage used in the 5B. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 18:18, 17 November 2014 (MST)

    

    

    Well, perhaps an emulator could be made to display a warning if it finds these bits to be nonzero. (It won't answer that question, but it would allow to test that a program isn't doing anything it isn't allowed to do.) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 14:50, 21 November 2014 (MST)
