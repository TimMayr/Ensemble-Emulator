# Talk:NES 2.0

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANES_2.0) | View [other pages](Special_AllPages.xhtml#Talk_NES_2_0)

## Contents

  * 1 Nintendulator
  * 2 Zero CHR ROM
  * 3 RAM size (mappers with internal RAM)
  * 4 RAM size (behavior on boards without RAM)
  * 5 Vs PPUs and $2002
  * 6 Use of unused features
  * 7 Call it NES 2 file format?
  * 8 Additional problem case: Waveform playback IC
  * 9 Vs. system byte needs link
  * 10 Mirroring bits and one-screen mirroring
  * 11 Bytes 12/13 and new display requirements



## Nintendulator

    "Kevin Horton reports that Nintendulator supports NES 2.0."

No it doesn't - it **detects** them (and the mapper interface has room for them), but it doesn't actually read any of the fields. --[Quietust](User_Quietust.xhtml "User:Quietust") 01:58, 5 January 2010 (UTC) 

  * ...though _now_ it at least handles larger ROM images (including "Super 8-in-1 99 King Fighter (Unl).nes", which should actually be Mapper 45 with 1MB PRG and 2MB CHR, but with the 256KB CHR blocks 0/1/2/3/4/5/6/7 rearranged to be 0/2/3/1/4/5/6/7). --[Quietust](User_Quietust.xhtml "User:Quietust") 03:30, 5 January 2010 (UTC)



An observation: it would've made more sense for the TV system byte to use bit 0 to mean "Supports NTSC" and bit 1 to mean "Supports PAL", since it would've made it trivial to support Dendy timing by just adding another bit. --[Quietust](User_Quietust.xhtml "User:Quietust") 04:52, 14 January 2011 (UTC) 

## Zero CHR ROM

Observation: if a ROM is marked as NES 2.0 and has zero banks of CHR ROM, it must set the CHR RAM size field to a nonzero value (ideally 0x07, for 8KB non-batteried), otherwise it technically doesn't have anything at all connected to the PPU bus. --[Quietust](User_Quietust.xhtml "User:Quietust") 02:20, 26 August 2011 (UTC) 

    Good point. I'll mention 0x07 as the most common CHR RAM size in the article. But I do know of two ways to make a cart with no CHR ROM or CHR RAM. The Game Genie uses one way: wire CHR address lines to data lines through a mux to make 4x4 pixel squares. The other way, which I've talked about before on the forum but have never seen used in a game, involves always enabling CIRAM and wiring PA13 to CIRAM A10, which gives 1-screen mirroring (all banks at $400-$7FF) plus 64 tiles of CHR RAM (at $000-$1FF). --[Tepples](User_Tepples.xhtml "User:Tepples") 10:57, 26 August 2011 (UTC) 

    Just to close the loop- nocash built a game that used VRAM as CHRRAM: (<http://forums.nesdev.org/viewtopic.php?t=9342>) and allocated it to [iNES Mapper 218](INES_Mapper_218.xhtml "INES Mapper 218").—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:46, 13 March 2013 (MDT)
    I think it would makes sense that if there is neither CHR ROM nor CHR RAM and the mapper is one that normally has at least one of these, then it should just use CIRAM for pattern tables, as if CIRAM A10 is wired according to the mirroring header (and/or software-controlled), and CIRAM is always enabled. Should you specify this? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 22:29, 21 March 2013 (MDT)

## RAM size (mappers with internal RAM)

I'm torn about how to deal with RAM-inside-mappers. As far as I know, there are five ICs that have some: [MMC5](MMC5.xhtml "MMC5"), [MMC6](MMC6.xhtml "MMC6"), [X1-005](INES_Mapper_080.xhtml "INES Mapper 080"), [X1-017](Taito_X1_017.xhtml "INES Mapper 082"), and [Namco 163](INES_Mapper_019.xhtml "Namco 163"). 

Some of these are easy: for the MMC6, X1-005, and X1-017, their internal RAM is at the exclusion of external RAM. The RAM size byte can just hold $40, $01, $10, or (per rounding up) $70 as appropriate. 

For the MMC5, that RAM can be thought of as either of CPU or PPU memory, it's not clear whether it should be marked (and where it should be marked). Apparently no game ever battery-backed _only_ the 1k internal RAM, so perhaps leaving the RAM size byte at 0 for MMC5s without external RAM is best, even if it's contradictory to the advice for the previous three. Only 2k, 8k, and 32k RAMs were available during the MMC5's commercial life, and they were only ever combined as (none, 8+0, 8+8, 32+0); clearly adding 1024 and rounding up would cause nonintuitive values here. (As an aside, there's also no way to express the 2+8 or 8+32 configurations in NES2.0; fortunately neither were used commercially) 

The N163 came in 3 of the 4 possible variants: neither battery nor external RAM; battery but no external RAM; battery and external RAM. The first two would easily be $01 and $10, but the last worries me—adding internal+external and storing the rounded-up value in the "RAM size" means we'd have $80=16384 for 8192+128 battery-backed, or $60=4096 for 2048+128 battery-backed. This feels unintuitive to me. Simply always excluding internal RAM here (in addition to being contradictory to the advice given for the MMC6) means we'll have the nonsensical value 0 with the battery bit set in the rest of the header. Maybe the right answer is to only count the internal RAM when there's no external RAM? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:43, 10 February 2013 (MST) 

    We can add the [Datach system](INES_Mapper_157.xhtml "INES Mapper 157") to the list. It provides a 256 byte EEPROM that is not strictly speaking part of the game, but rather the cartridge converter, and thus is shared between games. Perhaps here the byte should indicate whether the game uses the EEPROM, rather than whether it's available. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 03:04, 14 August 2013 (MDT)

## RAM size (behavior on boards without RAM)

If the NES 2.0 header still default to 8k SRAM at $6000-$7FFF like the more conventional iNES header, even when the RAM size is explicitly set at $00, how do I simulate a cartridge without SRAM and I really want open bus at $6000-$7FFF ? I thought some games such as Low-G Man and Battletoads&Double Dragon relied on this behaviour ?![Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 00:13, 29 August 2017 (MDT) 

    If the header is NES 2.0 then there is no default, WRAM size is explicit. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 00:44, 29 August 2017 (MDT)

    Then is it fine if I remove the paragraph I reworded yesterday entirely, since that paragraph contradict your answer ?

[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 02:07, 29 August 2017 (MDT) 

    

    The only thing that contradicts it is the word you added, "defaulting". I think the purpose of the paragraph was to clarify that most mappers don't have any banking functions for PRG-RAM, and that adding any RAM to a mapper that doesn't already use $6000-7FFF for something should have the WRAM appear in that region. This has nothing to do with using 0 as a "default" value, though. 0 WRAM means 0 WRAM in NES 2.0. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 03:18, 29 August 2017 (MDT)

    

    

    Is [this version](http://wiki.nesdev.org/w/index.php?title=NES_2.0&diff=14064&oldid=14057) any better? I dunno, I find it hard to massage all the information in those two paragraphs into something clear and transparent. There's probably a better way to say the same stuff but with less, but I'm not prepared to do a more "global" edit on it at this time. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 03:36, 29 August 2017 (MDT)

    

    The original point of that section was 

  * If the known instantiations of the mapper don't allow for RAM
  * AND does not have a contradictory definition (such as a mapper register mapped over $6000-$7FFF)
  * THEN Specifying 2 or 8 KiB of RAM in the header means it's decoded at $6000-$7FFF using a 74'20 or something similar.


    When you added the word 'defaulting', you assumed it meant "should ignore the header value and always provide 8 KiB RAM" which is not what it said. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 09:43, 29 August 2017 (MDT)

Oh, ok, that paragraph was originaly VERY poorly worded because I completely misuderstood its meaning. Basically it's saying the details about a behaviour that anyone'd expect. PRG-RAM elsewhere than $6000-$7FFF is hardly ever heard of, as far as I know only the MMC5 can do such a thing. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 16:03, 30 August 2017 (MDT) 

    Is [this version](http://wiki.nesdev.org/w/index.php?title=NES_2.0&diff=14069&oldid=14064) any better? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 17:53, 30 August 2017 (MDT)

## Vs PPUs and $2002

"RC2C05-02 (with ID ([2002h] AND 3Fh)=3Dh)" \- How can that be? Does the RC2C05-02 not implement the sprite overflow flag? --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 21:30, 21 March 2013 (MDT) 

    "Mighty Bomb Jack" checks it right on power-up— `$8011: lda $2002; and #$3F; cmp #$3D; bne Reset`. But after startup (where it spins on $2002.7) and checking these bits here, it seems to never read from $2002 for anything other than the side effect of resetting the $2006 lo/hi toggle. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 17:41, 22 March 2013 (MDT)

## Use of unused features

I like the feature that for mappers that don't otherwise use the PRG RAM size header, it should specify 8K if it is mapped to $6000-$7FFF or zero if it is open bus or registers or whatever. However, I have some other suggestion: For CHR RAM, if it is a mapper with no CHR bankswitching and no other special use of the PPU address space, then if 16K CHR RAM (battery or non-battery) is specified, zero CHR ROM is specified, and four-screen mirroring is specified, to just map all the CHR RAM to the PPU address space. If a mapper that normally uses non-battery CHR RAM is specified, and the non-battery CHR RAM amount is set to zero but the battery CHR RAM amount is nonzero, then it should emulate it the same as that mapper normally does, but save the contents of the CHR RAM to a file. One other possible concern is, what will the trainer ROM do in mappers that already have something mapped to $7000-$71FF area? I suppose it could simply override it. Also, you say don't assign mapper numbers greater than 255 yet. However, if you have to define a mapper which depends on NES 2.0 features and that the older iNES format is insufficient, then you should assign numbers greater than 255. For example, I have worked on my own mappers and wish to assign them; the other mappers discussed in discrete logic mapper toolbox should also be assigned, and then it could be used to make up games using them on an emulator. (I could write the articles in my user space and they are moved once numbers are assigned.) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 00:59, 29 June 2013 (MDT) 

    NES 2.0 allows the description a lot of things that don't have preexisting meanings. For example, only MMC1 and MMC5 have well defined meanings for having both the "nonvolatile" and "static" PRG-RAM nybbles set, but they can be set in the header regardless. Simply deciding that if both nybbles are set to signify 4KiB that e.g. the battery-backed one comes first doesn't make this definition useful or worth the complexity, nevermind that it precludes correct behavior if a later discovery reveals the opposite behavior. To rapidly run through your suggestions: Providing general purpose memory from PPU $3000-$3EFF isn't useful, and 16KiB SRAMs have always been rare. I do agree that "just specifying battery-backed CHR-RAM should DTRT", but once again, it's waiting on an implementation. The "trainer" question is ancient, but has been mostly rendered moot by getting rid of all dumps that request trainers (In general: trainers override any other device mapped in that range, because they are assumed necessary for operation). Mapper numbers will be allocated when a hardware implementation exists. A general purpose mapper description language (as your m768 proposal) is not an obvious fit for the iNES file format. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 02:09, 29 June 2013 (MDT)

## Call it NES 2 file format?

To my eyes NES 2.0 sounds like a program along with its version number. Usually file formats don't undergo constant revision and thus don't have x.y version numbers. Maybe the name NES 2.0 has already stuck, but even NES 2 file format doesn't look much like a program and version number. I'm thinking of phrases like "YANES supports NES 2.0" where it might sound like it is compatible in some way with the 2.0 version of a program called NES. OTOH, NES 2 sounds like a second version of the NES hardware. Maybe there are plans for NES 2.1 etc. Anyway, just noting how the name sounds. [Blargg](User_Blargg.xhtml "User:Blargg") ([talk](User_talk_Blargg.xhtml "User talk:Blargg")) 01:09, 19 January 2014 (MST) 

    If I remember correctly, some publications referred to NES-101 (the 72-pin toploader) as "NES 2". As for the naming rationale, you could always get on [IRC](NESdev_IRC_channel.xhtml "IRC"), bug kevtris, and report your results here. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 08:07, 19 January 2014 (MST) 

    USB & SCSI come to mind... ~~50.170.133.216~~ 03:56, 29 August 2014 (MDT) Resigned with account... [alphamule](User_Alphamule.xhtml "User:Alphamule") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Alphamule&action=edit&redlink=1 "User talk:Alphamule \(page does not exist\)")) 13:53, 29 August 2014 (MDT)

## Additional problem case: Waveform playback IC

For [iNES Mapper 086](INES_Mapper_086.xhtml "INES Mapper 086"): "An additional set of sample files is needed to emulate the sound chip." When the .nes blob is meant to represent all the static data, having no provision for including some seems like an error. Likewise [iNES Mapper 018](INES_Mapper_018.xhtml "INES Mapper 018"). [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 19:11, 21 April 2015 (MDT) 

    I kind of agree, some sort of "extra data" blob might be nice. Perhaps all we'd need is a 32-bit size, and that number of bytes appended to the file. Though, in this specific case, even if we had the field, we don't have any dumped data to stick in there. The only working sound emulation attempt for that mapper involves having a pile of WAV files in a specific directory, which is obviously less than ideal. We don't have a dump of the sample ROM, let alone a working emulation of the sample playback algorithm used. There just isn't much interest in getting it done. Can you think of any other games that might need extra data? Requiring a companion-file or special solution for one special ROM isn't really that onerous, IMO, just not ideal. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 19:56, 21 April 2015 (MDT) 

    Also [iNES Mapper 072](INES_Mapper_072.xhtml "INES Mapper 072"), [iNES Mapper 092](INES_Mapper_092.xhtml "INES Mapper 092"), [iNES Mapper 003](CNROM.xhtml "INES Mapper 003")…so, several rather than few. [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 18:53, 2 July 2016 (MDT)
    More accurately, [eight](List_of_games_with_expansion_audio.xhtml#NEC_.C2.B5PD7756C_.28Jaleco.29 "List of games with expansion audio"). [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 19:16, 2 July 2016 (MDT)

## Vs. system byte needs link

> (See the "Vs. system byte" description below for a detailed analysis) 

This should probably link to [Byte 13 (Vs. hardware)](http://wiki.nesdev.org/w/index.php/NES_2.0#Byte_13_.28Vs._hardware.29)

[PatHawks](https://www.nesdev.org/w/index.php?title=User:PatHawks&action=edit&redlink=1 "User:PatHawks \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:PatHawks&action=edit&redlink=1 "User talk:PatHawks \(page does not exist\)")) 20:08, 3 August 2016 (MDT) 

## Mirroring bits and one-screen mirroring

Since (4-screen & Vertical) is currently held to be indistinct from (4screen & ~Vertical), wouldn't it be more useful to have it mean 1-screen? (Leaving aside Magic Floor's strange use; it's already got to be special-cased for how it uses CIRAM). [Myask](User_Myask.xhtml "User:Myask") ([talk](User_talk_Myask.xhtml "User talk:Myask")) 00:29, 23 August 2016 (MDT) 

    Link to recent forum post outlining my proposal for that: [post 177995](http://forums.nesdev.org/viewtopic.php?f=3&t=14725#p177995). Most mappers that do single screen only do that, I think BNROM vs AxROM is the big exception, but it just ended up becoming separate iNES 1 mappers anyway. In this case, it's being proposed because it's come up with a new [homebrew board](UNROM_512.xhtml "UNROM 512") that has 4 different permuations of solder pad settings (4-screen, 1-screen, H/V). So... it would solve the problem for this particular board, maybe for other future homebrew frankenboards too. I don't think there's much danger that some fifth mirroring mode will ever be significantly relevant, so I think it's fine as an iNES 2 extension to say that ....1..1 specifies some kind of "1-screen" configuration. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:50, 23 August 2016 (MDT)

    

    To follow up on this, the only currently relevant mapper for this is UNROM 512, and the first publicly released ROM (Black Box Challenge) used 1..1 as 4-screen. ([UNROM 512#Nametable Configuration](UNROM_512.xhtml#Nametable_Configuration "UNROM 512")) NewRisingSun's recent revision documents bit 3 as the presence or absence of extra memory for 4-screen. I don't know what I was thinking several years ago but I think the more viable minimum working definition here is just "if the mapper has a 1-screen or other arbitrary mirroring options, the values in these 2 bits are special", and in practice I think this may only apply to UNROM 512. (Will post in that thread linked above.) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:08, 6 January 2019 (MST)

    

    

    I was not aware of that UNROM 512 speciality when revising the NES 2.0 page. I would not mind using one of the combinations with the 4-Screen Bit set as a generalized One-Screen mirroring option, even for other mappers, allowing a One-Screen-Mirrored NROM clone. However, it irritates me that 1-Screen-Mirroring would be the one with Bit 0 clear and 4-Screen-Mirroring the one with Bit 0 set. I would have made it the other way round, sort of thinking that Bit 0 clear means that the original meaning of Bit 3 is not modified, hence 4-Screen, and Bit 0 set means that the original meaning of Bit 3 is modified to 1-Screen. But I suppose it's too late to flip those two around, is it? [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 15:06, 6 January 2019 (MST)

    

    

    

    I'm indifferent, so I'm not going to argue either way, but it would probably be best to continue the discussion [from here](https://forums.nesdev.org/viewtopic.php?p=231633#p231633) if you want to make a case for it. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 16:00, 6 January 2019 (MST)

## Bytes 12/13 and new display requirements

Since we've now discovered several games that require a different hardware palette—[[1]](https://forums.nesdev.org/viewtopic.php?f=3&t=16868)—do we want to explicitly include the VT01's dichromatic palette in the Vs. PPU nybble? Or does it instead belong in the "TV" byte? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 18:55, 22 January 2018 (MST) 

    I'm already writing up a proposal to denote non-standard palettes as well as other non-standard console behavior. Specifically, it turns out to be necessary to distinguish between VT02/VT03 (2 KiB RAM) and VT09+ (4 KiB RAM), as I have found VT02 carts that fail with 4 KiB RAM emulation, and it turns out that [there is another plug-and-play (of yet unspecified type)](http://s4.zetaboards.com/PGC_Forums/topic/30178284/1/) that writes to VT09+ register addresses but does not expect VT09+ behavior from that. I would also like to denote VS Dual System. I'll make a post later. Basically, what it's going to boil down is that when both Vs. and PC10 flags are set --- which right now has no meaningful interpretation ---, byte 13 should become an "other console hardware" byte. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 23:33, 22 January 2018 (MST)
