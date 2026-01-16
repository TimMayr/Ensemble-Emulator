# Talk:NES 2.0 submappers/Proposals

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANES_2.0_submappers/Proposals) | View [other pages](Special_AllPages.xhtml#Talk_NES_2_0_submappers_Proposals)

## Bus conflict submapper 0

[This revision](http://wiki.nesdev.org/w/index.php?title=NES_2.0_submappers/Proposals&oldid=11603#002.2C_003.2C_007:_UxROM.2C_CNROM.2C_AxROM) makes the following suggestion: 

    _0: Normal (The game is believed to never elicit a bus conflict. In the event of a bus conflict, the emulator should warn, abort, or produce random values; the exact behavior is not known)_

I think the goal for submapper 0 should usually be the greatest compatibility, and backwards compatibility with iNES. Setting the NES 2.0 identifier shouldn't change the behaviour of submapper 0 (if all other fields are otherwise "equivalent"). Validation tools to "warn" or "abort" could work nicely with submapper 2 (i.e. emulate bus conflicts), but I don't think they would do anything except reduce compatibility if used for submapper 0. A validation tool really shouldn't be part of the submapper definition (that's its own tool with its own goal, e.g. like nintendulator DX's thing to warn on use of uninitialized memory). 

Cases 1 and 2 are good, it separates two things with specific conflicting needs. I think case 0 should just delegate a recommendation for 1 or 2. 

  * 002:0 = 002:1 ? [UxROM](UxROM.xhtml "UxROM") says that mapper 002 shares UxROM with compatible boards that require no bus conflicts. (Also, that DK pie factory hack?)
  * 003:0 = 003:2 ? [INES Mapper 003](CNROM.xhtml "INES Mapper 003") lists 1 game (Cybernoid) that requires bus conflicts, and 1 (Colorful Dragon, UNL) that requires none. Does Cybernoid get priority?
  * 007:0 = 007:1 ? a lack of bus conflicts is required for some existing games, but it seems that none require bus conflicts.



Alternatively I would just propose that submapper 0 be used for no bus conflicts, and submapper 1 be used for AND bus conflicts, since a lack of bus conflicts might just generally produce greater compatibility? If your goal is to give homebrewers a nice testing environment that will emulate bus conflicts for them, submapper 2 does that job great, it doesn't need to be the default. The only case I've seen listed so far is Cybernoid? 

I was wondering about other mappers besides 002/003/007 but I am guessing that the other bus-conflicting mappers don't have the ambiguity problems and can safely always have bus conflicts on? 

\- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:53, 31 August 2015 (MDT) 

    The default for any mapper that physically has bus conflicts (even if never elicited) must not be "no bus conflicts" for reasons we have already discussed and you find uncompelling.
    FCEUX has already decided that the iNES1 handler is "produce bogus values (always 0) for CNROM and UNROM" (although it used to be AND) and "no bus conflicts for other data-bus latch mappers". Nestopia has codified bus conflicts as AND, for the "standard" version of specific discrete-logic mappers (UNROM, UOROM, CNROM, BNROM) and none otherwise.
    —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 18:31, 31 August 2015 (MDT)

    

    I'm confused by this response? We're not discussing any mappers that always had bus conflicts here. We're discussing iNES mappers 2, 3, and 7, none of which unambiguously represents a physical mapper that had bus conflicts. Mapper 2 seems to include many obscure boards, some of which reputedly require a lack of bus conflicts? (I have no source for this other than statements found on the wiki; what are the actual compatibility cases?) Mapper 3 appears to be have the only compatibility conflict among these 3 mappers, and from the two cases listed a bus-conflicts submapper 0 seems valid to me. Mapper 7 submapper 0 requires no bus conflicts for compatibility. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 19:31, 31 August 2015 (MDT)

    

    

    Your central thesis appears to be that compatibility is the foremost goal, and my argument (from experience) is that prioritizing compatibility produces homebrew that can't run on hardware, so the default must be the strictest narrowest definition instead (and that people must explicitly opt into more lenient behavior). We've already had this argument. What's the point in having it again? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 20:41, 31 August 2015 (MDT)

    

    

    

    I'm not very much interested in restating opinions about ideological concerns for the format either, but I _am_ interested in the specific definition of mappers 2, 3, and 7 here, in response to the submapper 0 text you just proposed. I strongly feel that mapper 7's submapper 0 should not break the existing ANROM games. Are you really arguing that it should? I don't remember you ever responding about mapper 7. We don't actually seem to have a dispute about mapper 3 (i.e. submapper 0 with bus conflicts), but I am unsure about it because I can't find much information about the non-CNROM games that use this mapper? Mapper 2 there is even less knowledge about, but the Disch notes for mapper 2 state _" UxROM has bus conflicts, however mapper 002 is meant to be UxROM and compatible. So some mappers which were similar in function, but did not have bus conflicts are included"_, but again I can't find any existing research about the games/boards involved. Is the best we can do for this an emulator survey? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 12:40, 1 September 2015 (MDT)

Hi guys, just thought I'd mention some ideas here even though I don't have a lot of time for this kind of stuff and probably won't be back to debate it. IMO, ines 1 mapper numbers arent strict definitions but rather a vague guide for organizing a family of functionality. Moving forward, I would like to see 'submapper 0' for mapper N to be 'do what you can to get the game to boot' without increasing the strictness of the legacy mapper's definition. Then, exceptions can be written by forwarding to a submapper. For instance, we might see: "Mapper N Submapper 0 = Mapper N Legacy Mode: A game with this submapper can't be safely run without a game-specific logic. It might have bus conflicts (handle as Submapper 1) or no bus conflicts (handle as Submapper 2)." Essentially leave submapper 0 as flexible legacy hack zone ghetto. This allows us the latitude to actually prescribe as part of the specification how legacy garbage is to be interpreted as well as set a simple rule for how to define submapper 0. If there's a channel besides #nesdev I wouldnt mind lurking and giving further feedback, but I don't have time to take a strong stance [Zeromus](User_Zeromus.xhtml "User:Zeromus") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Zeromus&action=edit&redlink=1 "User talk:Zeromus \(page does not exist\)")) 21:25, 1 September 2015 (MDT) 

  
Ok, here's a pragmatic argument: What's the _practical_ difference between: 

  * 300 games as submapper 0 (the ones where the hardware has bus conflicts, but the games have a bus conflict prevention table)
  * 2 games as submapper 1
  * 2 games as submapper 2



And 

  * 302 games as submapper 0
  * 2 games as submapper 1



Especially when the size of the two minority groups is comparable? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:27, 2 September 2015 (MDT) 

    Scenario the First has four games which can be emulated unambiguously without hacks keyed to hashes. Scenario the Second has two games which can be emulated unamiguously without hacks keyed to hashes--so two of the games won't work on every emulator listing itself as in complete Mapper N compliance without the emulator having special cases for them. . . . . . . . . . . . Now that I've thought about it one more day, I have another proposition. Using submapper 0 as a catch-all for legacy behaviour is a bad idea. We should use iNES1 headers as an indicator that legacy behaviour is required. A rom should not be graduated to iNES2 until it can be assigned to a submapper that precisely defines it. In that scenario, we have 2 games as submapper 0, 2 games as submapper 1, and 300 games in iNES1 that cannot be legally turned into iNES2 without simultaneously encoding their bus conflict needs. Now, from an organizational perspective, this would require namespacing and maybe even documenting iNES1 and iNES2 mappers separately. You could resolve this by declaring that iNES2 is a strict superset defined as enclosing iNES1 within submapper 0 (which was my original proposition). Another way of writing the rule might be: "For mappers with >1 submapper, submapper 0 means, select another submapper using a database" [Zeromus](User_Zeromus.xhtml "User:Zeromus") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Zeromus&action=edit&redlink=1 "User talk:Zeromus \(page does not exist\)")) 21:14, 2 September 2015 (MDT)

    

    Yes, with submapper 1 and 2 as an explicit bus conflict indication, really the only thing left for submapper 0 is "unknown" or "doesn't matter", which wouldn't normally need an iNES 2 header anyway, but I think this might be used if, say, it's a rip from an unknown board but you need to specify WRAM or some other unusual iNES 2 only thing besides bus conflicts? What I'd like to know more about, though, is how many ROMs exist for mapper 2 and 3 that rely on a lack of bus conflicts. A reliance on bus conflicts (in mappers 2/3/7) is usually unintentional, isn't it? Cybernoid is a very strange case. Games that rely on a lack of bus conflicts, however, do so because of a deliberate expectation, and I'd expect there to be more of these than ones that rely on bus conflicts. Obviously, every real UxROM and CNROM game are excluded from this case, but both mappers 2 and 3 supposedly contain other boards that aren't. Anyhow, despite my suspicions, in light of [how inconsistent emulator implementation appears to be](http://forums.nesdev.org/viewtopic.php?f=3&t=13191#p154441), I think the previous text of the proposal was much more practical: _Normal (No advisory statement is made as to whether this game has bus conflicts)_. People that want an explicit behaviour should really have to choose submapper 1 or 2, since the expected default behaviour is unreliable. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 23:25, 2 September 2015 (MDT)

    

    

    There's two arguments here. One is whether submapper represents ABI (expects bus conflicts, works around; relies on no bus conflicts; relies on AND bus conflicts) or hardware (AND bus conflicts are present; bus conflicts are not present).
    The other is "what should submapper 0 be"? There's been two somewhat contradictory rules-of-thumb that have been proposed about submapper 0 "should" mean.
    Should it be "the vast majority of games are here"? In which case, for UxROM and CNROM, that means submapper 0 should be "Games with bus conflicts, and (depending on the first question) that work around those bus conflicts".
    Or should it be "No advisory statement is made, use existing heuristics"? In that case, we probably want a submapper explicitly for "this game has been checked and yes we know that it expects bus conflicts (and maybe works around those bus conflicts)".

    

    

    I originally was of the opinion that the submappers should just represent hardware. But as I've thought about [FCEUX SVN 2997](http://sourceforge.net/p/fceultra/code/2997/tree//fceu/trunk/src/boards/datalatch.cpp?diff=4fd2332971b75b3aad0001b3:2996) (which looks awfully like "intentionally breaking things"), I find myself thinking that a "fail fast" design is better.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 00:54, 3 September 2015 (MDT)

    

    

    

    I am not very good with rules of thumb, apparently. Every mapper seems a bit different case. Perhaps majority compatibility was a suggestion that confused things? Probably the idea that submapper 0 just means "do as iNES does" was the better idea. In this particular case, there just doesn't seem to be an established standard way to implement mapper 2 or 3. When updating the mapper info across the wiki I've tried to describe what someone should expect to see in emulators or hardware, and to be explicit about which is which.

    

    

    

    If someone wants to implement an emulator that spits out a warning or refuses to run when it finds something invalid, that's perfectly fine for them to do. Many emulators do this, or have options for this kind of thing. I don't mind suggestions and recommendations to do this, but they shouldn't be the definition of a mapper. They should be explained conditionally. "You can avoid problem X by doing Y." Like the reason I want to know if there are mapper 2/3 games that will break with bus conflicts is so that I can give that information to the implementer to make an informed choice, rather than just blindly following a recommendation.

    

    

    

    People put all these hacks and heuristics into their emulators because there was something they wanted to accomplish with it, usually a game they wanted to run. You can tell them "don't do this" all you want, but if you don't tell them why, they already have a "why" they should ignore you (the thing they want to run). - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 10:18, 3 September 2015 (MDT)

An aside about that FCEUX bus conflict implementation. Cybernoid has a bug that accidentally writes $0F against $50 when you toggle music during gameplay. Since it seems to exclusively use CHR bank 0, a "bus conflicts" implementation that forces 0 by luck doesn't happen to break it. No idea what prompted this as a "solution" for Top Gun, though. I haven't been able to find a bus-conflicting write in it. The [bug report](http://sourceforge.net/p/fceultra/bugs/621/) said something about mirror_in_use? It mentions a hacked [Highway Star](http://bootgod.dyndns.org:7777/profile.php?id=3918), but I don't know why it is being crammed into mapper 2. Very strange. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 10:49, 3 September 2015 (MDT) 

    Looks like cah4e3 had reconsidered the hack and reverted the "highway star" support, just forgot to revert the bus conflict code at the same time. I've gone and restored it. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:19, 3 September 2015 (MDT)

Well, with that all done, the only question remaining is: do we want the discrete logic submappers to codify ABI, or to codify hardware? Reserving an extra submapper for the former allows an emulator to explicitly mark an error condition, whereas the latter choice instead requires a run-time option. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:05, 3 September 2015 (MDT) 

    I'd rather have both submappers. I think it's more useful to be able to specify than try to impose a strict behaviour on submapper 0. I have created a set of test ROMs for mapper 3 so far: [forum post](http://forums.nesdev.org/viewtopic.php?p=154555#p154555). The only emulators I could find that currently try to bus-conflict mapper 3 were puNES and FCEUX (though I don't know if I'm running up against heuristics in some cases). - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:20, 3 September 2015 (MDT)

    

    I wasn't even talking about submapper 0 behavior. Instead, whether 1/2 are "bus conflicts Do Not/Do exist" or whether 1/2/3 are "bus conflicts are unexpected/bus conflicts are expected to produce AND/program anticipates bus conflicts and includes a bus conflict prevention table". —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:28, 3 September 2015 (MDT)

    

    

    Since that third option would run correctly on both 1 and 2, I'm not sure what the need is. Like, for UxROM/CNROM boards, really you want those in submapper 2 so that unexpected bugs behave more correctly, right? If it's a game that exists on two different boards, you could make two rips with two headers. If you want a validation mode for an emulator to warn you about unmatched bus conflicts, that's probably a good runtime option (Nestopia's debug build already does this for some things, I think? NST_VERIFY), but I am not sure how a submapper itself would help that. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:40, 3 September 2015 (MDT)

My idea about what submapper 0 should mean is similar to what is written near the top (I quote: "I think the goal for submapper 0 should usually be the greatest compatibility, and backwards compatibility with iNES. Setting the NES 2.0 identifier shouldn't change the behaviour of submapper 0"), but not quite. In some cases, the old iNES header is ambiguous and the header fields other than submapper number cannot disambiguate; in these cases, where emulators without NES 2.0 would use hash checks, there should be no hash checks when the NES 2.0 header is present; in these cases it may be necessary to set the submapper number when converting the header to NES 2.0 format. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 13:24, 13 August 2016 (MDT) 

## MMC5/MMC5B

Difference from MMC5 and MMC5B apparently is currently unknown. The bit3 of the submapper number for mapper 5 may be used to select between MMC5 and MMC5B, where if the bit is clear it is compatible with all known games (even if it does not correctly describe the hardware). The following differences are not considered as significant: 

  * If one variant initializes some mapper registers at power on which the other variant leaves uninitialized
  * If the rest of the contents of the header (including the rest of the submapper number) can indicate these differences
  * If one variant has more capacity for external ROM/RAM than the other variant
  * Electrical tolerances which do not affect software
  * Internal wiring of the cartridge which software does not detect



These differences will be significant if they exist: 

  * Presence or absence of PCM read mode
  * IRQ differences which can be detected by software
  * If spying on mirrors of PPU registers differs between variants
  * Writing to ExRAM through PPU if possible on one but not on the other



\--[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 13:14, 13 August 2016 (MDT) 
