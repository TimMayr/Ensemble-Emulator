# User talk:Zzo38/DotFami

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User_talk%3AZzo38/DotFami) | View [other pages](Special_AllPages.xhtml#User_talk_Zzo38_DotFami)

## Contents

  * 1 ROM size
  * 2 CPU mode
  * 3 VRC7 Audio
  * 4 Shortkana
  * 5 VT/OneBus Series Emulation Support
  * 6 Nova's Mapper
  * 7 Trainer



## ROM size

I saw this in Recent Changes, and it looks like you left only 8 bits for "bnk: Number of 8K ROM banks." Action 52 and Chinese Final Fantasy VII already don't fit, having 2048 KiB (256 banks) each. --[Tepples](User_Tepples.xhtml "User:Tepples") 18:30, 3 August 2012 (PDT) 

    And that is one reason why it is a draft (the other reason being that it is not complete). Thanks for telling me. Is 256 the maximum? (If so, the "`bnk`" can be one less than the number of 8K banks; if not, it can be expanded to a sixteen bit number of banks.) Other things that could be fixed is the "`cpu`" field for CPU flags; it could be entirely rewritten or expanded to sixteen bits whatever seems important; if you have ideas you can post. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 20:35, 3 August 2012 (PDT)

    

    By now, I'd probably just use a 32-bit integer for the number of bytes and be done with it. If I really wanted to pack it into 8 bits, I'd use a IEE754-like quarter-precision structure with a 2-or-3 bit significand and a 5-or-6 bit unsigned and non-offset exponent. Almost all—but not quite—NES roms use powers of two, and while the vast majority of things cataloged as **not** a power of two are complete garbage, there are a few that would require awkward padding without it. Of course, not having a power of two of data is already a strange concept, since at its most basic address lines are binary. (To answer your question: there are already 3 and 4MB PRG dumps known) --[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 01:15, 4 August 2012 (PDT)

    

    

    Thanks for information. (However note that this format is not only for existing ROM dumps but also for homebrew games; some of which might use their own mappers as long as their components do not differ too much from the existing ones.) Perhaps I should just make the "`bnk`" field to indicate the number of address lines like you suggested so it is 2 to the power of the number 0 to 255 as number of bytes (obviously this can result in extremely larger than anyone will ever need). Another possibility is to use sixteen bits for a number of banks (of some size); see what is preferred. CPU flag (not only for CPU, actually) could also be fixed, expanded to 16 bits if necessary, including also such things as the PPU differences that are switched in some VS Unisystem games and that the intensity bits have a different meaning in some systems. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 10:41, 4 August 2012 (PDT)

    

    

    

    Plenty of NES games don't use a power of 2. Particularly games that use CHR ROM will often be 3 or 5 times a power of 2, and there's one game that's 136 KiB (_[Morita Shogi](http://bootgod.dyndns.org:7777/profile.php?id=3479)_ : [SNROM](SxROM.xhtml "SNROM"), 128 KiB PRG ROM, 8 KiB CHR ROM). --[Tepples](User_Tepples.xhtml "User:Tepples") 10:55, 4 August 2012 (PDT)

    

    

    

    And here I was misreading your "number of 8kB blocks" meaning for each PRG and CHR. Yeah, the _total_ size is usually going to be 2ⁱ+2ⁿ; the exceptions I was mentioning above were ones where the size of PRG wasn't a power of two. --[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 11:17, 4 August 2012 (PDT)

    

    

    

    

    OK. Thank you. I could expand this field to sixteen bits, and then decide what to do with it (perhaps can be total number of 8K ROM banks, unless you have better idea). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 11:50, 4 August 2012 (PDT)

## CPU mode

What's your objective in supporting nonstandard clock frequencies? AFAIK, none of the famiclones support under- or over- clocking their NES-equivalents, so breaking hardware compatibility seems odd to me. --[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 11:17, 4 August 2012 (PDT) 

    OK, so there are none, I suppose there is no use, I can remove that. Thanks for notifying me; I think there is still more to fix about the CPU flags though (things to add, things to remove, things to change). --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 11:50, 4 August 2012 (PDT)

## VRC7 Audio

The VRC7 only existed in one hardware instantiation, with 6 channels and an admittedly unknown but nonetheless fixed group of built-in instruments; allowing these to be configured by the ROM image specifically allows the description of things that don't exist in reality. (And if you want that, you may as well define a cartridge with a full OPL2; and if you want _that_ , you may as well use [some random FM tracker](http://adlib.wave460.net/trackers.html).) 

I'm not actually clear where any of the mapper audio bits are referred to, come to think of it. They make it seem like this format is trying to be a replacement for both .NSF and .NES at the same time. 

In general, the mapper audio needs to be tightly coupled to the mapper hardware described anyway, otherwise there's no way to play it; it feels odd to call it out separately. --[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 19:12, 5 August 2012 (PDT) 

    I do not intend to include full OPL2 and all that stuff; I don't want too many things which are not used in existing Famicom cartridges (although some simple things might be added on, if they are both easy to emulate and easy to build hardware). I removed the number of channels, but you could change the instrument setting since "VRC7 built-in instruments are not the same as OPLL instruments" so obviously if you change the hardware then you can change the music (especially since register layout is the same anyways; so you could use OPLL but just not use the rhythm channels). For where it is mentioned: "Note that you can have multiple instances of each and that their parameters can differ as well as how the address lines are connected by using other mapper codes." The mapper codes that will tell them to be connected simply is not yet described in this document (but eventually will be). But still it is draft so things are added, removed, and changed, in order to make improvement. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 03:40, 6 August 2012 (PDT)

    

    If you want to allow the existence of the OPLL, which also has another different unique set of predefined instruments, you should probably define it as another mapper, or a variant of the VRC7 mapper. By putting the full instrument definitions as bundled-with but not configurable-by the ROM image, you both prevent an emulator from providing the most accurate emulation because the included numbers were (by some definition) wrong, and if the numbers don't strictly correspond to either OPLL or the VRC7 you now also describe something that cannot exist in hardware. (I am defining FPGAs as "software emulation") --[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 11:54, 6 August 2012 (PDT)

## Shortkana

Please just use UTF8 (or UCS2,UTF16,UCS4, whatever). We do NOT need Yet Another encoding for printable characters—especially one based on something as hostile as CP437/CP850. --[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 14:45, 14 August 2012 (PDT) 

    This may be considered. Thanks for your suggestion. However: Why do you hate CP437? I happen to like CP437. I don't like Unicode that much. Translating a UTF-8 stream into numbers is not too difficult, although it still uses varying length and actually displaying Unicode text becomes much more difficult. Have you read about difficulty some people were having to make hardware .NSF player to display Unicode text? (not that DotFami is likely to be directly interpreted by hardware; it would be converted into the needed hardware format such as VHDL or whatever if you wanted to use DotFami with hardware) So possibly since .NSF can be, it is a possibility to use with .NSF for hardware .NSF players as well. CHR ROM could be implemented using this format not too difficult, you have 34 extra spaces too. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 17:19, 14 August 2012 (PDT)

    

    CP437's completely inadequate for anything other than English text ([wikipedia:Code_page_437#Internationalization](https://en.wikipedia.org/wiki/Code_page_437#Internationalization "wikipedia:Code page 437")). By using it as a starting point, and obliterating what might be CP850 characters with kana, you restrict the range of options to just US, UK and Japanese musicians. Plus, it's about equally correct to present (e.g.) Uematsu in romaji or kana; either way his name is actually 植松 伸夫. --[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 21:44, 14 August 2012 (PDT)

  


## VT/OneBus Series Emulation Support

I would love to see OneBus console support (VT02, VT03, etc), I may create a page on it for this reason. Ca4h3e had made a incomplete UNIF mapper for FCEUMM. 

I will post the link once it is done... --[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") 05:49, 15 August 2012 (PDT) 

    I don't know what that means, but yes do post it especially if there is some relation to DotFami or some way they can help each other. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 13:14, 15 August 2012 (PDT)

It is essentialy an enhanced NES with double the APU (if 2 APUs are used) and added 4BPP graphics, as well as DMA. But it is not very well emulated or documented, because the emulator is a modded NESTER, and it seems closed. 

It uses $2008-201F, as well as $4018-$402F? More details will tell if it is more than that... 

The Document (WIP, in rewrite mode.) is originally in 2 types of Chinese, as well as Engrish (English with lots of Translation Errors) But I understand it enough to rewrite it. 

I am still working on it, and will post it when the time comes! --[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") 22:39, 16 August 2012 (PDT) 

  


## Nova's Mapper

I read about [Nova's Mapper](http://pineight.com/mw/index.php?title=Nova%27s_Mapper) on the Pin Eight wiki. Perhaps it may be possible to implement using DotFami's `.cart` files? (But first needs completing more of the DotFami specification) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 13:41, 24 August 2012 (MDT) 

    That design looks interesting, but it needs a lot of cleaning up. First of all, the register layout could use a complete rework, and I'll run this by Nova this weekend. Second, there appear to be two binary adders in the chain from PPU A12-A8 to CHR A15-A8; would that pose a delay problem in CPLD implementation? --[Tepples](User_Tepples.xhtml "User:Tepples") 20:13, 24 August 2012 (MDT)
    I've rewritten it from the ground up, and he liked it. But he told me that MMC3 with a 32 KiB CHR RAM provides most of the same benefits while being already possible with NES 2.0 or with very minor changes to existing boards. --[Tepples](User_Tepples.xhtml "User:Tepples") 09:53, 26 August 2012 (MDT)

  


## Trainer

Here is an idea of loading trainer data which I think is compliant with both iNES and DotFami: 

  1. If battery RAM file exists, keep its contents.
  2. If trainer exists, write trainer into $7000-$71FF the same as the CPU would write there. Ignore any IRQ triggered while doing so.
  3. If battery RAM file exists, load battery RAM chip(s) with battery RAM data (directly, not through the CPU). (This may overwrite the trainer.)
  4. If CPU ever subsequently tries to read from $7000-$71FF and gets open bus, return the trainer data instead, if there is one.



\--[Zzo38](User_Zzo38.xhtml "User:Zzo38") 15:05, 29 September 2012 (MDT) 
