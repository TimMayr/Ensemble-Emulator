# Tools

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Tools) | View [other pages](Special_AllPages.xhtml#Tools)

Useful tools for NES development. 

See also: 

  * [Projects](Projects.xhtml "Projects")
  * [Emulator tests](Emulator_tests.xhtml "Emulator tests")



## Contents

  * 1 Assemblers, compilers, and PRG-oriented tools
    * 1.1 Assemblers
      * 1.1.1 Commonly used assemblers
      * 1.1.2 Other assemblers
    * 1.2 Disassemblers
    * 1.3 IDEs
    * 1.4 Compilers
    * 1.5 Converters
    * 1.6 Pre-processors and other code (PRG) tools
    * 1.7 Compression related tools
  * 2 Graphics-oriented tools
    * 2.1 General NES graphics studios
    * 2.2 Animation tools
    * 2.3 Asset Converters
    * 2.4 Tile (CHR) editors
    * 2.5 Map (nametable) editors
  * 3 Music tools
    * 3.1 Trackers/sequencers
    * 3.2 DMC conversion tools
    * 3.3 Other conversion tools
    * 3.4 Engines
  * 4 Multicart Tools
  * 5 Miscellaneous other tools



## Assemblers, compilers, and PRG-oriented tools

### Assemblers

#### Commonly used assemblers

  * [asm6](http://3dscapture.com/NES/asm6.zip) \- written by Loopy because most other assemblers "were either too finicky, had weird syntax, took too much work to set up, or too bug-ridden to be useful".
  * [asm6f](https://github.com/freem/asm6f) \- fork of asm6, providing support for illegal opcodes, NES 2.0 headers, symbol files for FCEUX/Mesen/Lua, and [other features](https://github.com/freem/asm6f/blob/master/readme.txt). See Releases for Windows binaries
  * [CC65](https://cc65.github.io/cc65/) \- A portable 6502/65c02/65c816 assembler, linker, and C compiler.
  * [NESASM (MagicKit)](http://www.magicengine.com/mkit/) by Charles Doty, David Michel, and J.H. Van Ornum.
  * [Unofficial MagicKit](http://zzo38computer.org/nes_program/ppmck.zip) by [zzo38](User_Zzo38.xhtml "User:Zzo38"). Based on MagicKit but with many improvements; includes PPMCK support
  * [WLA DX](http://www.villehelin.com/wla.html) \- A portable GB-Z80/Z80/6502/6510/65816 macro assembler. Linux and MS-DOS versions are also available.



#### Other assemblers

  * [AS65](http://www.obelisk.demon.co.uk/6502/downloads.html/) \- written by Andrew John Jacobs. A macro assembler for 8 and 16-bit 65xx. Available for DOS/Win32 and Java. The Java port is strongly recommended.
  * [ACME](http://www.esw-heim.tu-clausthal.de/~marco/smorbrod/acme/) \- Marco Baye's ACME 6502/65c02/65c816 cross-assembler. Runs on several platforms, including Amiga, DOS, and Linux. Supports macros, local labels, and many other features.
  * [dasm](https://github.com/dasm-assembler/dasm) \- by Matthew Dillon of the DragonflyBSD project, extended and maintained by various other authors. A macro assembler intended for 6502, 6507, 6803, HD6303, 68HC11, 68705, and F8 architectures.
  * [FASM v1.0](http://nesdev.org/fasm10.zip) by [Toshi Morita](http://www.mobygames.com/developer/sheet/view/developerId,11349/). FASM was written as a quick replacement for the 2500 AD assembler for Nintendo 8-bit development. Licensed under the GPL.
  * [Merlin 32](https://www.brutaldeluxe.fr/products/crossdevtools/merlin/) \- by Brutal Deluxe Software. 6502/65816 cross-assembler/linker for Win32, Linux, and OS X. Influenced by the Merlin/16+ assembler/linker for the Apple IIGS.
  * [nescom](http://bisqwit.iki.fi/source/nescom.html) \- Joel Yliluoma's 6502 assembler; written in C++, based on xa65 and largely compatible with it, including with the o65 object file format.
  * [NESHLA](http://neshla.sourceforge.net/) by Brian Provinciano. A 6502 assembler specifically geared towards NES development.
  * [Ophis Assembler](https://michaelcmartin.github.io/Ophis/) by Michael C. Martin. An open-source 6502 assembler written in Python.
  * [P65 Assembler](http://hkn.eecs.berkeley.edu/~mcmartin/P65/) \- A portable 6502 assembler written in Perl.
  * [Telemark Cross Assembler](http://home.comcast.net/~tasm/) \- A shareware assembler for numerous 8-bit processors, including the 6502, Z80, and 8051.
  * [x816 v1.12f](http://nesdev.org/x816112f.zip) by minus. An assembler for 6502/65c02/65c816. MS-DOS only.
  * [xa65](http://www.floodgap.com/retrotech/xa/) \- Andre Fachat's open-source cross-assembler; written in C and supports the standard 6502 and 65c816 opcode lists. Sports a C-like preprocessor and supports label-hiding in a block structure. Produces plain binary files, as well as special o65 object files. Further tools include a linker, file and relocation utilities for o65 files.
  * [XORcyst](http://neotoxin.moccamagic.com/XORcyst.htm) \- "... a rather platform-independent set of tools and languages for 6502 software development" written by Kent Hanson, aka SnowBro.
  * [XTOOLS](http://www.bipom.com/products/us/676.html) \- table-based assembler and toolkit. Includes an assembler, disassembler, and several tools. Shareware (US$49); registered version includes a table-based assembly source translator. Note: files are generated as Motorola formatted hex files; you will need a converter (see Converters below)
  * [Kick Assembler](http://theweb.dk/KickAssembler/Main.html#frontpage) \- 6510 assembler with high level scripting. KickC targets at this assembler.
  * [NESASM CE](https://github.com/ClusterM/nesasm) \- fork of NESASM (MagicKit) by [Alexey Avdyukhin (Cluster)](User_Cluster.xhtml) with additional NES features: NES 2.0 headers, symbol files for FCEUX, etc.



### Disassemblers

  * [6502d](https://www.zophar.net/utilities/nesdev/6502d-disassembler.html) \- by Cortez Ralph. Win32 (GUI-based) rewrite of original 6502 disassembler (MS-DOS) by Bart Trzynadlowski.
  * [clever-disasm](https://bisqwit.iki.fi/source/nescom.html) \- by [Bisqwit/Joel Yliluoma](User_Bisqwit.xhtml "User:Bisqwit"). Part of the nescom assembler suite (see above).
  * [da65](https://www.cc65.org/doc/da65.html) \- part of the cc65 suite. Primarily intended for individuals using tools in the cc65 suite (ex. ca65).
  * [disasm6](https://forums.nesdev.org/viewtopic.php?t=7466) \- PHP-based 6502 (NES-oriented) disassembler intended for use with asm6 assembler. Pre-compiled Windows binaries are available.
  * [GhidraNes](https://github.com/kylewlacy/GhidraNes) \- Ghidra extension by Kyle Lacy
  * [NES Disassember](https://www.zophar.net/utilities/nesdev/nes-disassembler.html) \- by Morgan Johansson. MS-DOS.
  * [nesgodisasm](https://github.com/retroenv/nesgodisasm) \- a tracing disassembler that can generate output compatible with asm6/ca65/nesasm
  * [SmartRENES](https://www.romhacking.net/utilities/288/) \- by Hyde. Supports mappers 0, 1, 2, 3, and 7. Windows binaries only.
  * [TRaCER](https://www.zophar.net/utilities/nesdev/tracer.html) \- by [koitsu/Jeremy Chadwick](User_Koitsu.xhtml "User:Koitsu"). 6502/65816 disassembler intended for NES and SNES platforms. MS-DOS. (Does contain a confirmed bug related to one 65816-specific opcode)



### IDEs

  * [NESfab](https://pubby.games/nesfab.html) While NESfab is a programming language, it's also an IDE with asset management and level editing capabilities.
  * [NESmaker](https://www.nesmakers.com/index.php) A commercial, semi-walled garden, GameMaker-like IDE with asset management and level editing capabilities. Based on prewritten assembly engine modules. Runs on top of Asm6.
  * [Retro Puzzle Maker](https://puzzle.nes.science/) Another simplified, gamemaker-like puzzle games IDE. Can be run in browser or on desktop.
  * [NESICIDE](https://github.com/christopherpow/nesicide) (WIP) source code only, Github link for the NESICIDE IDE by cpow (Chris Pow)
  * [WUSDN](http://www.wudsn.com/) An NES IDE as of 2012 by the WUSDN team, Originally for Atari 8-Bit computers, Now also comes with NES capability, Requires Java Runtime Environment and Eclipse to run.



### Compilers

  * [CC65](http://www.cc65.org/) \- A portable 6502/65c02/65c816 assembler, linker, and C compiler.
  * [GBDK](https://github.com/gbdk-2020/gbdk-2020) \- GBDK is a cross-platform development kit for sm83, z80 and 6502 based gaming consoles.
  * [KickC](https://gitlab.com/camelot/kickc) \- C-compiler for creating optimized and readable 6502 assembler code. ([NesDev forum](http://forums.nesdev.org/viewtopic.php?f=2&t=20187))
  * [llvm-mos](https://llvm-mos.org) \- llvm-mos is a fork of LLVM supporting the MOS Technology 65xx series of microprocessors and their clones.
  * [Millfork](https://github.com/KarolS/millfork) \- A middle-level programming language targeting 6502- and Z80-based microcomputers and home consoles.
  * [Pas 6502](https://syntaxerrorsoftware.itch.io/pas6502) \- Pascal cross-compiler for 6502-based machines.
  * [NESFab](https://pubby.games/nesfab.html) \- A new programming language for creating NES games.
  * [vbcc](http://www.compilers.de/vbcc.html) \- vbcc is a highly optimizing portable and retargetable ISO C compiler.



### Converters

  * [Converters for INTEL/MOTOROLA HEX formats](http://www.batlabs.com/fileutil.html) to Binary (BIN) and back, for assemblers like XTOOLS.



### Pre-processors and other code (PRG) tools

  * [shuffle.py](https://github.com/pinobatch/croom-nes/blob/master/tools/shuffle.py) by Damian Yerrick - Preprocessor to rearrange variables, subroutines, and instructions in a file, for using other variables as [memory canaries](https://en.wikipedia.org/wiki/Buffer_overflow_protection#Canaries "wikipedia:Buffer overflow protection") or [watermarking](Watermarking.xhtml "Watermarking") beta builds



### Compression related tools

  * [Compress Tools](http://www.romhacking.net/utilities/882/) \- A multi-featured open-source compressor featuring many algorithms, extendable to new algorithms. Allows to break data in many small independent blocks and more by the usage of scripts.
  * [Huffmunch](https://github.com/bbbradsmith/huffmunch) \- A generic compression library for NES/6502 with very low RAM requirements.
  * [Donut](https://forums.nesdev.org/viewtopic.php?t=17313) \- A fast and efficient CHR compression library by JRoatch.



## Graphics-oriented tools

### General NES graphics studios

Collected here are editors that offer a holistic approach to the creation and editing of various NES native graphics assets. These tools may offer in more or less equal measures the editing of tiles, screens, maps, sprites, metasprites and palettes in tandem; allowing for a resource efficient approach. 

  * [NES Assets Workshop (NAW)](https://nesrocks.itch.io/naw), by Nesrocks. Built in GameMaker with a thematic GUI. NAW opts for a classic toolbox menu with toggle hotkeys, similar to Photoshop, Aseprite and many more. Has a special "Overlay" editing mode which lets you draw sprite overlays on top of backgrounds.
  * [NEXXT](https://frankengraphics.itch.io/nexxt), a continuation of NESST maintained by FrankenGraphics. Adds an arsenal of new drawing & layout tools, toolboxes, and asset management (such as meta-tiles and collision), along with speed-improved workflows, work safety, and bugfixes.
  * [NES Screen Tool (NESST)](https://shiru.untergrund.net/software.shtml) by Shiru (Deprecated). Tile oriented pixel art studio capable of editing CHR, backgrounds, sprites, metasprites, and palettes. Facilitates a modifier-key approach to make editing quicker, at some cost of discoverability (browsing through the readme is recommended).
  * [RetroSpriteEditor](https://github.com/xverizex/RetroSpriteEditor), by xverizex. a tile and nametable editor for Windows and Linux.



### Animation tools

  * [DONER](https://github.com/codemeow/doner) by Codemeow. Generates easing tables to use for position animations. Output is suitable for 6502 C or Assembly.



### Asset Converters

Converters are good choices for when you prefer to work on an asset in a general purpose graphics studio such as Photoshop, GIMP, Aseprite, GraphicsGale etc, and need to convert or extrapolate NES ready data from that work. 

  * [I-CHR](https://kasumi.itch.io/ichr) by Kasumi. Converts PC images and image sequences into NES compatible tilesets and nametables. Can also produce a NES ROM program displaying the graphics.
  * [NesTiler](https://github.com/ClusterM/nestiler) by [Alexey Avdyukhin (Cluster)](User_Cluster.xhtml). Converts PC images into pattern tables, nametables, attribute tables and palettes. This application can accept multiple images as input, the main point is to use single set of palettes (and tiles if required) for all of them, so it's possible to switch CHR banks and base nametable on a certain line, in the middle of rendering process. Can produce lossy result if image doesn't meet the limitations of NES. Uses command-line interface, so it can be used as a part of toolchain. Multiplatform.
  * [OverlayPal](https://github.com/michel-iwaniec/OverlayPal) by Michel Iwaniec (Bananmos). An image-to-NES-assets converterter that specializes in splitting the source image into a background layer and an "overlay" sprite layer.
  * [pilbmp2nes.py](https://github.com/pinobatch/nrom-template/blob/master/tools/pilbmp2nes.py) by Damian Yerrick - Command-line converter from indexed BMP or PNG to multiple consoles' tile formats.
  * [png2chr.py](https://github.com/zeta0134/nes-util/blob/master/png2chr.py) by Zeta0134. Takes any paletted image file, exactly 128px by 128px, and outputs it as 4KiB CHR ROM.
  * [Tilificator](https://sourceforge.net/projects/tilificator/) by Michel Iwaniec (Bananmos) - sprite optimisation tool. Takes an image and finds resource effective hardware tiles for sprite usage. Can import/export to NESST .msb formats. [A tutorial](http://retrocoder.epizy.com/tilificator_tutorial/?i=1) is available.



### Tile (CHR) editors

  * [8ted](http://pineight.com/pc/#ted) by Damian Yerrick (deprecated).
  * [Famitile](https://web.archive.org/web/20140420062726/http://zzo38computer.org/nes_program/famitile.zip) \- (Note: the link is recovered from archive.org ) Old, super-minimalist graphics tool by zzo38. Requires an sdl of version 1.x, which can be gotten [(here)](https://www.libsdl.org/release/). Includes command-line and graphical mode; supports editing CHR, but also standard nametables, and [MMC5](MMC5.xhtml "MMC5") extension nametables.
  * [Nasu](https://hundredrabbits.itch.io/nasu) by 100 rabbits. A Minimalist NES tile editor with features such as quick colour selection via hotkeys. For win/mac/linux via an ["UXN" emulator](https://100r.co/site/uxn.html).
  * [Tile Molester](http://www.romhacking.net/utilities/991/) by SnowBro, Central MiB, Lab313, and Mewster.
  * [YY-CHR](YY_CHR.xhtml "YY-CHR"), by YY. A tile editor that works like visual hex editor. Particularly popular in the romhacking scene, since you may modify an already assembled ROM image. Also has provisionary tile layout (nametable) capabilities.
  * [Tile Layer](Tile_Layer.xhtml "Tile Layer") a program for modifying graphics in NES ROMs.



### Map (nametable) editors

  * [8name](https://pineight.com/pc/#ted) by Damian Yerrick
  * [8name.py](https://github.com/pinobatch/thwaite-nes/blob/master/tools/8name.py) by Damian Yerrick
  * [Famitile](https://web.archive.org/web/20140420062726/http://zzo38computer.org/nes_program/famitile.zip) (mentioned above). While primarily a CHR editor, there is some support for nametable editing.
  * [YY-CHR](YY_CHR.xhtml "YY-CHR") (mentioned above). Has provisionary tools for nametable layouts.



## Music tools

### Trackers/sequencers

  * [FamiTracker](https://famitracker.org/) tracker-style music editor
  * [MCK driver](http://www.geocities.co.jp/Playtown-Denei/9628/) and MML translator for music creation (includes source code)
  * [Musetracker](http://kkfos.aspekt.fi/projects/nes/tools/musetracker) tracker-style music editor (formerly known as PornoTracker).
  * [Nerd Tracker II](http://nesdev.org/nt2/) tracker-style music editor
  * [PPMCK unofficial version](http://zzo38computer.org/nes_program/ppmck.zip) by [zzo38](User_Zzo38.xhtml "User:Zzo38") (includes MagicKit as well)
  * [NTRQ](https://neilbaldwin-nes.netlify.app/nes-audio/index.html) native NES tracker by Neil Baldwin
  * [Pulsar](https://neilbaldwin-nes.netlify.app/nes-audio/index.html) native NES tracker in LSDJ style by Neil Baldwin
  * [Nijuu](https://neilbaldwin-nes.netlify.app/nes-audio/index.html) NES music macro language assembler by Neil Baldwin



### DMC conversion tools

  * [FamiTracker](https://famitracker.org/) can import .wav files and convert to DMC samples, which can then be exported as .dmc files.
  * [NSF Live!](http://user.tninet.se/~zxy965r/ft/nsf.zip) NSF player that can export DMC samples from NSF songs as .dmc files.
  * [Pin Eight NES Tools](http://www.pineight.com/nes/#18) includes a command-line encoder and decoder by Damian Yerrick.



### Other conversion tools

  * [Drum sample to noise sound effect/channel converter](http://forums.nesdev.org/viewtopic.php?p=69538#p69538) by lidnariq. Written in C.



### Engines

Runtime engines for playing music and sound: see [Audio drivers](Audio_drivers.xhtml "Audio drivers")

## Multicart Tools

  * [Action 53 mapper](Action_53_mapper.xhtml "Action 53 mapper") \- a multicart mapper designed for homebrew games.
  * [Double Crossing: The Forbidden Four](Forbidden_Four.xhtml "Forbidden Four") \- a hack of _The Legend of Zelda_ to add a menu and three NROM games using MMC1.



## Miscellaneous other tools

  * [Visual 2A03](http://www.qmtpro.com/~nes/chipimages/visual2a03/) \- circuit simulator of the 2A03 
    * [Visual circuit tutorial](Visual_circuit_tutorial.xhtml "Visual circuit tutorial") \- usage guide
  * [Visual 6502](http://www.visual6502.org/) \- circuit simulator of the 6502 processor 
    * [Visual6502wiki](Visual6502wiki.xhtml "Visual6502wiki") \- mirror/archive of its wiki documentation
  * [Serial Bootloader](Serial_Bootloader.xhtml "Serial Bootloader") \- tool for uploading from PC to the controller or expansion port
  * [ADOS NES](User_Erana_ADOS_NES.xhtml "User:Erana/ADOS NES") \- an operating system project
  * [NESDev old homepage development tools list](https://nesdev.org/archive.html#PC)



Categories: [Audio](Category_Audio.xhtml)
