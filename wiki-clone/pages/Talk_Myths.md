# Talk:Myths

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AMyths) | View [other pages](Special_AllPages.xhtml#Talk_Myths)

## Contents

  * 1 Picture
  * 2 All three color emphasis bits with color 0D
  * 3 Source the myths
  * 4 Some Facts that not many people knew (or mention at least)



## Picture

\- An offhand comment. Wouldn't be better to change "NTSC NES picture" term into "NTSC NES PPU output"? Perhaps the second one sounds more technical than the original though. A NES picture could be anything like the straight thing: a picture of a NES unit, for example. When you say "PPU output", we have something more concrete. Yup, it's a picture, or "frame", but not "a picture of the NES", anyways... :) --[Zepper](User_Zepper.xhtml "User:Zepper") 23:11, 2 February 2011 (UTC) 

    I was under the impression that "picture" is the word used in video standards for any time that isn't a blanking period. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 19:31, 8 September 2015 (MDT)

## All three color emphasis bits with color 0D

According to my interpretation of various posts in this forum thread: [sony trinitron + color emphasis bits = scrambled image?](http://forums.nesdev.org/viewtopic.php?f=2&t=8623), if you use all three color emphasis bits and use color 0D, that color's signal may dip too low and confuse some TVs: 

    In the first post, [GradualGames says](http://forums.nesdev.org/viewtopic.php?p=90009#p90009) he used all three color emphasis bits to dim the screen and got "an extremely ugly bunch of scanline artifacts" on his Sony Trinitron. In a later post, [Sdwave says](http://forums.nesdev.org/viewtopic.php?p=90039#p90039) he also saw "some bending" using one of the palette demos on a Trinitron and suggested using 3F ~~[= 1D*]~~ for black instead of 0D. Later, [GradualGames confirms](http://forums.nesdev.org/viewtopic.php?p=90556#p90556) using 0E ~~[= 1D*]~~ instead of 0D got rid of the artifacts.

~~*According[NTSC video](NTSC_video.xhtml "NTSC video"), the xE and xF colors send the same signal level as 1D.~~

[Lidnariq points out](http://forums.nesdev.org/viewtopic.php?p=90015#p90015) that mathematically, even color 1D with all three color emphasis bits dips lower than the normal black: 

    _If all preemphasis bits are set, then color 1D ( "black") becomes blacker-than-black and 01-0C no longer decode to a valid RGB color from the received YIQ values (Their brightness is approximately equal to unattenuated color 1D). So if it looks like scanlines restarting all over the place, maybe that's why?_

~~But based on Sdwave's and GradualGames's comments, 1D with all three color emphasis bits was fine on their Trinitrons.~~

\--[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 21:10, 23 June 2013 (MDT) 

    Color emphasis does not get applied for phases $xE or $xF, and it similarly does not get applied during horizontal (x=270-327) or vertical (y=241-261) blanking. --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 17:23, 24 June 2013 (MDT) 

    Then someone's going to have to make a test ROM to run on an RGB system that displays emphasized $1D on a field of $0F to see how readable it is. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 17:41, 24 June 2013 (MDT) 

    I think I remember someone mentioning having a complete set of Kevtris's 2C03/4/5 dumps except for the "grey"scale dump for one of them; I'm pretty certain on those PPUs it indicated colors xE and xF being collapsed to x0 when the bit was set. ("snarky quotes" because scrambled palettes cause it to not necessarily be grey) I don't know if the greyscale bit is masked with the same flag as the emphasis bits, though. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:51, 24 June 2013 (MDT) 

    It doesn't tell you what it does with emphasis bits though. I too agree someone should test the function of emphasis bits of $xE and $xF on RGB systems (such as a Famicom Titler or Famicom TV). It would be useful to know, if making a emulator, hardware clone, or a game that uses the emphasis bits. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 01:47, 25 June 2013 (MDT)

  
Quietust, thanks for pointing out that using the emphasis bits doesn't attenuate the xE and xF colors, I didn't notice or remember that part of the [NTSC video](NTSC_video.xhtml "NTSC video") page. Everyone else, I was talking about an NTSC PPU sending a signal to an NTSC TV, not an RGB PPU sending a signal to an RGB monitor. My original point was that the Myths page says 

> **Myth:** Enabling more than one color emphasis bit at once will damage the PPU, or at least cause the TV to lose sync.

I agree it's unlikely that enabling multiple emphasis bits will damage the PPU, but according to the forum posts I mentioned, it does seem possible to confuse a TV's line sync if you use all three color emphasis bits and color 0D. And as Lidnariq posted, mathematically **(edit:) the attenuated** 1D level is lower that the black level, so perhaps using it with all three emphasis bits might confuse some TVs too? 

(Above, I misbelieved the xE and xF colors would also be attenuated and so they'd have the same level as an attenuated 1D, so I misinterpreted Sdwave's and GradualGames's comments as equivalent to saying 1D was a safe color to use with all three color emphasis bits. Now I understand the xE and xF colors aren't affected by the color emphasis attenuation, so it's clear they have the same signal level as an unattenuated 1D and they're safe to use with all three emphasis bits. It's not clear how the attenuated 1D level behaves with actual TVs (I didn't see 1D mentioned when the posters described problems), but mathematically I can see it might be wise to avoid it.)

If others agree the forum posts are describing line sync problems (or can recreate the results), maybe the Myths page can be revised to clarify it's possible to confuse (some?) TV's line sync when using all three emphasis bits with color 0D (and maybe color 1D too?). 

\--[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 20:38, 25 June 2013 (MDT) 

added links to specific posts, struck out misunderstandings in my original post above --[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 22:13, 25 June 2013 (MDT)

## Source the myths

I would be a great idea to know where the myths comes from, adding sources and link to wrong information whenever possible. This would also avoid people "making up their own myths", just for the fun of it.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 05:21, 26 May 2015 (MDT) 

    This any better? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 17:58, 26 May 2015 (MDT)

Why does the text appear so tiny on screen? 

    Bavi H mistyped a closing tag on small text above. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 09:49, 26 May 2015 (MDT)

## Some Facts that not many people knew (or mention at least)

Using an interrupt (the one used by the sample player, you could generate an interrupt on any screen line (200). In each H-blank you could write 3 colours (and since the 4th is transparent... so for something like a Horizontal shooter, levels with different coloured background images top & bottom. I even wrote a converter that took a picture drawn in OCP Art Studio+ (I just copied how OPC worked on the ST and adapted for images up to 256 colours) fixed to the 53 colours (0-C,10-1C,20-2c,30-3C & OD) available. On each line, I allowed the artist to change 3 colours and after it was drawn, the OCP scanned and came up with an image highlighting errors. These were solved by placing sprites over the error. Sprites could actually be multiplexed so on any lines with no change, I could reuse a sprite OR swap a palette. What can I say... it was as good as you can expect and taught me a useful lesson which made it into Tomb Raider Gameboy colour where I got 2304 colours on screen and pretty damned good images of the whole team. Access is A+B+R+U... or +D (I forget the last one BUT if you hit all 4 in the same frame, about 20 hidden pictures were added - we had to use the biggest cart but were left with 200 odd K.... so as well as the title screen, there were more & better images—([unsigned](https://en.wikipedia.org/wiki/Wikipedia:Signatures "wikipedia:Wikipedia:Signatures") post by ~~213.106.56.145~~ at 11:52, 7 September 2015) 

    There is **not** enough time to change even **one** palette entry during hblank, let alone three. Rendering must be disabled after pixel 256 (in order to be invisible) and enabled before pixel 320 (in order to be invisible). This is only 64 pixels, or 21⅓ CPU cycles. It takes 27 cycles to update a single palette entry: 

  * first half of PPUADDR (free, can happen before rendering is disabled)
  * disable rendering (1 cycle: the first 3 cycles are free)
  * second half of PPUADDR (4 cycles)
  * palette update (4 cycles)
  * load three new values for next three writes (6 cycles, can be skipped if some of the values are duplicates)
  * first half of PPUADDR to reset screen to correct location on screen after render stop (4 cycles)
  * second half of PPUADDR to &c (4 cycles)
  * enable rendering (4 cycles)


    In the specific (and unlikely) case where you can fit all the values that you'll write into just the 6502's three registers, you still have the completely unavoidable 2 pixels of jitter, one more than you can spare.

    Possible workarounds: 

  * Blank scanlines
  * Disable rendering early, as is done by the [Indiana Jones and the Last Crusade title screen](http://forums.nesdev.org/viewtopic.php?p=139925#p139925) losing any detail (it's now all the backdrop color) from 18 pixels in exchange for 6 CPU cycles. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 01:50, 8 September 2015 (MDT)



    

    The practical rate of color replacement with zero artifacts is about one 3-color subpalette per blank line, or the entire background palette in 4 blank lines, or background and sprite palettes in 8 blank lines. So you can have separate palettes for a playfield and status bar, or for both players' playfields in a split-screen 2-player game. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 19:29, 8 September 2015 (MDT)

    

    

    Ok, here's a goofy idea: generate _address_ bus conflicts and the use the bonus cycle from page crossing; write the first value to PPUADDR, and then have four LDA $22F7,x. Externally drive R/W and some of A0..A2 low; also drive the data bus. This allows eight writes in 20 CPU cycles, which, if the timing is permissible to the PPU, should be enough to update 3 palette entries, or update 1 palette entry and do a full 2006-2005-2005-2006 scroll reset. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 02:01, 9 September 2015 (MDT)
