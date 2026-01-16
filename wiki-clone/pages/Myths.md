# Myths

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Myths) | View [other pages](Special_AllPages.xhtml#Myths)

This page documents some **myths** about the NES hardware that originate in obsolete documents and emulators. 

## Contents

  * 1 NTSC picture height
  * 2 Scrolling registers
  * 3 Mappers
    * 3.1 MMC4
    * 3.2 MMC5
    * 3.3 VRC6
  * 4 Largest game
  * 5 Old programs
  * 6 Old tutorials
  * 7 PPU details
  * 8 Color emphasis



## NTSC picture height

**Myth:** The NTSC NES picture is only 224 pixels tall. (Seen in [nesfreq.txt](http://nesdev.org/nesfreq.txt), [a post by chaotic_thought on ngemu.com](http://ngemu.com/threads/true-resolution-of-psx-320x224.59392/#post-826552), and elsewhere) 

**Fact:** The NTSC NES picture is 242 pixels tall: 240 lines of picture and 2 lines of vertical border. The PPU fetches and generates a signal for all 240 lines, even if TVs [cut off the edges](Overscan.xhtml "Overscan"). 

Ideally, NES games place nametable [mirroring](Mirroring.xhtml "Mirroring") glitches in the overscan, and some emulators simulate overscan in order to hide these glitches. For example, PocketNES for Game Boy Advance hides the top 16 pixels, the bottom 11, and 8 on the left and right sides. The Mega Man Anniversary Collection (GameCube/XBOX/PS2) likewise hides the top/bottom 8 lines, plus an extra left/right clipping to hide the attribute color glitch on scrolling. Also, the Wii Virtual Console (NES) does not display the top and bottom 8 lines. 

## Scrolling registers

**Myth:** Usage of the [PPUADDR](PPU_registers.xhtml "PPUADDR") ($2006) register is needed to scroll. (Seen in [Nestech.txt section 10: Programming the NES](Nestech_txt.xhtml#Programming_the_NES "Nestech.txt")) 

**Fact:** The proper way to [set the scroll position](PPU_scrolling.xhtml "PPU scrolling") is to write the upper bits of the X and Y coordinates to [PPUCTRL](PPU_registers.xhtml "PPUCTRL") ($2000), and then bits 0-7 of the X and Y coordinates to [PPUSCROLL](PPU_registers.xhtml "PPUSCROLL") ($2005). The NES will update the VRAM address register near the end of the pre-render scanline (261 on NTSC, 311 on PAL). 

PPUADDR ($2006) is needed to scroll only when changing the vertical part of the scroll position during rendering time. This could happen if rendering is turned on late to free up more VRAM update time, or the screen is split. 

_Super Mario Bros._ appears to zero out PPUADDR after writing a buffer to PPUDATA, but this was later discovered to be a workaround for a rare [palette corruption](PPU_registers.xhtml#Palette_corruption "PPU registers") bug instead. 

## Mappers

### MMC4

**Myth:** The [MMC4](MMC4.xhtml "MMC4") is used in the Japanese version of _Mike Tyson's Punch-Out!!_ (source: [Nintendo Entertainment System Documentation v0.40 by Y0SHi](https://gamefaqs.gamespot.com/nes/916386-nes/faqs/2949)) 

**Fact:** The MMC4 is used only in three Japan-only games published by Nintendo in the _Famicom Wars_ and _Fire Emblem_ series. Known Japanese versions of _Mike Tyson's Punch-Out!!_ use the same [MMC2](MMC2.xhtml "MMC2") as their American and PAL counterparts. 

### MMC5

**Myth:** The [MMC5](MMC5.xhtml "MMC5") supports [memory for 4-screen nametables](Mirroring.xhtml#4-Screen "Mirroring") (source: [Nintendo Entertainment System Documentation v0.40 by Y0SHi](https://gamefaqs.gamespot.com/nes/916386-nes/faqs/2949)) 

**Fact:** MMC5 uses the 2 KiB of RAM in the Control Deck for two nametables. It has its own 1 KiB ExRAM, and one of the four possible modes for ExRAM uses it as a third nametable. It also supports fill mode, a fourth limited-function nametable filled with 960 copies of a single tile number. So in total, while the MMC5 supports four different modes for each nametable, it does not support memory for the sort of 4-screen nametables seen in _Napoleon Senki_ and _Gauntlet_. 

### VRC6

**Myth:** The VRC6 is a very complex mapper even superior to the MMC5. 

**Fact:** The [VRC6](VRC6.xhtml "VRC6") is a decent mapper able to do standard PRG and CHR bankswitching, a CPU cycle counter, and 3 extra sound channels. The [MMC5](MMC5.xhtml "MMC5") has extended video possibilities, a true scanline counter, and countless features that the VRC6 lacks, but only has 2 extra sound channels. Rumor has been made that the VRC6 was superior to the MMC5 because the MMC5 _Castlevania III: Dracula's Curse_ was censored, and (like other 72-pin games) didn't use extra sound. But in fact, _Castlevania III_ doesn't even come close to using all MMC5 capabilities, and it likely used MMC5 because it supported the VRC6's PRG ROM and CHR ROM bankswitching modes and was cheaper than Konami getting the VRC6 approved through Nintendo of America and Nintendo of Europe. 

## Largest game

**Myth:** _Dragon Quest/Warrior IV_ (DQ4) is the largest NES game, having 1 MiB (1,048,576 bytes) of ROM. (Source: [NES Technical/Emulation/Development FAQ version 1.4](http://nesdev.org/NESTechFAQ.htm#biggestgame) via [Reddit](https://www.reddit.com/r/todayilearned/comments/2qdz5q/til_the_biggest_offical_nes_game_ever_made_was/)) 

**Fact:** Both the Japanese _Dragon Quest IV_ and the American _Dragon Warrior IV_ releases use [SUROM](MMC1.xhtml "SxROM"), as pictured at [NesCartDB's entry for _Dragon Warrior IV_](http://bootgod.dyndns.org:7777/profile.php?id=1276). They use 512 KiB PRG ROM and 8 KiB CHR RAM, which is not larger than quite a few other licensed games. This rumor was due to a 1MiB overdump of _Dragon Quest IV_ floating around. 

Games larger than _Dragon Quest IV_ include the following: 

  * The largest licensed Famicom game is _Metal Slader Glory_ (512 KiB PRG + 512 KiB CHR + [MMC5](MMC5.xhtml "MMC5")).
  * The largest licensed NES game is _Kirby's Adventure_ (512 KiB PRG + 256 KiB CHR + [MMC3](MMC3.xhtml "MMC3")).
  * The largest unlicensed non-pirate NES game from the original era is _Action 52_ (1,536 KiB PRG + 512 KiB CHR + [custom mapper](INES_Mapper_228.xhtml "INES Mapper 228")).
  * The largest unlicensed non-pirate NES production from the modern era is _A Winner Is You_ (64 MiB), which is a music cartridge and not a game.
  * One large "Hong Kong Original" port is _[Final Fantasy VII](https://en.wikipedia.org/wiki/Final_Fantasy_VII_\(NES_video_game\) "wikipedia:Final Fantasy VII \(NES video game\)")_ (2 MiB PRG + CHR RAM).
  * Some pirate multicarts are 4 MiB or larger.



The largest games cannot be represented in the the original [iNES](INES.xhtml "INES") format, which has a practical limit of 2 MiB PRG ROM and 1 MiB CHR ROM. [NES 2.0](NES_2_0.xhtml "NES 2.0") should be used instead. 

## Old programs

**Myth:** If a binary file has a .nes file extension, it will work as intended on an NES, and emulators should be tweaked to match how it is supposed to work. 

**Fact:** No, especially older NES programs tend to have been tested only on bad emulators. Emulators should match the behavior of an NES or at least that of an accurate emulator. Dirty iNES headers might break it. 

A lot of emulators, especially prior to about 2005, were based on incomplete knowledge of how the NES works. Some old demos expect all internal memory ($0000-$07FF) to be $00. Since then, public knowledge of the [quirks of the NES hardware behavior](Errata.xhtml "Errata") has grown, and emulators such as Mesen and Nintendulator more faithfully reproduce the misbehaviors in sloppy or [cargo-cult-programmed](https://en.wikipedia.org/wiki/Cargo_cult_programming "wikipedia:Cargo cult programming") code. See [Program Compatibility](Program_compatibility.xhtml "Program Compatibility") for a list of homebrew known to have problems on an NES. 

## Old tutorials

**Myth:** [GBAGuy's NES tutorial](http://patater.com/gbaguy/nesasm.htm) is worth following. 

**Fact:** Old tutorials like these are full of cargo-cult programming because the authors apparently didn't fully understand the hardware. For example, this tutorial in particular treats the [OAM address](PPU_registers.xhtml "PPU registers") register as 16-bit (just like [PPUADDR](PPU_registers.xhtml "PPUADDR")) and attempts to initialize variables in [system RAM](https://en.wikipedia.org/wiki/.bss "wikipedia:.bss") using .db statements - a lot of the programs don't even work on a NES. [NES 101](http://nesdev.org/NES101.zip) and [Nerdy Nights](https://nerdy-nights.nes.science) are considered better. Even the webmaster of Patater.com now recommends Nerdy Nights. 

## PPU details

**Myth:** [OAMADDR](PPU_registers.xhtml "OAMADDR") ($2003) must be cleared on VBlank ending. 

**Fact:** The PPU itself sets the sprite address to zero at the end of VBlank, but due to a design flaw it can result in minor sprite RAM corruption if it was nonzero beforehand - in particular, it can cause values from one 8-byte "page" of sprite RAM to leak into another due to its lack of proper memory refreshing. Some Chinese games actually rely on this behavior and will lock up otherwise. 

**Myth:** "There is 16k of internal VRAM" (source [Nestech.txt 0.40 by Y0shi](https://www.gamefaqs.com/nes/916386-nes/faqs/2949)) 

**Fact:** The PPU has an address range of 14 bits addressing 16 KiB, but there's only 2 KiB of internal VRAM, typically used for nametables. This was corrected to "16 kbits" in later editions of [Nestech.txt](Nestech_txt.xhtml "Nestech.txt"). 

## Color emphasis

**Myth:** Enabling more than one [color emphasis](Colour_emphasis.xhtml "Colour emphasis") bit at once will damage the PPU, or at least cause the TV to lose sync. (Source: [Nintendo Entertainment System Architecture](http://fms.komkon.org/EMUL8/NES.html) by Marat) 

**Fact:** Enabling multiple color emphasis bits is perfectly safe - in fact, [some licensed games](Colour_emphasis_games.xhtml "Colour-emphasis games") including _Felix the Cat_ and _Just Breed_ enable all of them simultaneously to dim the screen. On the other hand, enabling all emphasis bits results in an unreadable white screen on an RGB PPU, such as that in the Famicom Titler or the Sharp C1 (Famicom TV). Worse, setting the PPU into _slave_ mode (by setting the _master/slave_ bit in [PPUCTRL](PPU_registers.xhtml "PPUCTRL")) is theoretically capable of causing actual damage, as it results in high current draw from the EXT pins (due to them trying to output +5V despite being wired to GND). 
