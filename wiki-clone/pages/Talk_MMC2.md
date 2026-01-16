# Talk:MMC2

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AMMC2) | View [other pages](Special_AllPages.xhtml#Talk_MMC2)

## Sprite Splits

I read carefully the operation of the PPU rendering and sprite evaluation, and it seems to me that you could use the sprites to adjust the CHR latch per scanlines (for example, make a sprite $FE and one line down $FD (if after the first one in the OAM) might make a single scanline to use the other bank). Is this correct? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 11:07, 16 June 2013 (MDT) 

    Yes, this could work. As you suggest, if you're willing to use sprites from the same pattern table as the background tiles you can do no-CPU-time raster effects. But that's also a big downside. I'm a little skeptical that the inclusion of two switchable banks is particularly useful, given how few pixels all the sprites can cover (using 8x16 sprites, only 14% of the screen). Maybe if you want to clip more than two sprites vertically on the same line, since currently the only solution is overlaying a sprite with another transparent sprite. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:37, 16 June 2013 (MDT)

## Circuit to convert MMC2->MMC4 and vice-versa

I just realize the circuits I drew here several years ago are completely useless, apparently Nintendo didn't use such circuit in their cartridges, and nobody else is likely ever going to use them. They serve no "encyclopedic" purpose, it's merely a theoretical possibility that was never put in practice. Is it okay if I remove them ? ~~128.178.180.88~~ 02:05, 15 April 2014 (MDT) I forgot to log in but it's me, who wrote the paragraph above.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 08:14, 15 April 2014 (MDT) 

    They illustrate the difference in a manner I find useful. I object to removing the MMC2→MMC4 one, at least; the MMC4→MMC2 one is enough more complex that I am open to argument. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:21, 15 April 2014 (MDT)

Then is it okay if I just remove the part on the MMC4 article (which is more complicated) and leave the one that is on this (MMC2) article which is more simple, "prooving" that it's possible to make a MMC4 from a MMC2 and a single 74xx series chip ? (although the proof is only theoretical, nobody ever prooved it for real as far I know) ? I'd say that if there is no objection I'll remove the MMC4 schematic in a few days. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 00:26, 29 April 2014 (MDT) 
