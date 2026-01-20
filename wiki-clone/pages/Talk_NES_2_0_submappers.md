# Talk:NES 2.0 submappers

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANES_2.0_submappers) | View [other pages](Special_AllPages.xhtml#Talk_NES_2_0_submappers)

## Contents

  * 1 Zzo38 likes this
  * 2 iNES Mapper 078
  * 3 MMC5
  * 4 MMC3
  * 5 "Don't care" bits
  * 6 iNES Mapper 185
  * 7 Does expansion volume warrant a separate field in the header?
  * 8 Major revision proposal
  * 9 UxROM with WRAM support for INES 2.0



## Zzo38 likes this

I like this. I have written some ideas relating to this [(as a user subpage)](User_Zzo38_Submappers.xhtml "User:Zzo38/Submappers") but your article is much better, but incomplete so we can improve it. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 14:39, 23 September 2012 (MDT) 

## iNES Mapper 078

My point, although poorly represented, is that no existing game needs mapper 78 to support hard-wired mirroring. It is not necessary for original games, and it is not necessary for homebrew: [User:Lidnariq/Discrete Logic Table](User_Lidnariq_Discrete_Logic_Table.xhtml "User:Lidnariq/Discrete Logic Table") shows that mappers 70 and 72 are equally capable. Thus there is no reason to allocate a submapper. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:14, 6 April 2013 (MDT) 

## MMC5

Should there perhaps be MMC5 submapper for CL and SL mode? (See also my comment on the MMC5 pinout) If there are significant differences between MMC5 and MMC5B (it says they are unknown; I am guessing there aren't any significant differences), should there also be submapper codes for this? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 14:11, 28 April 2013 (MDT) 

    No games were known released in CL mode. For the most part, I'm trying to not allocate a submapper until I can both describe the differences and describe a need for it. ... which makes me have second thoughts about the CNROM allocation; if the game cares about those diodes it'd be m185. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:27, 28 April 2013 (MDT) 

    Well, the mapper does _support_ CL, even if no existing games use it; it means that a homebrew game can use CL if it ever seems useful for whatever arcane reason (even CL games would probably work fine with SL though, but maybe CL would have other effects; I don't know if it would scroll only the colors or what). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 18:34, 28 April 2013 (MDT)

I would like to repeat my objection: You have now allocated 3 of the 4 bits in the submapper field, without ANYTHING ACTUALLY EVER USING ANY OF THEM. Please stop.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 20:52, 21 January 2014 (MST) 

    OK, but it says "Proposal"; they aren't actually allocated (although maybe proposals shouldn't belong there anyways). Anyways, the ExROM configuration does use them. Also, Disch's notes do mention that Uncharted Waters does use non-contiguous PRG RAM (although it will work fine with 64K contiguous RAM). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 21:18, 21 January 2014 (MST)

* * *

Ok, meaning of the bits here: I'm of the opinion that to the extent possible, all 0s should describe hardware as it existed. No MMC5 game ever provided PRG RAM without battery-backing precisely one RAM, and never battery-backed two RAMs. So the only instance in which the 2s bit of this submapper field would be set are if someone took an ETROM board and modified the jumpers. 

Also, encoding exceptions is a PitA; why should the bit have opposite meaning when the field specifies 16KiB PRGRAM than when it specifies any other number? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:54, 12 April 2014 (MDT) 

    I agree it is very klugy, but the reason I suggested it is so that if someone implements it, that it won't cause it to run incorrectly a file which doesn't use it. However, that was because I didn't know, no MMC5 game ever provided PRG RAM without battery-backing precisely one RAM. Since that is the case, I was clearly wrong, so that kluge can be removed. (I assume you mean, 16K is in two 8K RAMs, one of which is battery and one isn't. Therefore, I made a mistake!) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 18:52, 12 April 2014 (MDT)

## MMC3

Ok, NesCartDB knows of no games that use MMC3 and hardwired 2 screen mirroring, despite its jumper-ability on TEROM and TFROM. Was this supposed need an artifact of splitting off [iNES Mapper 206](INES_Mapper_206.xhtml "INES Mapper 206")? Also, am I correct in thinking that we believe that all games work correctly with MMC3C style interrupts? 

Given this ... what behavior does mapper 4 need submappers for? Should I roll my proposal over kevtris's, replace it with a subset that only handles MMC3A vs MMC3C, or just replace the entire section with an explanation about how why no submappers are necessary? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 22:13, 30 April 2013 (MDT) 

    Add the things that the jumpers on the mapper support, even if you know of no games that use it. Anyways, even if all of the games you know about work with MMC3C interrupts, some might not, and there should be a way to indicate this difference since it still can affect things. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 22:29, 30 April 2013 (MDT)

  
Thefox, why is mapping "1KB of RAM → MMC6" not adequate, rather than needing a submapper? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:48, 11 August 2014 (MDT) 

## "Don't care" bits

I see that some submapper numbers say to use 0 for don't care. I might suggest that the `Mapper/Number` and `Mapper/NumberEx` fields of [.nes.ini](User_Zzo38_Metadata_INI.xhtml "User:Zzo38/Metadata INI") to be used to indicate this more explicitly; using the `Number` you can specify multiple submapper numbers, with vertical bars, so if all combinations are specified then it means "don't care". --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 22:43, 30 April 2013 (MDT) 

## [iNES Mapper 185](CNROM.xhtml "INES Mapper 185")

Nestopia's internal database instead implements mapper 185 as two trinary numbers: D0 and D1 must be low/high/don't care. 

I wonder if that's better? 

For that matter, I've never heard of any 16kiB CHR m185 games, nor any games that have only one don't care line. Maybe simply allocating four submappers corresponding to the LSBs is better.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 20:09, 16 January 2014 (MST) 

## Does expansion volume warrant a separate field in the header?

You say: "_Furthermore, different 163-using PCBs also used a different resistor to change the relative volume of the expansion audio relative to the internal 2A03 audio. It is unclear if this variation warrants a submapper._ " I may suggest that you could make another header field (perhaps high six bits of byte 12?) for expansion volume. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 15:04, 18 January 2014 (MST) 

    Further modifying NES2.0 is uncalled for. The question is whether the information belongs _at all_ , not whether it belongs in the submapper field. (If it does belong, this is exactly what the submapper field is for) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:11, 18 January 2014 (MST) 

    If it doesn't belong in the NES 2.0 header at all, it can go in the .NES.INI instead, I suppose. However, depending on exactly what groups of cartridges use these different resistors, it might or might not be useful as a submapper number. Didn't I read somewhere that the actual use of different resistors in different cartridges is more confusing than specified here, or something like that? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 15:36, 18 January 2014 (MST) 

    The header needs to describe the behavior of the cartridge PCB. Something that is inside the cartridge and cross-cuts across multiple mappers belongs in a dedicated field. Examples of cross-cutting concerns include memory sizes (bytes 4-5, 8-11), mirroring (byte 6 bits 0 and 3), NTSC/PAL/Vs./PC10 (byte 7 bits 0-1 and bytes 12-13), etc. The next step is to determine whether mix volume is a cross-cutting concern or whether it's exclusive to the N163. Do all MMC5 games using expansion audio have the same volume relative to the 2A03 channels? Do all Sunsoft 5B games? Do all VRC6 games? (Among synths listed in the [NSF](NSF.xhtml "NSF") spec, FDS and VRC7 have one cartridge each, so we need not concern ourselves with those.) --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 18:23, 18 January 2014 (MST) 

    Correct; this is my reason to think if a expansion audio needs a separate field or not. If there is a separate header field, then it should be defined for VRC7 too (for consistency, and that someone might want to write a new program using different mixer volume) (and that emulators expecting a NES 2.0 header on the BIOS, will allow it to work with FDS too). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 19:08, 18 January 2014 (MST)

    

    

    

    All three VRC6 games use the same module with the 6-bit R-2R DAC and mixing resistor. The Sunsoft 5B is a singleton (Gimmick!). This leaves only MMC5 and the 163, but no-one's ever put an oscilloscope on the four MMC5 audio lines to find out exactly what the four pins do, or exactly what variation in resistors effect.
    So, first off: what [ExROM](ExROM.xhtml "ExROM") says about the resistors is wrong; R1,R2,R4, and C4 are clearly all part of the audio path on the HVC-EWROM, HVC-ETROM, and HVC-EKROM boards. HVC-ELROM has a substantially different layout, but since it has the same values of resistors and capacitors, it seems likely that it also supports expansion audio. 

  * The vast majority of games use the resistor and capacitor values that are silkscreened on the PCB: 15kΩ/15kΩ/10kΩ/0.1µF. This includes Sangokushi 2, Suikoden: Tenmei no Chikai, Nobunaga no Yabou: Sengoku Gunyuuden, L'Empereur, Ishin no Arashi, Daikoukai Jidai, Uchuu Keibitai SDF, and Gunsight.
  * A few games replace ~~R2~~ R1 with a 6.8kΩ resistor instead of 15kΩ, including Nobunaga no Yabou: Bushou Fuuunroku, Shin 4 Nin Uchi Mahjong: Yakuman Tengoku, Royal Blood, and Just Breed.
  * In a current NSF archive, only three games seem to use MMC5's audio: Just Breed, Metal Slader Glory, and Shin 4 Nin Uchi Mahjong. The two of those three that are in NESCartDB both use a 6.8k resistor.
  * We don't know what difference in volume is caused by changing ~~R2~~ R1.


    So, my conclusion: there _could_ be two different mixing volumes for MMC5, but most likely only one was ever used. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 22:39, 18 January 2014 (MST) 

    Perhaps someone should figure out what exactly ~~R2~~ R1 affects (for this purpose, there is probably no need to figure out the other resistors). Even if only one is used, the other way exists and should be specified, in my opinion which is to include all possibilities even if some aren't used. If ~~R2~~ R1 is figured out and does affect mixing volumes in the way which will become known, then this should be specified. If you are using submapper numbers to indicate the difference, you can assign bit2 of the MMC5 submapper number; set for 15k and clear for 6.8k. (This isn't necessary if nothing applicable to the audio is affected by R2, though.) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 01:18, 19 January 2014 (MST) 

    And if it does turn out that two mixing volumes were used on MMC5, we have a cross-cutting concern. I was thinking of repurposing some bits of byte 12 as a signed difference in decibels from each mapper's "normal" output level. Because it's signed, it still follows the overall principle of zero doing something expected. But I'd recommend testing how that replacing that resistor with a pot affects MMC5 channel balance and asking kevtris what he thinks before making a final decision, just as I asked him for clarifications on smaller-than-NROM-128 ROM sizes such as those used by Galaxian and Slappin'. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 08:16, 19 January 2014 (MST) 

    Yes, same as my thoughts; using the high bits of byte 12. I didn't think of making it signed, but you are correct that is a good idea. I don't know if "negative infinity" and "positive infinity" are useful at all; at least the former seems to be wanted by some people (to save CPU time when not emulating expansion audio) and is used on some cartridges having the same mapper number as ones with expansion audio, but the latter isn't used or supported in any cartridge board as far as I know, and probably is useless at the present time at least (although there are two uses I can think of: to silence the DPCM channel (if it is only being used for timing or for side-effects of reads, and to use the 2A03 audio to control a device connected to the Famicom expansion port (for example if only one player is allowed to hear it)). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 10:49, 19 January 2014 (MST)

Well, we've characterized what the variation in MMC5 _could_ be. I still object to marking such in the header until such time as we find a commercial game that uses this variation. It is **incorrect** to store every possible variation in the header because there are an exponential number of ways to configure an IC and in the case of analog components, an infinite number of ways. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:49, 21 January 2014 (MST) 

    According to the forum there are no square waves on R2, or anything at all in the Just Breed game at least. This discussion says it is R2 which is changed. ExROM says it is R1 which is changed, and increases the volume of the pulse waves by 6.9dB. (Which is it; R1 or R2?) If this is the case, then that is what bit2 of the submapper number for MMC5 should be assigned to: cleared if 6.8k, and set if 15k. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 16:10, 21 January 2014 (MST) 

    Bleh, I brainoed during this entire discussion. Fixed.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:25, 21 January 2014 (MST)

## Major revision proposal

This page is a complete mess, and I think this is largely responsible for why NES 2.0 is poorly and inconsistently adopted across emulators. I started by posting in the [submappers thread on the forum](http://forums.nesdev.org/viewtopic.php?f=3&t=9350) but I might as well outline a proposal here. 

First and foremost, we need a clear criterion for how to allocate a submapper. I propose this: 

  1. Demonstrate the problem by showing us an existing game that is incompatible with the default mapper implementation.
  2. Propose a submapper solution that will solve this problem.
  3. Subject that solution to peer verification of critera 1 and 2. (i.e. Does the problem exist? Is this a solution?)
  4. If it passes review at step 3, add the new submapper to the end of the existing list. Do not delete or deprecate or re-order existing submappers.
  5. When listing a submapper, cite the games the require it.



This is very similar to how I think we should allocate new iNES mappers. Make the software/game/rip/etc. first that requires it. Demonstrate an existing need before allocating. Don't make wishlist requests. (If you don't the ability to add your custom changes to one of the many available open source emulators to work on your project with, I highly doubt that you have the ability to make worthwhile software that needs it.) It doesn't matter if there are hardware variations; if they are compatible with all existing games then we don't need a submapper _yet_. Pre-emptively allocating submappers puts a burden on anyone trying to implement the NES 2.0 spec, which contributes to poor adoption of it. 

Second, I think we should wipe this page out and begin again from Kevtris' submapper document. If there are errors in it, please point them out. I believe all of his suggested submappers are motivated by roughly the same criteria that I just laid out, so I think it's a very good starting point. Add to it, don't take away (unless there is a mistake that causes a real problem), and don't re-implement it because you would have done it differently. There are additions on this page that are probably good candidates (e.g. the MMC3 IRQ variations), and should be allocated. 

Thirdly, I would like to advise against the idea of using PRG-RAM, size fields, or any other header information except _mapper_ and _submapper_ to select a mapper implementation to use. Yes, it's true that we could identify MMC1 variants by size fields, or MMC6 by PRG-RAM, but every time you do this you are creating special cases for mapper selection, each of them requiring their own extra selection logic. This makes the specification less clear and harder to adopt. Submappers are not scarce compared to the number of problem cases, and if we ever do run out (I doubt it) I would suggest allocating a second iNES mapper (using the same low-8 bits but on a different plane) for the excess. By making this arbitrary change, and re-allocating and deprecating Kevtris' allocations for mappers 1 and 4 we have created a schism, making it look like both specifications are worthless, quite frankly. We can't establish an authoritative version of this specification unless there is continuity. 

Finally, when you add a submapper to the list, cite the games that require it. This is essential for ongoing verification and evaluation. It's the only way someone can look at the list and go "oh, well do I need to implement this submapper? what is it for?" The answer is always that it's for a specific game. It's important to specify what game it is! 

\- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 22:00, 6 August 2015 (MDT) 

Also, proposals for new additions should go on a talk page, or the forum. Don't add them directly to the article. It should be a living specification that maintains continuity, not have proposals revolving in and out of it muddying the waters. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 22:03, 6 August 2015 (MDT) 

    I've restored Kev's choices for mapper 4 to the proposals page now that I understand the rationale. But what is "worthwhile software"? What's the correlation between experience with 6502 assembly language and experience with C or C++, which are the implementation languages of most NES emulators? Is there a tutorial anywhere for adding a mapper to any of the free emulators, or is it a "throw them in and see if they can swim" situation? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 09:23, 7 August 2015 (MDT)

    

    I was just being glib to make an argument that we shouldn't specify fantasy/future implementations as a general rule. I'm not going to argue about whether someone could in theory make a cool NES app with a new mapper without being able to modify an emulator. If someone wants to try it but can't do it, they should enlist someone else's help to modify an emulator, not add it to an important specification like this and hope people implement it for them before any software exists for it. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 09:48, 7 August 2015 (MDT)

I've gone over the article, moving anything that was not an implementable specification to [NES 2.0 submappers/Proposals](NES_2_0_submappers_Proposals.xhtml "NES 2.0 submappers/Proposals"). Everything in the article should be ready-to-implement. Anything new that gets added should be discussed and agreed upon before adding to the article, and remember to document new additions with the relevant games. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:51, 7 August 2015 (MDT) 

## UxROM with WRAM support for INES 2.0

I just went over a conversation with zeromus about WRAM support for UxROM, The conversation is here: [UxROM with WRAM support for INES 2.0 in FCEUX r3322](https://sourceforge.net/p/fceultra/bugs/752/) It should be possible now to add WRAM. And also fixes a couple more bugs during the revisions it took to develop this. 

So far only FCEUX r3322 supports this, Your mileage may vary! 
