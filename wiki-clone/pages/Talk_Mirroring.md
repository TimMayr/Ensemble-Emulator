# Talk:Mirroring

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AMirroring) | View [other pages](Special_AllPages.xhtml#Talk_Mirroring)

## Contents

  * 1 PAxx vs. PPU Axx vs. CHR Axx
  * 2 "L" mirroring
  * 3 4-screen capable mappers
  * 4 Man what happened to my mirroring chart!
  * 5 Mirroring images
  * 6 Games that depend on WRAM mirroring?
  * 7 MMC5 and "four screen memory"
  * 8 Castlevania 3
  * 9 Moving the bulk of scrolling update problems to scrolling page
  * 10 Minor Grammar Fix



## PAxx vs. PPU Axx vs. CHR Axx

These [edits](http://wiki.nesdev.org/w/index.php?title=Mirroring&diff=5576&oldid=5489) by infiniteneslives confuse me a bit. I've seen "CHR A10" and "CHR A11" used for the lines going to the CHR ROM. In ASIC mappers, these tend not to match PPU A10 (PA10 for short) and PPU A11 (PA11). --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 12:04, 23 February 2013 (MST) 

## "L" mirroring

Using a single and-or-invert gate, or three NAND or NOR gates, along with two bits from a latch gets you controllable 1/H/V/L mirroring. This is roughly what mapper 243 does. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 00:32, 4 May 2013 (MDT) 

## 4-screen capable mappers

lidnariq wrote [here](http://wiki.nesdev.org/w/index.php?title=Mirroring&oldid=10630): 

    None of MMC3, MMC5, m77, nor m99 "are capable of mapping some of their CHR[...] into the nametable space". The question of CHR as nametables is largely orthogonal to 4-screen layout; I don't think it belongs here.

I think any mapper that can map 4 different 1k pages into the 4 nametable regions is relevant to 4-screen mirroring (this is what I consider the definition of 4-screen mirroring). Why don't you think they belong there? 

  * [MMC3](MMC3.xhtml "MMC3") \- Rad Racer 2.
  * [MMC5](MMC5.xhtml "MMC5") \- Using fill-mode as a 4th nametable qualifies, I think, but is very limited.
  * [VRC6](VRC6.xhtml "VRC6") \- Lets you map CHR-ROM into nametable pages arbitrarily. (Not sure if any games do it.)
  * [Namco 163](INES_Mapper_019.xhtml "Namco 163") \- Lets you map CHR-ROM into nametable pages arbitrarily. (See Final Lap.)
  * [iNES Mapper 077](INES_Mapper_077.xhtml "INES Mapper 077") \- Uses combination of VRAM and CHR-RAM to create 4 RAM nametables. (Napoleon Senki)
  * [iNES Mapper 099](INES_Mapper_099.xhtml "INES Mapper 099") \- Not sure why this became a mapper, but as described I don't see why it doesn't qualify as 4-screen capable?



\- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 10:42, 14 May 2015 (MDT) 

    The sentence discusses _mapping some of their CHR into the nametable space_. Which is false for four of the examples, and orthogonal to how four-screen mirroring works. If rephrased to instead be what actually belongs in the section ("allows treating nametable address space as a 64x60 tile map") it's then redundant with the surrounding text. (MMC5 supports three real nametables and a fourth where all 960 locations are the same tile and same attribute... which I personally wouldn't count here) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 10:58, 14 May 2015 (MDT)

    

    I don't think it's orthogonal at all! What do you mean by this? Mapping CHR-RAM or CHR-ROM into the nametable space is one way of implementing 4-screen mirroring. I've separated MMC5 and the Vs mapper. VRC6, N163, and 77 all map their CHR-RAM or ROM chips directly into the nametable space, so I don't know which four examples you think are false; at most it would just be MMC3? I thought Rad Racer 2 mapped CHR-ROM as nametables, but I would have to check again. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:07, 14 May 2015 (MDT)

    

    

    CHR is stored on memory. Memory itself is not CHR. That _Napoleon Senki_ uses one 8 KiB RAM that's used for both two nametables and for 6 KiB of CHR-RAM doesn't mean that the **CHR** is mapped into the nametables. Additionally, I say it's orthogonal because you don't mention [JY Company](J_Y__Company_ASIC.xhtml "JY Company") or [Sunsoft 4](INES_Mapper_068.xhtml "INES Mapper 068") in this section (but instead mention them in the "other" section).
    If you replace "CHR" with "on-cartridge PPU memory" then this specific sentence is redundant with the surrounding text: "MMC3" is redundant with "Add a 6264 8 KB RAM on the board. CIRAM /CE is pulled high, and the cartridge RAM is enabled at $2000-$3FFF. The PPU itself never uses $3000-$3FFF during rendering, but 8 KB RAMs are easier to find than 4 KB RAMs.". The missing "MIMIC-1" is redundant with "Add an extra 2 KB of RAM on the board. Decoder logic enables CIRAM only for $2000-$27FF and the cartridge RAM $2800-$2FFF.". Mapper 77 is repeated in the list below. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:18, 14 May 2015 (MDT)

Okay, so what I was really hoping to provide was a list of mappers that can do 4-screen mirroring, and a brief description of each. I've reorganized them into a bulleted list in this way, and I hope this obviates the problem you had with the sentence as it was. What is MIMIC-1, should it be on the list? (I can't seem to find info on it in the Wiki.) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:28, 14 May 2015 (MDT) 

    Yes, this solves my complaint. Thanks!
    MIMIC-1 was Tengen's rebadged Namco 108, so it's redundant with the m206 mention. Might want to figure out how to rephrase the [Vs. System](Vs__System.xhtml "Vs. System") bullet too, because [the mappers](Vs__System.xhtml#See_also "Vs. System") used with it intrinsically had 4-screen layout, not just m99. Tangenting: No VRC6 games used 4-screen layout (let alone ROM nametables); apparently no m211 (JY Company) did either. But the hardware certainly supports it... —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:43, 14 May 2015 (MDT)

The only mapper *not* capable of 4-screen mirroring I can think of is MMC5. All the others are capable of it by adding an additional chip on the cartridge (or two). The fact very few games used this is a different matter. Also, the ROM nametables have absolutely nothing to do with 4-screen mirroring, it's two completely different things. When we talk about "Nametable mirroring", we normally have RAM in mind, ROM is a very rare and particular case that is its own weird beast.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 11:31, 14 May 2015 (MDT) 

    Whether nametables are ROM or RAM is important, and related, but it's not "mirroring" at all. Extra nametable RAM is its own thing, separate from mirroring, but it's generally very important how it is combined with it. We need to cover both here. (Also, with the iNES 2 format, it becomes an arbitrary choice for VRC6/N163.) Both cases are pretty rare; you're arguing that 3 games is less rare than 2? This is not a case where one is an edge case and the other dominates. (They are all edge cases.) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:40, 14 May 2015 (MDT)

    I like your point that any mapper can do 4-screen RAM, though. I added an entry noting that it's available as an iNES header flag, and I think the hardware how-to covers the case of actually adding it to a board that didn't have it. I think the rest of the list is important because these are the only boards you will find ready for 4-screen, just like it's important to know which boards would normally be set up for CHR-RAM and not CHR-ROM. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:51, 14 May 2015 (MDT)

## Man what happened to my mirroring chart!

Back when I created this, the idea was to have a quick and simple overview though the different basic mirroring/scrolling combinations, letting the reader adult enough to understand this on his own and extend this to whatever he feels. 

Someone apparently turned it into the current monstrosity that attempts to document every single mirroring/scrolling combination ever possible with NES hardware, including many which were never used by any games or not even homebrew tech demoes (I think especially of the so called "3-screen mirroring" here). This is downright ridiculous, and it lost its original sense of the very reason why I created this chart back then.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:55, 15 May 2015 (MDT) 

Once, including a commercial games, it will be helpful for the production of one of homebrew games.--[PGCX864](https://www.nesdev.org/w/index.php?title=User:PGCX864&action=edit&redlink=1 "User:PGCX864 \(page does not exist\)") ([talk](User_talk_PGCX864.xhtml "User talk:PGCX864")) 07:57, 15 May 2015 (MDT) 

    It's not currently helpful at all. I'm sorry. What you seem to be doing is trying to categorize everything you can find; this is very different than providing a helpful guide! Right now you have 4 columns (status bar, parallax, etc.) and 11 categories; that's 44 different cases you are throwing at the reader? Someone who is struggling to understand mirroring will have no idea how to pick from this or figure out what they need from it. This chart isn't even much use for someone who knows how mirroring works already.

    If you want to categorize every game in existence by how they do rendering and their mirroring approach, that might be okay on its own article page, like how we have things like [List of games with expansion audio](List_of_games_with_expansion_audio.xhtml "List of games with expansion audio") or [Sprite overflow games](Sprite_overflow_games.xhtml "Sprite overflow games"), but what you're doing now is too much information (or maybe just too poorly organized) for this article about [Mirroring](Mirroring.xhtml "Mirroring"). Maybe [List of games by mirroring technique](List_of_games_by_mirroring_technique.xhtml "List of games by mirroring technique")? Perhaps reference like that organized in one place would be useful to somebody, and we can just link to it from this mirroring article. Just leave a small handful of useful/common examples here, the initial grid was supposed to be a tutorial and quick summary. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 09:50, 15 May 2015 (MDT)

    

    To explain what I mean about it not being helpful. Think of what you need to know about mirroring and NES rendering to understand how to implement having a status bar on the top or bottom. Mirroring and NES rendering is not something you can just look up in a chart and pick an answer. Nobody's going to be able to make Guardian Legend or After Burner after reading that chart, they have to understand a lot more about rendering, not just mirroring. The mirroring teaches you next to nothing about how those games' rendering works. What we want to do in a tutorial is illustrate how mirroring works with a few easy to understand examples. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 10:02, 15 May 2015 (MDT)

Another way of thinking about this: if Battletoads can do 2D scrolling with a status bar, using only single screen mirroring and no IRQ, really you can do any scrolling method with any mirroring, if you want to work hard enough for it. The short of it is that for 1D scrolling, there are a few choices that make it easy (horizontal, vertical, metroid), and there's no turnkey solution for 2D scrolling but we can make a few good suggestions (like to look at SMB3), but beyond that the solutions are very game-specific and tricky, far too detailed to summarize here, and you can do it 1000 different ways (each of which would need a whole article to properly explain). Hmm, this is pretty much exactly what the chart already expressed before PGCX864 started adding to it: [revision 8215](http://wiki.nesdev.org/w/index.php?title=Mirroring&oldid=8215). I think there's use for a [comprehensive catalogue of which games use which mirroring techniques](List_of_games_by_mirroring_technique.xhtml "List of games by mirroring technique"), but it doesn't belong in this article. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 23:53, 16 May 2015 (MDT) 

That revision is way better, but is still too much because it mention the never-used L shaped and diagonal mirroring. I do not think there is a need to document the list of mirroring/scrolling combinations all games uses, this is almost endless and each game is specific. If you are interested about a particular game, just look at an emu with a NT viewer, it's always interesting. There's even less need to document every unused mirroring/scrolling combination we could ever think off. Personally, as the creator of this mirroring char back then I believe it's purpose has been completely misunderstood and that, with the distance, I can see this chart was never very useful. I believe removing it completely is probably the best option. The animation on the SMB [scrolling](PPU_scrolling.xhtml "Scrolling") article is probably a better approach at documenting the thing.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 06:46, 17 May 2015 (MDT) 

This revision [[1]](http://wiki.nesdev.org/w/index.php?title=Mirroring&direction=prev&oldid=6190) is probably how this page should have stayed, without those ugly image with barely readable letters [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 06:58, 17 May 2015 (MDT) 

    I've made an attempt to rebuild the table with just the simple/straightforward techniques, and I've moved the monster table over to [List of games by mirroring technique](List_of_games_by_mirroring_technique.xhtml "List of games by mirroring technique"), which will need some organizational work but I think might make a good start for a nice survey of the advanced techniques out there. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 17:00, 17 May 2015 (MDT)

    

    Thank you Rainwarrior, it is much better now. The table makes sense again. However, there is just a little tiny weakness. It says _Horizontal mirroring makes it difficult to use a status bar._ Taken like this without any additional information it is hard to understand why. I'd suggest replacing it with _Vertical scrolling makes it tougher to use a status bar._ This has the advantage of being more exact:

    

    1\. Vertical scrolling makes it tougher because you have to use $2006 as well as $2005
    2\. Vertical scrolling makes it tougher because you need to plan carefully where the status bar and playfield will go in VRAM (no matter the mirroring)
    3\. The use of _thougher_ makes it clear it's still perfectly possible and reasonable, the use of _difficult_ makes it sounds like it's not a reasonable choice.

    

    Also, why the wiki links for _status bar_? Is there an intention to create a status bar page later?[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 03:19, 18 May 2015 (MDT)

    

    

    Well, the reason I said "horizontal mirroring" and not "vertical scrolling" is that if you do vertical scrolling with a different type of mirroring (e.g. single screen) it's a lot easier to make a status bar. It seems to me that it's specifically horizontal mirroring that causes the problem, mostly because of how to lay out the nametables so you can scroll split to it. I don't think of having to use $2006 is tougher, we have a simple recipe for that and it doesn't require compromises or extra planning. There's no recipe for the nametable layout problem, though- you have to do something special to get around it. Every game I can think of that has horizontal mirroring + a status bar had to do something pretty advanced. SMB3 and Jaws get around the problem by limiting Y scroll. Crystalis has 2 splits requiring an IRQ. Can you think of any others? I can think of a lot of games that have vertical scrolling and status bars, but I'm having a hard time finding any others that use horizontal mirroring. Also, yes I figure [status bar](https://www.nesdev.org/w/index.php?title=Status_bar&action=edit&redlink=1 "Status bar \(page does not exist\)") should have its own article eventually, just leaving a redlink as a reminder of that. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:41, 18 May 2015 (MDT)

    

    

    I think there was also a scrolling tech demo a while back (can't find it at the moment) that relocated the status bar in the nametable every few frames, but again this is a really advanced thing to do! The Hiatus Ward demo also had horizontal mirroring, Crystalis style. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:49, 18 May 2015 (MDT)

    

    

    

    Well, you are correct, but vertical scrolling + status bar + vertical mirroring is likely even more difficult. Jungle Book and Krusty's Fun House does that (using 2 radically different techniques). Single-screen mirroring is the only "easy" way to have free-bidectional with status bar. As for $2006, you are correct it is not a problem today, but it appears that when the NES was released it took time for the developpers to start using this trick, it wasn't used before Zelda I believe. Using it for a status bar at the top is harder, but if the status bar is at the bottom it makes it harder to combine with a sprite zero hit on a unknown playfield, and to combine with gameplay's program. With IRQ capable mapper it's no problem indeed. Not related, but you should look into Kirby's Adventure, Double Dragon series, Gradius II and Gargoyle's Quest if you are interested in games with free-directional scrolling with horizontal mirroring and status bar. (neededless to say, all uses radically different techniques).[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 12:30, 18 May 2015 (MDT)

    

    

    

    

    Both single screen and horizontal mirroring is perfect for vertical-only scrolling with a status bar. Glitches hidden by the bar, easy to implement, etc. I don't know any games that do it, though, I've only seen it with 2D scrolling. An example would be good, but maybe this is what should be said in the chart. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:43, 18 May 2015 (MDT)

    

    

    

    Oh yeah, you are right (assuming you meant vertical mirroring), Life force does that in it's even levels, and Guardian force in spaceship shooter mode for example. It's hard to word what exactly is "difficult", and it's completely subjective after all. The only really difficult part is to "get the trick done", and even in the worst case it's still not the most difficult part of writing a complete working game.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:21, 19 May 2015 (MDT)

Explaining this reversion: [10848](http://wiki.nesdev.org/w/index.php?title=Mirroring&diff=10848&oldid=10845) \- Limited 2-screen (Gyromite, Wrecking Crew, etc.) I deliberately left out of the chart, because this is an issue of scrolling, not mirroring. The simple case for mirroring is exactly the same as if you are doing unlimited 1-axis scrolling. Mega Man does not actually use switching H/V, check it with a debugger. Zelda and Ducktales are also complicated. Just because these games do screen-locked scrolling doesn't mean they're doing it the naive way; the status bar especially throws a monkey wrench into it. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:08, 19 May 2015 (MDT) 

## Mirroring images

Bregalad complained that he doesn't like the lettering on the mirroring images. I actually like them a lot, but I have my own bone to pick with them, which is the addressing on the side. The base address of each nametable is the most important address, and it only appears on the left side tables. Everything else is just noise, like the 24CF at the end of a row isn't helpful to anybody trying to learn. (It's clear what it is to someone who already understands nametables, but it's useless to that person.) Here's a proposal for a new style: 

[![Mirroring proposal.png](../wiki-images/Mirroring_proposal.png)](File_Mirroring_proposal_png.xhtml)

  1. Removed the address columns in favour of larger embedded base addresses.
  2. More standard lettering.



Any thoughts about it? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:44, 17 May 2015 (MDT) 

    For me, it's way better.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:32, 18 May 2015 (MDT)

Finally got around to this. If the new style isn't to someone's liking feel free to take another crack at it. Here's the python program that generated the new set: [Category:Nametable mirroring diagrams](Category_Nametable_mirroring_diagrams.xhtml "Category:Nametable mirroring diagrams") \- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 01:26, 5 March 2017 (MST) 

## Games that depend on WRAM mirroring?

Apparently, there's 3, but no one can seem to remember them. They likely use a page# index and take advantage of ignoring higher bits and just increment infinite loop 0...FF without bothering to reset to 0 after 7, so go through 8-1F (ANDed with #$1F)? But that makes no sense as you could have just ANDed with #$07. Anyone know for sure what they did and why? [alphamule](User_Alphamule.xhtml "User:Alphamule") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Alphamule&action=edit&redlink=1 "User talk:Alphamule \(page does not exist\)")) 00:44, 15 February 2017 (MST) 

    Well, I _intentionally_ used WRAM mirroring in Driar's NROM port to move the stack relative to the self-modifying code in RAM—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:01, 15 February 2017 (MST)

## MMC5 and "four screen memory"

Let's not have an edit war. If Bregalad wants to discuss this, we'll discuss this here. But until such time as we come to an agreement here, neither of us should edit the page. 

Fact of the matter is, MMC5 does support four distinct renderable nametables at the same time. Arguing that it doesn't count because one is only 10 bits instead of 8160 doesn't change that it is a distinct thing. I just can't come up with any argument for why ROM nametables are allowed to count but the fill-mode nametable isn't. — [Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:33, 4 March 2017 (MST) 

    I think fill mode counts, and it's fully disclosed by the description so I don't see anything misleading about having it in the list. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 19:26, 4 March 2017 (MST)

Definition of 4-screen mirroring according to Rainwarrior on this very talk page : 
    
    
    I think any mapper that can map 4 different 1k pages into the 4 nametable regions is relevant to 4-screen mirroring
    (this is what I consider the definition of 4-screen mirroring).
    

and acording to the wiki page 
    
    
    With additional RAM and/or PPU address mapping present on the cartridge, 4 unique nametables can be addressed through the PPU bus,
    creating a 64x60 tilemap, allowing for more flexible screen layouts. Very few games made use of this type of mirroring.
    

MMC5 fits neither of those definitions. MMC5 supports 4 different modes for each nametable, but does not support 4-screen mirroring, even if each of the name tables is set to a different mode (leading to the 4 nametables having a different mode each) it won't work as the name table in the fill mode is not a 1k page of data, but a single repeated tile instead. This does not create a 64x60 tilemap. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:45, 5 March 2017 (MST) 

    I think it does fit the current definition in the wiki? It creates 4 different nametable pages. Yes one of them is a single repeated tile, but the important thing is that it's different from the other 3, and that since it's a strange case it should explain that (which it does). It's not like it just says "MMC5" in some misleading way, it explains the whole thing. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 01:54, 5 March 2017 (MST)

    Just because the fill-mode nametable is algorithmically generated instead of RAM doesn't mean that it's not a nametable. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:37, 5 March 2017 (MST)

    I should say, though, that I think it's something that should be mentioned as an oddball case whether or not you think we should use a definition of 4-screen that is strictly exclusive or inclusive to it. It's s special case worthy of a special mention, IMO, and is related _enough_ to 4-screen mirroring at any rate. If you think it's muddling the definition somehow or should be worded differently I'm open to suggestions, but my hope when I wrote the current wording was that it would be clear exactly what it is and isn't from reading it. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 02:10, 5 March 2017 (MST)

    We definitely agree MMC5 is a special case. In my opinion, the notion of "vertical" "horizontal" and "4-screen" mirroring only applies to regular mappers making regular use of VRAM. MMC5 is so weird it doesn't fall in any of those categories, this should be mentioned. The point of 4-screen mirroring is (mostly) to be able to scroll vertically and horizontally without updating nametables. MMC5 can't do that. The point of fill mode is not to have a 4th distinct nametable - but rather to have a repeated tile all across the screen - when Fill Mode is used, it's intented to be used for 4 screens at once. Using Fill Mode for just 1 screen and 3 nametables for the other 3 just doesn't make any sense - you couldn't scroll a video game level using that configuration. However it doesn't need to becase ExRAM is even more useful than 4-screen mirroring, for instance MMC5 allows glitch-free multidirectional scrolling thanks to mode #1 without the need for 4-screen mirroring. So the MMC5 is already extensively mentionned in the "other" paragraph:
    
    
    Other uncommon types of mirroring are available in other boards, such as TxSROM variations of the MMC3, extended techniques available to the MMC5, 
    arbitrary VRAM mirroring arrangements by the Namco 163, or ROM mirroring arrangements using mappers that allow ROM nametables.
    

There is no need to mention the MMC5 in the 4-screen mirroring paragraph, because 1) MMC5 does not support 4-screen mirroring and 2) it's already covered in the "other" paragraph where it is supposed to be, because it's where it fits. If information in "other" is lacking, then it should be extended there. 

[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 02:50, 6 March 2017 (MST) 

    Your argument why the fill-mode nametable doesn't count is equally applicable to ROM nametables. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:20, 6 March 2017 (MST)
    Which argument exactly are you refering to ? ROM nametables is an entierely other thing - you could use that with either mirroring and even mix ROM and RAM nametables if this makes any sense. MMC5 supports neither ROM nametables nor 4-screen mirroring.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 11:51, 6 March 2017 (MST)

    

    Your entire argument seems to boil down to ~the only possible purpose of 4-screen layout is to use it the exact way Gauntlet, Rad Racer 2, or Napoleon Senki do~, i.e. four independent not-remappable RAM nametables. But none of the other hardware listed in the section is used with a game that does that. And this section of the document isn't about "what do legacy games do" but rather "what hardware supports 4 simultaneous different 32x30 tilemaps in the 64x60 addressable space". —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:54, 6 March 2017 (MST)

    

    Exactly - and the MMC5 is perhaps the only Mapper who can't support 4 simultaneous different 32x30 tilemaps in the 64x60 addressable space. All other mappers can do that quite easily by adding RAM - but the MMC5 already tricks the PPU adress space in it's own way which is largely incompatible with the normal 4-screen mirroring configuration.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 05:51, 7 March 2017 (MST)

    

    

    Please explain exactly how a fill-mode nametable is not "a simultaneous different 32x30 tilemap". Additionally, please explain or take a guess as to why there's a list of hardware on the page, if you think the only possible reason to talk about 4-screen layout is to use it in the exact way that Gauntlet, Rad Racer 2, or Napoleon Senki do. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:28, 7 March 2017 (MST)

    

    

    Fill mode is a 1x1 tilemap, not 32x30. There's not 1k of data - there's just 1 byte. I do not understand what you mean in the second part starting with "please explain..."[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 00:12, 8 March 2017 (MST)

    

    

    

    Fill mode is a single tile that is _filled_ over an entire 32x30 tilemap. If you set up the MMC5 to display four different simultaneous nametables, one of which is (necessarily) the fill-mode nametable, it is still displaying four different nametables. I can easily construct situations in which three corners of the 64x60 tilemap would have "real" data and the last one would be the fill-mode nametable and the game would scroll through showing all four nametables at the same time.
    So, pray tell, what exactly is the correct description of the situation in which four different nametables are displayed onscreen simultaneously? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:34, 8 March 2017 (MST)
    Fill mode is not a "name table" per se because there's no "table", just a single byte.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 00:54, 10 March 2017 (MST)

    

    

    

    The PPU doesn't know how to render "a single byte". It knows what to do with a nametable and a pattern table.
    Arguing that the fill-mode _nametable_ isn't a nametable is to deny its function. To the PPU, it looks like a nametable, [swims like a nametable, and quacks like a nametable](https://en.wiktionary.org/wiki/if_it_looks_like_a_duck,_swims_like_a_duck,_and_quacks_like_a_duck,_then_it_probably_is_a_duck), so how can you seriously argue that it isn't a nametable?
    Just because it's entirely configured via a small number of register writes? So are ROM nametables!
    Does it somehow magically become _not_ a nametable just because it's configured to displayed next to three RAM nametables?
    Or do you somehow disqualify it just because it's low entropy? Does cleared RAM not count as a nametable them? The PPU doesn't care about entropy; it just cares about nametables and pattern tables. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:02, 10 March 2017 (MST)

    

    

    

    

    

> "Or do you somehow disqualify it just because it's low entropy?"

    Yes, that's exactly what I do. You don't get a 32x30 tilemap with fill mode, instead you fill an area with a single byte. It's something entierely different that falls outside of the regular shemes of mirroring (be it single-screen, horizontal, vertical, or 4-screen). You could even call this "signle-tile mirroring" if you'd like.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 08:14, 11 March 2017 (MST)

    

    

    

    

    

    Got it. Any time we instruct people to clear nametable memory, I will instead tell them to delete their nametables. And if there are any games with ROM nametables explicitly use a chunk of CHR-ROM that's all one byte as a nametable ... oh, wait, not a nametable, _sorry_ ... I'll be sure to pedantically point out that those aren't nametables because they're low entropy.
    Anyway, you've got three people who have voiced their disagreement with you, so don't make the edit.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:51, 11 March 2017 (MST)

    "Using Fill Mode for just 1 screen and 3 nametables for the other 3 just doesn't make any sense - you couldn't scroll a video game level using that configuration."

Yes, you could: [![Fill Mode Example.png](../wiki-images/Fill_Mode_Example.png)](File_Fill_Mode_Example_png.xhtml)

Metroid's level and art design fit the bill. [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 15:11, 6 March 2017 (MST) 

    Your example makes no sense. First the horizontal section is larger than 64 tiles, and the vertical section is higher than 60 tiles as well. So map scrolling has to be implemented to implement Metroid anyways. Also the fill mode nametable in your example is not displayed - so it could be anything and fill mode is not required. That's exactly what the real Metroid does - it switches between H and V mirroring depending on whether it's scrolling an horitzontal or vertical level. Neither 4-screen mirroring nor fill mode are required. And it's still a fact you can't scroll a 64x60 tile video game level using MMC5 fill mode - MMC5 does not support 4-screen mirroring.~~ 

    The idea was that you'd scroll more like Megaman X (having certain sections where the scroll is locked to one direction, but allowing diagonal scroll when transitioning between them). The doors could be removed; if the player stepped to the right of the intersection, the game would attempt to scroll up to align Y-wise with that nametable while scrolling X; and likewise if going below the intersection screen, it would scroll to align X-wise while scrolling to follow motion with Y. It would remove the wait that e.g. Super Metroid imposes while it lines the door up to the middle of the screen. The map data is even in screen/NT-sized chunks. 4-screen/MMC5 are not required, no, but it is an example of how it could be used, as you asserted it wasn't. And, as you can't see through walls, even if two corridors were adjacent in a four-screen manner (which I don't think they generally are in Metroid 1, but don't care to check) the fourth nametable contains room data that the player-character could not see, so a blank NT would be diegetic.

And if you're asserting about my image being more than 60 tles tall rather than the greater map from which it is sampled, it isn't; 60*8=480. [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 16:57, 7 March 2017 (MST) 

    

    Yes, this is indeed a nice example of usage of fill-mode. This isn't 4-screen mirroring though.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 00:12, 8 March 2017 (MST)

  
To be honest, I think the term "mirroring" is a confusing way to describe nametable memory layouts. Even on Nintendo's boards "H" means "vertical mirroring" and "V" means "horizontal mirroring"\-- clearly they though about it in the more natural/intuitive "horizontal arrangement" / "vertical arrangement" concept. This seems analogous to Pi vs Tau, where a less convenient version became popular and entrenched. Personally I would have preferred if "horizontal mirroring" meant vertical wrapping (even though the actual "mirrored" pages are arranged horizontally from each other). I don't know who made this terminology popular, probably Marat when he established the iNES format? 4-screen, of course, is a complete lack of "mirroring", in the hardware sense, but over time we seem to have ended up using the term "mirroring" for all arrangements of the nametable memory, and it doesn't _specifically_ mean mirrored memory in that context. 

Anyhow, I'm saying all of this merely to acknowledge that mirroring is confusing terminology. I _do_ think it's the standard terminology, though, and we should be consistent with the way it is used and what it normally means in this specific NES context. 4-screen is a "nametable mirroring mode", even if the etymology is stupid/broken. I don't want to argue what mirroring _should_ mean, what's important to me is what it _does_ mean. (I don't want to see this wiki text become a place full of internally-consistent jargon, where only the initiated know what words are supposed to mean.) 

What we're trying to describe in this article is different ways nametable memory can be arranged, which is unfortunately colloquially called "mirroring". The piece of the article in question is trying to be a inclusive list of all mappers that can do 4 screens in a special way, no matter how marginal that ability it is. MMC5 belongs with the others, in this respect. There's no deception or ambiguity about the description of this MMC5 possibility, the article fully qualifies that the fourth page is blank. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:44, 11 March 2017 (MST) 

I agree with pretty much all you've said there. "Mirroring" is an unfortunate term derived from iNES. The original iNES format allowed only horizontal, vertical and 4-screen mirroring, but any mapper would supplant those and come up with it's own system. MMC5 is different that all default iNES configuration, and should me mentionned as such (actually it is). What I specifically don't like about this page as it currently is (13 march) is that it's a major "don't repeat yourself" failure - MMC5 is mentionned twice, as well as other thigns. The page is desesperately over-complicated, going into details that should, IMO, belong to the respective mapper's page and not here. (most of the time the info is already here at the mapper's page). It seems you guys sometimes wants to fill the wiki with as much text as possible.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 08:40, 13 March 2017 (MDT) 

    I could possibly be persuaded that there is a reason to elide the entire 4-screen layout section and fold its content into the "other" section... very few games used the static unbanked 4-screen layout, even though it was always available on the Vs. System; especially in comparison to the number of games that used H/V/1. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 17:21, 13 March 2017 (MDT)

I don't know... The problem is that 4-screen mirroring is part of the iNES header, so this makes this layout "important" even if few games used it. And I just noted our whole argument were completely pointless... the list title says : "Mappers already used in combination with 4-screen mirroring:" (i.e. configurations that games actually used). I don't think any MMC5 commercial games ever used fill mode, let alone with 3 other nametables simultaneously, so... it shouldn't be on this list, no matter whether this "counts" as 4-screen mirroring or not. The mappers using ROM nametables also shouldn't be on this list, exept the Namco (since it's the only mapper where games actually seems to have used ROM nametables with 4 different nametables simultaneously). In fact, my personal opinion is that such a list is pointless... just mentionning that almost all mappers are capable of 4-screen mirroring theoretically but that this was extremely rarely needed would be sufficient for me.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:51, 15 March 2017 (MDT) 

    That's my point. Once we shave down the set of games to just the ones that use 4-screen layout in the very narrow way that you describe, there aren't really enough of them to warrant a whole subsection. It's just one more rarely-used way to handle the 4 KiB of address space for nametables. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:06, 15 March 2017 (MDT)

What if I just [separate the 2 mappers that are theoretical exceptions from the list of ones that we have game examples for](http://wiki.nesdev.org/w/index.php?title=Mirroring&diff=13459&oldid=13456)? Does that description look better? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:36, 15 March 2017 (MDT) 

It is indeed better, however I still think the page is largely improvable, but cannot see how exactly. It is kind of messy and does long ridiculously long explanations and lists of games for what is actually extremely simple concepts. As I said before, people just want to insert as many informations as possible, even if this is mid-relevant. (For example list of gamaes using any feature do normally not belong to a page explaining the feature, in my opinion).[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:52, 16 March 2017 (MDT) 

Definition of "Four screen" according to the glossary in this very wiki : 
    
    
    A mapping of cartridge memory to the PPU bus that expands the memory available for nametables to 4K, thus allowing four distinct nametables.
    

Notice how 4K memory is explicitly mentioned - exactly my point all along - MMC5 has 3K of nametable memory not 4K.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 02:43, 29 March 2017 (MDT) 

    Would you object to a revision of that glossary entry that didn't use that particular definition? Is [this edit](http://wiki.nesdev.org/w/index.php?title=Glossary&diff=13481&oldid=13478) acceptable? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 19:05, 29 March 2017 (MDT)

    I do not think it's acceptable. You're changing the definition to keep the mention of MMC5 twice on this page - but this page is the wrong part, not the glossary. Notice how your new definion of 4-screen mirroring is much more complex than the original one. Basically you want everything complicated instead of everything simple.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 02:24, 30 March 2017 (MDT)

    

    I made my best effort to accurately reflect what I think the term usually means. If you want to propose an alternative compromise, please make the edit. At this point I won't revert anything unless a consensus appears. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 00:55, 1 April 2017 (MDT)

## Castlevania 3

The article claims (since [March 2010](http://wiki.nesdev.org/w/index.php?title=Mirroring&oldid=1394)) that "Castlevania 3 uses the third nametable RAM available on the MMC5". Given that the Japanese version did **not** have 3 nametables at its disposal, I find this claim to be highly suspect - exactly where in the game is ExRAM used as a nametable? --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 07:34, 5 March 2017 (MST) 

    I just searched through the entire game data; the game seems to reliably copy ZP $25 to $5105 in the NMI, and it seems to always set _that_ using `LDA #im / STA $25`. The only immediate bytes I found were $44, $50, and $55. Seems likely that that's wrong... —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:06, 5 March 2017 (MST)
    It's used in the Aquarius level where the water rises - the difference between the VRC6 version and MMC5 version is major here. In the japanese version, the game switch to horizontal mirroring, enables left-8 column hide and has a glitchy scrolling for that part. The MMC5 version uses the 3rd nametable for the rising water and leaves the mirroring and scrolling engine to normal operation, resulting in clean scrolling.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 02:45, 6 March 2017 (MST)

    

    So in this area we have horizontal scrolling + a rising water overlay. (I made [an image](File_Castlevania_III_3_Screen_Mirroring_png.xhtml "File:Castlevania III 3-Screen Mirroring.png") to show it so the description on the wiki will be less mysterious.) The VRC6 version accepted errors at the edges and just used vertical arrangement instead? Since the water is an unmoving screen, I wonder if they could have just used the ROM nametable feature of the VRC6 for it? (As an aside, were there any uses of ROM nametables on VRC6?) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 19:23, 15 March 2017 (MDT)

    

    I haven't looked it up but if I were the coder I'd never use the ROM nametable for such a feature. The whole screen is basically several repeated water tiles, wasting 1k to store that repeated data seems unoptimal when you could write a simple loop that writes this to RAM. Actually, only screens with incompressible data on them would be worthy of ROM nametables.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:49, 16 March 2017 (MDT)

    

    

    Funnily enough, the last 1k CHR-ROM page is all 0s anyway, so they had apparently wasted 1k to store nothing instead. ;) This of course requires the hindsight to know they weren't going to need it though. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 17:34, 16 March 2017 (MDT)

    It's not "wasted" (per se) they use this page it for sections of vertical scrolling to disable BG display on some scanlines. (Of couse they could have done this through $2001 but for some reason they didn't - probably tradition as this couldn't be done on the MMC3 without screwing the scanline counter operation).[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 06:44, 17 March 2017 (MDT)

## Moving the bulk of scrolling update problems to scrolling page

I thought about the problem for a while, and my conclusion is that this page talks too much about scrolling (and mappers) rather than mirroring. I am wishing to move about a large part of this page, everything relating to scrolling update problems should go to the [scrolling](PPU_scrolling.xhtml "Scrolling") page. 

This page should restrict itself to explain the phenomenon of memory mirroring, and how it's abused to create wrapping-around tilemaps for scrolling. It should also mention mapper-controlled mirroring, single screen and four screen, but in a way more succinct than it does now - it is up to each mapper to document their mirroring control possibilities, not this page. A simple list to each mapper that does this is however welcome. The rest, in particular, which mirroring is recommended for which scrolling, should be moved to the scrolling page. 

I also think list of games that has features X or Y are inappropriate - those belongs to NESCartDB, or if they're here they should be on their own page. 

Thoughts ? [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:07, 13 April 2017 (MDT) 

    I consider 4-screen layout to be as esoteric as ROM nametables and will most likely disagree with any presentation that puts it on equal footing with 1/H/V layouts. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:43, 13 April 2017 (MDT)

PS : Perhaps if there was some sandbox so I could make a concrete new proposal for mirroring (and possibly scrolling) page, that'd be great. Is there such a possibility ? [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 02:10, 13 April 2017 (MDT) 

    Sandbox pages are all the pages under your personal namespace, such as the dozen I have under [User:Lidnariq](User_Lidnariq.xhtml "User:Lidnariq"). —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:43, 13 April 2017 (MDT)

  
I think the page is pretty good as-is, if mostly because of those big diagram images. Overall I think it has good hierarchical structure of its content and is easy to read, some points in response: 

  * The important stuff comes first (H/V/1) and each mirroring mode description has the most important stuff in their first few sentences.
  * I think the list of games is appropriate for 4-screen since there are only 6. It would be worse to say that this obscure mode exists and then list no games, why make someone look for them? It's a good illustration of exactly how (un)important/rare it is. I don't think we can possibly make lists for the 3 main modes, H/V/1, or which mappers support them, and there's no real point to that either (they are common and ubiquitous, hardly to do with mapper). It is good that we list a few common game examples for each, though.
  * The stuff about scrolling is a little chart way at the bottom (already de-prioritized by this), and I think it's good to have this much information just to illustrate how mirroring and scrolling typically interact. I thought it was bad when someone started adding lots of games to that chart so I cut it back and created [List of games by mirroring technique](List_of_games_by_mirroring_technique.xhtml "List of games by mirroring technique") for the completionists to maintain as they saw fit. I don't see why the chart would be better at [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling") or here, as the information is pertinent to both, so I'm indifferent on this point.



If you do want to create a sandbox proposal for a major change I'd gladly review it, but I actually think that most of the information on this page is good stuff worth keeping so I can't visualize what you'd have in mind otherwise. \- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:27, 13 April 2017 (MDT) 

In all cases those diagrams are good and will be kept. I really need to do this on my sandbox and you guys will see concretly what I have in mind. And lidnariq, I noted your point about 4-screen mirroring being less important than one screen I'll try to keep that in mind but the problem is that iNES header gave importance to 4-screen mirroring even though it's a gimmick. I'll try to take everything in account in my sandbox.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:46, 14 April 2017 (MDT) 

The proposal is now more or less ready. The idea is that the current 2 pages of [mirroring](Mirroring.xhtml "Mirroring") and [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling") are split along 3 pages instead dedicated to [mirroring alone](User_Bregalad_Mirroring_sandbox.xhtml "User:Bregalad/Mirroring sandbox"), [level scrolling techniques alone](User_Bregalad_Scrolling_sandbox.xhtml "User:Bregalad/Scrolling sandbox") and [loopy split scrolling alone](User_Bregalad_Split_Scrolling.xhtml "User:Bregalad/Split Scrolling"). I really think it would be an huge improvement over the current scheme, mirroring page talking about level scrolling in 60% of its content, and the scrolling page talking about regular scrolling only in a small paragraph and developing advanced loopy scroll split in most of it. Constructive critisism is welcome. But keep in mind that, because of my lack of familiarity with wiki syntax, they might not look as good as I originally intended. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 15:29, 16 April 2017 (MDT) 

    Okay, some thoughts: 

  * I like that "Memory Mirroring" is moved to the top of the Mirroring page since it's short and relevant.
  * I don't like the new split between "mapper controlled mirroring" and not; I think it was better the way it was. 1 screen should come before 4-screen, because it's more important. It's worth noting that some mappers can change their mirroring at will, but "mapper controlled" isn't a category of mirroring in itself, IMO.
  * The list of 4-screen games (and some other 4-screen info) has been removed, I don't like this, but we've had this conversation already.
  * It's hard to tell from these sandbox pages what's merely been moved and what's been rewritten, this will take longer to evaluate. I'm hoping that you haven't removed information that I (or others) thought was valuable in this process.
  * I do not like the name "split scrolling" and "scrolling" for these pages: 
    * "split scrolling" should retain the original name "PPU scrolling", and it is now strictly about the hardware scroll registers and how they operate?
    * "scrolling" should be named "scrolling techniques" perhaps? It is about a synthesis and application of both "mirroring" and "PPU scrolling", the other two articles are now a prerequisite for this.
    * The thing that bothers me a bit about this new division is that the scrolling techniques make a good demonstration of what the PPU scrolling hardware stuff means; especially tepples' scroll seam GIF and the explanation it had there, was a nice soft introduction to scrolling techniques before diving into the hardware register information. THe Mirroring article, on the other hand, stands well on its own, not really requiring any knowledge of the PPU register stuff to understand.


    So, like I said before, I was a bit indifferent as to whether that scrolling techniques chart at the bottom of [Mirroring](Mirroring.xhtml "Mirroring") should belong there or [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling"), because it really depends on both pieces of knowledge at once. I'm a little worried that moving it to yet another third page might make what's left at PPU scrolling harder to understand, but I'd appreciate commentary from others on this. It can probably be made to work in this 3-page way instead of 2-page but as I said before I didn't really think the current organization was a problem, and people seem to be taking well to the [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling") article since the last major reorganization (merging from the previous "skinny" article). I can see that you're trying to expand on the "scrolling techniques" stuff, though, so I do somewhat understand why you want to make a new page of it.
    \- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 03:27, 17 April 2017 (MDT)

Thank you for your feedback Rainwarrior. I tried to make it more clear what I moved arround and what I affected in the introduction of each article. As a sumarry : 

  * The [User:Bregalad/Mirroring sandbox](User_Bregalad_Mirroring_sandbox.xhtml "User:Bregalad/Mirroring sandbox") page is made of data from the current [mirroring](Mirroring.xhtml "Mirroring") page, pretty much entierely affected by myself.
  * The [User:Bregalad/Split Scrolling](User_Bregalad_Split_Scrolling.xhtml "User:Bregalad/Split Scrolling") page is unchanged, made from part of the current [scrolling](PPU_scrolling.xhtml "Scrolling") page.
  * The [User:Bregalad/Scrolling sandbox](User_Bregalad_Scrolling_sandbox.xhtml "User:Bregalad/Scrolling sandbox") page is mande of both unchanged and changed data from both [mirroring](Mirroring.xhtml "Mirroring") and [scrolling](PPU_scrolling.xhtml "Scrolling") current pages.



I agree with a large part of your critisism. The separation between hardwired mirroring and mapper controlled is a bit unfortunate, however we have to agree that having, e.g. vertical mirroring because PPU A10 is physically wired to CIRAM A10 is not the same as having a mapper controlling the CIRAM A10 line and having it being programmed to pass data from PPU A10 there through a multiplexer. The difference is minor however significant. 

I agree that 4-screen mirroring is not very important, however you have to admit it is hardwired. Because of the importance the iNES header gives to this mode, it can unfortunately not be ignored. The huge images makes this section large, even though I'd like them to be small. It would make this part take a very narrow place between both hardwirede section and mapper controlled section. 

And yes I consider the loopy scrolling page to be extremely hard to undersant (I myself don't understand it) some kind of "nuisance" that have nothing to do with level scrolling, which is intended to emulator authors and not to game authors, which is why I'm trying to move this to it's own page. I do not care about affecting the page's name, actually I agree my own names are poor. I'd not be against leaving "PPU scrolling" for the loopy split scrolling part, and having something like "level scrolling" or "scrolling techniques" for the other scrolling page. 

I hope I'm going to get some other feedback. 

PS: The reason why the list of 4-screen mirroring games has been removed is because it's in my opinion too long. Originally it was just 2 or 3 official games, and not it has grown to half-a-dozen of games, mostly pirate and homebrew. This doesn't belong to here, but if such a list is really important (I personally doubt it is) then it should be on its own page. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 09:05, 17 April 2017 (MDT) 

    So, a point of pedantry first: 

  * Major League (m32/G-101) has hard-wired one-screen mirroring. Magic Floor does too, but there's plenty of room to argue whether it counts.



    I see absolutely no utility in distinguishing between "mapper controlled" and "not". Does it matter when you're writing an emulator? I guess, there's lots of ways to abstract mirroring incorrectly, and it's remotely conceivable that this distinction might possibly help someone pick a right abstraction in the first place. Does it matter to a programmer writing a game? Only in that they have to decide if it's worth the cost.
    As a result of adding this distinction between "mapper controlled" and "not", you've put 4-screen layout in a prominent position on the page. As I said before, I'm basically going to disagree with any major edit that fails to lump it in with the "everything else".
    And as to whether “the importance the iNES header gives to this mode”: that's a rationalization, not a justification. There's plenty of other crap in the iNES header that isn't worth special prominence, such as the trainer bit.

    Only tangentially related, the issue about finding it difficult to understand the modernized slightly-friendlier version of Loopy's scrolling doc is interesting, and I would like to fix. But I don't know how to do that, given that doing so basically involves finding someone who _doesn't_ understand it, and have them take notes as they work through it. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:36, 17 April 2017 (MDT)

Ok, thanks for your feedback Lidnariq. Sounds like I'll have to abandon that distinction between hardwired and non-hardwired, maybe that's for the better. So if you guys dislike 4-screen mirroring being mentionned that much, all I can do is to barely mention this possibility at the end of the page and remove the diagrams, so that it looks like a very marginal and weird possibility. 

However, we have to be very clear about something : Hard-wired single-screen mirroring is completely different from normal, mapper-controlled, single-screen mirroring. The former sacrifices half of CIRAM, while the second allows two separate nametables to be used, but one at a time. That's two completely different setups. Currently, the wiki only doccuments the second, and my edit barely mention the 1st possibility. That's because it's extremely weird and not really used by any commercial games, however I didn't know about that _Major League_ game. I'll have to investigate it. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 13:01, 17 April 2017 (MDT) 

    Well, to use the 3 category distinctions under proposal, those two single screen types are not different for MIRRORING, but they are very different for SCROLLING TECHNIQUES, and not really PPU SCROLLING. If it helps, maybe at MIRRORING we could just use one mirroring image for the description of single screen (instead of the pair image tepples designed), and just note in its description that this is commonly done was two flippable screens. At SCROLLING TECHNIQUES you can go into detail how they affect game/hardware design or whatever is relevant. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:42, 17 April 2017 (MDT)

    Though, just as a footnote, I still think the description is fine as-is. The 2-screen single-screen setup is the most common and useful case, and it only seems to be slightly extraneous information when introducing these 3 categories. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:45, 17 April 2017 (MDT)

    As for the images, I think they are a very good size, and they really make a good impact as-is (when I revised them, I kept them at the same size as tepples'). On my screen, they aren't any longer vertically than the describing paragraphs either (except for the single screen double image). I don't really want to see 4-screen "demoted" to hanging out with the L-mirroring etc. obscurities, it is good enough to be one of the "big four" I think (Vs. system, iNES header prominence, a well known game like Gauntlet, etc.), just it definitely should not bump 1-screen. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:21, 17 April 2017 (MDT)

    

    Once upon a time I would have agreed that the Vs. System should have added to four-screen's prominence... But when orbit2002 started doing his array of Vs System-to-plain NES patches and I stuck my nose under the hood a few times, I discovered that far too many of Vs. System games barely use the extra memory at all. Most seem to either ignore the extra nametables altogether (never setting the scroll register to display from them) or else treat it as an H/V switchable situation without needing to explicitly ever switch the layout. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:49, 17 April 2017 (MDT)

    I've made an alternative "overlaid" version of the single screen diagram: [File:Single screen mirroring diagram overlaid.png](File_Single_screen_mirroring_diagram_overlaid_png.xhtml "File:Single screen mirroring diagram overlaid.png"). This makes its size footprint not much different from the others, at the expense of hiding the letter B, but I think the colour and its relationship to the other diagrams disambiguates that. (I also checked it against some colourblindness simulation tests, and the two colours do still seem recognizable through those tests.) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 16:01, 17 April 2017 (MDT)

  
Because this seemed uncontroversial, I made a [simple edit](http://wiki.nesdev.org/w/index.php?title=Mirroring&curid=59&diff=13591&oldid=13590) shifting "memory mirroring" to the top, and some very brief lead text about the two categories. I think bregalad had some small extra revision in mind for the memory mirroring, but now it can be done as continuous edit history. At this point I think it would probably be fine to move the mirroring chart to a new [Scrolling techniques](https://www.nesdev.org/w/index.php?title=Scrolling_techniques&action=edit&redlink=1 "Scrolling techniques \(page does not exist\)") article? (I don't really object to it being at [Mirroring](Mirroring.xhtml "Mirroring") or [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling") either, the only thing I really object to is its removal, but I think if bregalad wants to expand such a new page it would be fine to put there.) Looking at [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling") though, I really don't see anything that I would want to have removed from that article-- I think it is one of the best pages on the wiki, and I really do think it has been successful the last many months in guiding a lot of people that asked questions on the forums. Basically, I think "the common case" should not be moved to [Scrolling techniques](https://www.nesdev.org/w/index.php?title=Scrolling_techniques&action=edit&redlink=1 "Scrolling techniques \(page does not exist\)"), it should stay there, but if desired you could re-use some of that material on the scrolling techniques page. I know you hate redundancy, but I think the illustration really helps understanding of the article. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:14, 17 April 2017 (MDT) 

  
Overlaid graphic for single-screen mirroring is an amazing idea. It really gets to the point straight. As for the rest, I'll have to think about it for more time. I do not think the order in which mirroring techniques are exposed is so important, so I don't see why there is so much insistance on having 4-screen mirroring below 1-screen mirroring. The only reason I changed this in my proposal has nothing to do with the "importance" of each option, but instead because 4-screen is a type of hardwired mirroring while 1-screen typically (though not always !) is a form of mapper-controlled mirroring. I think this distinction is somwhat important, at least for those. Especially considering the 4-screen bit in iNES overwrites the mapper. Yes, I know iNES header is extremely old, and has lots of weird issues etc, etc... nevertheless this format IS the most widely used format for storing NES images, and as such is very important even though nobody likes it that much. What I mean is that I have nothing against restoring the original order in my proposal, however I think it would only bring more confusion between hardwired and mapper controlled mirroring. If I can somehow find a way to clear up this confusion then I have no objections to keeping that order. 

The images are about as tall as the text in the current mirroring pages, but are taller than the text in my proposal, because the text talking about level scrolling would be moved to its own page. That's why I wanted to make them smaller. I'd also love to have more sketches like the one displaying SMB worlds 1-2, for the level scrolling page it'd be great. I unfortunately have zero skill as to how to actually make it, let alone insert it in the wiki. So I'm reduced to poor little ASCII text diagrams. 

I do not "hate" redundancy, however I think it is usually an indication of poor planning as how information is given. I do however think it'd be a bad idea to have the level scrolling diagram on two different pages, to explain the same concept. 

An idea that would pop in my mind would be to still have an introduction to the PPU split scrolling article showing "normal" (non-split) scrolling, but without a game level, just the concept of scrolling. And of course link to the game level "scrolling techniques" page. 

Last but not least, I'd just like to point out that my mirroring proposal did not only change the order of the information, but a also how it is worded. I hope you guys noticed the huge improvements. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:19, 18 April 2017 (MDT) 

  


    Okay, I have had some time now to review the three sandboxes and compare them to the existing material. Here are my notes on them (hasty and unfiltered, sorry if they are brusque):
    [User:Bregalad/Mirroring sandbox](User_Bregalad_Mirroring_sandbox.xhtml "User:Bregalad/Mirroring sandbox")

  * I do not stylistically like the "disclaimer" sections; I think we can incorporate these into the text more naturally.
  * Memory Mirroring - the minor changes are good, there's a * in the second last paragraph that I do not understand the meaning of.
  * Nametable Mirroring - The old lead for this section mostly talked about wrapping and screen arrangement, which you've removed. I think it is helpful to have. Your new lead here dives right into PPU addresses, etc. I think this new information is useful too but we should keep both.
  * Nametable - Should retain convention of capital letters for hexadecimal numbers.
  * Nametable - Do not like the distinction between hardwired and mapper controlled, as mentioned before. (This is a good distinction to make on the other page, though!)
  * Horizontal - I think it's fine to remove the two paragraphs about the scroll seams etc. because that's all to be covered by [Scrolling techniques](https://www.nesdev.org/w/index.php?title=Scrolling_techniques&action=edit&redlink=1 "Scrolling techniques \(page does not exist\)"). I think it would be worth retaining a sentence that this naively applies to vertically scrolling games.
  * Horizontal, etc. - Suggest moving the paragraph about CIRAM A10 to below the "arrangement" paragraph. The arrangment is the important and easier to grasp detail and should come first, the A10 note is hardware reference. Probably not necessary to say "non-contiguous" about every tilemap, that's more of a [PPU Nametables](https://www.nesdev.org/w/index.php?title=PPU_Nametables&action=edit&redlink=1 "PPU Nametables \(page does not exist\)") concern?
  * Vertical - Same notes as for Horizontal, fine to remove the scroll seam stuff.
  * Single Screen - Again, describe the layout first, move the talk of CIRAM down a little. Put before 4-Screen. I think it's worth retaining mention of at least AxROM and SxROM since they are both very common. Fine to remove the stuff that is relevant to scrolling techniques.
  * 4-Screen - I don't like the removal of the short list of games or the list of mappers.
  * Other - The "more complex layouts" idea seems unnecessary to me, since nothing has ever done this, and no existing mappers can do it, what value is there in mentioning it?
  * Mirroring chart - Fine to relocate to [Scrolling techniques](https://www.nesdev.org/w/index.php?title=Scrolling_techniques&action=edit&redlink=1 "Scrolling techniques \(page does not exist\)")


    [User:Bregalad/Split Scrolling](User_Bregalad_Split_Scrolling.xhtml "User:Bregalad/Split Scrolling")

  * I suggest we keep [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling") as it is, and named that way. I do not like the name "split scrolling".
  * Keep "the common case", as mentioned above.
  * Other than the removal of "the common case" the only changes I see here is renaming "PPU registers" section to "PPU internal registers" (good change), and some very minor editing of that section's lead (also fine).


    [User:Bregalad/Scrolling sandbox](User_Bregalad_Scrolling_sandbox.xhtml "User:Bregalad/Scrolling sandbox")

  * Suggest naming this page [Scrolling techniques](https://www.nesdev.org/w/index.php?title=Scrolling_techniques&action=edit&redlink=1 "Scrolling techniques \(page does not exist\)")
  * Unidirectional scrolling - don't use this material from [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling"). This page should be more "high level", I think. The GIF might be reusable here, but otherwise just talk about how horizontal mirroring is suitable for vertical scrolling, and vice versa and leave the details about writing to $2005 etc. over at [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling").
  * Multi-directional scrolling - this is a good addition to the wiki
  * Mirroing chart - Perfectly fine to relocate here


    Overall I think the creation of a [third page](https://www.nesdev.org/w/index.php?title=Scrolling_techniques&action=edit&redlink=1 "Scrolling techniques \(page does not exist\)") to discuss how mirroring choices interact with a game's overall scrolling implementation is a good idea. It's like an expansion of that mirroring chart into something more comprehensive. I can see the purpose of the revisions you've made in service of that. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 23:50, 22 April 2017 (MDT)
    One more note: I think the fixed single-screen thing should be placed under "other" due to its obscurity. I actually added [this image](File_Single_screen_2000_mirroring_diagram_png.xhtml "File:Single screen 2000 mirroring diagram.png") to the "other" gallery with this in mind. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 23:56, 22 April 2017 (MDT)

Thank you rainwarrior for your suggestions. I agree with most of them so I updated (or I will update) according to this. I disagree however with the following 2 things: It is important to mention "more complex layouts are possible", because this wiki is here to doccument what the NES console can do, not only what released commercial NES games does. The second think I disagree is list of games using 4-screen mirroring. The intent of the page is not for list of games using this or that - if the list was only to include the 3 licenced games that might be barely ok, but any list longer than that is out of the question - and the list in the current page is too long and out of topic. If such a list is interesting, it should be somewhere else. I do not think ROM nametables counts as 4-screen mirroring, but as "other", as well. List of mappers is completely pointless since 4-screen mirror is orthogonal to mapper and basically bypass mapper controlled mirroring. Contrary to what I said earlier, actually the MMC5 could support 4-screen mirroring if a chip were to be added, I was completely wrong in beliving it's the only mapper who can't do that. But this is getting off-topic here, sorry. 

I think it's important to mention the non-contigousity of the nametables, because it makes them extra annoying to adress. For example, if I use vertical mirroring, and want to move one column to the right, I have to add 1 to adress most of the time, but if I'm at the end of the line I have to add $3e1 instead. This is quite annoying, and as such, quite different from a "real" 64x30 tilemap. 

I agree about the renaming of my pages "scrolling" as "scrolling techniques", and "split-scrolling" as "PPU scrolling". However I unfortunately don't think I can rename them. If by any chance you happen to know how to rename them, please tell me how I can do that. I only think I can create new pages and link to them, and remove content from old pages. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 04:37, 23 April 2017 (MDT) 

    Why is 3 acceptable but 6 out of the question? It's a complete list, not a representative selection (like any attempt would have to be for the other 3 categories). What does "licensed" mean? Of the 6, only 2 are licensed by Nintendo. 3 are Famicom-only, and 1 is a Sachen game. I don't know/care about the Sachen game but I don't understand why 3 is "OK" but 6 is too much?
    As far as renaming, you need heightened privelages to "move" articles on a wiki, but there's not really any need to rename sandbox files. I would actually just recommend starting an article at [Scrolling techniques](https://www.nesdev.org/w/index.php?title=Scrolling_techniques&action=edit&redlink=1 "Scrolling techniques \(page does not exist\)"). I don't think that one needs a sandbox anymore. (FYI: the canonical "non-admin wiki move" is to copy to a new article linking the original article in the edit summary, and then replace the old article with a redirect.) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 17:39, 23 April 2017 (MDT) 

    

    I don't think I have special permissions to move an article? It's under "More" for me, and it preserves the edit history across the move. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 18:53, 23 April 2017 (MDT) 

    Ah, looking up our wiki policy it says "autoconfirmed" members get "move". You can find it in a tab at the top of the page if you have it (between history and watch), and _should_ use the move feature instead of the other way if you've got it. (At Wikipedia it was considered acceptable to do the lo-fi version if you didn't have the privilege yet, but it was also harder to get the privilege.) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 22:06, 23 April 2017 (MDT) 

    Nowadays, [the list of user group rights on Wikipedia](https://en.wikipedia.org/wiki/Special:ListGroupRights "wikipedia:Special:ListGroupRights") also lists "Move pages `(move)`" as available to autoconfirmed and confirmed users. We must have been on Wikipedia for a _long_ time to remember a time when cut-and-paste moves weren't in MediaWiki. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 09:28, 24 April 2017 (MDT) 

    Who said anything about mediawiki not having a move? All I remember is that ~10 years ago it took me several months of active editing to earn the move privilege on Wikipedia, and "manual" moves were considered acceptable, presumably because of that. Anyway, do you have any opinions of bregalad's proposal? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 21:34, 24 April 2017 (MDT)

    

    As an aside, though, what I object to is the removal of information about four-screen mapping that I think is valuable. As a compromise, if it's intolerable to have this information in [Mirroring](Mirroring.xhtml "Mirroring") for whatever reason, I would not oppose moving it to a dedicated [Four-screen mapping](https://www.nesdev.org/w/index.php?title=Four-screen_mapping&action=edit&redlink=1 "Four-screen mapping \(page does not exist\)") article. As I've already made clear, I don't think there's enough information there to warrant that, but it wouldn't really offend the issue I actually care about to have it moved to its own small article, similar to how I insisted that the mirroring examples chart remain small, and created [List of games by mirroring technique](List_of_games_by_mirroring_technique.xhtml "List of games by mirroring technique") for those that thought a completionist's list was important. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:36, 23 April 2017 (MDT)

@Rainwarrior: What I oppose to is to claim to have a "complete list of games using 4-screen mirroring", because there's no such a list. I can release 10 games using 4-screen mirroring tomorrow if I want, making this list obsolete. Same with mappers supporting 4-screen mirroring, since all of them support it in theory, by definition. In addition, this data just gets in the way of the mirroring page. Compromises I'd be ready to made would be: 1) Link to Bootgod's database with a search function searching for games having 4-screen mirroring, I don't think it's very valuable but at least it makes sense. Or 2) I currently have a list of 3 ways to implement 4-screen mirroring in hardware, I agree to give games examples for all 3 which would happen to be Gauntlet, Rad Racer II and Napoléon Senki (in that order), since all 3 of them uses different techniques. I do not want any pirated chinese games mentionned, since I do not consider that info to be any valuable. ROM nametables is not "4-screen VRAM", but something else entierely, and that is already mentioned below. There is game examples in the "other" section, but it doesn't pretent do be an exhaustive list, which is the way it's supposed to be. 

About another of our disagreements: I do not think it's valuable to mention horizontal mirroring apply "natively" to vertical scrolling and vice-versa, because that would deny the very existance of Mega Man 3,4,5,6. Those does the exact opposite, they scroll horizontally with horizontal mirroring and vertically with vertical mirroring. In fact it's possible to scroll to any direction using any mirroring, so that's why I want to mention those techniques exclusively in "scrolling techniques" and not in "mirroring". 

Finally, I have nothing against leaving the mario animation in "PPU Scrolling" but I think it'd be most valuable in "scrolling techniques", since this animation shows clearly how level tiles are updated to VRAM. I'd like to do other animations such as this one but, once again, I lack the skills. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 00:49, 25 April 2017 (MDT) 

    To clarify my meaning, I used the word "naively" rather than "natively", meaning that in a simple/unsophisticated way they are suited for those purposes, which I believe is true and helpful information. The word "native" has other implications that were not what I intended. It is _also_ worth mentioning that scrolling can be done in any direction with any mirroring too! Tomorrow, when you release 10 new 4-screen games we can talk about the size of that list, but today there is 6, and I don't want to argue about hypothetical future games. I feel that ROM nametables are 4-screen, as did lidnariq above, but this has already been clearly stated; of course it would be nice if more people would comment on this for greater consensus. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 02:48, 25 April 2017 (MDT)

    As for GIF examples, it's okay if you can't provide any, if one is needed you can make a request (here or on the forums)-- there are lots of people in this community that can. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 02:51, 25 April 2017 (MDT)

## Minor Grammar Fix

"There is also a lot" should be "There are also a lot" [SpiderDave](https://www.nesdev.org/w/index.php?title=User:SpiderDave&action=edit&redlink=1 "User:SpiderDave \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:SpiderDave&action=edit&redlink=1 "User talk:SpiderDave \(page does not exist\)")) 22:24, 31 May 2021 (MDT) 
