# Talk:NSF

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANSF) | View [other pages](Special_AllPages.xhtml#Talk_NSF)

## Contents

  * 1 Emulator playability
  * 2 $4010 value
  * 3 $4017 register
  * 4 Multi expansion
  * 5 Consistency
  * 6 More clarification
  * 7 IRQs and hardware NSF player



## Emulator playability

A NSF tune is perfectly playable in an emulator; in short words, a "normal" NES emulator, not something clipped for exclusive NSF playing and behaviour. 

Regarding 4015h, well... it's empirical. My experience says that setting 4015h to 0Fh is required in order to get *a lot of* tunes starting playing. I don't remember of *any* broken tune by setting such value. So, it's recommended *to follow* such thing. --[Zepper](User_Zepper.xhtml "User:Zepper") 14:25, 29 March 2012 (PDT) 

    Adding NSF support involves a mapper, an executable format, and a piece of shell code to draw the title, artist, and publisher, run the player in a loop, and switch tracks. But you're right that an emulator incapable of playing NSF is incapable of emulating something that runs on an NES, as the PowerPak has an NSF player. --[Tepples](User_Tepples.xhtml "User:Tepples") 20:29, 29 March 2012 (PDT)

Yup, I assigned mapper 256, since iNES cannot assign it. And yes, it's required a player (I wrote my own in asm) and commands to switch tracks, but that's all. Perhaps I wasn't crystal clear... :( --[Zepper](User_Zepper.xhtml "User:Zepper") 14:31, 30 March 2012 (PDT) 

    [NES 2.0](NES_2_0.xhtml "NES 2.0") can assign mappers up to $FFF.F. Yes, that's a [hexadecimal point](https://en.wikipedia.org/wiki/radix_point "wikipedia:radix point"), used to separate the mapper number from the variant number. No variants are officially assigned though. --[Tepples](User_Tepples.xhtml "User:Tepples") 15:00, 30 March 2012 (PDT)

I admit that my first statement sounds like "any NSF file can be loaded into any NES emulator", which **isn't** true. Proper support for NSF is required, as tepples noted. So, the emulator must create a mapper (I assigned 256, since **iNES 1.0** cannot assign it), a player (I use 6502 code loaded at $4018-$4FFF) and controls for changing tracks, plus the frame rate control. The NSF header is parsed and then the file is loaded as specified by the document. Sorry for the confusion! --[Zepper](User_Zepper.xhtml "User:Zepper") 10:05, 31 March 2012 (PDT) 

    [NES 2.0](NES_2_0.xhtml "NES 2.0") can use mapper number 256. I would recommend mapper number -1 since NES 2.0 cannot assign it, and DotFami also specifies that mapper number -1 is used in the ines.map file to indicate it is NSF. (If it has to be unsigned, you could also use a mapper number greater than $FFF.F instead of -1; it won't be compatible with DotFami but you might not consider that important.) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 16:48, 1 September 2012 (MDT)

## $4010 value

The page appears to recommend writing $10 to $4010 but $00 to most other APU registers. However [APU DMC](APU_DMC.xhtml "APU DMC") says there's nothing mapped at D4 of $4010. Where did this come from? --[Tepples](User_Tepples.xhtml "User:Tepples") 12:55, 31 March 2012 (PDT) 

    That's an odd one. It's in Kevin Horton's spec. /shrugs [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 17:57, 31 March 2012 (PDT)

## $4017 register

What about this register? For NSFs, is correct to write $00 to $4017 (frame IRQ enabled) and re-writing the last value on reset? 

    I don't think there should be any specification about what to do with this register for a couple of reasons. 1. NSF does not support any kind of IRQ; so enabling it is inappropriate. 2. Most software NSF players do not support resetting the frame sequencer by writing to $4017. 3. If your hardware player is NMI driven, you probably want to avoid writing to it so that it does not reset and get out of phase. 4. Alternatively your hardware player may be driven by the frame sequencer interrupt (esp. if there is no PPU), in which case the needs of the player dictate how the register should be used anyway. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 22:14, 31 March 2012 (PDT)

    If anything I think it might be good to forbid the use of $4017 in the NSF spec, but I'm not sure on this. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 22:17, 31 March 2012 (PDT) 

    Forbidding use of the frame IRQ ([$4017 D6](APU_Frame_Counter.xhtml "APU Frame Counter")) is fine, as it confuses IRQ-driven hardware players. To support all rate/period values, a hardware player implementing the NSF mapper will be using its own timer IRQ anyway, [insha](https://en.wikipedia.org/wiki/Insha%27Allah "wikipedia:Insha'Allah")'[411ah](http://forums.nesdev.org/viewtopic.php?p=81006#p81006). But if you forbid $4017 writes entirely, then you forbid use of the length counter, linear counter, sweep units, and hardware envelopes, as their rate depends on [$4017 D7](APU_Frame_Counter.xhtml "APU Frame Counter"). --[Tepples](User_Tepples.xhtml "User:Tepples") 06:11, 1 April 2012 (PDT)
    Allow me to clarify further: If you keep the wording "$4017 should not be written to, as it may be needed by the player implementation", you're implying that NSFs SHOULD NOT use anything clocked by the frame counter, and that music engines using these features (such as that of Klax) SHOULD be converted to apply these effects in software. --[Tepples](User_Tepples.xhtml "User:Tepples") 14:36, 1 April 2012 (PDT)

    

    

    I wasn't trying to forbid it entirely, but I do think it should be recommended against. Maybe "should not interfere with the $4017 register" could be "should avoid interfering with..." Forbidding $4017 writes doesn't forbid the use of frame sequenced events at all, it merely forbids tampering with their default timing. The interrupt itself IS explicitly forbidden by the standard. From what I've seen, using $4017 to force a frame sequencer event/reset is widely unsupported by NSF players. For instance, I've heard of writing $4017 to force a sweep clock when attempting to use hardware sweeps to set the high bits of a square channel (avoiding the phase reset). What does Klax do, and how are NSF players responding to it? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 15:01, 1 April 2012 (PDT)

    

    

    

    Okay, I have rephrased those sections to instead address the inability of NSF players to support it. Does this seem okay, or does it need further revision? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 15:11, 1 April 2012 (PDT)

    

    

    

    

    What Klax does is heavily use the sweep for precise detunes and arpeggiated chords and repeating hardware envelopes for fine rhythm. Have you tried going through the NSF library to see which use a 4-step sequence and which use a 5-step? --[Tepples](User_Tepples.xhtml "User:Tepples") 15:30, 1 April 2012 (PDT)

    

    

    

    

    

    Here's what I found with a quick check of about 80 random NSF files I had on hand: FamiTracker NSFs set $4017 = $C0 followed by $40 on init, and I believe it also does this when it is beginning any hardware slide. ppMCK and SuperNSF do not use $4017. Zelda, Zelda 2 FDS, Nazo no Murasamejou, SMB, SMB FDS, SMB2, and SMB3 set 4017 = $FF every frame. Metroid, Metroid FDS, Tetris, and Uchuu Kebitai SDF set $4017 = $C0 every frame. Shin 4-Nin Uchi Mahjong sets $00 then $40 on init. Skate or Die 2 sets $FF on init. All the rest I tried (including Klax) completely ignored $4017. So, interestingly, except for Skate or Die 2, the games that would be selecting a 5 step sequence are generally resetting it every frame, which I believe effectively makes it the same as the 4 step sequence. Skate or Die 2 is probably being incorrectly played by a lot of players (it does use envelopes). - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 16:20, 1 April 2012 (PDT)

    

    

    

    

    

    I guess that also means that it might be generally safe to write $4017 as long as the interrupt disable flag is set, since there is some precedent with a few extremely common Nintendo game NSFs. Maybe the only thing that should be forbidden is clearing the IRQ disable bit, and if there are hardware players relying on the frame counter IRQ they can deal (maybe a brief note would be good). Made another revision, let me know if okay. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 17:00, 1 April 2012 (PDT)
    ←

This version should be fine, and it's close in meaning to what I had. "Kate Oar Diiie" 2 isn't well supported by NSF players anyway due to other limits of NSF. I don't think a lot of hardware players use the APU frame counter because it isn't flexible enough rate-wise for NSFs whose playback rate differs sharply from $4100/$411A. --[Tepples](User_Tepples.xhtml "User:Tepples") 17:25, 1 April 2012 (PDT) 

    What kind of problem/annoyance such NSF suffers? Remember that my emu threats a NSF like a normal game (regarding $4017). --[Zepper](User_Zepper.xhtml "User:Zepper") 19:20, 1 April 2012 (PDT) 

    As I tried to hint with my nickname for the game, NSF inherently can't encode the 4-bit PCM samples in SOD2's theme song. --[Tepples](User_Tepples.xhtml "User:Tepples") 05:43, 2 April 2012 (PDT)

I remember of... Gil Galad (?) had fixed a bunch of NSFs, regarding $4017. I'm seeing an excessive "caution" about the NSF section, so be easy with it. --[Zepper](User_Zepper.xhtml "User:Zepper") 14:43, 2 April 2012 (PDT) 

## Multi expansion

I like the new section with multi expansion notes. I have a few thoughts about it. 

  * It's probably worth suggesting to not use mirrored register addresses in general. I don't think mirrored addresses are widely supported in existing NSF players, possibly because the NSF spec never mentions them.
  * Perhaps some recommendations can be made for NSF players as to how to avoid the conflict, i.e. writes to VRC7 registers should not be mirrored to VRC6 as well, etc. This may facilitate the implementaion of multi-expansion players, so that a consistent practice can be maintained.
  * Does the 5B actually mirror E000 to E000-FFFF? (...and C000 to C000-DFFF?) I can't find any good documentation on it; the emulators I've checked so far only respond to $C000/$E000. Does the FME-7 mapper page need an update?
  * FDS and writable memory needs a clarification; these areas conflict with VRC6, VRC7, and 5B, I think. It seems writing those addresses would corrupt your memory unless you're careful to make sure your code isn't stored there, or perhaps we should suggest that players disallow FDS-RAM writes at these addresses for enabled expansions.



I think a good practice for resolving write conflicts could be done fairly easily by having a standardized order in which to issue writes to the various expansions, like, off the top of my head, maybe N163 VRC7 5B VRC6 MMC5 APU FDS, where each enabled layer is allowed to take ownership of the write or pass it to the next layer in turn. Alternatively, perhaps it should just be suggested that mirrors not be supported by NSF players (which also conveniently avoids conflicts). - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 02:28, 1 April 2012 (PDT) 

Another thought: it's not necessarily correct to insist that VRC6-VRC7 should conflict, for instance, since the NSF spec defines a mapper that does not include the mirrored registers. I don't believe there are existing NSF rips that rely on these mirrors, and they aren't really supported by playeres anyway. If you were to attempt a mult-expansion hardware solution (e.g. the TNS-HFE4), it's really up to you as the hardware designer to decide how to resolve these conflicts (or not resolve them, if the solution is too difficult and warn the user). For the most part, the various combinations of hardware seem to work pretty well in NSF players; it doesn't seem necessary to suggest that there _should_ be a conflict. FDS, may be a problem though; I suspect some players have a potential memory corruption problem when using FDS with VRC6/VRC7/5B. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 02:54, 1 April 2012 (PDT) 

    I was going from the perspective of soldering all the synths to a circuit board and putting that board in a Famicom's cart slot. Your "how to avoid the conflict" and "practice for resolving conflicts" would be implemented in a decoder circuit. This way, people building these hardware solutions will know what behavior to implement, and people building NSF players will know what behavior to emulate. If you've been making 5B tests, you might want to convert them to .nes format and post them to the BBS so that people who have made gimmicky devcarts can verify these hypotheses. --[Tepples](User_Tepples.xhtml "User:Tepples") 05:42, 1 April 2012 (PDT)

    

    I do have a small test NSF/NES I could share. Nobody with hardware to test with responded to my thread; interested people I had shared it with directly, but I will post it to the BBS as well. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 13:15, 1 April 2012 (PDT)

    Short suggestion: support for multiple APU instances. :) --[Zepper](User_Zepper.xhtml "User:Zepper") 08:07, 1 April 2012 (PDT) 

    That'd depend on figuring out how to make a crossover cable between two systems' controller ports. Make a proof of concept (in .nes format) of keeping two consoles synchronized in this manner, and I'll consider documenting. Likewise with using a Game Link cable between an NES and a Game Boy. --[Tepples](User_Tepples.xhtml "User:Tepples") 10:43, 1 April 2012 (PDT)

    

    I'd save a multi APU suggestion for NSF2 (and I think it's about time to get the ball rolling on NSF2). I'm not trying to rewrite the NSF format here, I just want to clarify what is commonly practiced and what isn't. Multi-expansions of all combinations are widely allowed by NSF players, but there may be some variation in the details. In this area I would like to avoid "must" suggestions unless they agree with practice (avoiding mirror registers appears to be a must). If we suddenly start telling people that FDS-RAM in 8000-DFFF must be disabled for multi expansions, there will be corruption in some existing players. I'd rather provide a guide that helps people make their multi-expansion NSFs in a safe/robust way. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 12:13, 1 April 2012 (PDT)

I've edited the section to keep the conflict notes you added, Tepples, but I've tried to reorganize how the suggestions are made. I haven't seen any software NSF players that have any problem with the mirror conflicts (probably because, as I said, the mirrors aren't generally implemented). So for the most part, this is a problem for someone trying to build a hardware multi-chip solution to solve, and that's how I think the suggestion should be made. (In the notes I found for the TNS-HFE4 it looks like they did not sort out the VRC6/VRC7 conflict.) The FDS memory problem I suspect has been an issue but due to some luck nobody has noticed yet. There's only one suggestion I can think of that would be safe (i.e. avoid placing code/data at these addresses). Since this is inconvenient, and probably won't be followed anyway, it's probably prudent for the player to prevent this. The least-impact way to resolve it I think it just to block memory writes at the specified audio registers (which I will do in my next version of NSFPlay), but a complete disable of the write behaviour in that area might be easier to do for a hardware solution. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 13:10, 1 April 2012 (PDT) 

## Consistency

I converted all 0XXXXh hex numbers to $ for consistency. Since all the references to the file header were of the form 0XXh I kept the extra 0 just to make it easier to spot the difference between header addresses and 8 bit hex numbers. I also converted *starred* words to _italic_ , but some of them were then changed to ALLCAPS. I don't care really what style is used, but I'd like it to be consistent. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 13:20, 1 April 2012 (PDT) 

## More clarification

Is the bank switching registers supposed to be write-only? Exactly how much stack space, clock cycles, etc are assumed to be reserved for the player? What assumptions can be made about the stack pointer register? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 11:05, 1 September 2012 (MDT) 

    I believe that: 1- NSFs are made assuming that the bankswitching registers are write-only, but whether they are or not is immaterial. 2- The NSF spec doesn't allow sharing any resources, i.e. it doesn't leave any space in the 2KB RAM for other programs, even if most NSFs don't use all of it. 3- most NSFs initialize their own stack pointer 4- NSFs *should* be capable of running on actual NES hardware for the refresh rate specified (i.e. 60Hz → 30KCy/callback max) but several (earlier) software NSF players didn't enforce this constraint. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 14:23, 1 September 2012 (MDT) 

    My reasons for asking some of these things (such as if bank-switching registers are write-only) are also because I wanted to write a program to combine multiple NSFs into one NSF file, although it is difficult. In some cases it could be simple (such as, no bankswitching, no FDS), although may still need to do some other things, such as selecting empty RAM/ROM/etc. A more complicated way would be to trace the program execution and use that to relocate banks and so on. Is the _play_ address allowed to be RAM (such as outside of the $8000-$FFFF range)? Another thing I was interested in and asking these questions for is a somewhat unrelated thing: I was writing [my own design for hardware NSF](User_Zzo38_Hardware_NSF.xhtml "User:Zzo38/Hardware NSF"). Currently it can use any playback rate although it won't work if the NSF program sets its own stack pointer. Other than that it looks (to me) like it is following the .NSF specification. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 16:42, 1 September 2012 (MDT) 

    I don't think there's any restrictions on the PLAY and INIT pointers; LOAD is apparently supposed to point to nothing below $8000 for [hysterical raisins](http://www.catb.org/jargon/html/H/hysterical-reasons.html) (and it obviously needs to point to initialized memory).—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 18:00, 1 September 2012 (MDT)

    

    

    

    There are a few players (e.g. NotSoFatso) that don't allow LOAD below $8000, but it makes little sense not to support it if you're supporting the FDS (though there is a workaround using bankswitching). Bankswitch registers should probably be read+write but I haven't yet noticed any NSFs trying to read them (what use would this have?). I don't think it's true that most NSFs initialize the stack pointer, but they don't seem to be forbidden to do so either. PowerPak's NSF player actually uses the top 16 bytes of the stack for its own variables, and I would say it plays the majority of NSFs perfectly fine like this. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 23:13, 1 September 2012 (MDT) 

    Either way it does not make sense for LOAD or INIT to be below $6000 (even if FDS is used); but PLAY should be allowed to be as low as $0000. However like I have mentioned above I do have a potential reason for a NSF to read the bankswitch registers. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 13:09, 2 September 2012 (MDT)

    

    

    

    

    

    LOAD should be at $6000 for an FDS rip that has music code/data in that location (e.g. Mr. Gold). Below $6000 there's no point, really, but I do think it's sensible to support this for FDS' sake. It could theoretically have been relevant for FME-7 as well, but I don't think any FME-7 games had music data at that location, so it doesn't really matter. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 16:09, 2 September 2012 (MDT)

    

    

    

    Actually, since INIT and PLAY must end with RTS, it's not really acceptable for either to set the stack pointer. I think this directly implies that the stack pointer should not be initialized by the NSF. Probably we should make this clear in the NSF article. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 23:17, 1 September 2012 (MDT) 

    I agree; why would anything work if the stack pointer were initialized by the NSF? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 13:09, 2 September 2012 (MDT)

    

    

    

    One of the items in the "caveat" section suggested an NSF should not try to store variables in $1F0-1FF, which is consistent with how the PowerPak uses its stack. I would like to ask, are you aware of any NSFs that attempt to initialize the stack pointer in INIT? It seems like this wouldn't run at all on a lot of players... - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 23:30, 1 September 2012 (MDT) 

    My own hardware NSF (not yet implented) uses only three bytes of stack, and only uses a single byte of RAM for its own variables, which is part of the cartridge and is not available while the NSF is playing. It also uses 30 clock cycles to call the PLAY routine; I thought it should be less but I don't know how. I would like to see the clock and memory information for other hardware NSF. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 13:09, 2 September 2012 (MDT)

    

    

    

    

    

    PowerPak's NSF player source is at loopy's site: <http://home.comcast.net/~olimar/NES/>, though it does not include source for the NSF mapper. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 16:10, 2 September 2012 (MDT)

## IRQs and hardware NSF player

Rainwarrior, in his most recent edit, said 

> The reason given for no IRQ is probably false (I don't know if there was ever a hardware player that used IRQ to drive PLAY?)

If nothing else, kevtris's [HardNES](http://kevtris.org/Projects/hardnes/) uses IRQs to drive play. (at least I'm pretty sure it's IRQs and not NMIs) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 23:12, 16 February 2019 (MST) 

    I knew about the existence of the HardNES, but I don't really know any details that aren't on that page. Do you have another source of information for it? (Or does it say on that page and I'm just missing it?) Though really, even if it was, I don't think a one-off closed-spec piece of hardware like this needs to be accommodated by the article. If it was IRQ driven and that was publicly known, that might offer a little bit of perspective as to why rippers collectively eschewed IRQs in NSF rips. I went digging through the forums (and the _old_ forums) to try and find anything relevant, but mostly I just came up with people (disch, blargg, memblers, quietust, gil-galad) mentioning that many NSF player programs didn't have IRQ support. The statement about hardware players that I removed was something I'd added to this page in 2012 [[1]](https://wiki.nesdev.org/w/index.php?title=NSF&diff=3394&oldid=3393) so my edit was trying to amend what may have been overreaching or assuming on my part. I think I understood that IRQ was forbidden, but I'm not confident I understood why when I made that change. I think I did believe HardNES was using IRQ, but I don't know that I had any actual basis for it. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 02:53, 17 February 2019 (MST)
