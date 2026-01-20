# User:Tepples/Should I use MMC5?

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ATepples/Should_I_use_MMC5%3F) | View [other pages](Special_AllPages.xhtml#User_Tepples_Should_I_use_MMC5_)

![](../wiki-images/Edit-clear.svg) |  This is an **opinion piece** by a user of this wiki. It has yet to gain broad consensus among users. Comments are [the talk page](https://www.nesdev.org/w/index.php?title=User_talk:Tepples/Should_I_use_MMC5%3F&action=edit&redlink=1 "User talk:Tepples/Should I use MMC5? \(page does not exist\)").   
---|---  
  
In 2025, it's still recommended to **think twice before you use[MMC5](MMC5.xhtml "MMC5")** for development of original software for NES. 

Some developers want MMC5 for 8x8-pixel color areas, more PRG ROM, more PRG RAM, more sprite CHR windows (for 2 players, 2 enemies, and general projectiles), fast multiplication, more robust power-down protection,[1] or just wanting to have "the most powerful licensed mapper" out of lacking a sense of a project's scope. Yet MMC5 has been called "half-baked" by members of the NESdev Discord server. In particular, the fact that extended attribute mode (nicknamed ExGrafix) provides only enough memory for single-screen [mirroring](Mirroring.xhtml "Mirroring") makes heavy use of parallax scroll splits less convenient, and the rationale behind the PCM channel's read and IRQ functionality is baffling to more experienced developers. 

MMC5 is poorly understood. It is still possible to accidentally make an MMC5 program that works on top-tier RAM cartridges, such as the EverDrive N8 Pro, and in top-tier emulators, such as Mesen, and fails on an MMC5 cartridge board. For example, a homebrew game may fail to run on an authentic MMC5 board because its mapper initialization code proves inadequate. And because no game was shipped using SL mode for fine vertical scrolling in a vertical split, the community hasn't had a chance to research the pitfalls of this mode. 

MMC5 display modes that use ExRAM (third nametable and extended attributes) require the mapper to provide nametable memory. This works on an authentic NES or Famicom or a correctly wired clone console, such as AVS or Nt mini. It fails on some clone consoles, particularly that use V.R. Technology's [VT02 or VT03 system on chip](VTxx.xhtml "VTxx") and use the [/PA13 or VRAM /CE pin](Cartridge_connector.xhtml "Cartridge connector") to distinguish an NES cartridge from a OneBus cartridge. These same consoles also have problems with games using [4-screen VRAM](Mirroring.xhtml#4-Screen "Mirroring") or [ROM nametables](INES_Mapper_068.xhtml "INES Mapper 068"). 

Testing on an MMC5 cartridge board is impractical for many. The developer must rewire the board and cut the cartridge shell for a socketed flash EEPROM and modify the front-loading NES Control Deck to accept socketed cartridges. This brings electrical engineering and soldering on top of programming, music, and visual art as a skill that all developers must have coming in. It also closes the hobby to people with hand tremors or other disabilities that make soldering not feasible. 

Publishing an original game that uses MMC5 is impractical because MMC5 is discontinued. To produce a game cartridge using MMC5, one must harvest the ICs and board from an authentic MMC5 game. There are a dwindling supply of such donor cartridges, especially since a sharp rise in the price of vintage NES cartridges in the early 2020s. 

Most features of MMC5 are also present in other mappers that are more cohesive, better understood, and more widely cloned in new parts. One of these options may be right for your project, especially projects of smaller scope: 

  * For PRG ROM larger than 512 KiB, use oversize [mapper 4](MMC3.xhtml "MMC3"), available in several third-party MMC3 clones.
  * For PRG RAM larger than 8 KiB, use [FME-7](Sunsoft_FME_7.xhtml "FME-7"), also available as a clone.
  * For multiple features, use [Rainbow](NES_2_0_Mapper_682.xhtml "NES 2.0 Mapper 682"). You may want to start a project on MMC5 and port it to Rainbow halfway through once you have lined up a manufacturer.
  * Other mappers containing subsets of MMC5 functionality, such as only 8-pixel-tall [color areas](PPU_attribute_tables.xhtml "PPU attribute tables"), are under development.



More obscure emulators, such as emulators that run on platforms other than PCs or provide niche [enhancement](Enhancement.xhtml "Enhancement") or speedrun features, may not support some of these newer mappers. Yet because of the popularity of games like _Castlevania III: Dracula's Curse_ and _Just Breed_ , many of them support MMC5 well enough to run a game that uses features that the licensed library exercises. If a lot of your audience use such emulators, you might find it worthwhile to make a game work on both MMC5 and another mapper. 

## References

  1. ↑ "Why Your Game Paks Never Forget". _Nintendo Power_ #20 (March 1991), pp. 28-31.


