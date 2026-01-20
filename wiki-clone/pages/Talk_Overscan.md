# Talk:Overscan

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AOverscan) | View [other pages](Special_AllPages.xhtml#Talk_Overscan)

## PAL DAR

If I'm doing the math correctly, PAL display aspect ratio is ≈1.48:1, or close enough to 3:2. So do PAL games basically come pre-widescreen? How does the vertical alignment compare?—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 18:53, 9 April 2013 (MDT) 

    I have no access to a PAL NES, PAL famiclone, or PAL TV, but I'm getting in the neighborhood of 1.39:1. Could you explain the math that leads to 1.48:1? But for this comment, assume that the 240px of the NES picture is in fact vertically centered on the 288px tall PAL picture. A widescreen TV will show 216px minus overscan, and overscan is (I'll guess) 12px per side, so anything designed for a 192px tall safe area will fit properly. And the old Nintendo BG planning sheet did assume a 192px tall safe area. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 19:29, 9 April 2013 (MDT) 

    Just going on what was said in the page: 1.3862 × 256 ÷ 240 = 1.4786. Am I doing that wrong? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:46, 9 April 2013 (MDT) 

    Oh, my fault, I misread your first comment. 1.48 is the DAR. I've been so focused on getting proper PAR that sometimes I forget about DAR. But you'd need a DAR near 1.78 to be widescreen friendly. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 21:38, 9 April 2013 (MDT) 

    Having a vertical active area of 200px (back of the envelope: 256×9÷16×1.3862) puts "de-letterboxed widescreen PAL 16:9" as convenient as 4:3 SDTV, which is perhaps worth keeping in mind.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 01:02, 10 April 2013 (MDT)

## Difference between PocketNES safe area and title safe area?

Why is those two areas displayed, since it is generally admitted that data will be displayed in all cases in both of those areas ? This makes the smallest of the bunch (title safe area) useless since it is smaller than data that we know will be displayed in all cases; as such in my opinion it is irrelevant to overscan. ~~208.71.141.54~~ 00:06, 13 April 2018 (MDT) It's me there I forgot to log in.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 00:06, 13 April 2018 (MDT) 

    I always thought PocketNES was a strange inclusion here. There's lots of emulator environments on low resolution screens that might apply (e.g. VGA 320x200?) I don't really see why many people would care about PocketNES in particular. It's already extremely niche, emulating a 30 year old system on a 15 year old one. :P That said, its inclusion here isn't offensive to me, though it does add a few seconds of confusion to reading this article. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 12:05, 14 April 2018 (MDT) 

    What PocketNES cuts off is a practical maximum for what non-pathological TVs will cut off. Title safe is intended for two things: A. to cover even pathological cases, such as TVs in need of professional H/V-size readjustment, and B. to document the practice embodied in markings on Nintendo's background planning sheets. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 13:58, 14 April 2018 (MDT)
