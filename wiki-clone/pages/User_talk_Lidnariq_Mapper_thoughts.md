# User talk:Lidnariq/Mapper thoughts

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3ALidnariq/Mapper_thoughts) | View [other pages](Special_AllPages.xhtml#User_talk_Lidnariq_Mapper_thoughts)

## 8x8 attributes

You have to also be able to write to the alt-attributes. Is the other-chip-halves banking to allow that you can write there "just normally" by PPUDATA?[Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 18:56, 13 September 2016 (MDT) 

    You got it. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 23:41, 13 September 2016 (MDT) 

    All right. That means the last four tiles of each bank are occupied by attributes, yeah? Separately, "latch y and x in address: 1x YXYY YYyX XXXx" the "1x" could be a source of confusion. [occurs twice] [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 00:38, 14 September 2016 (MDT) 

    Well last four tiles of each 64-tile bank. Given CNROM-like banking, it's instead sixteen random tiles from the lower half. But lacking MMC3-style IRQs, I figure you can just put the sprites on the "left" pattern table, and having to work with "only" 240 tiles for sprites should be ok. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 01:30, 14 September 2016 (MDT) 

    I was working off of the leading assumption of "a mapper with 4 banks of 8 KiB each of CHR-RAM." 64 tiles indicates you were thinking of the trailing musing of 1KiB. [P.S: delete the will of "will, would"; I feel it's probably rude to actually edit someone else's userspace uninvited.] [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 01:59, 14 September 2016 (MDT)

## Test

Checking if a Usertalk-space page that, while not under my User, I created, works. [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 01:06, 25 October 2016 (MDT) 

    It does.[Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 01:06, 25 October 2016 (MDT)
