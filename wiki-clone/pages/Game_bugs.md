# Game bugs

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Game_bugs) | View [other pages](Special_AllPages.xhtml#Game_bugs)

## Contents

  * 1 General bugs
  * 2 Reliance on power-on mapper state
  * 3 Reliance on RAM values
  * 4 "Impossible" controller input
  * 5 Overscan ugliness
  * 6 References



Listed are games that have been tested on NES or Famicom hardware and verified to look wrong or odd. This can be caused by NES hardware limitations, programming errors, or even intentional effects within the game. Refer to this if you're developing an emulator and find a game that looks wrong, before you look for a problem in your emulator. If you are attempting to give your emulator ["bug for bug" compatibility](Accuracy.xhtml "Accuracy"), you'll want to make sure that these glitches (or unusual behaviors) appear the same in your emulator as they do on the NES. 

This is an incomplete list that concentrates on commercial games. For an overview of common compatibility problems in homebrew games, see [Program Compatibility](Program_compatibility.xhtml "Program Compatibility"). Sometimes, a bug slips into a game to make it rely on less-than-intentional features of the hardware; for those, see [Tricky-to-emulate games](Tricky_to_emulate_games.xhtml "Tricky-to-emulate games"). 

## General bugs

[![](../wiki-images/Smb3-attribute-glitch.png)](File_Smb3_attribute_glitch_png.xhtml)

[](File_Smb3_attribute_glitch_png.xhtml "Enlarge")

Attribute glitch in _Super Mario Bros. 3_

Game  | Problem   
---|---  
_The Addams Family_ | The in-game status bar occasionally bumps vertically by 1 pixel, caused by non-solid background pixels overlapping the sprite zero that is used for the status bar split.   
_Ai Senshi Nicol_ | Loading CHR-RAM can fail due to a race condition, causing level graphics to be incorrect. This occurs if an NMI lands between the two [PPUADDR](PPU_registers.xhtml "PPUADDR") writes for the graphics copy, because the NMI handler reads [PPUSTATUS](PPU_registers.xhtml "PPUSTATUS"), clearing the address write latch. Whether this happens depends on FDS disk access timing, which does not take a well-defined, fixed amount of time. It is not clear if this can happen with real-world timing variance on real hardware, but it is a concern for emulators.   
_Akumajou Densetsu_ | 

  * When a door is opened/shut when the player goes through them, the screen shakes by one pixel.
  * During Stages 6-4 and 6-5, the game uses horizontal scrolling with [vertical arrangement](Mirroring.xhtml "Mirroring") (with one nametable reserved for the rising water) and [masks the leftmost column](PPU_registers.xhtml "PPUMASK"), causing tile/attribute glitches at the left edge and clipping part of the status bar. _Castlevania III_ instead uses a third nametable from the [MMC5](MMC5.xhtml "MMC5")'s ExRAM for the rising water, preventing these issues.

  
_Akumajou Special_ | The game over screen is sometimes glitched when it occurs in bidirectional scroll stages. (Validated in Game Center CX.)   
_Armadillo_ | 

  * The in-game status bar occasionally bumps vertically by 1 pixel.
  * Slowdowns occur frequently.
  * Crashes on Dendy consoles - fix: [[1]](https://forums.nesdev.org/viewtopic.php?p=270654#p270654)

  
_Asmik-kun Land_ | Sets the triangle period to $0003 at the end of snare/tom pitch bends, causing high-pitched (but not ultrasonic) beeps in some music tracks.   
_Castlevania_ | 

  * Revision 0 of the US version is prone to crashing when too many sprites are active (for example, when fighting Death), due to the game loading the wrong ROM bank after jumping to the NMI routine.
  * In the US versions, due to [DMC DMA controller corruption](APU_DMC.xhtml#Conflict_with_controller_and_PPU_read "APU DMC"), holding Up when Simon is hit can occasionally cause the game to pause. _Castlevania_ only uses DPCM for a short grunt sample when Simon is hit, and the US versions do not have any controller re-read code, instead seemingly relying on inputs being ignored while the sample plays; Start is still processed to allow pausing, however, and if DMC DMA causes a controller bit deletion before Start is read, an Up input will be read as Start instead. The Japanese versions (FDS and Famicom) use controller re-read code, so they are exempt from this issue.

  
_Castlevania II - Simon's Quest_ | Sometimes incorrect tiles are shown on the playfield.   
_Castlevania III - Dracula's Curse_ | 

  * The [DMC channel](APU_DMC.xhtml "APU DMC") in music sometimes mutes.
  * When pressing the B button at the exact time that Trevor falls off a block, the whip sound plays, but Trevor doesn't attack.

  
_Chip 'n Dale Rescue Rangers_ | As the intro sequence is fading out, the game displays sprites with incorrect tiles in the positions for the player initials and life indicators at the top of the screen. When the screen fades back in, the same positions have the correct graphics.   
_Commando_ | Uninitialized nametables and garbage sprites are frequently visible.   
_Crystalis_ | The scanline above the status bar and above text boxes looks wrong.   
_Dizzy The Adventurer_ | Resets the sound phase every frame, causing a nasty 60Hz buzz.   
_Donald Land_ | When the player progresses too quickly by boosting off of apples, the background loads fall behind and the scroll seam becomes visible.   
_Door Door_ | [Palette RAM](PPU_palettes.xhtml "PPU palettes") is not initialized correctly if the VBL flag ([PPUSTATUS](PPU_registers.xhtml "PPUSTATUS") bit 7) is set on power-up/reset due to the PPU's [power-up state](PPU_power_up_state.xhtml "PPU power up state").   
_Double Dragon_ | 

  * "Garbage sprites" (sprite 0 (for [sprite-0 hit](PPU_OAM.xhtml "Sprite-0 hit")) and sprite 1) can be seen in the lower right of the game screen. 
    * Sprite 0 consists of tile $FF (a black tile with 2x2 non-background pixels (i.e. a tile with a 2x2 "dot" in it, visually similar to ▣ or ⚀)), and the priority bit set.
    * Sprite 1 consists of tile $FE (a tile consisting entirely of a single non-transparent colour, often palette entry $0F but varies per stage).
  * The screen will sometimes shake vertically on heavy sprite usage.

  
_Double Dragon II_ | The status bar may suddenly change colors: sometimes when scrolling vertically it shows incorrectly for a couple of frames.   
_Double Dragon III_ | Same status bar issue as _Double Dragon II_.   
_DuckTales_ | The scroll split between the status bar and playfield is placed in the middle of the status bar's bottom-most scanline, causing it to jitter when scrolling horizontally. Additionally, the game uses different sprites for the status bar and playfield that are swapped mid-frame via a [PPUCTRL](PPU_registers.xhtml "PPUCTRL") write, and hides the status bar's sprites whenever the player's metasprite overlaps it, further complicating sprite 0 hit behavior and causing the bottom-most scanline to scroll across the entire screen instead.   
_Duck Maze_ | The original 60-pin release for famiclones relies on decimal mode working as it does in a 6502. Without decimal mode, score counting works wrong, and the game may never finish counting down at the end of a level. (Fixed for HES's 72-pin rerelease.)   
_Eliminator Boat Duel_ | 

  * Occasionally does not boot on Dendy-style PAL famiclones when the [PPUSTATUS](PPU_registers.xhtml "PPUSTATUS") read in the 8-cycle [warm-up loop](PPU_power_up_state.xhtml#Best_practice "PPU power up state") always coincides with the exact start of vblank.
  * Audible buzzing due to the [DMC channel](APU_DMC.xhtml "APU DMC") being enabled in its [power-on state](CPU_power_up_state.xhtml "CPU power up state") every frame.[1]

  
_Exed Exes_ | When pausing, the immediate next note of the music will play after the pause jingle completes.   
_F-1 Race_ | 

  * The game displays garbled graphics and/or freezes if the Start button is held during power-up/reset.
  * Will not boot on NES-001 (frontloader) due to its [PPU warm-up loop](PPU_power_up_state.xhtml#Best_practice "PPU power up state") being too short.

  
_F-15 City War_ | The last boss is glitched when playing through the game from start to finish in its original [INES Mapper 173](INES_Mapper_173.xhtml "INES Mapper 173") and AVE "1.x" version. It is not glitched when jumping to the last boss through cheating in those versions, and never glitched in the later AVE "1.1" and Gluk Video versions.   
_Ghostbusters (J)_ | Loads the ending text from the wrong CHR bank, causing it to display a blank screen that eventually scrolls "りり" (hiragana "riri") on the screen.   
_Ghosts 'n Goblins_ | 

  * Relies on the VBL flag ([PPUSTATUS](PPU_registers.xhtml "PPUSTATUS") bit 7) being clear to [wait before enabling NMIs](PPU_power_up_state.xhtml#Best_practice "PPU power up state") during the reset handler. If it is set, the write to [PPUCTRL](PPU_registers.xhtml "PPUCTRL") occurs too early and the game fails to boot.
  * Garbage sprites sometimes appear on-screen due to incomplete display lists being uploaded to OAM. (see _Strider_)

  
_Gimmick!_ | The game's controller polling routine relies on extra [DMC DMA controller corruption](APU_DMC.xhtml#Conflict_with_controller_and_PPU_read "APU DMC") only present on select hardware revisions (original RF, Twin, and Titler Famicoms) to detect if a controller read was corrupted. On NES consoles and the New/AV Famicom, DMC DMA glitches are not as pronounced, causing the routine to accept a corrupted read. This results in occasional spurious inputs such as pausing when holding up, or turning/inching rightwards while idle. The PAL 2A07 CPU does not have this conflict, so the glitch is avoided entirely in the European release.   
_Hebereke_ | 

  * The triangle channel in the title screen music is silent, except after exiting from the password screen.
  * The HUD disappears when the player overlaps its vertical position, due to both using the same CHR ROM sprite bank that is swapped mid-frame.

  
_Hottarman no Chitei Tanken_ | If gameplay slows down due to excessive on-screen objects, scrolling may glitch and display garbage values.   
_The Immortal_ | Uses color $0D (blacker than black) as black and color $0E (normal black) as a darker gray than color $2D while setting all three deemphasis bits, causing picture instability on many television sets and TV capture cards. (see [Color $0D games](Color__0D_games.xhtml "Color $0D games"))   
_Ironsword: Wizards and Warriors II_ | Noise channel doesn't work properly, sometimes plays longer notes and sometimes mutes.   
_Kirby's Adventure_ | 

  * When Kirby copies a new ability, the status bar icon may flicker or display incorrect attributes. (?)
  * Slowdowns occur frequently.
  * Jump and attack inputs are sometimes ignored.

  
_The Legend of Zelda_ | Vertical scrolling between screens is very messy, jumping by 3-4 scanlines because of an incorrect fine y from the $2006 writes, two reads from $2007, and imprecise timing on these accesses that can cause a $2007 read's y increment to be lost.[2] Vertical scrolling while DPCM is playing can cause the vertical scroll to momentarily jump due to conflicts between 2nd $2006 writes and automatic fine y increments. See [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling") for details on how these registers interact with scrolling.   
_The Lord of King_ | Will not boot on NES-001 (frontloader) due to an inadequate [wait before enabling NMIs](PPU_power_up_state.xhtml#Best_practice "PPU power up state"). Fixed in _Astyanax_.   
_Magic John_ | Same as _The Lord of King_. Fixed in _Totally Rad_.   
_Mega Man_ | 

  * When pausing and unpausing the game with the Select button, the high bits of the triangle channel's period are not rewritten until Mega Man's teleporting animation finishes, causing distorted, high-pitched noise due to them being left at 0 until then.
  * If Fire Man is defeated and the stage clear orb is obtained while his ground flame remains active, it will incorrectly use tiles loaded for the stage clear text before being deleted, due to them occupying the same space in CHR RAM.

  
_Mega Man 3_ | 

  * The first scanline of the weapon menu and the scanline above Shadow Man in the stage select screen are glitched due to improperly timed scroll splits.
  * In revision 0 of the PAL version, when the Wily Machine is fought, the floor heavily jitters up and down across the screen. This is because two [MMC3](MMC3.xhtml "MMC3") scanline counter IRQs are used for the Machine and the floor, but after the first IRQ, the new reload value (116) is written to $C000 without a write to $C001, which can sometimes happen at the same time the counter is clocked from 0 by PPU A12, overwriting it with the old reload value (58) and causing the split to happen much earlier. This was fixed in later revisions by changing the timing of the $C000 write.

  
_Mega Man 5_ | In Gyro Man's stage, inside the two elevators, the floor moves up by a few pixels when the elevator goes up, and move back down when the screen is fast-scrolled.   
_Metroid_ | The ending song of the Japanese version is [supposed to use volume envelopes](https://youtu.be/NXlgXfI-6eQ?t=859), yet the in-game playback plays [without them](https://www.youtube.com/watch?v=YpLP8aGXcWs&t=3208s). The reason is that the memory location that holds the value that will be written to $4080 on the next start of a note is re-used for some other purpose (routine at PC $6779 in revision 3).   
_Micro Machines_ | Graphical problems on PPU revisions prior to 2C02G due to [OAMDATA](PPU_registers.xhtml "OAMDATA") unreadability. Resetting on Famicom and NES-101 (toploader) doesn't always work because the game fails to clear [PPUCTRL](PPU_registers.xhtml "PPUCTRL") and [PPUMASK](PPU_registers.xhtml "PPUMASK") on reset.   
_Mitsume ga Tooru_ | Garbage data is visible in the upper side of the status bar by when shaken by an earthquake, due to the status bar and playfield both sharing the nametables.   
_Othello_ | The FDS DV 2 and Famicom versions can crash and show garbled graphics[3] during CPU matches, due to a stack overflow caused by nested subroutine calls in the CPU player logic. Seemingly fixed in FDS DV 3,5(?), leaked Rev 1 FC, and NES versions.   
_Othello_ (Famiclone) | The original 60-pin release for Famiclones relies on decimal mode working as it does in a 6502. Without decimal mode, spots are counted wrong. (Fixed for HES's 72-pin rerelease.)   
_Panic Restaurant_ | The in-game status bar is bumped up by 1 pixel.   
_Rad Racer_ | Steer to the far left, and a background scanline will be seen on the road.   
_Rambo_ | One scanline is occasionally glitched, for the same reason as in _Super Mario Bros._[4]  
_Rampart_ (Jaleco) | During build phase, the drums (on the noise channel) drop out fairly early.   
_Snow Bros._ | When you clear the stage and rise to the next floor, incorrect CHR bank switching results in glitches in the new floor's graphics.   
_Solar Jetman_ (NTSC version) | Some songs use the sweep registers heavily, which are not restored after a sound effect plays that uses the sweep registers as well. The PAL version seems to have corrected this error.   
_StarTropics_ | The music data for two songs were not set up correctly.[5]

  * The island map music (NSF track 1) has a problem with the second pulse channel: it is intermittently silent or playing the wrong notes after the first minute or two, because the music data was not made to fit/repeat properly.
  * The music for one of the ending cutscenes has a silent triangle channel except for a single glitched note due to an error in the music data.

  
_Strider_ | The NMI handler pushes an incomplete display list to OAM during lag frames, leading to visible garbage sprites.[6]  
_Super Cars_ | The top of the screen flickers on PPU revisions prior to 2C02G due to [OAMDATA](PPU_registers.xhtml "OAMDATA") unreadability.   
_Super Mario Bros._ | 

  * The status bar shakes horizontally on heavy sprite usage and the music slows down. This can be seen especially in worlds 6-4, 7-4 and 8-4, where the large number of hammer objects created by Bowser's code causes the processing time to overshoot a frame. NMIs are disabled on entry to the NMI code and only reenabled when the CPU is "idle", thus when the overshoot occurs, the CPU "misses" a frame, causing general slowdown and status bar shakiness.
  * At various parts of 1-2, in certain CPU/PPU alignments, a single scanline gets glitched. This is caused by writing to [PPUCTRL](PPU_registers.xhtml "PPUCTRL") to reenable NMI at the exact end of the previous scanline, causing the PPU to begin that scanline from the first nametable instead of the second.[7]
  * When hitting a `?` block while maintaining a run at top speed, after the bounce animation finishes, the block disappears for up to two frames before settling in the background. (Updating the nametable at the [scroll seam](PPU_scrolling.xhtml "PPU scrolling") has priority.)

  
_Super Mario Bros. 3_ | 

  * The first scanline after a scroll split is glitched. This shows up as garbage above the left side of the status bar and as incorrectly scrolled lines in the "spade" (not N-spade) bonus game.
  * Note blocks containing items become squarer for a second while the item is popping out. (This is an artifact of the [sprite priority](PPU_sprite_priority.xhtml "Sprite priority") exploit that it uses.)
  * If a Hammer Bros. battle ends precisely when a note is starting, the note will freeze on an incorrect duty cycle.
  * Big fat attribute glitch on the right side of most levels, because this game uses horizontal scrolling with [vertical arrangement](Mirroring.xhtml "Mirroring"). Discussed heavily.[8]

  
_Teenage Mutant Ninja Turtles_ | Sprite overflow (due to large numbers of enemies) causes the status bar to flicker.   
_Tetris_ (Bullet-Proof Software)__|  Sets [APU Triangle](APU_Triangle.xhtml "APU Triangle") period to longest value instead of actually muting it.   
_Tetris 2 + Bombliss_ | The 2-player mode will become softlocked (requiring a reset) if the second player presses the start button to pause the game, as the game does not properly account for the start/select buttons being present on the second controller in this scenario. This is possible if expansion port controllers are connected, or if the game is being played on the NES or the New/AV Famicom.   
_Zelda II: The Adventure of Link_ |  Reads from [PPUDATA](PPU_registers.xhtml "PPUDATA") during the title screen twice, moving the background upward by 2 scanlines after the split point.   
_Zombie Nation_ | Same as _Tetris_ (Bullet-Proof Software), above   
  
## Reliance on power-on mapper state

Game  | Problem   
---|---  
_Digital Devil Story: Megami Tensei II_ |  When powered-on with uninitialized SRAM, a "DDS II" logo is flashed from left to right before proceeding with the normal introduction. The logo's background looks garbled because the game has not fully initialized the [CHR bank select registers](INES_Mapper_019.xhtml#CHR_and_NT_Select_\(%248000-%24DFFF\)_w "INES Mapper 019") at that point.   
_Mega Man 5_ | Neglects to [disable MMC3 IRQs](MMC3.xhtml#IRQ_disable_\(%24E000-%24FFFE,_even\) "MMC3") on reset and executes a CLI instruction before the game has fully initialized, causing the game to not boot if an [IRQ](IRQ.xhtml "IRQ") was pending at that point.   
_Populous_ (Prototype) | Neglects to initialise the [MMC1 control](MMC1.xhtml#Control_\(internal,_%248000-%249FFF\) "MMC1") and [CHR bank 0](MMC1.xhtml#CHR_bank_0_\(internal,_%24A000-%24BFFF\) "MMC1") registers. CHR-RAM contents will be filled with incorrect data at power-on if CHR bank mode = 1 (switch two separate 4 KB banks) and CHR bank 0 = 0.   
  
## Reliance on RAM values

Several games erroneously rely on RAM areas being pre-populated with certain values at power-on, despite RAM contents not being in a consistent state on power-on. Other games rely on similar values, but in PRG-RAM (WRAM), or CHR-RAM. 

Note that using power-on RAM content as a seed for _random_ number generation (such as in Final Fantasy) is not a game bug, even if it makes speedruns harder to verify on console. 

Game  | Problem   
---|---  
_Cheetahmen II_ | Suspected that certain RAM values may affect being able to reach the final two levels of the game (levels 5 and 6).[9]  
_Chinese Kungfu: 少林武者_ | The game will not display the first self-running demo correctly if $0707 contains the value $FF at startup. Values $00-$09 will cause one of nine self-running demo sequences to play first, while values above that will cause the game to always begin with the first demo sequence. For the Western localization (_Challenge of the Dragon_ , not to be confused with the Color Dreams game of the same name), the developers seemed to have noticed this problem and went out of their way to initialize this memory location with $00.   
_Cybernoid_ | On the title screen, the default selection for the difficulty level changes based on the contents of RAM at power on. Also, the music may not start when starting a game (depending on uninitialized RAM values).[10]  
_Dancing Blocks_ (Sachen) | Game will not boot when addresses $EC and $ED are both set to $FF.[11]  
_Erika to Satoru no Yume Bouken_ | Plays uninitialized sound RAM as a waveform on the title screen, resulting in a buzzy tone on some power-ons[12].   
_F-1 Race_ (Beta Version only) | Title screen will be skipped if $6B and $70 contain non-zero values.[13] Game blindly reads and uses values from $51, $55, $70, $A4, and $0200-02FF (via sprite DMA).[14]  
_Famicom Jump II: 最強の7人_ | If save RAM is initialized to all 00s, the game will freeze at the very first power-on with a black screen. It will work normally after a soft reset as well as subsequent power-ons.   
_Gun.Smoke_ (FDS version only) | The music player's RAM is not cleared before starting the title screen song, resulting in a garbage first noise channel note, with random properties, if that RAM is not zero.   
_Huang Di_ | Uses uninitialized RAM at $0100 to determine if Cheat Mode is enabled or not. When it's zero, cheat mode is enabled, allowing infinite jumps in midair.   
_Huge Insect_ | Artifacts show up on the screen when nametable RAM is initialized with random values (the game only appears to initialize one of the 2 nametables).   
_Keroppi to Keroriinu no Splash Bomb!_ | Can crash during the final boss if any of $0680-$069F contain any value between $30-$33. This is because the game's playfield data begins 2 rows down from the top of the screen, so when checking for playfield collision, the position of the falling fire is adjusted by $20 and underflows at the top of the screen, using uninitialized RAM after the playfield for these non-existent rows. If the value indicates the fire can land there, the game uses an invalid jump table index and crashes.   
_Layla_ | The music player's RAM is not cleared before starting the title screen song, resulting in a flubbed first note, sometimes with a frequency sweep, depending on power-on RAM content.   
_Minna no Taabou no Nakayoshi Daisakusen_ | Requires that address $11 be initialized to a value other than $00, otherwise the game will not start.[15] A PRG1 version corrects the issue.   
_Silva Saga_ | When save RAM is initialized with all 0s, the game incorrectly creates 3 blank saved games which do not work properly.[16]  
_Super Mario Bros._ | Blindly reads and uses variables used for the continue feature's starting world number (accessed by pressing A+Start on the title screen) and the hard mode/second quest toggle. _Tennis_ is often used to access "glitch worlds" beyond world 8 as a result of this.[17]  
_Super Mario Bros._ (bootleg versions) | Relies on portions of RAM containing $00, otherwise player starts at world 0-1.[18]  
_Terminator 2: Judgment Day_ | The copyright screen is skipped if RAM is filled with $00 (more generally, if a high score table checksum happens to be valid).[19]  
_Ultima: Exodus_ | Relies on PRG-RAM contents before they're initialised; a fresh/new game may see artifacts such as shaking/wobbly text during the initial text intro screens, corruption of text intro screen fonts, and possibly audio anomalies.[20]  
  
## "Impossible" controller input

Many games behave oddly when button combinations that would be impossible (or at least very hard) to input on a [standard controller](Standard_controller.xhtml "Standard controller") are pressed. This comprises pressing left+right and up+down simultaneously. Such impossible controller input should probably be prevented by default in an emulator, but they can be optionally allowed for experimental purposes. 

Alternatively, [controller reading routines](Controller_reading_code.xhtml "Controller reading code") in most games combine inputs from both the standard ports and the Famicom's [expansion port](Expansion_port.xhtml "Expansion port") (even in international versions) to support [replacement controllers](Four_player_adapters.xhtml#Adapters "Four player adapters"), so it is possible to press one direction in a standard controller and simultaneously the opposing direction in the replacement controller to achieve the same effect. 

Game | Problem   
---|---  
_Bad Street Brawler_ | A [Power Glove](https://www.nesdev.org/w/index.php?title=Power_Glove&action=edit&redlink=1 "Power Glove \(page does not exist\)") Gaming Series game which maps a unique attack to left+right. It instantly kills one enemy per stage.   
_Battletoads_ | Pressing up+down in the vertical tunnel level kills the player instantly. Additionally, pressing left+right causes the player to walk up/back even when in pure 2D stages, which can result in certain areas becoming impossible to complete.   
_Mega Man 1_ and _2_ | By pressing up+down at the top of a ladder, one may enter the "climbing ladder" state briefly above the top of the ladder. This allows "zipping" through walls.   
_Panic Restaurant_ | Pressing up while crouching (by pressing down, thus pressing up+down simultaneously) the player character's sprite uses garbage data including the damage sprite. This does not occur if up is pressed before down; the player chef merely stands still.   
_Predator_ | Pressing left+right+A/B in normal levels, or up+down+A/B in the level number screen before 'big mode' levels, skips the current level. Inputting these is possible on a Famicom by connecting a controller to the expansion port and splitting the directional inputs. The game merges these inputs and explicitly checks for the combinations.   
_Spy vs. Spy_ | The character turns into an airplane and other garbage sprites when pressing left+right or up+down.   
_Star Soldier_ | Cheat code button combinations include left+right and left+right+up+down inputs, which rely on the fact that both controller inputs are ORed bitwise when polling.[21]  
_Super Mario Bros. 2_ | Ladders can be climbed very fast by pressing up+down.   
_Teenage Mutant Ninja Turtles_ | If you press the attack button while you jump while pressing up+down, the player character uses a garbage sprite, and attacks will use unusual (i.e. garbage data) hitboxes.   
_Teenage Mutant Ninja Turtles II: The Arcade Game_ | When you jump with a left or right move while pressing up+down, the player character will move in unusual directions and speeds, possibly screen-wrapping.   
_Tetris_ (Nintendo) | Holding left+down+right will cause the current and next piece sprites to flicker, slowing the game down in the process.   
_Tiny Toon Adventures_ | The player can gain unusual speed when pressing left+right.   
_Zelda II: The Adventure of Link_ | Link can gain tremendous speed when pressing left+right.   
  
## Overscan ugliness

Some games display junk tiles in the [overscan](Overscan.xhtml "Overscan") area, which is usually not seen (or is at least partially obstructed) on real TV sets. Examples include the NTSC versions of _Metal Gear_ (e.g. in the jungle area when gameplay starts) and _Solstice_ (on the title screen). 

  * [![Metal Gear](../wiki-images/Metal-gear-junk-tiles.png)](File_Metal_gear_junk_tiles_png.xhtml "Metal Gear")

_Metal Gear_

  * [![Solstice – The Quest for the Staff of Demnos](../wiki-images/Solstice-junk-tiles.png)](File_Solstice_junk_tiles_png.xhtml "Solstice – The Quest for the Staff of Demnos")

_Solstice – The Quest for the Staff of Demnos_




## References

  1. ↑ [Eliminator Boat Duel](https://forums.nesdev.org/viewtopic.php?t=18278)
  2. ↑ [Zelda Screen Transitions are Undefined Behaviour](https://www.gridbugs.org/zelda-screen-transitions-are-undefined-behaviour/): A deep dive into Zelda scrolling that is largely good until the "Scroll Down to Scroll Up" section, which has significant factual errors.
  3. ↑ ["オセロ LEVEL３がバグる ファミコン" ("Othello level 3 CPU bugged on Famicom") by Hiroshi Tsuchiya](https://youtu.be/28QRWfkvN7E) (Japanese, 8 minutes)
  4. ↑ <http://forums.nesdev.org/viewtopic.php?p=154894#p154894>
  5. ↑ ["Fixing StarTropics" by Brad Smith](https://www.youtube.com/watch?v=EmUvGH2HMuc) (5 minutes)
  6. ↑ ["The Garbage Sprites in Strider (NES) - Behind the Code" by Displaced Gamers](https://www.youtube.com/watch?v=01aBYq91KnA) (15 minutes)
  7. ↑ <https://forums.nesdev.org/viewtopic.php?p=112424#p112424>
  8. ↑ [http://forums.nesdev.org/viewtopic.php?f=10&t=7844](http://forums.nesdev.org/viewtopic.php?f=10&t=7844)
  9. ↑ <http://forums.nesdev.org/viewtopic.php?p=178154#p178154>
  10. ↑ [http://tasvideos.org/forum/viewtopic.php?p=463659&sid=7cfe55942ee3420275d8f16b2a59770a#463659](http://tasvideos.org/forum/viewtopic.php?p=463659&sid=7cfe55942ee3420275d8f16b2a59770a#463659)
  11. ↑ [http://tasvideos.org/forum/viewtopic.php?p=463659&sid=7cfe55942ee3420275d8f16b2a59770a#463659](http://tasvideos.org/forum/viewtopic.php?p=463659&sid=7cfe55942ee3420275d8f16b2a59770a#463659)
  12. ↑ [N163 Sound RAM Initialisation Behaviour](https://forums.nesdev.org/viewtopic.php?p=285135)
  13. ↑ <http://forums.nesdev.org/viewtopic.php?p=178168#p178168>
  14. ↑ <http://forums.nesdev.org/viewtopic.php?p=178015#p178015>
  15. ↑ <http://forums.nesdev.org/viewtopic.php?p=185663#p185663>
  16. ↑ [https://forums.nesdev.org/viewtopic.php?f=11&t=11045](https://forums.nesdev.org/viewtopic.php?f=11&t=11045)
  17. ↑ ["Access Glitch Worlds in Super Mario Bros. via NES Tennis" by Retro Game Mechanics Explained](https://www.youtube.com/watch?v=hrFHNgJlJSg) (13 minutes)
  18. ↑ <http://forums.nesdev.org/viewtopic.php?p=178163#p178163>
  19. ↑ <http://forums.nesdev.org/viewtopic.php?p=180752#p180752>
  20. ↑ <http://forums.nesdev.org/viewtopic.php?p=185163#p185163>
  21. ↑ [Star Soldier cheat codes](https://taotao54321.github.io/StarSoldierResource/cheat/) (Japanese)


