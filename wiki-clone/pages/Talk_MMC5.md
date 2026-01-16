# Talk:MMC5

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AMMC5) | View [other pages](Special_AllPages.xhtml#Talk_MMC5)

## Contents

  * 1 List of Mysteries
  * 2 Don't Repeat Yourself failure
  * 3 MMC5-internal RAM
  * 4 Even more extended PRG RAM
  * 5 PRG Bankswitching
  * 6 iNES Mapper 5 page



## List of Mysteries

Resolved? | Issue | Notes/Clues/Theories   
---|---|---  
Yes  | Why does the MMC5 have a PPU /WR input?  | Internal expansion RAM can be used as a normal nametable, and some games write to it using PPUDATA.   
No  | Why does the MMC5A (only) apparently listen to writes to PPUADDR?  | Reads and writes from/to PPUDATA directly control the PPU address bus in range $0000-3FFF (A0-A13). Since the MMC5A would already know the PPU address this way, it stands to reason that the normally unused address bits A14 and A15 are captured for some reason. In normal use, A14 and A15 are ignored, resulting in 4 mirrors. Since the MMC5A has control over CIRAM /CE, it is possible that the MMC5A takes control of some or all of these mirrors for some purpose, possibly for the extended attribute data. Further investigation required.   
No  | What is the function of pins 29, 30, 81, 82?  | The pins appear to behave as digital inputs with internal ESD protection diodes, but no function has been observed.   
No  | Why are CHR A0, A1, A2 not used on any known PCB?  | Wiring the board in "SL mode" (with SL3-SL6 closed and CL3-CL6 open) routes PPU A0-A2 through the MMC5 chip, allowing split screen mode to perform smooth vertical scrolling without causing the tile data to "roll in-place". It's possible there was a downside to this configuration, but it is not known.   
No  | Why does the MMC5 apparently monitor writes to PPUSCROLL?  | This could be to offset from fine vertical scrolling in split screen mode (see previous issue).   
No  | Why does the MMC5 apparently monitor writes to OAMDMA?  | Could this trigger the MMC5 to listen to the OAM data and capture bits 2,3,4 in byte 2 of each sprite for some purpose? If so, could those bits be selecting a CHR bank for the sprite? If not so, does that function need to be enabled in a register somehow?   
No  | Why does the MMC5 apparently listen to PPUSTATUS reads?  | The MMC5's scanline detection appears to already do quite well detecting scanlines and v-blank.   
No  | PCM audio "read mode" is intended to result in CPU efficiency savings, as stated in its patent. There is no known way that this mode could save any CPU cycles.  | The present theory is that the feature failed its objective did not end up to be useful.   
No  | Scanline detection diagram and pseudocode do not match each other well in the wiki.  | Further investigation required. These items should be able to reference each other directly/clearly.   
No  | Is expansion RAM backed up by the battery? If so, is it subject to holding reset at power off to prevent corruption?  | Yes, the MMC5's internal RAM is backed up by the battery. No games seem to use it, and it's unclear why. See also this post: <https://forums.nesdev.org/viewtopic.php?p=246243#p246243>  
No  | Is it possible to use an external CHR-RAM for extended attributes on more than 1 nametable?  | This seems like too obvious of an omission based on the level of other features in this chip. Since PPU reads occur every other PPU cycle (due to its multiplexed data/address bus), there is an extra cycle everywhere that can potentially be used to read extended attributes from elsewhere from the same RAM.   
  
## [Don't Repeat Yourself](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself "wikipedia:Don't repeat yourself") failure

Ok, it's crazy to have two completely different pages explaining the MMC5 mapper, one on iNES mapper 5 and the other on MMC5. I think the info should be present on a single page (like it is for all other mappers). 

Well in fact it seems it's Zeromus who added Dish's notes on all iNES mapper pages. This would be nice if the info wasn't already present on the wiki - having twice the same info isn't very logical is it ? I don't know what to do but something should probably be changed...Bregalad 00:36, 23 March 2012 (PDT) 

    Disch' format is **much** better for reading. Funny, I was really thinking to discuss about such thing. :) --[Zepper](User_Zepper.xhtml "User:Zepper") 14:44, 23 March 2012 (PDT)

    Problem solved. @Zepper : If there is a particular point that should be improved about how the mappers are presented on the wiki, then please change it (or at least say more precisely what is _much_ better).[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 06:02, 20 April 2015 (MDT)

    

    Rather than just deleting them, I've been trying to review the disch notes and compare them to the existing article, integrating anything that it was adding before removing it. It's more time consuming than just deleting them, but the whole point of zeromus pasting them here was that they might improve the articles. Sometimes they add nothing, for sure, but it's worth reviewing before deleting, I think. In the process you might find other things here and there to improve in the mapper articles too. This review process is healthy for the wiki content. We don't need to be in a hurry to scour the disch notes from the wiki. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 20:38, 20 April 2015 (MDT)

    

    They have been here for more than three years, so how much time should have been waited for their removal in your opinion? 10 years? As I said they are still available, so you can just download them, and do the review work using your local copy of Dish notes. No point to have them pasted here, period. I absolutely agree that such a review work is benefical, and if there is a particular part you'd want me to review, just ask.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 03:13, 21 April 2015 (MDT)

    

    

    If you'd bothered to pay attention to what Rainwarrior's been doing, he's been carefully looking over each page and incorporating Disch's documentation as he goes, not just blindly removing everything.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 04:53, 21 April 2015 (MDT)

    

    

    I did not just remove it but link to the original place. That's a different thing. Please explain me why Rainwarrior cannot download dish's document on his hard drive and work with that copy. (answer : he can, and that makes a lot more sense). Anyways I'm sick of editing this wiki for a while. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 06:04, 21 April 2015 (MDT)

    

    

    

    There are several reasons why I would not do it that way:
    \- If I download the document and begin integrating it into the Wiki, it can no longer be a collaborative process. If I do that, I have to do every single one myself (nobody can help, sensibly, without being redundant), and there is really no good way to know which have been done already.
    \- Several of the Disch notes sections have had edits in the time since being pasted here, and all of these deserve an overview before being removed.
    \- I believe zeromus' addition of the Disch notes to the Wiki was overall a good thing, filling in content where it was missing at the temporary expense of redundancy.
    \- Simply reverting someone's changes without reviewing them treats them in bad faith.
    "if there is a particular part you'd want me to review, just ask." Please review every deletion you make. That's what I am asking, and that was the project I had begun, myself. If you don't want to do this, please just leave it there. I will get to it eventually! I had started working on this carefully (but slowly). It will probably take me a few months if I do it alone, but I really don't appreciate someone coming in with sweeping deletions and making it difficult for me to try and attempt to finally make good on zeromus' good-faith effort to have the wiki improved by Disch's notes. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 10:48, 21 April 2015 (MDT)

Oh because those HAVE HAD EDITS ?! :O 

Now everything makes even less sense. I give up! This wiki is a huge mess anyway and will always be. Collaborative work just doesn't work. My only regrets is to have worked so hard for the hardware page and the FDS page, when they are doomed to be in the little of this huge disorganized mess with no head and no tail. Any change I make is systematically criticized, even if I asked in the forums if it was ok to do it, and nobody reacts which I belive is "I'm ok with that", but after the change everytime screams like "Begalad you're doing it all wrong, revert those changes!" It's the fourth time there is an issue like that where changes I made in a lame attempt to make this wiki a slightly better place are refuted only AFTER I made them. 

By the way a great thing would be to report those edits to RHDN, so that at least a clean and up to date version of Dish' doccument lies somewhere (and that is on RHDN, not here).~~185.26.182.29~~ 12:47, 21 April 2015 (MDT) 

    If consensus for large scale editing has been established in a topic on the forum, it would be a good idea to include the URL of this topic in the edit summary of each edit so that we know what you're doing. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 12:56, 21 April 2015 (MDT)

    I can't complain about your changes before you make them, unless you tell us what your intentions are first. If you want to make large scale changes unannounced, yes, people will complain after the fact. When else would they complain? I think we want the same thing in the end (no more redundant Disch notes), but I am unhappy with the way you are doing it. I began several days ago a long term effort to do it carefully to make sure we aren't losing information as the Disch notes are removed, but suddenly you have come in to just bulldoze it to the ground, and this does bother me. This isn't collaboration, this is in effect just you having an edit war with zeromus with a 2 year delay. I don't know what argument you had about the FDS article, or what, but it's not at all relevant to the current discussion. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:13, 21 April 2015 (MDT)

    Yeah, it really suck because we're making efforts and we get in the way of eachother. That's just how this wiki works and is doomed to work. As for the thread it's [here](http://forums.nesdev.org/viewtopic.php?f=14&t=12621), but it was in an abandoned state after one day. If I say "should we do that" and nobody react I assume nobody cares if I to do it. Well wrong assumption I guess. I was absolutely sure the Dish docs were untouched that is why I acted this way. However the edits from the are not lost because they're in the history logs. I'll track for them, and update new Dish docs to RHDN to apologize for this behaviour (so I don't have to deal with the wiki anymore but still repair what I have broken). I hope this is okay for everyone. It will take a few days, though.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 13:28, 21 April 2015 (MDT)

    

    I reacted, and I started working on the problem. I didn't want to just revert your deletions because it would basically be an edit war if we don't talk about it first. Undoing a single revision I can explain in the comment summary, but doing many is kind of hostile. I know you spent a good amount of time (in good faith) making the edits, though zeromus also spent a good amount of time putting that stuff up in the first place. Both of you wanted to improve the wiki, and I'm trying to mediate this by keeping anything of value that was there. You don't need to revert anything yourself, at this point; I have been reviewing the changes, and keeping a list of deletions at [User:Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior"), and I'll eventually get through them one by one as part of the ongoing project I started. I want to keep all the document links you added, for eaxmple, because those are good. As for updating RHDN, you can do that if you like, but I'm only concerned about the wiki. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:42, 21 April 2015 (MDT)

## MMC5-internal RAM

What is the logic in the ASIC that causes it to write zero if the PPU is not rendering? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 01:51, 22 September 2012 (MDT) 

Another question about the ExRAM is, what happens when you try to read/write ExRAM nametables through the PPU registers, and if extended attribute mode is selected, what happens when reading/writing attribute tables using the PPU registers (when it isn't rendering the picture, and in any potentially random order)? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 22:23, 24 February 2014 (MST) 

Yet another question : Is the ExRAM battery backed ? It would seem no, but technically the Battery is connected to the MMC5 so who knowns ? [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 04:38, 5 May 2014 (MDT) 

## Even more extended PRG RAM

Parsimony of silicon strongly implies that the higher address lines (corresponding to the 0x78 bits of the register) are still driven for the registers from $5114 to $5116 even when RAM is selected, meaning >64KiB PRG-RAM would be usable when mapped to $8000-$DFFF. It's conceivable that these same bits of the register at $5113 (controlling PRG-RAM bank) are implemented, since they have to feed a multiplexer anyway. Something to test, maybe. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 23:34, 20 January 2014 (MST) 

## PRG Bankswitching

The PRG bankswitching section was much better before the recent edits by Ben Bolt. In particular, I do not see why non-existing registers $5112 etc... are even mentioned at all. The current page is a huge mess and I don't see any new info that wasn't there before, except that it's now much less readable and non understandable.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 00:40, 26 November 2018 (MST) 

Also, this is not related to recent edits, but the info in this wiki clearly contradict Dish notes. 

Wiki says: 

_Mode 1 - Select a 16KB PRG ROM bank at $C000-$FFFF (pass through CPU A13 to PRG A13 instead of using bit 0)_ _Mode 0 - Select a 32KB PRG ROM bank at $8000-$FFFF (pass through CPU A13 and A14 to PRG A13 and A14 instead of using bits 0 and 1)_

Before the recent edits it said: 

_Mode 0 - Select a 32KB PRG ROM bank at $8000-$FFFF (ignore bottom 2 bits)_ _Mode 1 - Select a 16KB PRG ROM bank at $C000-$FFFF (ignore bottom bit)_

which is equivalent, but simpler to understand in a sowftware viewpoint. 

Dish however says: 

_Note that unlike most other mappers, these CHR pages are in *actual* sizes. IE: when in 4k mode, registers contain 4k page numbers. But when in 2k mode, register contain 2k page numbers._

Which is the oposite ! This was already contradictory before the recent edits but I didn't notice. So who's right, who's wrong ? At least Castlevania III uses 16k banks with MMC5 so this should be easy to figure out. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 08:50, 26 November 2018 (MST) 

    You'll note that the former comment is about PRG bankswitching, and Disch's comment is about CHR bankswitching. Both are true. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:29, 26 November 2018 (MST)

    

    Ah Okaaay... so the MMC5 did the shifting trick for it's CHR pages but not PRG pages where bigger sizes are used like usual by ignoring lower bits... that's very tricky indeed. This should probably be mentionned after the PRG bankswitching part is fixed up.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 13:56, 26 November 2018 (MST)

OK I hope everything is fixed now, tried to present the information in a way that is simultaneously as concise and complete as possible.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 15:10, 21 December 2018 (MST) 

    Thanks for cleaning up my mess [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)"). [Ben Boldt](User_Ben_Boldt.xhtml "User:Ben Boldt") ([talk](User_talk_Ben_Boldt.xhtml "User talk:Ben Boldt")) 11:33, 1 January 2019 (MST)

## iNES Mapper 5 page

It is mentioned in a previous section here that Disch had his own version of this MMC5 page, in the "don't repeat yourself" section. Does anyone have a link to that or a backup? I would like to go back and make sure we have integrated everything from Disch properly. [Ben Boldt](User_Ben_Boldt.xhtml "User:Ben Boldt") ([talk](User_talk_Ben_Boldt.xhtml "User talk:Ben Boldt")) 17:48, 3 August 2021 (UTC) 

    In case anybody's still interested, it's available at: <https://www.romhacking.net/documents/362/> [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 14:05, 26 March 2024 (UTC)
