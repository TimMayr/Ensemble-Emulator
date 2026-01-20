# Talk:PPU registers

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3APPU_registers) | View [other pages](Special_AllPages.xhtml#Talk_PPU_registers)

## Contents

  * 1 Naming the registers
  * 2 Palette
  * 3 Emphasis bits
  * 4 Allcaps vblank?
  * 5 Immediately?
  * 6 Sprite zero hit, sprite overflow bit



## Naming the registers

Am I the only one being annoyed by those "PPUMASK" "PPUSCROLL" etc... names ? Because Mr Tepples often refers them as it doesn't mean everyone often refers them as it. I like to use plain $2000, $2005, etc... Personally I'd vote for removing references to those names from the Wiki but I don't want to force it if other people disagree. Bregalad 21:29, 24 February 2010 (UTC) 

* * *

It depends. It's not only Tepples that promoted those names. I'm a firm believer that using symbolic constants makes the code easier to read. As long both are available in the wiki I don't see the issue. 

I can give you an example as code. Which one is clearer when you read back the code? 

This:  

    
    
    lda #%1010000
    sta $2000
    

Or: 
    
    
    ; In another file (nes.h)
    PPU_CTRL_NMI %10000000
    PPU_CTRL_SPRITE8x16 %00100000
    PPU_CTRL $2000
    
    ...
    
    ; Somewhere in the code
    lda #PPU_CTRL_NMI + PPU_CTRL_SPRITE8x16
    sta PPU_CTRL
    

Once you know the convention, it makes the code easier to read. Of course for the registers only it could be argued for a while since there is not that much on the nes but it's always good to follow good programming practice. In a professional environment, I will always promote the second once since it makes the code clearer. 

You don't know how many time I saw code samples for newb with no comments at all and you have to figure out what the hell was done with anonymous labels to make it worst. At the least those constants give some visual feedback on what you're trying to do. It's only a matter of getting used to the naming convention. 

In brief, I think it's a good practice to use them but nobody is forcing you do to so. There's just there to try to make a convention and of course not everyone will agree with it (i.e. see how many linux distro..) 

[Banshaku](User_Banshaku.xhtml "User:Banshaku") 01:35, 25 February 2010 (UTC) 

* * *

Well it's fun because I always do it like your "bad example" exept I write numbers in hex instead of binary, and I never had problems reading it. Symbolic names just makes the thing longer and more confusing. In fact I think it is the exact opposite of what you said - it makes things harder to read in my opinion. Then it depends if you remember best words or numbers - I know I remember numbers better personally. 

Like you said, as long as there is both there isn't any problem... but my issue is that I found several pages on the Wiki which ONLY refers the symbolic names which are nothing official anyways. So my proposal would be to not touch this page, but change the others to mirror the numbers (e.g. $2001) instead of symbolic names that makes few sense (PPUMASK really makes no sense to me it sounds like carnival or something). 

Again I don't want to force anything but I'd vote for at least replacing all instances which ONLY have the symbolic names to show the true register instead. Bregalad 08:20, 25 February 2010 (UTC) 

* * *

I was not aware that some page of the wiki only refer to the name. In that case, yes it can be confusing since there is no official standard for naming them. To make both crowd happy, we could always put both at the same time too. 

Which pages are like that? 

As for using constants For the registers, the benefit is quite negligible since there is not that much but for flags or your own values in your game, it will save you from many headache when you use the same value many time. Of course, if you use long name and you have to remember all of them by heart, everybody will agree with you and say it's quite a pain in the butt (I do agree) but with a good editor, the editor will give you suggestion once you start to type a word. For example, in Visual Studio, Eclipse and many IDE, if I would type PPU, it would give me a suggestion list of all the words that start with PPU. In the case of nes programming this is an issue because there is no such editor that gives that functionality. I wanted to make one for that reason since I cannot remember all my constants, which is normal. 

[Banshaku](User_Banshaku.xhtml "User:Banshaku") 12:05, 25 February 2010 (UTC) 

* * *

    See [GBATEK](http://problemkaputt.de/gbatek.htm) for the kind of style that I was attempting to follow when I instituted the PPU registers' names in the first place. The GBA community never refers to registers by their address. --[Tepples](User_Tepples.xhtml "User:Tepples") 12:55, 25 February 2010 (UTC)

* * *

Tepples there is a big difference : The NES adresses are 16-bit and so you only have 4 digits per adress, which makes them much easier to remember. Another big differences is that all those names are made up - they are nothing official and probably aren't "often refereed as" as this page says Ian Bell uses the following definitions in his open source "tank demo" so if anything were to be official it'd look more like this (and again I doubt it's official it's probably made up by Ian Bell himself) : 
    
    
    VCR:		EQU	$2000	; video control reg base address
    VIDEO0:		EQU	VCR+000	; CTRL0
    VIDEO1:		EQU	VCR+001	; CTRL1
    VSTAT:		EQU	VCR+002	; video general status register
    OAM_ADR:		EQU	VCR+003 ; sprite attribute address register
    SCROLL:		EQU	VCR+005 ; scroll h/v registers appear here
    VRAM_ADR:		EQU	VCR+006	; video address register
    VRAM_DAT:		EQU	VCR+007	; video data register
    
    SPRITE_DMA_ADR	EQU	$4014
    WRST		EQU	$4015	; DMA WRST/RDST
    
    CONTROLLER1:	EQU	$4016		; Joystick and DMA
    CONTROLLER2:	EQU	$4017		; Ports
    

So according to what Banshaku said, I guess I can go ahead and remove places where registers are refereed only by their GBA-style name and replace them with true adreses. If this cause problems someone can still undo the changes. \--Bregalad 15:10, 25 February 2010 (UTC) 

* * *

    Game Boy addresses are 16-bit, yet the same fellow behind GBATEK also wrote [Pan Docs](http://problemkaputt.de/pandocs.htm). --[Tepples](User_Tepples.xhtml "User:Tepples") 16:02, 25 February 2010 (UTC)

* * *

We should just be careful thought. It's not because there is no official naming convention coming from Nintendo that we cannot make our own in the first place. If we go that extreme, we could say that we don't agree with Loopy_V/Loopy_T naming convention because that is not official too. 

There is nothing wrong to try to make a naming convention that seems simple for new users. The one shown above is not any simpler the the currently proposed one on the wiki. It's all a mater of preference. Don't forget that people have complained many time that the wiki was "not organized" so we shouldn't shoot down people that try new ideas. There is nothing wrong we that. You may have missed that Ddribin did some example in it's own sandbox for some possible formatting and naming convention [here](User_Ddribin_PPU_Sandbox.xhtml "User:Ddribin/PPU Sandbox"). I thought his idea was interesting and it passed under the radar of everyone. 

[Banshaku](User_Banshaku.xhtml "User:Banshaku") 23:46, 25 February 2010 (UTC) 

* * *

I just noticed this mass-removal of named registers and it's ridiculous. The point of this Wiki is to make thigs clearer. Using numeric addresses everywhere adds needless detail and harms its primary purposes of educating NES programmers. 

The point of using an assembler is to work at a higher level than machine code. The assembler allows you to use the names of instructions rather than their op codes. Likewise, it allows one to give addresses names and work with them. Labels in programs are one example, and named constants are another. If one really believed that using hexadecimal values instead of named registers was good, one would have to argue the same for labels and instructions, and thus use only .byte directives in the assembler. 

New NES programmers have plenty to remember already. Being able to say PPUSTATUS instead of $2002 lightens their mental load, and reminds them of the register's purpose wherever it's used, without having to look anything up. Even an experienced NES programmer who takes a break will have to re-learn the values to some extent, and make more errors during the process. 

All NES programmers make typos when programming. Making a typo on PPUSTATUS will result in an assembler error in almost all cases, rather than a program that misbehaves. Making a typo on $2002 can easily result in a program that assembles fine but fails to work, without any way to easily find the typo. 

When a programmer is reading source, addresses like $2002, $2003, $2005 all tend to look somewhat alike, while PPUSTATUS, SPRADDR, and PPUSCROLL are more distinct, and can be set up as keywords for the syntax highlighter. 

If one is used to using numeric addresses, he will have to learn the names for them, but this has none of the disadvantages listed above. The names are based on the function, rather than arbitrary numbers. And misremembering a name will be caught almost immediately. I can't help but think that the desire for removal of names is to avoid having to learn them, or make up for deficient tools or something. 

[Blargg](User_Blargg.xhtml "User:Blargg") 22:18, 28 February 2010 (UTC) 

* * *

It's a battle of the B's, and I'm still the odd one out even after buying a copy of Bee-52 :-) 

Martin Korth uses purported official names in GBATEK and Pan Docs, but he appears not to use names in [Everynes](http://problemkaputt.de/everynes.htm#iomap). My early GBA header files used different names from the names in GBATEK. Part of this was to be different on purpose to avoid appearing to support the use of unlawfully acquired trade secrets. What are "official" names other than names in leaked Nintendo manuals? The other part was to take advantage of the various arrays implicit in the GBA register map. In fact, some elements of my naming scheme, particularly with respect to these arrays, eventually made it into devkitPro's libgba header file, which otherwise follows GBATEK. (See for example BGCTRL[].) 

So, Bregalad, do you object to the idea of having the community invent a consistent set of names, or do you just object to the specific names I invented? --[Tepples](User_Tepples.xhtml "User:Tepples") 02:44, 1 March 2010 (UTC) 

* * *

I tend to agree to what Blargg said. Making a typo on a name will make the compiler fail but a typo on an address won't. By using constant you avoid that possible issue. 

For beginners, those constants are a must. How many times when we look at the code sample they send that we can be sure that they really meant $200X and it was not a typo? With the constant, it makes it plain obvious and easier to correct them. 

I think it will be hard to make everyone happy in that case. For now, we only have one person objecting to this naming convention. I just checked the updated page and since I didn't program the nes for a while it didn't make any more sense to me anymore. When you have to juggle with many platforms during the day, constants and the proper tools the way to go. If I could find an editor that do some intellisense with my own defined constant I would be happy. I need to check if Notepad++ can do it, that would be great. 

[Banshaku](User_Banshaku.xhtml "User:Banshaku") 03:57, 1 March 2010 (UTC) 

* * *

Well I don't know. You guys says a newbie already have a lot of things to learn, but learning those confusing names just sounds like one more thing to learn to me. Remembering "PPUSTATUS" you have to remember 9 letters instead of 4 numbers in $2002 which is more complicated (you really have to remember 1 number, because the first 3 are always $200x). I also disagree about the typo things, you can't really do a typo when typing "$2002" without noting it immediately, however you can very easily do typos with words. 

You are right tough about that there is no reason the community can't made up names - and I wasn't intending we should replace those names by more "official" names, just using the register is fine for me. My references to Ian Bell's code (which isn't really "leaked" since he did give it himself on purpose to the community) was misplaced - I didn't intend to use them. 

About the argument that it would be less confusing when switching to from a system to another, I disagree. Every system has other registers anyways so it wouldn't change a thing. 

I just think most of those name didn't make sense to me and are too long to be remembered, but I don't really care if you all like them go away and use them (and undo my modifications), I'll just not use them in my code and anyone is free to use them. It's just that I figured those names caused me trouble as I always had to had the registers page open in another tab to convert from those names to actual registers.Bregalad 07:57, 1 March 2010 (UTC) 

* * *

\- It's not ridiculous. I don't play very often with NES assemblers, but I suppose that those labels should be pre-defined in an header file perhaps? Really, it's pointless. The $2002 means PPUSTATUS; fine. Is this more educational than $2002? Do you know about ppu address mirroring, of address AND 7, just for the case? 

\- Personally, I prefer the numbered registers instead of labeled ones. If it's not part of a compiler, it's up to the user to define a couple of labels. Yup, it avoids errors, but I _really_ dunno you would do such mistake; probably, your algorithm or opcode usage has more chances to spot an error than a $200X "whoops". Zepper, 1 March 2010 

* * *

  
As for number of characters to remember: "PPU" is a concept, and "status" is a concept, and humans are better at remembering concepts. It's easier to remember a name like "Jennifer" than a phone number like "903-5768". Otherwise, why do we even use assemblers at all? "LDA #" has more characters than "A9". 

Different people have historically used different names. What MOS Technology, its second sources, and ARM call "EOR" everyone else calls "XOR". Likewise, Sony's highly-6502-inspired SPC700 CPU uses different mnemonics even for instructions that by rights should be the same. "MOV A,X" could have been "TAX" for familiarity to people who have programmed the NES and the other half of the Super NES. 

Bare addresses have different purposes based on what bus they're on. Is $2000 the first PPU control register, or is it the start of the first nametable? 

Register address mirroring is probably more useful to emulator developers and people trying to obfuscate or digitally fingerprint code than to developers of original NES software. 

Yes, the constants should be predefined in a header file, and the header file should be on this wiki. I fully intend to make such a header file at least for ca65 once we standardize the names and other parts of the style guide. 

\--[Tepples](User_Tepples.xhtml "User:Tepples") 17:07, 1 March 2010 (UTC) 

* * *

You are right that concepts are easier to remember than numbers. However you say that if I don't use the labels you made up to assign registers, I might as well not use an assembler, which is absolutely not what I meant. I am not against labels as a whole - I am just against the fact that some labels one guy made up were used on this wiki instead of the real addresses of the register without any reference to the real address next to it, and that the name of the labels themselves didn't make much sense to me. 

I completely disagree that this should avoid errors. Since I started coding for the 6502 I've had to deal with MANY typo errors, but they were always for other reasons (misspelling of labels, bad use of anonymous labels and confusion betwen X and Y should hold the top tree (X and Y are one next to the others here, unlike english keyboards)). So if you really want to use the labels on the wiki go ahead, but then also refer to the register name, or at lest link to it, so that people who don't make any sense from these names can have an idea what you are talking about.Bregalad 17:48, 1 March 2010 (UTC) 

* * *

The comment about not using an assembler at all was a bit of [hyperbole](https://en.wikipedia.org/wiki/hyperbole "wikipedia:hyperbole"). I have a disability that sometimes makes it difficult for me to discern the line between what is acceptable and what is not. As a coping mechanism, I sometimes use hyperbole to establish lower and upper bounds for the discussion. But if the solution is to link to the article defining the labels from every other article that uses the labels, I have no problem with that. Redirects would help with that; it'd be as easy as, say, `[[PPUSTATUS]]`. --[Tepples](User_Tepples.xhtml "User:Tepples") 18:44, 1 March 2010 (UTC) 

* * *

\- Quick question: perhaps other people want to debug or trace your code using a disassembler. Does it display $2002 or PPUSTATUS? ;) So, it's something fully optional, including empirical labels for such registers. Well, regarding $2000, it was silly. We have CPU and PPU addressing the things! How would you differ the labels? \--[Zepper](User_Zepper.xhtml "User:Zepper") 21:25, 1 March 2010 (UTC) 

* * *

Quick answer: The disassembler displays whatever label set you have loaded in. It starts with just the 30 registers of the NES, but as I learn more about how a game works, the label set fills up with memory locations and subroutine entry points. If I really wanted to thwart disassembler users, I could have the assembler generate 10 random bits and make them different for every access to the PPU registers. This brings up another idea: if we make a header file for ca65, we could also make a label file for FCEUX. --[Tepples](User_Tepples.xhtml "User:Tepples") 21:41, 1 March 2010 (UTC) 

* * *

Actually, I don't know any disassembler with such feature. Plus, there are lots of good h4ck3r5 that uses only an hexa editor. ^_^;; But perhaps it's another story. --[Zepper](User_Zepper.xhtml "User:Zepper") 21:56, 1 March 2010 (UTC) 

* * *

Here's an example of what I'm using in all my nes project: [CA65 constants](User_Banshaku_CA65_Constants.xhtml "User:Banshaku/CA65 Constants"). Yes, the name are verbose because I'm trying to find a naming convention that make sense. I'm still in a trial and error process. I always had that file open when I need to use them but I found out that with Notepad++ you can create a list of word that can work like intelissense in Visual studio. This mean I won't need to remember them anymore once I create that file for my editor. 

[Banshaku](User_Banshaku.xhtml "User:Banshaku") 23:34, 1 March 2010 (UTC) 

* * *

The debugger in FCEUXD (and subsequent derivatives like FCEUXD SP and FCEUX) has such feature: [NL files](http://fceux.com/web/fceux-2.0.2.htm?%7B8A78E5FE-C7EB-418D-A921-F9A6782663F0%7D.htm). --[Tepples](User_Tepples.xhtml "User:Tepples") 17:16, 2 March 2010 (UTC) 

* * *

Say someone discovers a new CPU architecture, but no official document about its instruction set was disclosed to the public. Examples include the microcontroller in the NES and SNES CIC and the CPU in the Pokemon mini video game system. What mnemonics should appear in documents about the CPU designed for an emulator author or in a debugging emulator's disassembler? --[Tepples](User_Tepples.xhtml "User:Tepples") 21:21, 4 March 2010 (UTC) 

* * *

Given all I said above, I have to say that the current names (PPUXX) are not that readable (to me at least), due to the repeated prefix. 
    
    
    PPUCTRL
    PPUADDR
    PPUDATA
    

Those just meld into a mess to my eyes. These are for some reason much more readable to me: 
    
    
    PPU_CTRL
    PPU_ADDR
    PPU_DATA
    

It may be that I'm partial to underscores, using them heavily in naming in all the languages I use. But I think visually it makes the beginning of the second part visually distinct so the eye can quickly make out the shapes of the beginning. Lowercase might of course be better as well, but I think that lowercase isn't the best idea to use for registers, as the uppercase makes it clear in code that it's not just some variable. 

    Remembering "PPUSTATUS" you have to remember 9 letters instead of 4 numbers in $2002 which is more complicated (you really have to remember 1 number, because the first 3 are always $200x).

Memorizing PPUSTATUS is just memorizing PPU + STATUS, for a register that gives the current status of the PPU. Remembering $2002 is remembering the base address of the PPU ($2000, an arbitrary value) and an arbitrary offset (+2). And when you recall $2002, you can't be sure you remembered it correctly without consulting something. 

    I just think most of those name didn't make sense to me and are too long to be remembered, but I don't really care if you all like them go away and use them (and undo my modifications), I'll just not use them in my code and anyone is free to use them. It's just that I figured those names caused me trouble as I always had to had the registers page open in another tab to convert from those names to actual registers.
    [...]
    So if you really want to use the labels on the wiki go ahead, but then also refer to the register name, or at lest link to it, so that people who don't make any sense from these names can have an idea what you are talking about.

So you're saying you have trouble remembering the names of registers, but not their addresses? This is an odd argument, because the whole point I'm making is that new NES programmers have to remember arbitrary addresses, which is more difficult. It seems your whole argument comes down to "I've memorized the addresses, but not the names, and I don't want to memorize the names, therefore names are bad."

I won't avoid admitting that the biggest problem with names is agreeing on them and then using them consistently. You don't have that problem with addresses, save for choosing the base (hex, decimal, or perhaps binary heh). 

    I completely disagree that this should avoid errors. Since I started coding for the 6502 I've had to deal with MANY typo errors, but they were always for other reasons (misspelling of labels, bad use of anonymous labels and confusion betwen X and Y should hold the top tree (X and Y are one next to the others here, unlike english keyboards))

How do you know you haven't ever made a typo for a register address? The whole point is that there is no way to reliably diagnose them. 

    The comment about not using an assembler at all was a bit of hyperbole.

Actually, I meant it somewhat seriously. There are probably some old-time programmers who prefer to write machine code instead of assembler, because they find the mnemonics confusing and obfuscating of the actual opcodes. 

The main point was that the argument against symbolic names applies to mnemonics as well. In both cases, you don't see the actual value generated, just the name. The argument against symbolic names because they're confusing or hard to remember applies equally well to mnemonics. If the arguer is fine with mnemonics, it shows that he has not discovered his actual reasons for being against symbolic names. One likely explanation is that his argument is simply for things he's familiar with, and against things he's unfamiliar with. There's nothing wrong with that as a personal approach to things, but it's not suitable for community decisions. 

And in case it's not clear, the idea of using symbolic names here is that the page describing the register shows its numeric address, and perhaps a register summary as well. Any other page mentioning the register just uses its name, which is also a link to the page describing the register (and thus giving its numeric address). 

[Blargg](User_Blargg.xhtml "User:Blargg") 17:07, 6 March 2010 (UTC) 

* * *

As for the underscores, I always been using then in my [CA65 constants](User_Banshaku_CA65_Constants.xhtml "User:Banshaku/CA65 Constants") since when you uses all caps, you cannot distinguish were the words end. If I define variables, you can always do zpJoy1Status and you can read it quite easily. 

[Banshaku](User_Banshaku.xhtml "User:Banshaku") 02:36, 7 March 2010 (UTC) 

* * *

Well Blarg you're probably right, I remembered the address instead of the names just because that's what I used first. If the very first "hello world" program example I've used would have used the symbolic names instead, maybe things would be different. In all cases, I think it's good practice if any reference to a register is wiki-linked to the description to the said register - but I find it the most consusing where there is a symbolic name, no address and no wiki-link. Also I have to say it looks much better with underscores to me. Bregalad 19:12, 10 March 2010 (UTC) 

* * *

I too remember the NES addresses better than the symbolic names, because I also started without names. Symbolic names should be used throughout the Wiki, but it should still make it easy for those who learned without the names to understand. I agree that mentions of registers should have links to the pages describing them, and any other minor things to help. 

[Blargg](User_Blargg.xhtml "User:Blargg") 19:34, 11 March 2010 (UTC) 

* * *

I see we're coming to a consensus. I will see if I can find time and first change the name with underscores then revise the wiki so that all reference to them link back to the actual topic about it. 

Is there any name that may still be hard to understand? For example, I didn't know why PPU_MASK was used. Maybe it's something in English that I'm not used to hear. 

[Banshaku](User_Banshaku.xhtml "User:Banshaku") 01:04, 12 March 2010 (UTC) 

* * *

I understand the name because it masks out (hides) the left pixels, or non-red/non-green/non-blue/all colors, or entire layers. Since the functionality is enable, clip and color emphasis: I'm not certain what a better collective name is. 

[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 14:28, 12 March 2010 (UTC) 

* * *

Lidnariq was on the right track: see [wikipedia:Mask (computing)](https://en.wikipedia.org/wiki/Mask_\(computing\) "wikipedia:Mask \(computing\)"). When you mask off bits in the register, you mask off parts of the screen. When you turn on bit 0, it masks off bits 3-0 of each palette entry. And if bits 1-4 are the [gobo](https://en.wikipedia.org/wiki/Gobo_\(lighting\) "wikipedia:Gobo \(lighting\)"), bits 5-7 are the [gel](https://en.wikipedia.org/wiki/Color_gel "wikipedia:Color gel"). --[Tepples](User_Tepples.xhtml "User:Tepples") 14:52, 12 March 2010 (UTC) 

* * *

Well it still don't make sense to me. PPU is a integred ciruit in the NES, mask is something that is meant to be put on a head. "Video Control" or "Rendering Control", "Video Switches", or something like that would make more sense to me.Bregalad 11:07, 13 March 2010 (UTC) 

* * *

You have the same reaction as me because maybe we share something in common: we're French natives. Maybe mask is used in English in other ways that we are not used to hear in daily conversation. 

[Banshaku](User_Banshaku.xhtml "User:Banshaku") 12:54, 13 March 2010 (UTC) 

* * *

What we could do is use the name of each register's closest counterpart on a Nintendo platform for which the homebrew community has adopted "official" names. For example, on the Game Boy Advance: 

  * $2000 is [BG0CNT](http://problemkaputt.de/gbatek.htm#lcdiobgcontrol)
  * $2001 is [DISPCNT](http://problemkaputt.de/gbatek.htm#lcdiodisplaycontrol)
  * $2002 is [DISPSTAT](http://problemkaputt.de/gbatek.htm#lcdiointerruptsandstatus)
  * $2005 is [BG0HOFS and BG0VOFS](http://problemkaputt.de/gbatek.htm#lcdiobgscrolling)



But then it falls apart for $2003, $2004, $2006, and $2007. Unlike on the NES and Super NES but like on the Game Boy, the GBA's VRAM, OAM, and CGRAM are memory-mapped. \--[Tepples](User_Tepples.xhtml "User:Tepples") 16:19, 13 March 2010 (UTC) 

* * *

This naming convention makes almost no sense just by looking at it. The PPU_SOMETHING makes more sense already. It doesn't mean that it's official that the naming is better. 

[Banshaku](User_Banshaku.xhtml "User:Banshaku") 02:57, 14 March 2010 (UTC) 

I don't really have a problem with naming the registers, especially since I haven't seen any official sources come up with their own names for them. Also, Bregalad, it makes more sense to name $2001 PPUMASK (or some variation on that) if you think of masking in a broader sense. Like, one might mask their face by concealing it with a mask, but one might try to mask the foul odors in a room by putting in air fresheners, or mask their presence by camoflage. In this way, it becomes easier to consider that a programmer may mask or reveal sprites and the background by using the register at $2001. 

And nobody is forcing anybody to use the names Tepples has provided, or any names at all. 

\--[Doppel](https://www.nesdev.org/w/index.php?title=User:Doppel&action=edit&redlink=1 "User:Doppel \(page does not exist\)") 20:25, 22 September 2011 (UTC) 

* * *

Should mention that the overflow flag is cleared at the start of the pre-render line here. (Posting to get account activated.) -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 18:45, 15 March 2013 (MDT) 

## Palette

$3F00-$3FFF has no palette data. Well, I would understand "palette" as RGB entries, or similarly. What's stored in that space are color indexes, not palette data. --[Zepper](User_Zepper.xhtml "User:Zepper") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Zepper&action=edit&redlink=1 "User talk:Zepper \(page does not exist\)")) 18:21, 27 March 2013 (MDT) 

    Many other parts of the Wiki (e.g. [PPU_palettes](PPU_palettes.xhtml "PPU palettes") and the "palette" entry on [Glossary](Glossary.xhtml "Glossary")) use "palette" when referring to the $3F00-$3FFF range though, and switching to "color index" might be confusing. To me, a palette is just a set of colors, regardless of representation. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 00:33, 28 March 2013 (MDT) 

    I disagree. When you dump $3F00 region, there are no "colors" or palettes like in other systems, but a 256-bit number, or 32 palette indices. At anyway, that's me. :) --[Zepper](User_Zepper.xhtml "User:Zepper") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Zepper&action=edit&redlink=1 "User talk:Zepper \(page does not exist\)")) 18:20, 28 March 2013 (MDT) 

    There is no reason "palette" should imply "RGB". And they do contain HSL entries.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 18:51, 28 March 2013 (MDT) 

    One thing is the hardware palette. Another thing is the [colour look-up table](http://en.wikipedia.org/wiki/CLUT), that's what the NES does. --[Zepper](User_Zepper.xhtml "User:Zepper") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Zepper&action=edit&redlink=1 "User talk:Zepper \(page does not exist\)")) 20:24, 28 March 2013 (MDT) 

    You'll notice that this article explicitly refers to the CLUT as a palette. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 22:24, 28 March 2013 (MDT)

    I disagree. "Palette" means exactly a selection of colours. It does not mean "RGB data", not at all, unless that is the type of palette you are using. VGA has a 256-colour palette. EGA has a 16-colour palette. CGA has a 4-colour palette. NES has 8 4-colour palettes. [This palette](http://inseansopinion.files.wordpress.com/2010/04/palette.jpg) has 6 colours. The format of how a particular colour is specified is specific to the hardware, but ultimately the data is an index, yes, whether that index is RGB, RGBA, HSV, greyscale, or the NES' form. The larger selection of colours which the palette can be built from may also be called a palette (though for clarity's sake it's probably best not to call both the full gamut and the limited on-screen selection "palette" in the same sentence). What I am saying is that "palette" is the most precise, accurate, and common term to use for this. I think it is also a "color index", and a "color lookup table", but palette is the most useful thing to call it. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 22:20, 28 March 2013 (MDT) 

    All about "palette" I mean RGB entries OR **similarly**. When a _normal user_ dumps $3F00, he won't find colors (RGB or whatever format you wish), but just a lookup table, so that's what I meant and only that. Also, it **IS** a color lookup table, unless _Q says it isn't. AFAIK, there's no direct access to the _hardware colors_. --[Zepper](User_Zepper.xhtml "User:Zepper") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Zepper&action=edit&redlink=1 "User talk:Zepper \(page does not exist\)")) 14:21, 29 March 2013 (MDT) 

    It is a palette, of colors specified in voltage/phase format, not RGB. But it is a lookup table to select the color (in voltage/phase format) from the pattern table, nametable, and attribute table data, reading from the palette memory, and then output according to the voltage/phase (or using a further lookup table to a RGB palette stored in ROM, in the case of RGB Famicom). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 14:28, 29 March 2013 (MDT)
    What do you consider "similar" to RGB? What is "whatever format you wish"? What are "hardware colors"? (Also what is a "normal user" who is dumping PPU contents but neglected to learn what they contain?) I don't understand what you think is special about the NES' colour data format that isn't equally special about every other possible way to encode colours. There is no "hardware" colour table hiding in the PPU that this is an index to, it is just a bit-packed number that gets translated directly into the video signal. There is no RGB data in an NES, you would only find it in an emulator that uses an RGB lookup table to translate for RGB hardware. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:32, 5 April 2013 (MDT) 

    There's no RGB data in a frontloader or NES-101, but there is in 2C03-powered Famiclones manufactured by Sharp under license from Nintendo: [C1 NES TV](https://en.wikipedia.org/wiki/C1_NES_TV "wikipedia:C1 NES TV") and [Famicom Titler](https://en.wikipedia.org/wiki/Famicom_Titler "wikipedia:Famicom Titler"). In any case, there are two stages that could be described as "palette lookup": lookup in $3F00-$3F1F (under the program's control), and then lookup in either the NTSC signal generator or the RGB table (not under the program's control). --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 18:52, 5 April 2013 (MDT) 

    I think the point of pedantry is that _there is no "lookup" in the 2C02_. The value stored in $3Fxx _is_ the direct value written to the analog output stage, much as the values written to the 2600's COLUPx registers are, or the values written to the TG16's register at $FF:$0404 are the direct 9-bit values written to the video DACs. (Tangentially: are there a canonical set of register names for the TG16?) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:23, 5 April 2013 (MDT)

    

    

    

    

    

    Actually, the real point I was trying to make is that "palette" is the standard term for what this is (and has been on every single platform I've ever developed for), and it's also completely consistent with its dictionary definition. Claiming that it's different because it's not RGB, and should be called a "lookup table" instead will only serve to obfuscate its definition. I think Zepper is confused about what palette actually means, and is arguing to change the terminology in the article to fit his confusion. As for the VS/Sharp PPU variants, yes they may actually have a hardware RGB lookup table. I don't think it's useful to call the NTSC signal generator a "lookup", but if you want to make an academic argument that it does the same thing, sure. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 19:39, 5 April 2013 (MDT)

    

    

    

    

    

    

    [Edit conflict] It is a lookup. To continue the pedantry: In the 2C02, there's still a layer of digital "decoding" after $3Fxx lookup and before the output stage. A hue of 8 might be decoded to 00111111, 11110000, or 00000011 depending on the phase, a hue of 0 to 11111111, and a hue of D to 00000000. How do I give a citation in [Visual 2C02](Visual_2C02.xhtml "Visual 2C02")? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 20:16, 5 April 2013 (MDT)

    

    

    @Rainwarrior: too bad you have fired me with a bunch of "why this?", "why that"... a sign of lack of respect, unfortunately. I really refuse to clarify anything. --[Zepper](User_Zepper.xhtml "User:Zepper") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Zepper&action=edit&redlink=1 "User talk:Zepper \(page does not exist\)")) 11:22, 6 April 2013 (MDT)

    

    

    

    Then please clarify it to _me_. [Definition disputes derail discussions](http://c2.com/cgi/wiki?LaynesLaw "wiki:LaynesLaw"). --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 13:25, 6 April 2013 (MDT)

    

    

    

    

    This discussion is _about_ a definition. Ironically the stuff about what the hardware does internally was the more derailed part. My argument is that this data stored in the PPU is a palette, as this data directly specifies a "[selection of colours](http://en.wikipedia.org/wiki/Palette)" for the NES to use, and similar things have been called a palette on [all](http://en.wikipedia.org/wiki/List_of_videogame_console_palettes) [sorts](http://en.wikipedia.org/wiki/List_of_8-bit_computer_hardware_palettes) [of](http://en.wikipedia.org/wiki/List_of_16-bit_computer_hardware_palettes) other platforms. Zepper's argument is that it is not a palette, and all of my questions are asking what reason he thinks it should not be called so. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 19:00, 6 April 2013 (MDT)

    

    

    

    I asked a question about each thing I do not understand. If I did not respect you I would not ask. As I've stated, it is my strong opinion that "palette" is the correct word to use, and I think trying to avoid this term in favour of "lookup table" will make it harder to understand for others. Please do not confuse my opinion of how these terms should be used with my opinion of you. As I said earlier, palettes are more generally indices and lookup tables, and it is fine to mention that it is also these things, but this data is still a palette, and that word is the most useful one we can be using for it. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:41, 6 April 2013 (MDT)

## Emphasis bits

It says the emphasis bits are unused if the selected color is $xE and $xF. Is this true with RGB as well or only with composite? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 16:40, 23 June 2013 (MDT) 

    I PM'd Memblers to ask; he said that that behavior is only true for the 2C02/2C07; the 2C03,4,5 emphasis bits do not exclude those colors. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:08, 2 June 2016 (MDT)

## Allcaps vblank?

Just curious as to why vblank has been changed to VBLANK across the page? I understand allcaps register names as a convention for a defined constant, but I have no idea what this is supposed to signify for vblank. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:20, 17 March 2015 (MDT) 

    Just a guess, but it might be a progression from VBI (initialism for Vertical Blanking Interval) through VBL to vblank. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 15:57, 17 March 2015 (MDT)

    FWIW, it was [this](http://wiki.nesdev.org/w/index.php?title=PPU_registers&curid=11&diff=9726&oldid=9710) edit. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:22, 18 March 2015 (MDT)

## Immediately?

The PPUCTRL section contains this sentence: 

_To avoid this problem it is prudent to read $2002 immediately before writing $2000 to clear the vblank flag._

Which implies that all writes to PPUCTRL must look like this: 
    
    
    bit PPUSTATUS
    sta PPUCTRL
    

But is that really necessary? It seems like reading PPUSTATUS at any part in the NMI handler would be sufficient, so long as it happens before writing PPUCTRL. That's not "immediate" at all. 

The code below would work, no? 
    
    
    nmi_handler:
      bit PPUSTATUS ; read at the start of NMI
    
      ; ... do some nmi stuff ...
    
      lda #PPUCTRL_NMI_ON
      sta PPUCTRL ; write to PPUCTRL several hundred cycles after reading PPUSTATUS
    

[Pubby](https://www.nesdev.org/w/index.php?title=User:Pubby&action=edit&redlink=1 "User:Pubby \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Pubby&action=edit&redlink=1 "User talk:Pubby \(page does not exist\)")) 14:15, 2 June 2016 (MDT) 

    My reading on that section is that it's describing how to safely write to PPUCTRL _outside_ of the NMI handler. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:58, 2 June 2016 (MDT)

## Sprite zero hit, sprite overflow bit

Sprite zero hits are described both here and in [PPU_OAM#Sprite_zero_hits](PPU_OAM.xhtml#Sprite_zero_hits "PPU OAM"). This brings unnecessary confusion. 

Sprite overflow bit is explained in detail in the [PPU sprite evaluation](PPU_sprite_evaluation.xhtml "PPU sprite evaluation") page, howver from an NESDev (not NESemdev) point of view, this level of detail is not useful. My understanding is that this bit works as follows 

  * Cleared at the start of the frame
  * Stays clear as long as < 8 sprites are present on each line
  * Could be "randomly" set if >= 8 sprites are on a given line
  * Will reliably be set if the 9th sprite is immediately following the 8th in OAM
  * Once set, remains set until the end of the frame



Is this summary correct ? If so, it should be added either here on in the [PPU OAM](PPU_OAM.xhtml "PPU OAM") page. 

[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 10:55, 12 February 2025 (UTC) 
