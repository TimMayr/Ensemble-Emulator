# Talk:VRC7 audio

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AVRC7_audio) | View [other pages](Special_AllPages.xhtml#Talk_VRC7_audio)

The OPLL datasheet (transcribed here- <http://www.smspower.org/maxim/Documents/YM2413ApplicationManual#tableii2>) suggests that the delays should be 12cy (A->D) and 84cy (D->A) of the 3.6MHz xtal, or ~3 and 21 NOPs [Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 20:26, 15 May 2012 (PDT) 

    Yeah, I looked it up afterwards when I realized this info should be in the datasheet. I plan to do some testing of this later to make sure the numbers are good enough; the earlier ballpark guesses were just from trying things until it worked-- I believe some of the registers finish updating faster than others, so my guesses were a bit off from the recommended times, apparently. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 20:44, 15 May 2012 (PDT)

## Example recordings

Do we want to link to the two "VRC7 audio is not YM2413 audio" recordings somewhere on this page? (Where?) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 20:09, 1 February 2016 (MST) 

## Audio Enable

Does the audio enable just mute things? Does it reset all state? Or somewhere in-between?—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:28, 30 August 2018 (MDT) 

    I'll give it a test tomorrow. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 22:50, 30 August 2018 (MDT)

    Appears to empty the registers and silence it. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:44, 31 August 2018 (MDT) 

    So, seems plausible it's equivalent to the YM2413's /InitialClear pin? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 18:57, 31 August 2018 (MDT)
