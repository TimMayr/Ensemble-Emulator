# Talk:NES 2.0 Mapper 360

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANES_2.0_Mapper_360) | View [other pages](Special_AllPages.xhtml#Talk_NES_2_0_Mapper_360)

Why does this warrant a mapper number? It literally involves no programmer's interface at all.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 22:43, 3 January 2019 (MST) 

    The programmer's interface involves translating DIP switch settings to outer bank and mirroring selection. Doing so is neither self-explanatory, nor part of a generalized DIP Switch handling scheme, but PCB-specific. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 22:47, 3 January 2019 (MST)

How is this better than 31 separate NROM dumps? How does this solve any problems of encapsulation? How does this not just make the user experience awful, but making them manually select a number in a special dialog with a virtual piece of paper to let them know what's where, instead of using the filesystem chooser for the game they want? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 22:52, 3 January 2019 (MST) 

    It's better than 31 separate NROM dumps in that it represents the fact that the PCB does not have 31 separate PRG/CHR-ROM chips, but just one of each type. The awful user experience in emulators accurately replicates the awful player real-hardware experience of having to flip DIP switches to select a game. It's all about accurately preserving cartridges, their hardware and ROM chip content, not necessarily about making a wonderful user experience (although I am willing to make exceptions in the case of input devices...). And if you object to this cartridge being preserved as it is, you must also object to [NES 2.0 Mapper 357](NES_2_0_Mapper_357.xhtml "NES 2.0 Mapper 357"), for it is similar in the aspects you are faulting. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 22:58, 3 January 2019 (MST)

  * The emulator's implementation is not the programmer's interface. There's no programmer's interface here—software cannot know it's on this cartridge. There's only the end-user interface, which is awful.
  * Why is archiving the exact order of the contents of a ROM desirable when no individual part _anywhere_ can know whether it's part of this bundle? Physically, inside a single ROM there's no salient difference between having 31 different physical ROMs and the exact same data in just one—either way there's row and column selection hardware, and the difference between a demultiplexer outside the ROM is circuit-wise identical to a row selector inside the ROM. The same information is contained when unpacked—absolutely no work has been thrown away, unlike multicarts that have dedicated software menus—and then the UX isn't bad and everyone gets to do less work. This mapper can't even be dumped without repetitive physical intervention, unlike well-behaved mappers.
  * And yes, I object to mapper 357 also, but didn't notice it at the time. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 00:58, 4 January 2019 (MST)



    I don't see why it should matter whether an individual part can know whether it's part of a bundle or not. It just dawned on me that your objection not only applies to mappers 357 and 360, but actually to any _reset-based multicart_ , [some of which were specified decades ago](INES_Mapper_060.xhtml "INES Mapper 060"). I have explained why the mapper needs to be there given that one wants to preserve the ROM chip's data as it is, but if you want to discuss the more basic question of what ROM chips are worth archiving, that should be discussed elsewhere. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 04:07, 4 January 2019 (MST)

There are two major parts to my objection. 1- New mappers shouldn't require novel UI unless that UI is explicitly an input method. Reset-based multicarts at least pass that test. 2- Unlike with a reset-based multicart, where it's remotely plausible that each game could pick up with the RAM state from the previous game, with a switch based one it really requires that the user remove power between games, so there's no way for games to communicate even using that side channel. 

So while it's true that I'm less than thrilled about mapper 60, this is, in my opinion, even worse. 

Finally, while I agree that 'archiving all data' is the goal, I disagree that "verbatim" is the necessary and correct way to do this. For example, MESS's database includes SNES games that were distributed as two ROMs as two separate dumps. If a game was released twice, it is present twice. Byuu has rightly decided that that's just silly.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 11:55, 4 January 2019 (MST) 

    

  * The criteria "new mappers should not require novel UI" and "multicart games should be able to communicate with each other" (?) are novel and arbitrary. I have never read that these are criteria for mapper inclusion, and do not see why they should be, especially that last one.
  * That the first criterion is not fulfilled is not true either: An emulator can implement game switching in several ways; not just by displaying a menu, but also by increasing the virtual DIP switch upon soft-resetting. That is actually the way FCEUX implements DIP switches in all non-Vs. mappers, so it would be nothing new.
  * I do not understand how that SNES example applies here either: the game was released as one cart, not 31, [seen here](http://www.masterdisk.byethost15.com/blog/libg/images/BIT31in1.jpg), therefore it should be one file, not 31. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 13:19, 4 January 2019 (MST)



  * "should be able to communicate with each other" is another way of phrasing "some code, somewhere, depends on some other code being in this place". That's just not possible here.
  * Asking that a person hit the reset button 31 times in order to go back one DIP switch is worse than special casing the UI.



    

    Correct, but irrelevant. A novel UI still is not required, it merely makes for better usability. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 14:03, 4 January 2019 (MST)

  * The SNES example is the argument that the physical dice are not what's important, and shouldn't be. Only the data is. It's in the opposite direction of what you're describing here (anti-helpful splitting instead of anti-helpful merging), but ultimately it's the same argument. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:47, 4 January 2019 (MST)



    

    You did not answer though why your two novel and arbitrary criteria should be the deciding criteria instead of "physical dice", or why they should not be considered novel and arbitrary, even as they have not been stated in any other situation here. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 14:03, 4 January 2019 (MST) 

    The criterion I'm using is "How is this different from 31 distinct cartridges" and the answer is _it isn't_. The reason my "arbitrary" criteria have not been stated in any other situation is because _it hasn't been relevant before_ and it's actively disingenuous to use that argument. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:32, 4 January 2019 (MST) 

    Then I shall leave it at that, lest I make any more "disingenous" arguments. I will not remove that page; if you remove it, the so-headered ROM image will still be out there, I shall keep emulator support for it, and shall not reassign the number. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 15:56, 4 January 2019 (MST)

All I'm saying is: This is a duplicate of mapper 0. 

Everything that you have qualified as "a novel criterion" is me looking for _any_ loop-hole that might possibly validate any slimmest argument for how it could possibly be anything other than a duplicate of mapper 0. But no, it's really just 100% a duplicate of mapper 0, plus some crap that must be ignored by the emulator. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 18:23, 4 January 2019 (MST) 

    Utter nonsense. Mapper 0 is NROM, and ["NROM refers to the Nintendo cartridge boards NES-NROM-128, NES-NROM-256, their HVC counterparts, and clone boards."](NROM.xhtml "NROM") The Bit Corp. P3150 cartridge PCB is not a clone of the NES-NROM-128/256 PCB. The NES-NROM board has 32/8 KiB of PRG/CHR-ROM, while the P3150 board has 512/256 KiB of PRG-/CHR-ROM, making it a different mapper, regardless of user/programmer interfaces, side channel communication and whatnot. Producing 31 NROM images means creating multicart extracts, but not a multicart ROM image. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 18:37, 4 January 2019 (MST)

I have a series of hypotheticals here, and I'd appreciate it if you would explain where you would say where you would define this as a new mapper, vs multiple dumps, vs some other option I haven't thought of. 

As a baseline, start with a cart with 32K PRG and 8K CHR, ordinary NROM. 

Then, a cart with 64K PRG and 8K CHR, in all of the following combinations: 

  * containing two identical copies of those 32K of data PRG...
  * containing 32K of padding and 32K of PRG...
  * containing two unique 32K chunks of PRG, but these are identical to preexisting dumps...
  * containing two unique 32K chunks of PRG, but these are extremely similar but subtly different from preexisting dumps...
  * containing two unique 32K chunks of PRG, believed to be identical copies to existing carts but not yet found...



In every combination of: 

  * A15 is tied high directly on the PCB, in way that's hard to access, but technically anything can be reworked
  * A15 goes to a cuttable jumper, and the user can change it with a soldering iron but not too much more effort than that
  * A15 goes to a pair of contacts on the edge of the PCB. The user can, at their own expense, add a paperclip to tie the two contacts together, selecting the other half of PRG.
  * A15 goes to a physical switch



Plus each option of: 

  * The shell makes access to the switch/rework area difficult
  * The shell makes has a cutout to make it easier



Plus each option of: 

  * No mention is made of the other data inside the ROM
  * Associated documentation (for example, a sticker on the shell) mentions the other possibility.



Finally, why did you choose those distinctions? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:21, 4 January 2019 (MST) 

    My standard is: One cart, one file. Almost all mapper pages refer to PCBs and cartridges, physical objects. The "physical dice", as you called it, indeed defines what goes into a file and what goes into separate files, not far-fetched arcane notions about programmer interfaces, mutual ROM part awareness, side-channel communications, and absurd thought experiments about paperclips. The mapper page stays. End of discussion. [NewRisingSun](https://www.nesdev.org/w/index.php?title=User:NewRisingSun&action=edit&redlink=1 "User:NewRisingSun \(page does not exist\)") ([talk](User_talk_NewRisingSun.xhtml "User talk:NewRisingSun")) 03:22, 5 January 2019 (MST) 

    Even if the data requires extraordinary effort to access? Even if it's a duplicate? Even if the user has no reason to think there's extra data there? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:46, 5 January 2019 (MST)
