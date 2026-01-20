# Tricky-to-emulate games

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Tricky-to-emulate_games) | View [other pages](Special_AllPages.xhtml#Tricky_to_emulate_games)

At the very least the following games depend on hard-to-emulate or just obscure behavior. (If you're looking for a good first game for your new emulator, try anything made in 1984 or earlier, such as _Donkey Kong_.) 

## Contents

  * 1 Troubleshooting games
  * 2 See also
  * 3 References
  * 4 External links



_Abarenbou Tengu_ (J), _Captain Tsubasa 2_ (J), _Noah's Ark_ (E), _Rampart_ (U), _Zombie Nation_ (U)
    These refer to CHR ROM banks outside their size. A CHR ROM of the correct size will wrap the addresses correctly by discarding the most significant bits, as do most emulators. But if you are developing a flash-cart that just pre-programs its flash/SRAM memory with the CHR ROM data without address wrapping, graphical bugs will happen. If you want to simulate that behaviour in emulator, increase CHR-ROM banks number in iNES header twice and paste zeros in the the CHR area.
_Adventures of Lolo 2_ , _Ms. Pac-Man_ (Tengen), and _Spelunker_
    These games rely on proper timing for when the CPU polls for interrupts. When NMI becomes enabled while the vblank flag is already set, the resulting NMI occurs late enough in the instruction that another instruction is able to execute before the NMI is serviced.
_Air Fortress_ (J)
    Expects RAM to be enabled at power-up, as it clears WRAM before enabling it ($E000 D4). If MMC1 powers up with RAM disabled, the values written in the init routine go nowhere. If RAM is not enabled is used and the RAM's power-up value is anything but $00, unwanted color emphasis gets applied until the reset button is pressed. (The North American version enables WRAM first.)
_Arkista's Ring_
    Crashes after completing the first loop if a read from PPU open bus returns 1 on bit 6, which is 0 unless using a mod such as NESRGB or Hi-Def NES that uses EXT output.
_Balloon Fight_
    It reads the nametables through [PPUDATA](PPU_registers.xhtml "PPUDATA") ($2007) to determine the current tile of each star in the background when twinkling them. (The code is at $D603.) The scroll split in "Balloon Trip" also depends to an extent on the correct number of CPU cycles from the start of NMI to the start of display, but it's not particularly picky.
_Bandit Kings of Ancient China_ , _Gemfire_ , _L'Empereur_ , _Nobunaga's Ambition II_ , _Romance of the Three Kingdoms II_ , and _Uncharted Waters_
    Koei's MMC5 RPGs and strategy games use 8×8-pixel attributes and large work RAM.
_Bases Loaded II_
    The screen glitches after a pitch is thrown ([screenshot](File_Bases_Loaded_2_without_re_NMI_jpg.xhtml "File:Bases Loaded 2 without re-NMI.jpg")) if writing $00 then $80 to [PPUCTRL](PPU_registers.xhtml "PPUCTRL") during vertical blank does not cause an additional [NMI](NMI.xhtml "NMI").
_Batman: Return of the Joker_ , _Dragon Quest_ , and _Milon's Secret Castle_
    These read level data and control logic from CHR ROM. The $2007 read must take into account not only the 1-byte delay (see entry for _Super Mario Bros._) but also CHR bank switching. _Batman: RotJ_ also executes code from PRG RAM.
_Battletoads_
    Infamous among emulator developers for requiring fairly precise CPU and PPU timing (including the cycle penalty for crossing pages) and a fairly robust sprite 0 implementation. Because it continuously streams animation frames into CHR RAM, it leaves rendering disabled for a number of scanlines into the visible frame to gain extra VRAM upload time and then enables it. If the timing is off so that the background image appears too high or too low at this point, a sprite zero hit will fail to trigger, hanging the game. This usually occurs immediately upon entering the first stage if the timing is off by enough, and might cause random hangs at other points otherwise.
_Battletoads & Double Dragon_ and _Low G Man_
    They read from WRAM at $6000–$7FFF despite there being none on the cartridge, relying on the values produced by [open bus behavior](Open_bus_behavior.xhtml "Open bus behavior").[1] Additionally, _LGM_ disables WRAM through $A001, which some emulators disregard in order to kludge [MMC6](MMC6.xhtml "MMC6") games into working. If WRAM is present and enabled, some pre-loaded values will cause _BT &DD_ to crash at the end of stage 1 when Abobo makes his first appearance and _LGM_ to crash when playing boss music.[2]
_Bee 52_
    This needs accurate DMC timing and relies on [PPUSTATUS](PPU_registers.xhtml "PPUSTATUS") bit 5 (sprite overflow) as well.
_Bill & Ted's Excellent Adventure_ and some other [MMC1](MMC1.xhtml "MMC1") games
    These depend on the mapper ignoring successive writes; see [iNES Mapper 001](MMC1.xhtml "INES Mapper 001") (the talk page for that page might be informative too). _Bill & Ted_ also turns off and re-enables rendering midframe to switch CHR banks (such as in the black border above dialog boxes).
_Burai Fighter_ (U)
    It accesses [PPUDATA](PPU_registers.xhtml "PPUDATA") during rendering to draw the scorebar. Incorrect emulation clips the scorebar to half size. See the notes on accessing [PPUDATA](PPU_registers.xhtml "PPUDATA") during rendering on the [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling") page.
_B-Wings_ , _Fantasy Zone II_ , _Demon Sword_ / _Fudō Myōō Den_ , _Fushigi no Umi no Nadia_ , _Krusty's Fun House_ , _Trolls in Crazyland_ , _Over Horizon_ , _Super Xevious: GAMP no Nazo_ , and _Zippy Race_
    They write to CHR ROM and expect the writes to have no effect.[3][4]
_Captain Planet_ , _Dirty Harry_ , _Infiltrator_ , _Mad Max_ , _Paperboy_ , _The Last Starfighter_
    Mindscape games which rely on the [open bus](Open_bus_behavior.xhtml "Open bus") behavior of controller reads and expects them to return exactly 0x40 or 0x41; see [Standard controller](Standard_controller.xhtml "Standard controller").
_Cobra Triangle_ and _Ironsword: Wizards and Warriors II_
    They rely on the dummy read for the `sta $4000,X` instruction to acknowledge pending APU IRQs.
_Crystalis_
    Uses MMC3 scanline for a moving vertical split to wrap the playfield while accommodating the status bar. Incorrect MMC3 timing will create a moving seam as you wander up and down the map.
_Crystalis_ , _Fantastic Adventures of Dizzy_ , _Fire Hawk_ , _Indiana Jones and the Last Crusade_ , _StarTropics_ , and _Super Off Road_
    These do mid-frame palette changes.
_Daydreamin' Davey_
    Changes the background palette color mid-frame; also does an OAM DMA for its sprite-only status bar below the split. (See also: Stunt Kids.)
_Door Door_
    Writes to the PRG-ROM area frequently through a pointer explicitly set to $8080. Harmless on NROM.
_Final Fantasy_ , _River City Ransom_ , _Apple Town Story_[5], _Impossible Mission II_[6] amongst others
    Use the semi-random contents of RAM on powerup to seed their RNGs. Emulators often provide fully deterministic emulated powerup state, and these games will seem to be deterministic when they weren't intended to be.
_Fire Hawk_ , _Mig 29 Soviet Fighter_ , and _Time Lord_
    These need accurate DMC timing because they [abuse APU DMC IRQ](APU_DMC.xhtml#Usage_of_DMC_for_syncing_to_video "APU DMC") to split the screen.
_Galaxian_
    Requires proper handling of bit 4 of the [P register](Status_flags.xhtml "CPU status flag behavior") for /IRQ.
_G.I. Joe_ and _Mickey in Letterland_
    These turn [sprite display](PPU_registers.xhtml "PPUMASK") off and leave the background on. Correct timing of MMC3 IRQs requires that the sprite fetches still clock the scanline counter when either is enabled.[[1]](https://forums.nesdev.org/viewtopic.php?f=3&t=14103)
_Huge Insect_
    Depends on obscure [OAMADDR](PPU_registers.xhtml "OAMADDR") ($2003) behavior in the OAM DRAM controller; see [PPU registers](PPU_registers.xhtml "PPU registers").
_Ishin no Arashi_
    Plays sound effects through [MMC5 pulse channels](MMC5_audio.xhtml "MMC5 audio") and times them using $5015 reads.
_Jurassic Park_
    The wobbling OCEAN logo on the title screen is very sensitive to slight delay in the [MMC3](MMC3.xhtml "MMC3") IRQ and could have incorrectly scrolled lines if mistimed.
_The Legend of Zelda_
    Writes to [PPUADDR](PPU_registers.xhtml "PPUADDR") midframe to [set the coarse Y scroll](PPU_scrolling.xhtml#%242006_\(PPUADDR\)_second_write_\(w_is_1\) "PPU scrolling") for vertical scrolling between screens. See the [Game bugs](Game_bugs.xhtml "Game bugs") page for buggy behavior arising from its implementation.
_The Magic of Scheherazade_
    It maps two non-contiguous PRG ROM pages next to each other, then executes code across the page boundary. Emulators which use pointers to fetch sequential instruction bytes from ROM will fail when taking damage in the RPG-style battles. (Use password `5W` to test this easily.)
_Marble Madness_ , _Mother_ (J), and _Pirates_
    These switch CHR banks mid-scanline to draw text boxes (such as at the beginning of each _MM_ level). Getting these to render correctly requires fairly precise timing.
_Micro Machines_
    Requires correct values when reading [OAMDATA](PPU_registers.xhtml "OAMDATA") ($2004) during rendering, and also relies on proper background color selection when rendering is disabled and the VRAM address points to the palette (see the "background palette hack" on [PPU palettes](PPU_palettes.xhtml "PPU palettes")).
_Punch-Out!!_
    MMC2 snoops PPU fetches. If the PPU does not fetch the 34th tile, the ring will be glitched.
_Puzznic_ and _Reflect World_ (FDS)
    These use [unofficial](Programming_with_unofficial_opcodes.xhtml#Watermarking_instructions "Programming with unofficial opcodes") opcode $89, which is a two-byte NOP on 6502 and BIT #imm on 65C02. ([Puzznic tasvideos discussion](http://tasvideos.org/forum/viewtopic.php?p=306520#306520)) The instruction in _Puzznic_ is $89 $00; emulating $89 as a single-byte NOP will trigger a BRK that [causes the screen to shake](http://tasvideos.org/forum/viewtopic.php?p=306559#306559).
_Rollerblade Racer_
    Has an unusual status bar using only sprites with the background disabled. (See also: _Daydreamin' Davey_.)
_Slalom_
    Does a JSR while the stack pointer is 0, so that half of the return address ends up at $0100 and the other half at $01FF. Also executes code from RAM.
_Solar Jetman_
    Enables the decimal bit through manipulation of a [flag byte](Status_flags.xhtml "Status flags") pushed to the stack and expects addition to continue to operate in binary.
_Spot_ and _Quattro Sports_
    These poll input multiple times per frame, and may not respond to emulated input that can only change at one specific time during the frame[7][8]. Emulators generally don't have the option to poll the controller many times per frame in real-time, so the solutions used may need to compromise (e.g. game-specific solution, user option to decide when during the frame input changes, non-deterministic input change time). Tepples created a test ROM to explore this behaviour: [Telling LYs?](https://forums.nesdev.org/viewtopic.php?t=18998)
_Star Trek – 25th Anniversary_
    Forces MMC3 to fire IRQ at scanline 0 which on some MMC3 versions or flashcarts causes glitching during split-screen scenes.
_StarTropics_
    Disables rendering at the top of the status bar to change palettes, but also re-enables sprites when rendering comes back on. For hardware mapper emulation, the specific timing is critical. If the MMC3 IRQ timing is delayed by a cycle or two, this will begin to cause all sprites to flicker erratically. Even with a correctly timed hardware implementation, there are still some subtle corruption interactions to do with [turning rendering off, then re-enabling sprites mid-frame](PPU_sprite_evaluation.xhtml#Rendering_disable_or_enable_during_active_scanline "PPU sprite evaluation") which might not be possible to emulate correctly with current knowledge[9].
_Stunt Kids_
    Mid-frame OAM DMA for split-screen gameplay.
_Super Mario Bros._
    This is [probably the hardest game to emulate](https://forums.nesdev.org/viewtopic.php?p=22022#22022) among the most popular [NROM](NROM.xhtml "NROM") games, which are generally the first targets against which an emulator author tests their work. It relies on JMP indirect, correct palette mirroring (otherwise the sky will be black; see [PPU palettes](PPU_palettes.xhtml "PPU palettes")), sprite 0 detection (otherwise the game will freeze on the title screen), the 1-byte delay when reading from CHR ROM through [PPUDATA](PPU_registers.xhtml "PPUDATA") (see [The PPUDATA read buffer](PPU_registers.xhtml#The_PPUDATA_read_buffer_\(post-fetch\) "PPU registers")), and proper behavior of the nametable selection bits of [PPUSTATUS](PPU_registers.xhtml "PPUSTATUS") and [PPUADDR](PPU_registers.xhtml "PPUADDR"). In addition, there are several bad dumps floating around, some of which were ripped from pirate multicarts whose cheat menus leave several key parameters in RAM.
_Super Mario Bros. 3_ , _Mystery World Dizzy_ , _Double Action 53_ , and _Haunted: Halloween '86: The Curse of Possum Hollow_
    This relies on an interaction between the [sprite priority](PPU_sprite_priority.xhtml "Sprite priority") bit and the OAM index to put sprites behind the background. _SMB3_ uses it for powerups sprouting from blocks. _Mystery World Dizzy_ puts Dizzy behind a blue pillar ([screenshot](https://forums.nesdev.org/viewtopic.php?p=210736#p210736)). _RHDE: Furniture Fight_ in _DA53_ uses it for characters behind furniture. _HH86_ uses it when Donny or Tami passes behind a telephone pole or steps into polluted water.
_Teenage Mutant Ninja Turtles_
    Uses Y scroll values greater than 239, causing the PPU to read the attribute table as nametable data before looping back to the same nametable instead of rolling to the next nametable down.
_Time Lord_
    This is sensitive to the power-on state of the NES. The Vblank flag in PPUSTATUS must be set for the first time within 240 scanlines, otherwise there will be a frame IRQ which is never acknowledged, which will mess up the DMC IRQs used elsewhere and cause the game to crash.
_Ultimate Stuntman_ , _Skate or Die 2_
    _Ultimate Stuntman_ plays PCM drum samples on the DMC channel during idle portions of the frame. _Skate or Die 2_ does it on the title screen. (See also: _Battletoads_ introduction.)
_Wario's Woods_
    Uses [MMC3](MMC3.xhtml "MMC3") IRQ with unusual configuration of BG using CHR page $1000 and sprites using CHR page $0000. On some [CPU-PPU alignments](PPU_frame_timing.xhtml#CPU-PPU_Clock_Alignment "PPU frame timing") (assigned randomly at reset), [the IRQ receives an extra clock on every second frame](MMC3.xhtml#IRQ_Specifics "MMC3"), causing the last 48 pixels of the green ground to flicker, but not on all resets[10].
_Wizards and Warriors 3_
    It writes new tile graphics for the sprites at the screen split after the sprites have been drawn, but before the frame has ended. Emulators which draw the sprites all at once using graphics data from the end of the frame will have glitches in the main character's sprite.
_The Young Indiana Jones Chronicles_ and _Zelda II: The Adventure of Link_
    These access [PPUDATA](PPU_registers.xhtml "PPUDATA") during rendering to perform a glitchy y scroll. _Young Indy_ uses it to make the screen shake when cannonballs hit the ground, and _Zelda II_ uses it to skip scanlines on the title screen. See the notes on accessing [PPUDATA](PPU_registers.xhtml "PPUDATA") during rendering on [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling") page.

## Troubleshooting games

If a scroll split doesn't work, and a garbage sprite shows up around the intended split point, then the game is probably trying to use a sprite 0 hit, but either the wrong tile data is loaded or the background is scrolled to a position that doesn't overlap correctly. This could be a problem with nametable [mirroring](Mirroring.xhtml "Mirroring"), with CHR bankswitching in [mappers](Mapper.xhtml "Mapper") that support it, or with the CPU and PPU timing of whatever happened above the split. _Battletoads_ , for one, uses 1-screen mirroring and requires exact timing to get the background scroll position dead-on. 

## See also

  * [Game bugs](Game_bugs.xhtml "Game bugs"): These games have glitches on NES hardware, so don't go "fixing" them while breaking your emulator.
  * [Sprite overflow games](Sprite_overflow_games.xhtml "Sprite overflow games")
  * [Unofficial opcode games](CPU_unofficial_opcodes.xhtml#Games_using_unofficial_opcodes "CPU unofficial opcodes")
  * [List of games that run code from outside of PRG-ROM](List_of_games_that_run_code_from_outside_of_PRG_ROM.xhtml "List of games that run code from outside of PRG-ROM")



## References

  1. ↑ [thread](https://forums.nesdev.org/viewtopic.php?p=184535#p184535): Battletoads Double Dragon Powerpak Freeze
  2. ↑ [thread](https://forums.nesdev.org/viewtopic.php?p=184539#p184539): Battletoads Double Dragon Powerpak Freeze - pre-loading $FF reliably crashes _BT &DD_, $00 reliably does not.
  3. ↑ [thread](https://forums.nesdev.org/viewtopic.php?p=192541#p192541): KrzysioCart - Home made cartridge that support>80% NES games
  4. ↑ [thread](https://forums.nesdev.org/viewtopic.php?p=211244#p211244): Hong Kong 212 PCB,128+1024?
  5. ↑ <http://forums.nesdev.org/viewtopic.php?p=183064#p183064>
  6. ↑ [http://forums.nesdev.org/viewtopic.php?f=3&t=18111](http://forums.nesdev.org/viewtopic.php?f=3&t=18111)
  7. ↑ [forum thread](https://forums.nesdev.org/viewtopic.php?t=16355): Spot bug in Mesen and Nintaco
  8. ↑ [forum thread](https://forums.nesdev.org/viewtopic.php?f=5&t=1798): Quattro Sports BMX Simulator uses extra controller?
  9. ↑ [forum thread](https://forums.nesdev.org/viewtopic.php?p=283981#p283981): OAM corruption in StarTropics
  10. ↑ [forum post:](https://forums.nesdev.org/viewtopic.php?p=284036#p284036) discussion of Wario's Woods behaviour on real hardware



## External links

  * [Software Index](https://www.smspower.org/Development/Software-Index) on SMS Power
  * [Tricky-to-emulate games](https://gbdev.gg8.se/wiki/articles/Tricky-to-emulate_games) on GbdevWiki
  * [ACCURACY.md](https://github.com/nba-emu/NanoBoyAdvance/blob/master/docs/ACCURACY.md#game-compatibility) from NanoBoyAdvance docs



Categories: [Lists of games](Category_Lists_of_games.xhtml)
