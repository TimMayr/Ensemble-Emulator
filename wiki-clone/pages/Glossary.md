# Glossary

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Glossary) | View [other pages](Special_AllPages.xhtml#Glossary)

These terms are commonly used in technical discussions about the Nintendo Entertainment System and Nintendo Family Computer. Additional well-known computer science terms are listed at [Before the basics](Before_the_basics.xhtml "Before the basics"), and terms specific to integrated circuits are at [Visual circuit tutorial#Terms](Visual_circuit_tutorial.xhtml#Terms "Visual circuit tutorial"). 

## Contents

  * 1 0
  * 2 A
  * 3 B
  * 4 C
  * 5 D
  * 6 E
  * 7 F
  * 8 G
  * 9 I
  * 10 L
  * 11 M
  * 12 N
  * 13 O
  * 14 P
  * 15 R
  * 16 S
  * 17 T
  * 18 U
  * 19 V
  * 20 W
  * 21 Z



## 0

10NES
    The program running on the [CIC](CIC_lockout_chip.xhtml "CIC lockout chip").
60-pin
    Referring to Famicom cassettes. The Famicom and most famiclones for the Asian market use 60-pin cassettes, which have audio passthrough and no lockout chip.
72-pin
    Referring to NES Game Paks. The NES and most famiclones for the North American and European markets use 72-pin Game Paks, which have expansion port passthrough and a different lockout chip for each market.

## A

Address decoding
    Translation of addresses on the console's address buses into chip enables and addresses for the various ROMs and RAMs in the cartridge. In most cartridges, this involves some form of bank switching.
ASIC
    Application specific integrated circuit. Used with mappers involving complicated logic that the IC chip is made specific to this purpose.
Attribute
    A part of video memory that selects which part of the palette is used for a given sprite or area of the background.
Attribute table
    The 64-byte table of background tile attributes at the end of each of the four nametables.
Audio Processing Unit ([APU](APU.xhtml "APU"))
    The hardware on the NES/Famicom that generates audio output.

## B

Background
    A grid of tiles (programmed using [nametables](PPU_nametables.xhtml "Nametable")).
Backdrop
    Pixels which do not contain either an opaque background or opaque sprite pixel, which display either the color in entry 0 of palette RAM or the palette index inputted on the PPU's EXT pins (normally 0).
Bank switching
    The primary function of a mapper. Because the CPU and PPU don't have enough address space to see the entire ROM at once, the ROM is conceptually divided into "pages" or "banks", where the most significant bits of the ROM address are treated as a page number. The CPU writes a page number to the mapper's port, and the mapper puts this number on the most significant bits of the ROM address to make the selected page visible to the rest of the system. More advanced mappers have "fine-grained bank switching", which makes separate banks available in separate windows of the address space.
Battery RAM
    RAM made nonvolatile by the addition of a battery backup circuit. This is used to let the player save progress in games with more than one chapter. Almost all battery RAM in NES games is PRG RAM; a couple games on very uncommon mappers have battery-backed CHR RAM.

## C

Camera
    Abstraction for the part of a world that is displayed on the screen. The illusion of a moving camera is created by scrolling the background and moving all the sprites at once.
Cassette
    

  1. Japanese term for "cartridge" that corresponds to the term "Game Pak" used in other markets.
  2. Philips [Compact Cassette](https://en.wikipedia.org/wiki/Compact_Cassette "wikipedia:Compact Cassette"), the magnetic tape storage medium for the [Famicom Data Recorder](https://en.wikipedia.org/wiki/Famicom_Data_Recorder "wikipedia:Famicom Data Recorder") and for homemade mix tapes of NES game background music.



cel
    a frame of animation
CGRAM (Color Generator RAM)
    Used by Nintendo to mean Palette RAM.
CHR (character)
    Another word for pattern tables, after the traditional name [character generator](https://en.wikipedia.org/wiki/character_generator "wikipedia:character generator") for a tiled background plane.
CHR RAM
    An SRAM on the cartridge, usually 8192 bytes, normally mapped at $0000-$1FFF and holding pattern tables.
CHR ROM
    ROM in the cartridge which is connected to the PPU, normally mapped at $0000-$1FFF and holding pattern tables.
CIC (Checking Integrated Circuit)
    Nintendo's term for what the public knows better as the lockout chip.
CIC clone
    A microcontroller generating the same pseudorandom stream as the authentic CIC. Examples include Tengen's Rabbit and Kevin Horton's CIClone.
CIC stun
    Freezing the CIC by using a charge pump to pulse negative voltage on the data pins.
CIRAM (Console-Internal RAM)
    A 2 KiB SRAM in the NES, normally mapped at $2000-$3FFF on the PPU bus and used to hold nametables and attribute tables. The cartridge controls where and even if this is mapped. CIRAM may have originally stood for Character-Internal RAM, but this is rarely mapped in the pattern table region.
CL
    Used on PCBs to refer to solder jumpers that are connected by default and must be cut to disconnect them.
Collision
    The coincidence or overlapping of two "objects" which may be made of background tiles, one or more sprites, or a combination of these. Collision detection is done by the software since the Famicom/NES hardware has no collision detection circuitry.
Control Deck
    The console itself, into which the Game Pak is inserted and to which the controllers and outputs are connected.

## D

Dendy
    A brand name of clone Famicom consoles from Russia that use PAL signals but using timings to allow NTSC games to run.
Discrete logic
    Mapper logic involving standard digital logic ICs such as 74xx series.
Display list
    A set of data to be copied to video memory. On the NES, it is usual to prepare display lists for sprites and the background during draw time and copy them to video memory during vertical blank.
DRAM
    Dynamic random access memory, or memory that needs to be periodically refreshed.
Draw time
    Time outside vertical blanking, when the PPU is busy drawing the picture.
Duty cycle
    The fraction of a cycle a signal stays high. For example, most input clock signals have a duty cycle of 50%. The [pulse channels](APU_Pulse.xhtml "APU Pulse") in the APU have a variable duty cycle.

## E

Expansion audio
    Extra audio in the cartridge in several titles, used in Famicom only, not in NES.

## F

Famiclone
    A third-party clone of the Famicom and/or NES hardware. A few units have swapped [APU duty cycles](APU_Pulse.xhtml "APU Pulse"), as 12.5, 50, 25 and 25 negated instead.
Family Computer, Famicom, FC
    A video game console released by Nintendo in 1983 that would later be redesigned and rebranded as the Nintendo Entertainment System, or NES. There are a few key [differences](Family_Computer.xhtml#Differences_from_the_NES "Family Computer") between the two consoles.
Fixed bank
    A window of address space that always points to the same bank, or the bank that appears in this window. Because a fixed bank is unaffected by bank switching. Often used to store interrupt handlers, code that accesses large amounts of data spread across several banks, or anything else that must always be available. Several mappers fix the last bank of the PRG ROM at $C000-$FFFF or $E000-$FFFF. Mappers that use outer bank switching will have a separate fixed bank for each outer bank.
Flip
    Reflection of an image. For example, a "vertical flip" turns an M into a W as the pixels move vertically across a horizontal axis.
Forced blanking
    Turning off the rendering circuitry by writing zero to [$2001](PPU_registers.xhtml "PPU registers"). When bits 4 and 3 of this port are both clear (`xxx00xxx`), the CPU can access VRAM through the PPU ports outside the vertical blanking period.
Front-loader
    NES-001, the original version of the 72-pin NES. It uses a pseudo-ZIF socket for Game Paks.
Four screen
    A mapping of cartridge memory to the PPU bus that expands the memory available for nametables to 4K, thus allowing four distinct nametables.

## G

[Game Genie](Game_Genie.xhtml "Game Genie")
    A lock-on unit developed by Codemasters and distributed by Galoob that patches NES games with user-entered single-byte changes.
Game Pak
    A cartridge for a Nintendo console. It contains ROMs, mappers, and possibly other circuitry.
[Golf](https://en.wikipedia.org/wiki/Code_golf "wikipedia:Code golf")
    Aggressive micro-optimization of a program for speed or size, often as a friendly competition to produce the fastest or smallest version of a particular routine. Compare the concept of "frame wars" in the tool-assisted speedrun community. In practice, [golf is far less necessary](https://en.wikipedia.org/wiki/Premature_optimization "wikipedia:Premature optimization") on the NES than on the Atari 2600 except in a few cases. These include squeezing 17K of code into 16K for an [NROM](NROM.xhtml "NROM")-128 release, squeezing a routine into a ROM hack of an already tightly packed game such as _Super Mario Bros._ , getting a routine to finish in one frame or one vertical blank, or producing a code library used by other programs that have a similar reason to be space or time efficient.

## I

IRQ (Interrupt Request)
    A maskable hardware interrupt. Used most often by external hardware to do split-screen scrolling or to trigger events (like disk-loading on the Famicom Disk System or ending a game on the NES-EVENT boards).

## L

Left pattern table
    The pattern table at PPU $0000-$0FFF. So-called due to display conventions in debugging emulators.
Lock-on
    Referring to a cartridge with two connectors: one to connect to the console and another to connect to another cartridge to use its CIC and/or to modify its program.
Lockout chip
    Nintendo's scheme used in the front-loading NES to block the use of infringing copies of its games and to reassure retailers that the NES wouldn't have the flood of low-quality games seen on second-generation consoles. Nintendo was successfully sued in antitrust court over this and other practices. It consists of two 4-bit CIC microcontrollers, one in the Control Deck and one in each Game Pak, passing a stream of pseudorandom numbers back and forth.
Lockout defeat
    Various ways of getting a program to run without an authentic CIC. These include CIC clone and CIC stun.

## M

[Mapper](Mapper.xhtml "Mapper")
    Circuitry on the Game Pak to perform address decoding and counting.
Mapper hack
    A patch to a ROM that uses one mapper to make it work with a different kind of mapper.
Memory Management Controller (MMC)
    Name for ASIC mappers made by Nintendo.
Metatile
    A block of tile numbers written to the background as a unit. These blocks, often 2x2 tiles (16x16 pixels) in size, are common building blocks for area maps in NES games.
[Mirroring](Mirroring.xhtml "Mirroring")
    Presence of one memory area at more than one place in the memory map. In the PPU memory, mirroring is used in the nametables to repeat the screen horizontally or vertically, thus the distinguishing terms "horizontal mirroring" and "vertical mirroring" are used.
[Multicart](Multicart.xhtml "Multicart")
    A cartridge containing several otherwise unrelated programs.

## N

[Nametable](PPU_nametables.xhtml "Nametable")
    Any of four areas in VRAM (PPU addresses $2000-$23FF, $2400-$27FF, $2800-$2BFF, $2C00-$2FFF) specifying which tiles to display at which places in the background. Each is 32 by 30 tiles and ends with an attribute table.
Nintendo Entertainment System, NES
    A video game console released by Nintendo in 1985 that was redesigned and rebranded from the Family Computer (Famicom).
NMI
    Non-maskable interrupt. Used in NES/Famicom for vertical blank.

## O

[Object Attribute Memory (OAM)](PPU_OAM.xhtml "PPU OAM")
    A 256-byte DRAM inside the PPU holding the sprite display list.
[Open bus](Open_bus_behavior.xhtml "Open bus")
    A data bus that repeats the last read value when no memory or I/O circuit responds to a particular address.
Outer bank
    A large bank that encompasses a complete set of smaller banks. A mapper supporting an outer bank has one set of ports for "normal" bank switching and a second set of ports that switches higher address lines to make completely separate programs appear. It usually even switches otherwise fixed banks. Used most commonly in multicarts.

## P

Palette
    A color lookup table.
Palette RAM
    A 28-entry RAM inside the PPU specifying the colors used by the eight 4-color palettes.
Pattern table
    Two 4096 byte areas of video memory, mapped at PPU $0000-$0FFF and $1000-$1FFF. Each contains 256 tiles.
Picture Processing Unit ([PPU](PPU.xhtml "PPU"))
    It generates the video signal.
Piggyback
    Synonym for lock-on used by [Home Entertainment Suppliers (HES)](http://www.consoledatabase.com/companies-organisations/hes/).
Port
    An address mapped to input or output instead of ROM or RAM.
PRG (program)
    Memory in the cartridge on the CPU bus.
PRG RAM
    RAM in the cartridge on the CPU bus, in contrast to the internal RAM at $0000-$07FF which is usually called main RAM, system RAM, or just RAM. With few exceptions, PRG RAM is located at $6000-$7FFF and decoded with the equivalent of [a circuit like this](PRG_RAM_circuit.xhtml "PRG RAM circuit") inside the mapper.
PRG ROM
    ROM on the CPU's bus, containing a program to be executed and data for the program to use. In games using CHR RAM, it may also contain data to be copied to CHR RAM.
Priority
    Which graphics appear in front of other graphics. In general, sprites earlier in OAM appear in front of later sprites, and a sprite may appear in front of or behind the background.
Proportional font
    A font whose glyphs (character pictures) have different widths. These can fit more text onto the screen than the more common monospace (fixed-width) fonts but are rarely used on NES because of the processing overhead involved with drawing each line of text.

## R

Raster
    A horizontal line of the output picture. (Also: scanline)
Ratchet scrolling
    Scrolling that is only allowed by the game engine to occur in one direction. _Super Mario Bros._ is a famous example of this.
Rendering
    The video memory access and processing done by the PPU to produce a video signal. Rendering is enabled through bits 3 and 4 of $2001; disabling it is called forced blanking. Rendering is in progress if it is enabled and the PPU is not in vertical blanking.
Right pattern table
    The pattern table at PPU $1000-$1FFF. So-called due to display conventions in debugging emulators.

## S

Scanline
    A horizontal line of the output picture. (Also: raster)
[Scrolling](PPU_scrolling.xhtml "PPU scrolling")
    Movement of the visible portion of a background.
SL
    Used on PCBs to refer to solder jumpers that are not connected by default and must be soldered to connect them.
Sliver
    An 8x1-pixel section of a tile. The PPU operates by fetching 2-byte slivers from pattern tables and passing them through shift registers.
[Spin](https://en.wikipedia.org/wiki/Busy_waiting "wikipedia:Busy waiting")
    To repeatedly read a particular address in a loop until its value changes. A program can spin on an input port or on a RAM address that is modified by an interrupt handler.
Sprite
    An entry of 4 bytes in OAM, controlling a small block of 8x8 pixels that can be moved around on the screen. It has X and Y coordinates, a tile index into one of the pattern tables, vertical and horizontal flip switches, and priority.
Sprite 0 hit
    A PPU feature that performs pixel-perfect collision detection between sprite 0 and the background. When a non-transparent pixel of the first sprite in OAM overlaps a non-transparent pixel of the background, bit 6 ($40) of PPU register $2002 is set to 1. This is commonly used to provide timing for split-screen scrolling when the cartridge doesn't have some kind of IRQ.
Sprite limit
    The PPU cannot render more than eight sprite slivers on one scanline. Sprites beyond the eighth will drop out.
SRAM
    [Static random access memory](https://en.wikipedia.org/wiki/Static_random_access_memory "wikipedia:Static random access memory") is memory made with latches, which does not need to be refreshed but takes four to six times as much die space as DRAM. PRG RAM and CHR RAM are generally SRAM. Occasionally, it stands for "save RAM", which means the same as battery RAM.
[Stack](Stack.xhtml "Stack")
    An area of memory used to store values in last-in, first-out (LIFO) manner, especially the area at $0100-$01FF used to store return addresses when a subroutine is called.
Strobe
    A signal that defines sampling points for another signal. Think of a strobe light that captures the state of whatever it's shining on when it's on to get the analogy.

## T

Tile
    An 8x8 pixel piece of graphics. Tile data on the NES has 2 bits per pixel, and each pixel in a tile can have one of three colors or transparency.
Top-loader
    NES-101, a 72-pin console released during the end of the NES's commercial era that takes cartridges in the top, like most other consoles. It has no lockout chip.
Trainer
    In video games, a [trainer](https://en.wikipedia.org/wiki/Trainer_\(games\) "wikipedia:Trainer \(games\)") is a patch to add a cheat menu. In [iNES](INES.xhtml "INES"), a "trainer" is an extra 512-byte segment of PRG ROM that can be used by a ROM image that has had a trainer added but is more commonly used by support routines used by mapper hacks for obsolete floppy-disk-based copiers.
Transparent
    In the case of overlapping graphics, a transparent pixel means that pixels from the graphics behind this graphic show through.

## U

Unlicensed
    Referring to video games for the NES published outside of a contract with Nintendo. Most of these use a lockout defeat; a few use lock-on.

## V

Variable-width font (VWF)
    Synonym for proportional font.
Vector
    One of three addresses at the end of the [CPU memory map](CPU_memory_map.xhtml "CPU memory map"), telling the CPU where to start executing instructions after an [interrupt](CPU_interrupts.xhtml "CPU interrupts") is asserted.
Vertical blank
    The interval between video frames.
VRAM
    Video RAM. This is a generic term for the PPU bus which encompasses memory for pattern tables, nametables, attributes tables, and palettes.

## W

Window
    A set of contiguous CPU or PPU addresses mapped to a particular bank of memory.
Work RAM, WRAM
    See PRG RAM.

## Z

[Zapper](Zapper.xhtml "Zapper")
    The [light gun](https://en.wikipedia.org/wiki/light_gun "wikipedia:light gun") for NES.
