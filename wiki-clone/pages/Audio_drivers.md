# Audio drivers

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Audio_drivers) | View [other pages](Special_AllPages.xhtml#Audio_drivers)

An **audio driver** or **replayer** is a program that runs on a game console to play music. An audio driver for NES reads music sequences and instrument definitions from the ROM and writes to the [APU](APU.xhtml "APU") ports, usually once per frame. 

  


## Contents

  * 1 Tracker replayers
  * 2 Game replayers
    * 2.1 Famistudio music engine
    * 2.2 FamiTone2
    * 2.3 FamiTone 5.0
    * 2.4 GGSound
    * 2.5 Lizard music engine
    * 2.6 NSD.Lib
    * 2.7 Penguin
    * 2.8 Pently
    * 2.9 OFGS
    * 2.10 Sabre



## Tracker replayers

NSFs exported from FamiTracker and 0CC-FamiTracker contain replayers that can handle all tracker features, but they use a lot of ROM and RAM space and can't handle sound effects. 

  * [FamiTracker](http://famitracker.com/) 0.4.2 replayer (0.4.6 is similar) 
    * No sound effect support
    * Size: 245 bytes BSS, 20 bytes zero page, 5547 bytes ROM


  * [Famitracker j0cc](https://famitracker.org/) fork


  * [Musetracker](https://fo.aspekt.fi/)


  * [Nerdtracker II](http://nesdev.org/nt2/)


  * [PPMCK/MML](http://ppmck.web.fc2.com/ppmck.html)


  * [Famistudio](https://famistudio.org/)



  


## Game replayers

Replayers intended for use in games have limits on the instrument, effect, and expansion features they can use. Most can't handle expansions at all, as NES games in an unmodified NES cannot use expansion audio. They usually operate by translating a text export into assembly language data files that are included into the game's source code. 

  


### Famistudio music engine

[Famistudio music engine](http://forums.nesdev.org/viewtopic.php?f=2&t=20825&p=256016) by bleubleu 

Famistudio's driver is originally of FamiTone2 genealogy, but has been massively reworked. Some of its key features is being highly configurable to cater for different needs, and extensive expansion audio support. 

  * Meant to go with its own music tool (Famistudio). Famitracker projects are importable.
  * NESASM, CA65, ASM6 versions



Basic features: 

  * Pulse channels, triangle channel, noise channels
  * C0-B7 note range (96 notes)
  * Instruments with duty cycle, volume, pitch and arpeggio envelopes
  * Absolute and relative pitch envelopes
  * Looping sections in envelopes
  * Release points for volume envelopes
  * Ability to change the speed (FamiTracker tempo mode only)
  * Ability to loop over a portion of the song
  * Up to 64 instruments per export, an export can consist of as little as 1 song, or as many as 17
  * Can enable/disable features to save ROM and RAM
  * VRC6, VRC7, FDS, Sunsoft 5B, Namco 163 support (only one at a time)
  * PAL/NTSC playback
  * DPCM
  * Sound effect support (configurable number of streams)
  * Blaarg Smooth Vibrato technique to eliminate "pops" on square channels (incompatible with SFX at the moment)
  * FamiTracker/FamiStudio tempo mode.
  * Volume track
  * Fine pitch track
  * Slide notes
  * Vibrato effect
  * Arpeggio effect (not arpeggio envelopes in instruments, which are always available)



### FamiTone2

[FamiTone2](https://shiru.untergrund.net/code.shtml) by Shiru 

  * Notes limited to C-1 through D-6
  * Instrument: volume, arpeggio, and pitch envelopes. Duty envelopes longer than 1 frame unsupported. Pitch limited to accumulated distance of 63 units in each direction. No release phase.
  * No volume column
  * DPCM for instrument 0 only
  * Limit of 64 instruments, 17 songs, and 16 KiB of DPCM



Effects: 

  * `Fxx` (speed / tempo)
  * `Dxx` (pattern cut / skip to next frame and start at xx)
  * `Bxx` (loop / jump to frame)



Requirements: 

  * 3 bytes ZP
  * 186 bytes other RAM
  * 1636 bytes ROM (source: README)



  


### FamiTone 5.0

[FamiTone 5.0](https://github.com/nesdoug/famitone5.0) by dougeff 

Modification of FamiTone2 that adds 

  * Adds volume column support
  * Note range A0 - B7
  * Duty cycle envelopes
  * Sound effects that are bigger than 256 bytes



and adds effects: 

  * `1xx` (portamento up)
  * `2xx` (portamento down)
  * `3xx` (glissando)
  * `4xy` (vibrato)
  * `Qxx` (slide up to note)
  * `Rxx` (slide down to note)



  


### GGSound

<https://github.com/gradualgames/ggsound> by Gradual Games 

  * NESASM3, ASM6, and ca65 syntax supported
  * Note range: C0 - B7
  * Instrument: volume, arpeggio, pitch, and duty envelopes, all looping
  * SFX playback on up to two channels
  * Pause / Unpause
  * Note cuts
  * 128 instruments (each with its own set of envelopes)
  * 128 songs
  * 128 SFX



Effects: 

  * `Bxx` (loop / jump to frame).



Difference vs famitracker: Loops each channel individually, so must be placed in all channels for proper song looping. 

Requirements: 

  * 66 bytes ZP (57 without DPCM)
  * 168 bytes other RAM (144 without DPCM)
  * 3048 bytes ROM (out of date, needs test on current version)



  


### Lizard music engine

[Lizard music engine](http://forums.nesdev.org/viewtopic.php?f=2&t=18957&p=239567) by Rainwarrior 

Features (subset of Famitracker): 

  * Volume column supported
  * No DPCM
  * No Hi-Pitch macros (but regular pitch works)
  * Tempo fixed at 150 (use Speed instead)
  * SFX on square 1 and/or noise channel



Effects: 

  * `Bxx` (loop / jump to frame)
  * `D00` (Skip to next frame; no row support)
  * `F0x` (just speed, not tempo)



Requirements: 

  * 22 bytes ZP
  * 105 bytes other RAM
  * 2152 bytes of code and tables
  * ~1800 cycles per frame



  


### NSD.Lib

<http://shaw.la.coocan.jp/nsdl/> by S.W. 

Uses linear pitch. Defaults to an uncommon A=442 tuning, though Damian Yerrick has developed a [tuning table generator](https://github.com/pinobatch/little-things-nes/tree/master/nsdl-tune) in Python. 

### Penguin

[Penguin](https://github.com/pubby/penguin) by pubby 

  * Constant cycle count of 790, allowing use in a raster effect
  * Plays sound effects without using additional cycles
  * Famitracker Exported
  * [Allegedly](https://forums.nesdev.org/viewtopic.php?p=205095#p205095) has most features of FamiTone2
  * No DPCM
  * 12 bytes ZP
  * 86 bytes other RAM
  * Music data not particularly size-optimized
  * Sound effects are expensive in terms of size



  


### Pently

[Pently](https://github.com/pinobatch/pently) by Damian Yerrick 

  * Notes limited to A-0 through C-7; changeable at build time
  * Instrument: volume, duty, and (absolute) arpeggio envelopes. Pitch envelopes unsupported. The volume envelope can't loop, and the duty and arpeggio envelopes stop at the end of the volume envelope. No release phase.
  * Volume column limited to 4 levels
  * No DPCM
  * Limit of 51 instruments, 25 drums, and 255 patterns
  * tempo ("Rows per minute" tempo model allows runtime correction for NTSC, PAL, and Dendy)



Effects: 

  * `45x` (vibrato)
  * `3xx` (portamento)
  * `Sxx` grace note / delayed note
  * `Gxx` grace note / delayed cut
  * legato (change note pitch without restarting envelope)
  * `0xy` (arpeggio with 1- or 2-tick scheme)


  * and loop (`Bxx`)
  * Vibrato and portamento use "linear pitch" model similar to that of [0CC-FamiTracker](https://github.com/HertzDevil/0CC-FamiTracker)



Requirements: 

  * 32 bytes ZP
  * 112 bytes other RAM
  * 1918 bytes ROM
  * Some effects are space-intensive and can be disabled at build time through a configuration file to reduce ROM and RAM size. [A feature set comparable to FamiTone2](https://forums.nesdev.org/viewtopic.php?p=205093#p205093) uses 1283 bytes of ROM.
  * Compatible with ca65. In 2019, an experimental ASM6 port was added. Python 3 is used to preprocess the score and generate the RAM map.



  
The native input format ("Pently score") is inspired by MML and intended to be familiar to users of PPMCK or LilyPond. Conversion from FT text export is through [ft2pently](https://github.com/NovaSquirrel/ft2pently) by NovaSquirrel. As Pently score was originally intended for human writability, some features don't map well onto FamiTracker features, requiring manual configuration of ft2pently or manual editing of the Pently score it produces: 

  * Pattern start has row granularity, allowing a pattern such as a drum fill to replace the end of another pattern.
  * Pattern length is not fixed, allowing long melodic patterns and short drum loops.
  * Patterns are shared among channels. Use this with delayed pattern start for 2-channel echo.
  * Patterns can be transposed. Use this for parallel fourths, fifths, or octaves, or for gear changes like that in the _Mermaids of Atlantis_ soundtrack.
  * Drum channel instead of noise channel. Drums are played as pairs of sound effects that interrupt a note played on the same channel, making Tim Follin-style triangle+noise drums somewhat easier than with the fixed arpeggio that one would use in FT.
  * Envelope divided into an attack phase, similar to FT envelopes, and a decay phase with a constant duty and decreasing volume, similar to NerdTracker II envelopes.
  * "Attack track" mode, where the attack phase of a note on MMC5 channel 1 temporarily overrides the note on pulse channel 1. Useful for staccato ostinatos and occasionally 1-channel echo.
  * Automatic conversion doesn't use the newest features (linear portamento and 2-frame arpeggio) due to converter author's unfamiliarity with 0CC-FamiTracker.



  


### OFGS

<http://offgao.net/ofgs/>

  


### Sabre

<https://github.com/CutterCross/Sabre> by CutterCross 

Features: 

  * CA65 and ASM6 syntax supported
  * Note range: A0-B7 (full pulse/tri note range)
  * Note cuts
  * Speed and tempo
  * DPCM supported for music (not for SFX)
  * Instrument envelopes: Volume, arpeggio (absolute), pitch (relative) and duty.
  * Loop points for all supported envelope types
  * 63 instruments
  * 255 unique envelopes total shared between instruments.
  * 256 tracks
  * 256 SFX
  * 128 patterns per track
  * 1 pattern per SFX
  * NTSC, PAL, Dendy tempo & period adjustments
  * Linear counter trill for Triangle channel
  * Play / Pause / Stop Track routines
  * Mute / Unmute channel routines



Effects supported: 

  * `Bxx` (loop / skip to frame)
  * `C00` (halt / end song)
  * `D00` (skip to next frame; specify row not supported)
  * `Fxx` (speed / tempo)
  * `Zxx` (set DPCM delta counter ($4011); affects tri, noise and DPCM volume)



Requirements: 

  * 42 bytes ZP
  * 121 bytes other RAM
  * 1749 bytes ROM



Notable usage notes: 

  * Instruments share a common envelope set.
  * Song format doesn't lay down redundant note period data if the instrument or length changes
  * Loop/forward points should be in first channel, rather than all channels


