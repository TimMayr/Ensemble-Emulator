# Talk:INES

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES) | View [other pages](Special_AllPages.xhtml#Talk_INES)

How does a PlayChoice hint screen work? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 19:08, 21 September 2012 (MDT) 

## Last 4

Should this be "if the last _5_ bytes are not all zero"? "A general rule of thumb: if the last 4 bytes are not all zero, and the header is not marked for NES 2.0 format, an emulator should either mask off the upper 4 bits of the mapper number or simply refuse to load the ROM." -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 03:45, 14 July 2013 (MDT) 

    When I wrote 4 as the rule of thumb, I didn't know how many not-widely-adopted extensions to original iNES were floating around, and I guessed that one or more might have used byte 11 for something, but all of the ROMs I saw with DiskDude and similar tags had at least something in the last four bytes of the header. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 07:48, 14 July 2013 (MDT) 

    Makes sense. Just wanted to check whether it was a typo. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 09:06, 14 July 2013 (MDT)
    Nestopia checks the last 6, and clears/ignores the last 9. (It also actually does pay attention to bytes 8 and 9)—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:21, 14 July 2013 (MDT)
    If the header contains "DiskDude!" or one of those other things (I forget now, but I knew at one time and figured this out), then bit2 of flags 7 will be set; otherwise bit2 and bit3 should both be cleared. This may be another way. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 19:51, 20 August 2013 (MDT)

## 4-screen mirroring

Reference needed I'm afraid. 

"Theoretically bit 3 could be used for most mappers that had hard-wired mirroring to transparently provide 4KB of VRAM for 4-screen instead. However, many emulators will ignore this bit except for mappers with prior 4-screen variations."

I'm fairly confident most emulators will happily gives you 4-screen mirroring when asked for it, even if they are in mappers where such a possibility wasn't used. I remember at some point considering using 4-screen mirroring with CNROM and it worked well. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:55, 13 January 2019 (MST) 

    In my limited experience, most emulators will let you start with 4-screen layout, but if the mapper has a mirroring control register, using it will irrevocably remove the extra nametables. For example, this is what goes wrong when trying to run Vs. Goonies on FCEUX as mapper 75 instead of mapper 151. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 02:13, 13 January 2019 (MST)

    

    Ah ok. This is super weird, such a behaviour would be almost impossible to pull out in real hardware.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 02:20, 13 January 2019 (MST)

    

    

    As a curious aside, mappers [356](NES_2_0_Mapper_356.xhtml "NES 2.0 Mapper 356") and [512](NES_2_0_Mapper_512.xhtml "NES 2.0 Mapper 512") could do that if powered on in 4-screen mode. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 12:35, 13 January 2019 (MST)

    How do you think this should be worded? The point I'm trying to be careful about is basically that there are only two ROMs that have ever put this bit to actual meaningful use (4, 206). The rest is "theoretical" because nobody's released a game or test ROMs to verify the hundreds of other mappers that could maybe apply it. Because of that I have very low confidence that "most emulators" will get a completely untested feature correct. Testing this would actually be quite a big research project though. I am trying to keep this describing what exists, and not making it into a rule of "emulator authors should/must do it this way" that might not match reality and give people the expectation that they can safely use such an untested feature. Can you help write an alternative/better way to express this? I'll switch the wording to say "largely untested" for starters, but I'm sure it could be said better. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 12:28, 13 January 2019 (MST)

    

    Well I don't know exactly, but something like that : "The 4-screen mirroring feature is largely untested with mappers wiout prior games using a 4-screen variation". This means that it's supposed to work, however it's untested and that's what makes the most sense in this contex. Saying "most emulators will ignore the bit" seems overreaching to me. [Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 01:21, 15 January 2019 (MST) 

    I can tell you that Nintendulator and NintendulatorNRS set up nametable mirroring in each mapper's code, and if that mapper has no provisions for 4-Screen Mode, the bit will not have any effect. Emulators that obey the 4-screen bit for any mapper will do so because they enforce that bit outside of the mapper emulation code. It is basically a normative decision to specify whether that bit is supposed to be thought of as "informative" for the mapper emulation code, or as "overriding" anything that the mapper normally does. Unless you want make it obligatory which understanding is correct --- something I would like, but only for NES 2.0 ---, a page like this, which is mostly descriptive of a legacy format, should simply state: "The four-screen bit cannot be assumed to override the normal mirroring settings of mappers that have not existed in four-screen variants during the NES era, or as real hardware for homebrew mappers. Some emulators will allow for such an override, while others (e.g. Nintendulator) do not." [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 01:45, 15 January 2019 (MST)

    

    

    Nintendulator is just ONE emulator, not "most emulators". Besides, you have to remember that, originally in iNES 1.0, it seemed very clear to me that this bits overriedes other mirroring-related behaviour. For instance, MMC3 has no "provisions for 4-screen mirroring" or anything like that - 4-screen mirroring is an hardware feature that can be installed on any cartridge regardless of it's mapper, and tha happened to have been installed on two officially released MMC3 games. In this regard, I think Nintendulator is actually wrong, if it does not gives you 4 independant nametables when this bit is set with any mapper that does not have complex VRAM features. It should be able to give you NROM/CNROM/GNROM/UNROM/MMC1/MMC2/MMC4, etc... with 4-screen just fine. If it doesn't I'd consider it a bug in the emulator that does not respect the iNES format. HOWEVER with advanced mappers such as MMC5, VRC6 or Namco Mapper, it's understandable the mapper already fucks with nametables and there's no canonical way to implement 4-screen mirroring without conflicting with other features, so in those cases it's fine to ignore the bit.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 02:59, 15 January 2019 (MST) 

    The Everdrive N8 behaves similar to Nintendulator in having the mapper code decide whether 4-screen applies or not; in fact, it incorrectly does not even allow 4-screen on Mapper 206, where it is actually needed. The iNES format makes no explicit statement that the 4-screen bit should override mapper-controlled mirroring any more than the vertical mirroring bit should override mapper-controlled mirroring, so neither alternative is truly "clear". Regardless of which alternative you consider the right one, it obviously remains unsafe to assume that 4-screen overrides mapper-controlled mirroring, and a descriptive page describing a legacy format should state as much. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 05:45, 15 January 2019 (MST)

    

    

    

    Still that is TWO emulators, out of a list of 90+. You'd need to find that behaviour in more than 50 emulators to state "most emulators" \- something that is dubious. Also, what is your source for iNES being unclear ? What do you consider an authoritative document that describe the iNES header, and is not clear ?[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 08:46, 15 January 2019 (MST) 

    It turns out that Nintendulator does support four-screen mirroring on discrete mappers with hard-wired mirroring, though not on MMC1 and such. The Everdrive N8 however I verified to not support it. It is not an emulator, but a flash cart, and a very popular one. I do not have to prove anything about "most emulators" because I have not made any claims about most emulators; rainwarrior did. All I have been saying for the past two posts is that it is not safe to assume anything about the effects of the four-screen bit on boards for which historically no four-screen variant existed. That statement would be true even if one were to agree that iNES "clearly" called for the ability to specify 4-screen mode on any mapper, or at least any hard-wired one, and that all emulators and flash carts that do not allow that were wrong. If a mainstream emulator or popular flash cart will not play 4-screen CNROM correctly, then it's an unsafe choice for a ROM author, regardless of whose fault that is. Basically, my wording would give a piece of advice without making the sweeping claim that rainwarrior's previous version does. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 09:50, 15 January 2019 (MST)

Does the Everdrive flash card support 4-screen mirroring at all ? If not that might be why it does not support it in combination with the mappers that didn't use it historically. Also, why indent every answer ? After 6 tabs this gets ridiculous.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 13:19, 15 January 2019 (MST) 

The Everdrive N8 flash cart supports 4-screen mode for iNES Mapper 004. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 13:35, 15 January 2019 (MST) 

    Re: Bregalad, I had already removed "many emulators will ignore this bit" and replaced it with the "largely untested" wording. There is no need to argue against "most emulators", that's not something I'm arguing for, nor is anyone else AFAICT. (The word **most** was never even used here?) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 13:48, 15 January 2019 (MST) 

    Indeed, you said "many", not "most"; that was a misquote by Bregalad. Sorry. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 13:58, 15 January 2019 (MST)

    

    

    Ok ok, the page is fine now[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 00:49, 16 January 2019 (MST)
