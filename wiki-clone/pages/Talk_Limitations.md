# Talk:Limitations

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ALimitations) | View [other pages](Special_AllPages.xhtml#Talk_Limitations)

## Higher Limitations on Famicom

Here I will discuss some ways to overcome some of the limitations than how it is written in the article, using Famicom (rather than NES; sometimes even AV Famicom or a modified AV Famicom might be necessary; some of these things apply to NES too, though). Here is a list: 

  * For input devices, you can even use the Famicom keyboard and/or SNES mouse if that helps. (If the keyboard is used, you can also use the tape for save data.)
  * For RAM, in addition to the 8K PRG RAM in the cartridge, there is the internal RAM at $0000-$07FF, and you might be able to use unused parts of CIRAM, CHR RAM, and palette RAM as additional general purpose RAM. In addition, MMC5 can have up to 64K RAM (it is also possible to make simpler mappers having such features). If you are storing the copy of OAM in CPU addressable RAM, some of the OAM bytes of unused sprites can be used as additional general purpose RAM, too. (In some games that don't need to move sprites, you might even store OAM in ROM.)
  * For 16x16 attribute areas, again MMC5 can avoid it (as mentioned in the article), although it can be possible to make simpler mappers having such features too. It says it puzzles that each playfield is limited 6x12, although single player would be wider, you can use only four colors (like the CGA low-res graphics mode in a PC), or you can even make up a game based around the 16x16 attribute limitation and 8 sprites per scanline limitation (my WIP game "_Attribute Zone_ " (currently in QBASIC, though) is like this).
  * Fighting: To avoid the 8 sprites per scanline limitation, you could make up some kind of fighting game involving vertical arrangement instead of horizontal. You could also have a kind of team game with two characters below and two above.
  * For music, you can add additional channels using expansion audio of the mappers that have them; even a AY-3-8910 alone is sufficient to make a mapper having both expansion audio and bankswitching (and even software-controlled 1-screen mirroring if this is wanted). A cartridge could possibly even be made which can filter the 2A03 audio without adding its own channels.
  * For games with many bullets, you could also use the sprite limitations as part of the strategy of the gameplay.
  * For games with many players, one thing you can do is take turns; another possibility is potential use of parallel and serial multi-player adapters being used together for up to eight players. For many colors of sprites, another thing that can be done is if you use only single color on each sprite, duplicate the graphics many times in the pattern tables using different colors and then you can have up to twelve colors of sprites. If it would help, you could even use the automatic CHR bankswitching of MMC4 so that the alternate color sprites can be in the other bank (although this may limit the number of sprites in each scanline less than eight).
  * To improve speed, you could even store some programs in RAM and use self-modifying code.
  * There are other things too that I did not mention here. I am sure other people have figured out many other things that I don't know, too.



\--[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 14:32, 2 September 2013 (MDT) 
