# Talk:UxROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AUxROM) | View [other pages](Special_AllPages.xhtml#Talk_UxROM)

@lidnariq : Seriously, this paragraph was a bunch of shit, it is speculatively mentionning non-existant oversize variations, speculatively mentionning a multicart that never existed and speculatively talked about a non-existing mapper that isn't even mapper #2, cloning the SUROM behaviour but without a MMC1. Why did you re-insert it ? Is it any valuable when it comes to doccumenting the existing games/hardware and mappers ? Is the goal of this wiki to doccument existing things, or to speculate about what could have been done ? [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 05:09, 18 March 2014 (MDT) 

    By now I'm starting to agree with you that because emulators have standardized on the behavior with the additional 7432, SUROM-style behavior belongs in a separate page, just as other multicart mappers get their own. Let Farid do what he does best. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 11:37, 18 March 2014 (MDT)

    The entire point was that the cost of the second 74'32 can be exchanged for having every 16th bank be identical.
    This is no different in scope than using an inverter to prevent bus conflicts.
    Since UxROM uses discrete logic, it continues to be a perfectly viable target for physical production.
    Saying the sole purpose of the wiki is to describe what _was_ is to say that the sole purpose of the wiki is for archiving details necessary for emulation.
    Finally, if you want me to take you seriously, try not cussing. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:56, 18 March 2014 (MDT)

    

    Even if describing only what was, if it was used in a known homebrew, that makes it relevant (though it is stupid that the "known" homebrew was not named in the text, which seems in contradiction with "known"). I do think it's worth mentioning practical extensions of a mapper, provided they are clear-cut and natural extension, and not merely one of a billion other irrelevant possibilities. In this particular case, I think 512k UxROM is worth mentioning, but I don't think it's good to offer 5 different recipes for building the hardware. That's more confusing than it is helpful. We should not give users advice and alternatives based on the potential cost of chips, for example. That's a discussion to be had on the forums for the benefit of someone who actually wants to build it. For the wiki, this kind of stuff is just noise inhibiting comprehension of the page. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior"))

    

    

    What exactly do you think the sections of a wiki page are for? It's not like the average person visiting the page is going to care about the Hardware section.
    For that matter, why leave the comments about the 7402 or 74157 if you're worried about confusing the poor little naïf of a reader?
    Nintendo released three boards (UNROM, UOROM, UN1ROM, and m180) of the relevant type. All used one 74161 and one quad 2-input logic gate.
    _Emulator authors_ decided that the platonic ideal of mapper 2 is "one bank switched, one bank fixed to the last one". But this is emulator-centric thinking.
    Because BK2 was developed _in an emulator_ , of course it counts as _works like the emulators say it does_.
    So it's equally plausible that the concept represented by UxROM is "one quad OR gate and one latch", the same as the vast majority of the other discrete logic mappers with more than 4 bits of state.
    Note that I'm arguing about **UxROM** (hardware), **not mapper 2** (emulators). —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 21:08, 19 March 2014 (MDT)

    

    

    

    My objection has nothing to do with preferring emulation information to hardware information. I think it's great to have good info about the hardware. My objection is to needless speculation and hypothetical implementations. The hardware diagram and description on this page is great, at least where it's describing the actual board used. The moment it starts speculating about different ways to build it are where I think we've gone off track. "A 74HC02 quad NOR gate can be used instead..." begins a hypothetical design situation (unless you are describing actual practice used by some games, in which case please say so in the text). "If an actual multiplexer (74HC157 quad 2:1) is cheaper..." is not relevant to how this board works. I would be similarly opposed if someone were to add to the page 4 different C++ implementions of the mapper, along with commentary about the strengths and weaknesses of each approach. The 512k variant is relevant, and it needs to be mentioned, but this is not the place to hash out all the permutations of possibly ways to design its hardware. Do that on the forums for the benefit of someone who is actually having trouble building it. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 23:27, 19 March 2014 (MDT)

    

    

    

    Also, maintaining relevance on the page is for naive and experienced readers alike, my statements about comprehensibility aren't about the skill level of the reader. Keeping the pages organized is important for anyone trying to find the information they need. For example, trying to incorporate and eliminate redundant Disch notes is important because a thorough author trying to implement a mapper correctly will read the entire page to make sure he didn't miss something. It wastes the time of all readers to clutter the page with redundant or irrelevant text, not just the inexperienced ones. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 23:52, 19 March 2014 (MDT)

    

    

    

    

    There is plenty of other advisory content elsewiki. e.g. [Programming with unofficial opcodes](Programming_with_unofficial_opcodes.xhtml "Programming with unofficial opcodes"), [PRG RAM circuit](PRG_RAM_circuit.xhtml "PRG RAM circuit"), and [6502 assembly optimisations](6502_assembly_optimisations.xhtml "6502 assembly optimisations"), and I see no reason this wiki should contain solely descriptive pages. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:15, 21 March 2014 (MDT)

    

    

    

    

    

    These three pages are all exclusively about this kind of content, though. In that respect they are well organized. I think all of the mapper pages should primarily be descriptive reference. The PRG RAM circuit page is a good example of material that would be bad to insert into a mapper page, even though this is information about how to build part of a mapper. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 23:17, 21 March 2014 (MDT)

    

    

    

    

    

    

    So how do you answer a question that a person doesn't know they can ask? This is the kind of information for which to ask the question is to already know the answer. Saying that "that information belongs in the forum" is equivalent to saying "that information doesn't belong _anywhere at all_ ". —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:14, 24 March 2014 (MDT)

    

    

    

    

    

    

    

    I don't understand that logic. This wiki is full of links to the forums for content that is related but too involved or special-purpose for the wiki article. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 20:23, 24 March 2014 (MDT)

    

    

    

    

    

    

    

    

    So you're saying you're ok with something to the effect of "Other oversize variants are possible" which is a link to a forum post? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:42, 25 March 2014 (MDT)

    

    

    

    

    

    

    

    

    

    Absolutely. I'd also be fine with a discrete logic mapper design tutorial as its own wiki article, or some other way to organize this content. I'm not opposed to the content, just its inclusion here on this page. Where there is a single canonical hardware implementation of a mapper, I think it's fine to have on a mapper page. When it becomes a how-to description of many possibilities, I think this belongs somewhere else. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 10:14, 26 March 2014 (MDT)

    

    

    Exactly what Rainwarrior said. Anything with the last bank which is not fixed in not mapper #2 and even less UxROM and have nothing to do on this page. "Oversize" variants are worth mentioning and they are already mentioned outside of this useless and confusing paragraph. Anyone with any tad of knownledge of electronics will know several ways of implementing them, there is no need to explain how to implement everything that "could be done" on this wiki.

[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:07, 19 March 2014 (MDT) 

    

    

    Well, I think it's great time we find a concensus, about what exactly terms like SxROM, UxROM, AxROM means, what should be described on which page and how. I'm not caring about naive readers, I'm just caring about myself, as I'm a ego-centric person ^^ And, more seriously, yes, the average person visiting here ***will*** care about the hardware information. In my opinion (which is not agreed by everyone) the term "UxROM" refers purely to a set of hardware PCBs, while the term "mapper #2" refers to the concept you have one bankswitching register to bankswitch to switch $8000-$bfff, and the last bank fixed at $c000-$ffff. There is literally infinite ways to implement this on hardware, and anyone knowing digital electronics is going to be able to do it. I can do it with 74 series chips, a PAL, a CPLD, a FPGA, with a microcontroller which is a couple of times faster than the NES, or with plain transistors if I feel like it. I could even implement this with a non-power of two ROM size, the concept works just as well. Documenting how Nintendo (and possibly other companies) implemented the mapper is very relevant. Documenting every possibility of implementation is not, as you could always find another one.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:35, 20 March 2014 (MDT)

In fact I think that's a great idea : Let's put one wiki page with all the ideas to make discrete logic mappers, and leave other pages free of them. That way the info is on the wiki, and it doesn't interfere with software mappers or Nitendo's boards info. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 06:38, 28 March 2014 (MDT) 

## 74HC02 in Metal Gear (USA)

I wonder why is an additional 74HC02 (quad NOR gate) chip doing [on the PCB of Metal Grear (USA)](http://bootgod.dyndns.org:7777/profile.php?id=79). My random wild guess would be it disables bus conflicts by inverting PRG-R/W, can anyone confirm or infirm this ? 

Why not use only a single 74HC02 as the main OR gate (by reordering PRG banks to compensate the unwanted inversion after OR-ing adress lines) and to disable bus conflicts, then ? [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 04:27, 6 May 2014 (MDT) 
