# Limitations

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Limitations) | View [other pages](Special_AllPages.xhtml#Limitations)

Some of the limitations of NES hardware and common mapper hardware severely limit the system's capability to perform well in some genres. This could be part of why these genres took off after 1991 when more powerful hardware became more readily available. 

To clarify something: Some of these limitations don't mean "can't" as much as "too expensive". In a commercial game project, managers have to balance the cost of solving technical problems like those listed below with the cost of exploring, implementing, refining, and balancing the game rules. A game for a powerful system will often be programmed inefficiently because it gets the game out the door faster. Building your own clone (or emulation device) of mappers and other external devices is also possible, although this may also be expensive and/or time consuming. 

[A clone or a port to a less powerful system](https://allthetropes.org/wiki/Videogame_Demake) keeps most of the game design, which is already paid for, and cuts down the design in ways that the porting team thinks the players won't care about. A team of amateurs with no deadline can eventually squeeze more capability out of a system than can a commercial game developer bound by opportunity cost and return on investment. Even commercial developers in a [country with a low cost of living](https://en.wikipedia.org/wiki/Penn_effect "wikipedia:Penn effect"), such as China or Brazil compared to Japan or the United States, have a time-money tradeoff biased toward time: witness the "Hong Kong Original" Famicom reductions of 16-bit fighting games, platformers, and RPGs, and the Brazilian ports of games to Sega Master System and Sega Genesis. 

Other limitations mean that the NES can display something, just not fast enough. The NES CPU can render arbitrary shapes to pixels and send the pixels to CHR RAM through the PPU, and numerous essentially turn-based games have done this. Because of the video memory bandwidth limit, however, it may not happen at a frame rate suitable for real-time games without blitter hardware on the cartridge. 

At times, the mapper capability achievable in the economic environment has _decreased._ From 1994, the end of the licensed NES era, to 2008, when the [CIC](CIC_lockout_chip.xhtml "CIC") was publicly reversed and commercial NES homebrew became viable, there was almost no demand for new NES mappers. Thus due to low volume, the NES scene stepped backward from [MMC3](MMC3.xhtml "MMC3") and [MMC5](MMC5.xhtml "MMC5") to discrete logic mappers such as [UNROM](UxROM.xhtml "UNROM"). Gradually, the cost of programmable logic (CPLDs and FPGAs) decreased to where a custom mapper could be affordably incorporated in a cartridge at volumes of 1,000 units or less. By roughly 2020, such FPGAs were powerful enough to hold designs that outdo MMC5, such as the [Rainbow mapper](NES_2_0_Mapper_682.xhtml "NES 2.0 Mapper 682"). 

## Contents

  * 1 Simulation
  * 2 Creation
  * 3 Driving
  * 4 Puzzle
  * 5 Fighting
  * 6 Bullet hell shooter
  * 7 Party multiplayer
  * 8 Music
  * 9 External links



## Simulation

Limitations: 8 KiB PRG RAM; no 3D graphics hardware 

Some kinds of simulator games, like _SimCity_ , _The Sims_ , _Harvest Moon_ , and _Animal Crossing_ , have large grid-based world maps and tend to need 8 KiB just to store this map, let alone the rest of the world's state. Very few NES cartridge boards (such as [SXROM](SxROM.xhtml "SXROM") and [EWROM](ExROM.xhtml "EWROM")) provided more than 8 KiB, and most of them were rare and/or Japan-only. Nintendo has been known to cancel finished products such as the NES port of _SimCity_ and the English version of _Mother_ because replication cost would kill the return on investment. Games would have to be planned carefully to fit their state into a cheap chip. Nowadays, it's possible to make a custom mapper on a CPLD that bankswitches a large work RAM and saves to an unused area of the PRG flash. 

Games that allow extensive customization of the player's appearance, as in _Animal Crossing_ and _The Sims_ , typically rely on 3D texture mapping so that the changes to the appearance can be seen from all angles. An NES game with customizable characters, such as _Cocoron_ , has to store each animation of each body part seen from each angle and composite them at runtime, either drawing them into CHR RAM or overlaying numerous sprites (and risking flicker). 

**Can software rendering help?** _Tony Hawk_ series for Game Boy Advance soft-renders a 3D player character to CHR RAM in real time, but the GBA CPU is also fast enough to _emulate_ the NES CPU. 

**By the Super NES era:** Several games were 32 KiB or larger battery RAM, and a game using the GSU might barely be able to soft-render a 3D player character, though at a reduced frame rate. 

## Creation

Limitations: Input devices, mostly 

Games that allow for modification, such as modified game graphics, maps, and scripts, tend to be extremely limited. Part of this has to do with input limits: the NES never had an official alphabetic keyboard or precise pointing device. In Japan, _Family BASIC_ shipped with [a keyboard](Family_BASIC_Keyboard.xhtml "Family BASIC Keyboard"), and the _Oeka Kids_ paint program shipped with a graphics tablet, but these use the Famicom's DA15 expansion port, which the NES doesn't have. _Videomation_ , a paint program for the NES with a [unique cartridge board](CPROM.xhtml "CPROM"), has to use a clunky control scheme similar to the speed control in _Image Fight_. (Its animation is also severely limited compared to _Mario Paint_ for Super NES, and it can't save.) In 2011, some NES software was written to use the [Super NES Mouse](Mouse.xhtml "Mouse") through a Super NES controller to NES adapter. 

There are some external devices which can be used for extended save data: the [Famicom Disk System](Family_Computer_Disk_System.xhtml "Famicom Disk System"), the [Family BASIC Data Recorder](Family_BASIC_Data_Recorder.xhtml "Family BASIC Data Recorder"), the [Battle Box](Battle_Box.xhtml "Battle Box"), and the [Turbo File](Turbo_File.xhtml "Turbo File"). However, all are Japan-only, exceptionally rare, and the latter two only hold 8KiB of data anyway. 

**By the Super NES era:** Support for the Super NES Mouse was common, and _Mario Paint_ and the Japan-only _Dezaemon_ led the way. 

## Driving

Limitations: No 3D graphics 

There are two ways to do a driving game: behind-the-car with a _Pole Position_ style track (as seen in _F-1 Race_ and _Rad Racer_) and overhead/isometric style (as seen in _RC Pro-Am_ and _Micro Machines_). In a behind-the-car view, the car can't turn all the way around. The "morphmation" technique used to [bend the racetrack](http://www.extentofthejam.com/pseudo/) limits the variety of scenery and track layout: for example, the track can't have hairpin turns or narrow segments without using an obscenely large multi-bank CHR ROM like that seen in _Cosmic Epsilon_. Overhead view severely limits how far ahead one player can see, resulting in usually slower movement, and the camera generally has to follow one player or the other. (_Micro Machines_ uses the screen edge as a game mechanic.) 

**Can software rendering help?** Not much. The NES game _f-ff_ draws a racetrack to the nametable using coarse pixels each 4x4 NES pixels in size. Any finer than that would be too much data to push in a single frame, as shown by the map of one of the _Final Fantasy_ games. Even on the Super NES, _Race Drivin'_ ended up with an inadequate frame rate. 

**By the Super NES era:** Limit broken initially using mode 7 and later the GSU. 

## Puzzle

Limitations: 16x16 pixel attribute tiles 

A lot of tile-matching games, such as _Columns_ (ported to NES as _Magic Jewelry_ and _Mystic Pillars_), _Yoshi's Cookie_ , _Puyo Puyo_ , _Wario's Woods_ , and _Palamedes_ , have 16x16 pixel tiles. This limits each player's playfield to about 6x12 cells. Games with smaller tiles typically can't go over three unique colors, as seen in _Dr. Mario_ and _Tetris 2_. One compromise involves drawing tiles in both players' playfields as a dithered combination of orange, green, and blue, much as CGA graphics were limited to three colors plus the backdrop. It's not as clean as a Sega Master System game, but it gets the job done without having to destroy one of [about a dozen donor games](https://nescartdb.com/search/advanced?ines_op=equal&ines=5) to use the 8x8 pixel color areas of [MMC5](MMC5.xhtml "MMC5") ExGrafix. Another workaround would be to make a custom mapper on a CPLD that simulates some subset of ExGrafix. _Klax_ uses a mapper with a scanline counter to change the scroll position in mid-frame, making its 16-pixel-wide attribute areas less than 8 scanlines tall. 

Yet another thing that can be done is to make the single-player playfield wider than the 2-player playfield. This was tried in things like _Magical Drop 2_ , _Wario's Woods_ , and the 4-player "Familiss" mode of BPS's _Super Tetris 3_. _Lumines_ for PSP borrows the mechanic of two players fighting over space in a single playfield from _FantaVision_ for PS2, and at least one fan-made clone of the single-player game _Zoop_ makes the game cooperative with two shooters on one big field. 

**By the Super NES era:** Limit suffered comminuted fracture. Super NES has 8x8 pixel color areas, HDMA to change the vertical scroll position, lots of sprites with lots of overdraw, the 12x12 trick in mode 1, and a 256-color playfield in mode 3. 

**By the 2020s:** FPGAs became cheap enough to reignite interest in designing homebrew mappers capable of detail that meets or exceeds that of ExGrafix, such as the [Rainbow mapper](NES_2_0_Mapper_682.xhtml "NES 2.0 Mapper 682") and MXM-1. 

## Fighting

Limitations: [Overdraw](PPU_sprite_evaluation.xhtml "PPU sprite evaluation")

If both players' graphics are drawn as sprites, characters will have to be no wider than 32 pixels. Otherwise, they'll flicker like crazy when you cross the 8-sprite limit. You can get away with wider jumping poses because it's less likely that both characters will be in the air at once. _Shaq-Fu_ was poor in execution, but its core idea of smaller characters could be made to work well on an NES. You could aim for something like _Super Smash Bros._ , using platforms in the playfield as an additional source of tactics. 

The other way to do this, allowing big characters like in _Street Fighter_ , is to draw one character as 8x16 pixel sprites and the other as background tiles. This allows up to 64-pixel-wide characters, or slightly smaller counting projectile attacks. But you have to design the game such that only one player faces either direction, so that you can draw the other player with mirrored sprites. That means you can't easily jump behind or roll past the other player and attack from behind unless the frames for facing the other player's back have dedicated cels. In addition, backgrounds will be plain, and you'll probably need an advanced mapper with a scanline timer to perform multiple scroll splits at the status bar, play area, and ground. 

**Can software rendering help?** Compositing one fighter's pixels over the pixels of background scenery is possible, though not at anywhere near a playable frame rate because of the practical limit of copying 8 tiles from work RAM to CHR RAM per [60 Hz](Cycle_reference_chart.xhtml "Cycle reference chart") vblank. In addition attribute clash remains an issue. It'd look like either a ZX Spectrum game or at best a Super Game Boy game. 

**By the Super NES era:** Limit scarred across the chest with a Shoryuken. A competent port of _Street Fighter II_ was a system seller. However, by _Street Fighter Alpha 2_ , the Super NES was showing its age. Games like _Killer Instinct_ and _Samurai Shodown_ also had to have camera zooming cut out because of limits of mode 7. 

**By the 2020s:** Once platform fighters were seen less as direct clones of _Super Smash Bros._ and more as a genre, there was room for _Super Tilt Bro._

## Bullet hell shooter

Limitations: Overdraw, CPU speed, 5-line OAM DMA delay 

Shoot-em-ups can be hard, like _Recca_ , but they can't have more than 64 things in the air at once. Some platforms (like GBA and Neo Geo) can rewrite part of the sprite table during rendering to expand the sprite capability, but the NES can't without disabling and reenabling rendering. This turns shmups like _Over Obj_ into a flicker-fest. And even then, more than 8 on a line will flicker so bad that bullets will be invisible half the time. 

**Can software rendering help?** Not at anywhere near a playable frame rate unless a game uses an extremely coarse bullet field and little other scenery, as in _Lunar Limit_. 

**By the Super NES era:** Boss and bullet patterns became more complex, but the need for constant access to OAM outside of vertical or forced blanking meant shmups still couldn't overcome the 128-sprite capacity of OAM. Parts of _Yoshi's Island_ , such as the title screen, use the GSU as a compositor; it remains to be seen whether this is adequate for bullet hell. 

## Party multiplayer

Limitations: Sprite palettes; overdraw 

An NES [Four Score](Four_player_adapters.xhtml "Four Score") hub provides four controller ports, one for each of the four palettes. If each player's car, uniform, etc. has the same graphics with a palette swap, there are no palettes left for anything else. If each player uses one palette, other things like projectiles and enemies would need to use the same palette as a player, or they would have to be background objects. It might be easier if the player sprites leave one color unused or if the game splits the players into two teams whose characters aren't palette swaps. Four 16x16 or 16x32 pixel sprites will also hit the 8-sprite limit with no room for projectiles unless the game is designed not to encourage them to be at the same vertical position. (Games using 8x8 or 8x16 pixel player sprites, such as _Micro Mages_ , have more room.) 

Memory accesses during DPCM sample playback occasionally cause a bit from the controllers to be skipped. It takes longer to read 16 bits than 8, making rereading less practical, but the [Four Score](Four_player_adapters.xhtml "Four Score")'s signature bits help detect bit deletions. 

**Can software rendering help?** Nametable animation is possible if coarse movement like that of the first _Gauntlet_ is acceptable. Otherwise, there is too much data to transfer to render four player characters to background tiles at a reasonable frame rate. 

**By the Super NES era:** Limit blown to bits by _Super Bomberman_. Overdraw increased to 32 sprites with 34 tiles and sprite palettes increased to eight, leaving plenty of room for players and their projectiles. And no more sample playback glitch. 

## Music

Limitations: Audio hardware, storage 

A [rhythm game](https://en.wikipedia.org/wiki/rhythm_game "wikipedia:rhythm game") for the Famicom could put a digital audio player on the PCB. The [JF-13](INES_Mapper_086.xhtml "JF-13") board in the Japanese version of _Bases Loaded_ does just this for the umpire's voiceover. But unlike the 60-pin Famicom cart edge, the 72-pin NES cart edge lacks a pair of pins for an audio signal. Instead it has ten pins connected directly to the NES-001 Control Deck's [expansion port](Expansion_port.xhtml "Expansion port"). A 47 kΩ resistor from one of the cart pins to an audio pin on the expansion port enables audio out of the expansion port as on a Famicom, but Nintendo never released an accessory with this resistor, and top-loading NES-101 consoles don't even have this port. So NES games usually have to use the 2A03's internal [APU](APU.xhtml "APU") channels. A mapper could trigger an IRQ when each sample is ready and offer a digital output on $4011 so that `inc $4011` to read a sample from the mapper and write it back to the APU, but it was determined that OAM DMA inserts 290-microsecond gaps that audibly interfere with that approach.[[1]](https://forums.nesdev.org/viewtopic.php?p=101429#p101429)

Popular rhythm games, such as _Beatmania_ , _Dance Dance Revolution_ , and _Amplitude_ , use CD-quality audio played from a disc. Though the NES APU is capable of competent covers of popular music, as seen in _D-Pad Hero_ , a lot of players demand the original studio recordings, or at least an electric guitar that sounds like an electric guitar and chords that don't sound [warbly](Arpeggio.xhtml "Arpeggio"). Witness poor sales of the Game Boy Color versions of _Beatmania_ and _DDR_. 

Apart from _DDR: Disney Dancing Museum_ for the Japanese N64 and games on Nintendo handhelds, most games like this have come out on platforms with hundreds of megabytes of cheap storage: PCs and disc-based consoles. A lot of rhythm games especially for consoles use a custom controller, shaped like a musical instrument or like the [Power Pad](Power_Pad.xhtml "Power Pad"). Because cartridges were much more expensive to replicate, bundling a controller with your game cartridge was cost prohibitive. Did anyone actually _buy_ the Miracle Piano Teaching System? 

Disc-based games using a standard controller, like _Parappa the Rapper_ and _Frequency_ , came on discs 600 MB to 4 GB in size and thus could afford to store the music tracks as two long samples: one for background music and one for the foreground instrument. (This pattern is called "keysound" in the _Beatmania_ simulation community, especially in cases where the foreground instrument's samples repeat throughout a song to save space.) _Guitar Hero On Tour_ games for Nintendo DS are 128 MB each after several Moore's law doublings of ROM density. But with a typical NES ROM size of 128 to 512 KiB, a big fat sample like that is out of the question. 

A music cartridge titled _A Winner Is You_ contains a 64 MB ROM. It spends practically all CPU time copying sample data from the cartridge to the PCM registers. This leaves little time to visually move the step chart, as finding flat points of the waveform long enough for even an [OAM](PPU_OAM.xhtml "OAM") DMA update isn't always easy. 

**By the Super NES era:** The audio subsystem supports streaming vocals from ROM through HDMA, but ROM was still limited to 32 Mbit (4096 KiB) with a handful of largely Japan-only exceptions. 

**By the 2020s:** The game _Tactus_ , designed for the Rainbow mapper, uses [expansion sound equivalent to a VRC6](VRC6_audio.xhtml "VRC6 audio"). It copies samples from the cartridge to the APU using the `inc $4011` instruction. To avoid OAM DMA, it writes to $2004 using self-modifying code, and to circumvent the loss of vertical blanking time caused by programmatic writes to $2004, it writes nametable data to pseudo-dual-port ExRAM in the mapper. The game _Former Dawn_ by Something Nerdy Studios ships with a jumper board to route audio through a front-loader's expansion port. 

## External links

  * [Forum: Easier and harder game genre to code/developp](https://forums.nesdev.org/viewtopic.php?p=26438#p26438)
  * [Forum: The essence of great games on the NES](https://forums.nesdev.org/viewtopic.php?p=44710#p44710)
  * [Preliminary analysis of what would have to be cut to put _Animal Crossing_ on the NES](https://pineight.com/mw/index.php?title=Animal_Crossing_\(NES_game\))
  * [Forum: Proposed SRAM layout for port of _The Sims_ to NES](https://forums.nesdev.org/viewtopic.php?p=53873#p53873)


