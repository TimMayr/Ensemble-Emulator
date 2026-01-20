# Talk:Mapper

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AMapper) | View [other pages](Special_AllPages.xhtml#Talk_Mapper)

## Contents

  * 1 Allocations
  * 2 unicode joke terms
  * 3 n\T icon
  * 4 Why some mappers are assigned to "Rare, Ltd."'s logo
  * 5 32x32 upscaling
  * 6 A newbie-friendly page introduction
    * 6.1 Mapper
  * 7 Mapper Grid
  * 8 pirate/pirate MMC3
  * 9 mappers 256-9
  * 10 Whirlwind Manu
  * 11 Notes on Undocumented/Unimplemented Mappers
  * 12 Icons for "pirate" mappers that aren't just for pirates
    * 12.1 Lists of Mappers that should Use/Ignore the Mirror Bit



## Allocations

By FCEUX 2.2.1, they allocated mapper 160 to describe an undersize [NINA-003-006](NINA_003_006.xhtml "NINA-003-006") board (32k PRG, 16k CHR). This seems superfluous. They also allocated mapper 181 to describe another board like [iNES Mapper 185](CNROM.xhtml "INES Mapper 185") that didn't fit with the existing heuristic. It would be nice to take the opportunity to promote [NES 2.0 submappers#iNES Mapper 185](NES_2_0_submappers.xhtml#iNES_Mapper_185 "NES 2.0 submappers"). —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 21:41, 23 June 2013 (MDT) 

    Also, wasn't 256-511 supposed to be for developers or debugging use? I need to request a reserved number for future hardware designs. I'll probably go find the IRC channel for that. ;) BTW: The documentation for [NES 2.0](NES_2_0.xhtml "NES 2.0") header format has different allocation planes. It's best to fix this BEFORE someone goes and tries to update/make any mapper code libraries. Again, going to bring this to attention of mods in chat. [alphamule](User_Alphamule.xhtml "User:Alphamule") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Alphamule&action=edit&redlink=1 "User talk:Alphamule \(page does not exist\)")) 13:51, 29 August 2014 (MDT) 

    Rule of thumb: once you have final hardware, then you can get a mapper #. Before then, pick something arbitrary and otherwise unusable such as m100, or use UNIF. There's more theoretical possible mappers than there will ever be space for. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:40, 29 August 2014 (MDT)
    In the plan as described by KH in the original NES 2.0 doc, 256-511 were to be filled after 0-255 had been filled. Because of the continuing pace of new Chinese discoveries since then, a revised, Unicode-inspired plan splits this into 256-511 for western games and 512-767 for eastern games. Presently, 3840-4095 are reserved for private use, as well as [100](INES_Mapper_100.xhtml "INES Mapper 100") and [248](INES_Mapper_248.xhtml "INES Mapper 248"). --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 15:34, 29 August 2014 (MDT) 

    Cool, thanks for the info! This should be helpful to future readers, as well. I found out that blargg is working on a similar project so I'm waiting for a PM response. I also need to get a working NES to test the hardware out on. [alphamule](User_Alphamule.xhtml "User:Alphamule") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Alphamule&action=edit&redlink=1 "User talk:Alphamule \(page does not exist\)")) 07:42, 30 August 2014 (MDT)

## unicode joke terms

Might we rename "Supplementary Multilingual Plane" and "Supplementary Ideographic Plane"? Jokes about the unicode standard are rather obscure, and don't help anyone understand what these categories are for. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 16:33, 16 April 2015 (MDT) 

    Jokes about unicode ?! I would never have guessed that was some. Anyways, jokes do not belong to here.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 04:12, 17 April 2015 (MDT)
    I found them funny, but it was a laugh and an eye-roll. [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 23:55, 20 April 2015 (MDT)

  
So, the second question to ask is what to name them. From what I can tell, the two planes are meant to distinguish mappers for new games (plane 1) from mappers that are used for illegal compilations/reproductions/modifications (plane 2). Presumably the idea is that emulator authors may wish to de-prioritize pirate mappers that don't contribute new games to the library. There's a bit about "east asia" in the current description which seems unnecessary to me; shouldn't pirate carts from any region be included here? (Russian? Brazilian?) China probably has the most prolific market for it, sure, but I don't think that needs to be part of the plane's definition. My suggestion: 

Plane 0: iNES 1 

  * Still has a few slots open for new homebrew?
  * Put new pirate mappers in plane 2, not here.



Plane 1: Homebrew 

  * New games developed for newly created mappers
  * Undiscovered commercial-era mappers (if any exist)



Plane 2: Pirate 

  * Multicarts used to illegally compile existing games
  * Modifications and hacks of existing games



\- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 10:06, 17 April 2015 (MDT) 

    True, title hacks, graphics hacks, pure mapper hacks, and multicarts "that don't contribute new games to the library." But not all games made for the East Asian market fall into these categories. Some are ports from a more powerful system, such as _Final Fantasy VII_ , _Pokemon_ , and _Somari_. Some are original games (ab)using others' characters, such as _Kart Fighter_ and various _Pokemon_ and _Harry Potter_ -themed games. But what makes East Asian games stand out from games for other markets is provision in the mapper for writing systems with more than a hundred characters, which needs either CHR RAM, ExGrafix, or precomposed phrases. (See [CHR ROM vs. CHR RAM](CHR_ROM_vs__CHR_RAM.xhtml "CHR ROM vs. CHR RAM").) Games for Russian and Brazilian markets can get away with assuming a 1:1 relationship among tiles, glyphs, and characters. Besides, with the deprecation of [UNIF](UNIF.xhtml "UNIF"), we need to allocate mappers for a bunch of CaH4e3 dumps, and separating out the Chinese stuff will help keep that effort from colliding with homebrew too much. So I've renamed the planes to "Plane 1" and "Plane 2" but left the East Asian wording. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 17:45, 19 April 2015 (MDT)

    

    Okay, so you're not trying to separate pirate/multicart from homebrew/legitimate? Your goal is just to separate them by language/region? The language in place right now doesn't seem to mention an appropriate place for new pirate stuff, unless it already falls in the East-Asian category. You should probably write a description that designates that material to one plane or another, so that it's clear where to allocate. Of course, nobody has wanted to allocate anything yet, except zzo38 who appears to have pre-allocated [380](NES_2_0_Mapper_380.xhtml "NES 2.0 Mapper 380"), despite not having released any ROMs for it? I'm never going to be the maintainer of pirate mapper allocation, so I don't feel very strongly about how they are specifically organized. I just found the names borrowed from unicode the opposite of helpful when trying to figure out what the planes were being designated for. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 21:09, 20 April 2015 (MDT)

## n\T icon

The n\T icon ([File:Mfr icon Namco Tengen.png](File_Mfr_icon_Namco_Tengen_png.xhtml "File:Mfr icon Namco Tengen.png")) is a bit harder to read than the ir\AVE icon because both have the same (transparent) background and similar (reddish) colors, so the strokes of the twosort of blend into each other. [NesCartDB](http://bootgod.dyndns.org:7777/search.php?keywords=206&kwtype=pcb) lists 20 games developed or published by Namco and only three (Fantasy Zone, Gauntlet, and Indy 2) that Namco did not develop or publish. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 08:00, 11 June 2015 (MDT) 

    Not sure how you're getting only three non-Namco [for mapper 206](http://bootgod.dyndns.org:7777/search.php?publisher_op=!%3D%60%40%60&publisher=Namco&ines_op=%3D%60%40%60&ines=206&group=groupid&field=2&order=asc&rfa=1+2+11+10); plus Tengen Gauntlet is a pretty high-profile game. I see some Data East games in there, too! (Perhaps you missed the pagination?) [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 19:32, 11 June 2015 (MDT) 

    Most of the rest were renamed and/or ported by Tengen for the US Market, but are originally made by Namco. (AFAICT, the entire reason that Tengen used the MIMIC-1 is because they were importing Namco games to the US market to get around Nintendo's limitation on a maximum of 6 (or was it lower?) games per publisher per year. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:57, 11 June 2015 (MDT) 

    Okay, that knocks out RBI and Ring King. And Pac-Mania is so obviously a Namco-_developed_ game it hurts, but doesn't seem to have had a Famicom release? Super Sprint and Toobin', though... Still, the icons are for manufacturers (that is, publishers), and the Tengen boards _are_ different, so does it really matter for our "whose-mapper-is-this" if someone was pulling an ljn trick? [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 21:14, 11 June 2015 (MDT)

## Why some mappers are assigned to "Rare, Ltd."'s logo

TQROM board's mapper was only used by Rare, Ltd. but uses a chip from Nintendo, the MMC3. It could be labelled Nintendo. 

Mapper #7 was used most often by Rare, Ltd., but not exclusively. Solstice was not developed by Rare, Ltd. but uses mapper #7. Also, the PCB and carts were made by Nintendo, and contains no chip developed by Rare, Ltd, not even the PCB. Other discrete logic mappers which were used by various 3rd party developers but licenced by Nintendo are tagged Nintendo, there is no real reason mapper #7 would escape that rule. 

It's true there is no mapper #7 games that was developed by Nintendo, however, this is true for many other mappers as well, such as the MMC5.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 03:38, 12 June 2015 (MDT) 

    I don't think it's reasonable to expect perfect accuracy when you reduce a mapper to a single 16x16 image. There's no good scheme for choosing that will be entirely consistent. It's always going to be subjective. The point of the images is just to make it easy to spot which mapper is which, so we're not just looking at some monolithic block of buttons. For that reason, I'd rather see a "mostly Rare" mapper with a Rare icon than have yet another Nintendo icon in there, but that's just my subjective opinion. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:08, 12 June 2015 (MDT)

    For the same reasons, I think [Image:Mfr icon Namco Tengen.png](File_Mfr_icon_Namco_Tengen_png.xhtml "File:Mfr icon Namco Tengen.png") is pushing the limits of utility for these icons, and I'd really hate to see someone try to stuff 3 companies into an icon. Don't try to cram too much information into these things. It's supposed to be easy to read at a glance, not tell you 20 years of game development history. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:15, 12 June 2015 (MDT)

    Also, the saturated green of the current icon really stands out from the others in an ugly way (it's very harsh against the white background). Personally I'd go with the 16/32-bit era logo with the gold rimmed blue badge, but again this is a rather subjective suggestion. (I don't think they had a consistent logo before then?) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 15:30, 12 June 2015 (MDT)

    

    Agreed. I intended the Nintendo icon to be used where no third party developer or publisher has a majority. Of the 34 unique games on mapper 7, 26 were either developed or ported by Rare, and an additional four were developed or ported by Zippo Games, which appears to also be Rare in the sense that Ultra is Konami. Hence Ꝛ on 7. Namco likewise has a stake in a supermajority of 206, which is why I think 206 deserves the Namco n. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 19:04, 12 June 2015 (MDT)

It's no problem as long as the way icons are chooses was never meant to be consistent. I was just pointing to an inconsistency.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 12:51, 14 June 2015 (MDT) 

## 32x32 upscaling

I'd like to offer a guideline for upscaling mapper images 32x32. The vast majority display resolution is 16x16, and for that reason all of these images should be optimized for 16x16 first. What this means is that pixel art should be 16x16 and nearest-neighbour rescaled, creating the optimal image at 16x16 and defeating unpleasant linear resampling at 32x32. Do not upscale pixel art by adding smoother edges, etc, as this reduces legibility when downscaled to 16x16. For things that are vector art, text that is already anti-aliased, etc. feel free to take advantage of the full detail level at 32x32. Just remember the most important thing is legibility at 16x16. After doing a quick pass over it, I think everything we've got now is OK. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 22:17, 12 June 2015 (MDT) 

## A newbie-friendly page introduction

As a beginner I found this page to be confusing and unhelpful. The opening paragraphs are overly technical and contain too many details and acronyms for a beginner to grok. The information contained may be factually true and helpful to some, but I'd really like to see a plain-English introduction added in addition to what's already on the page. Something like: 

### Mapper

NES games come in cartridges, and inside of those cartridges are various circuits and hardware. Different games use different circuits and hardware, and the configuration and capabilities of such cartridges is called their mapper. Mappers were used to extend the system and bypass its limitations, such as by adding RAM to the cartridge or even extra sound channels. More commonly though, mappers were used to allow games larger than 32K to be made. 

Example games and their mappers: 

  * Contra uses a mapper called UNROM, providing it with 128K of PRG ROM space.
  * Castlevania 3 uses a mapper called MMC5 offering additional sound channels (on Famicom systems only).
  * Bible Adventures uses the "Color Dreams" mapper which contains CIC-defeating circuitry.
  * Super Mario Bros. uses the simplest mapper: NROM, which contains 32K of PRG ROM and 8K of CHR ROM. Many early NES games and modern homebrew games use the NROM mapper.



[Pubby](https://www.nesdev.org/w/index.php?title=User:Pubby&action=edit&redlink=1 "User:Pubby \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Pubby&action=edit&redlink=1 "User talk:Pubby \(page does not exist\)")) 11:21, 11 May 2016 (MDT) 

    I wanted it to provide true and complete information. I may be in the habit of using technical vocabulary. Which acronyms are newbie-unfriendly besides PRG and CHR? CIRAM? (also, Famicom version of Castlevania 3, 悪魔城伝説, is on VRC6, not MMC5. Also, on some metrics, onechip mapper is simpler than NROM.) [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 11:17, 10 July 2017 (MDT)

    Pubby - this is an easy to read introduction. I agree the page as it is now is hard for a newcomer like myself. I would suggest adding some kind of introductory text like this describing the general problem mappers solve. [Ericandrewlewis](User_Ericandrewlewis.xhtml "User:Ericandrewlewis") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ericandrewlewis&action=edit&redlink=1 "User talk:Ericandrewlewis \(page does not exist\)")) 08:56, 3 February 2019 (MST)

## Mapper Grid

This causes this page to choke when you load it. Should it really be on the "explaining mapper" page?[Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 11:28, 10 July 2017 (MDT) 

    Would it help if the images were 1/4 the size, at the original 16x16 instead of the 32x32 they were "upgraded" to? I never liked the upscale effort, since all of them are meant to be displayed at 16x16 on a normal screen anyway. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 15:58, 10 July 2017 (MDT)

    

    Then again, after PNG compression they seem to be mostly he same size anyway. I don't know what exactly is "choking" in this case; bandwidth? Number of image fetch requests? Would ditching the company specific icons help, alternatively? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 16:03, 10 July 2017 (MDT)

    

    

    The page serves the 16x16 downsampled version in the first place, and ~~uses javascript to replace them with~~ _uses the new imgset html img parameter to serve_ the high-resolution version if appropriate. That said ... I really just kicked off the upsampling project to get rid of the of the things with baked-in chroma error ("subpixel AA"). —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:48, 10 July 2017 (MDT)

    

    

    It's the table, I expect. I'm not sure what precisely makes it that a preview of that is taking "Parser profiling data: CPU time usage 1.033 seconds Real time usage 1.081 seconds" whereas this is only 0.003s. (Server-side, not client-side.) [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 16:39, 10 July 2017 (MDT)

    

    

    

    That's pretty clearly caching the resultant HTML for subsequent servings. Quickly experimenting: neither changing the specified rescale size (|16px) nor changing the specific image served (changing them all to "Generic") sped things up at all. Only removing the ≈800-images-are-equivalent-to-transclusions sped up preview. Regardless, due to caching, is this even worth caring about? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:48, 10 July 2017 (MDT)

Well, at any rate maybe it would be worth getting rid of the NES 2.0 plane grids? If the table really is a performance concern, having 1/3 of them would surely help that problem, wouldn't it? Just put the 4 plane 1 mappers into a list, there's no need to show the whole empty grid for that. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:40, 10 July 2017 (MDT) 

## pirate/pirate MMC3

The specific bootleg company logos have replaced this information. I'm uncertain if this is a good thing. [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 11:28, 10 July 2017 (MDT) 

    I'm really not clear how the "pirate/pirate MMC3" grouping was ever useful. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:48, 10 July 2017 (MDT)

    

    Tepples [first introduced the pirate icon](http://wiki.nesdev.org/w/index.php?title=Mapper&diff=next&oldid=4749) when he began to add images. [I introduced the MMC3-like pirate icon](http://wiki.nesdev.org/w/index.php?title=Mapper&diff=4825&oldid=4824), and I think my goal was simply to break up the "pirate" category using the [MMC3-like mapper category](Category_MMC3_like_mappers.xhtml "Category:MMC3-like mappers"). I don't think any of the "pirate MMC3" ones have subsequently been replaced by any company icons, but I don't really have a strong opinion whether this visual categorization is "useful". The goal was just to collect/express that MMC3-like categorization that the wiki already had. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:31, 10 July 2017 (MDT)

## mappers 256-9

"have been tentatively allocated" per rainwarrior's edit…citation? ~~I looked at FCEUX's source and couldn't find it.~~ <https://sourceforge.net/p/fceultra/code/3372/tree/fceu/trunk/src/ines.cpp#l715> Nvm, found it. Annoyingly, incorrectly, their source code calls NES 2.0 "ines 2.0"… [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 11:28, 10 July 2017 (MDT) 

    I did not make that edit, lidnariq was responsible: [http://wiki.nesdev.org/w/index.php?title=Mapper&diff=11931&oldid=11727](http://wiki.nesdev.org/w/index.php?title=Mapper&diff=11931&oldid=11727) \- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 15:54, 10 July 2017 (MDT)

    

    Whoops. You're right, of course. [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 16:39, 10 July 2017 (MDT)

## Whirlwind Manu

If somebody could find a logo of Whirlwind Manu and make an icon out of it, it could be used similar to Kaiser's for their respective FDS conversion amppers. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 03:39, 25 April 2018 (MDT) 

    [Front of a catalogue](https://fcgamer.files.wordpress.com/2015/04/img_5063.jpg). [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 07:43, 25 April 2018 (MDT) 

    I have no idea how to stuff either of those very-wide logos into the square boxes we've been using... but I did clean them up a tiny bit:[![Whirlwind-Manu-Whirlwind.png](../wiki-images/Whirlwind-Manu-Whirlwind.png)](File_Whirlwind_Manu_Whirlwind_png.xhtml), [![Whirlwind-Manu-Logotype.png](../wiki-images/Whirlwind-Manu-Logotype.png)](File_Whirlwind_Manu_Logotype_png.xhtml) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:21, 25 April 2018 (MDT) 

    I'd go for their initials, using the font they used most of the time on their cover. [Example A](https://1.bp.blogspot.com/-pCDHiWEiVGA/WXUuBPh7T_I/AAAAAAAARbE/vHOn6bnaQnccbcYLGEW7T622rY-E4z73QCLcBGAs/s1600/wm_tennis.png), [Example B](https://3.bp.blogspot.com/-Mo5PFSc9I4k/WXTs-CqocmI/AAAAAAAARZI/xAbhUAa1CvAp0gdUNNH7KzdZhZ-C8cLjgCEwYBhgL/s1600/LF53.png) \--[MLX](https://www.nesdev.org/w/index.php?title=User:MLX&action=edit&redlink=1 "User:MLX \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:MLX&action=edit&redlink=1 "User talk:MLX \(page does not exist\)")) 12:05, 25 April 2018 (MDT) 

    The font used there appears to be [Romic Extra Bold](http://www.identifont.com/show?3G50). —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:27, 25 April 2018 (MDT)
    Well, here's an uppercase W in Romic Extra Bold, black on transparent: [![Mfr icon Whirlwind Manu.png](../wiki-images/Mfr_icon_Whirlwind_Manu.png)](File_Mfr_icon_Whirlwind_Manu_png.xhtml)—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:10, 25 April 2018 (MDT) 

    Wonderful. A dream. Thank-you. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 14:18, 25 April 2018 (MDT)

## Notes on Undocumented/Unimplemented Mappers

Even though a mapper is undocumented on the wiki, it does not necessarily mean it is or has not been used in the past. Here are known use cases or reasons why the mapper is not used : 

**Plane 0**

  * 51 - 11-in-1 Ball Games [p1][!]
  * 53 - Supervision 16-in-1 [p1]
  * 98 - No known usage or assignment
  * 102 - Duplicate as 284
  * 109 - Duplicate of 137
  * 110 - Duplicate of 243
  * 120 - Tobidase Daisakusen (FDS Conversion)
  * 122 - Duplicate of 184
  * 124 - Super Game Mega Type III
  * 127 - Double Dragon 2 Pirate - With Cheats
  * 128 - 1994 Super HiK 4-in-1 [UNROM Games]
  * 129 - Duplicate of 58 and 213
  * 130 - Duplicate of 331
  * 131 - Duplicate of 205
  * 135 - Duplicate of 141
  * 160 - Duplicate of 90
  * 161 - Duplicate of 1
  * 162 - Legend of Zelda - Tri Force (ES-1096) (Ch) [U][!]
  * 170 - Shiko Game Syu (Unl)
  * 175 - 15-in-1 (Kaiser - KS-122) (Unl)
  * 179 - Duplicate of 176
  * 204 - 4-in-1 (Mapper 204) [p1].nes, 64-in-1 [p1][!]
  * 214 - Super Gun 20-in-1 [p1][!]
  * 216 - Bonza (R) (REV0) [!].nes, Magic Jewelry 2 (As) [!]
  * 217 - 500-in-1
  * 223 - Duplicate of 199
  * 238 - Contra Fighter (Unl) [U][!]
  * 239 - No known usage or assignment
  * 244 - Decathlon (Asia) (Ja) (Unl)
  * 247 - No known usage or assignment
  * 251 - Duplicate of 45



**Plane 1**

  * 275-280 - Assigned by dragon2snow of nesbbs a.k.a. zxbdragon of NESDev forums for Nestopia Plus! for unknown games
  * 311 - Bad dump of SMB2J pirate using Mapper 43
  * 316-318 - Assigned by dragon2snow of nesbbs a.k.a. zxbdragon of NESDev forums for Nestopia Plus! for unknown games
  * 321 - Duplicate of 287
  * 343 - Sheng Tian 2-in-1 #1 ("reset based" mapper only works with a particular bank order)
  * 352 - Kaiser 4-in-1 (KS106C) (reset-based NROM multicart variant, issue with dealing with mirroring switching between games)
  * 377 - 1998 Super Game 8-in-1 (JY-111).nes, Mapper 267 with 256KiB outer PRG bank
  * 399 - Star Versus
  * 400 - 8 BIT XMAS 2017
  * 401 - Super 19-in-1 (VIP19)
  * 402 - 22-in-1 (J-2282) Olympic Games
  * 403 - Tetris Family 19-in-1 (89433)
  * 404 - 1998 Super HiK 8-in-1 (JY-021B)
  * 405 - City Patrolman, Space Castle (UM6578 games)
  * 406 - Haradius Zero
  * 407 - Win, Lose & Draw Plug 'n Play with Tablet & VT03 Hardware
  * 408 - Konami Collector's Series Advance Arcade Plug & Play (Last Mapper currently assigned in NES 2.0 Plane 1)



**Plane 2**

  * 531 - LittleCom PC-95 (includes speech chip)
  * 532 - Sangokushi II - Haou no Tairiku with VirtuaNES EX-assisted/overlaid Chinese Translation
  * 545 - 4-in-1 (ST-80 PCB)
  * 546 - 10-in-1 Tenchi wo Kurau
  * 549 - Meikyuu Jiin Dababa (Kaiser FDS Conversion)
  * 553 - Dong Dong Nao 1 (Asia) (Ja) (Unl).nes (Penguin and Seal original Sachen release )
  * 554 - Akumajou Dracula (Kaiser FDS Conversion) (Last Mapper currently assigned in NES 2.0 Plane 2)



    Well they don't have to be undocumented forever. If you've got the info, adding even a stub explanation article for these would be useful. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 21:40, 16 May 2020 (MDT)

# Icons for "pirate" mappers that aren't just for pirates

Mappers such as 176 and 268 are _mostly_ used for bootleg cartridges, but occasionally by homebrew or otherwise legitimate releases as well. What icon shall be used for them? Still "Pirate MMC3"? [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 13:47, 2 July 2020 (MDT) 

    Mapper 176 already has the Waixing icon instead. Mapper 268 ... eh, switching it to Mfr_icon_homebrew wouldn't be wrong. It is nice to have more descriptive icons, and Mfr_icon_Generic, Mfr_icon_pirate, and Mfr_icon_Pirate_MMC3 are really about as non-descriptive as you can get.
    On the other hand, as you say, m268 _is_ mostly bootleg multicarts, and plenty of homebrew has been written for other pre-existing mappers and neither has nor should preempt other logos. We don't know anything about the provenance of the mindkids/coolboy ASIC, right? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 20:01, 2 July 2020 (MDT)

## Lists of Mappers that should Use/Ignore the Mirror Bit

Is it possible to have a definitive list of which mappers should honor the mirror bit and which should ignore it? Thanks. 
