# CHR ROM vs. CHR RAM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/CHR_ROM_vs._CHR_RAM) | View [other pages](Special_AllPages.xhtml#CHR_ROM_vs__CHR_RAM)

An NES cartridge has at least two memory chips on it: PRG (connected to the [CPU](CPU.xhtml "CPU")) and [CHR](PPU_pattern_tables.xhtml "CHR") (connected to the [PPU](PPU.xhtml "PPU")). There is always at least one PRG ROM, and there may be an additional PRG RAM to hold data. Some cartridges have a CHR ROM, which holds a fixed set of graphics tile data available to the PPU from the moment it [turns on](PPU_power_up_state.xhtml "PPU power up state"). Other cartridges have a CHR RAM that holds data that the CPU has copied from PRG ROM through a port on the PPU. ([A few](MMC3_with_CHR_ROM_and_CHR_RAM.xhtml "MMC3 with CHR ROM and CHR RAM") have both CHR ROM and CHR RAM.) 

The PPU can see only 8 KiB of tile data at once. So once your game has more than that much tile data, you'll need to use a [mapper](Mapper.xhtml "Mapper") to load more data into the PPU. Some mappers bankswitch the CHR ROM so that the PPU can see different "pages". Other mappers are designed for CHR RAM; they let the CPU switch to a PRG ROM bank containing CHR data and copy it to CHR RAM. 

## Contents

  * 1 CHR ROM
    * 1.1 Advantages
    * 1.2 Applications
  * 2 CHR RAM
    * 2.1 Advantages
    * 2.2 Applications
  * 3 Effects possible with both types
  * 4 Switching to CHR RAM
  * 5 Notes



## CHR ROM

### Advantages

  * Takes less time and code for a beginner to get at least something displayed. The "hello world" program for a CHR ROM mapper is about 16 lines shorter.
  * Switching tiles is fast, needs no vblank time, and can be done mid-frame or even mid-scanline.
  * Can be used together with [MMC3](MMC3.xhtml "MMC3") and PRG RAM on a donor cartridge. Only [six games](http://bootgod.dyndns.org:7777/search.php?keywords=TNROM&kwtype=pcb) have a board with MMC3 + PRG RAM + CHR RAM, and all are Japan-exclusive. Only [three NES games use TGROM](http://bootgod.dyndns.org:7777/search.php?unif=NES-TGROM) (MMC3 + CHR RAM) and [two NES games use TQROM](http://bootgod.dyndns.org:7777/search.php?unif=NES-TQROM) (MMC3 + CHR RAM + CHR ROM) even without PRG RAM. However, this should become less of a consideration as MMC3-compatible cartridges with all new parts hit the market.



### Applications

Big static screens
    Smash TV's title screen alone uses more than 8 KB of tile data.

Dedicated bank for status bar
    A horizontal status bar might use a separate set of tiles from the playfield. This needs either a mapper with a raster interrupt or a sprite 0 overlap trigger. (e.g. Super Mario Bros. 3)

Artifact blanking
    A game that scrolls in all four directions will often have artifacts on one side of the screen because the NES doesn't have enough VRAM to keep the "seam" where new map data is loaded clean. To hide this, a game such as Jurassic Park might display tiles from a blank pattern table for the first or last 8 to 16 scanlines.[[1]](http://forums.nesdev.org/viewtopic.php?p=38049#p38049)

Pseudo texture mapping
    In some pseudo-3D games, each row of the floor texture can be stored in a separate bank. Both CHR ROM and CHR RAM let the program switch the background between CHR banks in $0000 and $1000 using [PPUCTRL](PPU_registers.xhtml "PPUCTRL"),[[2]](http://jonathan.microclub.ch/NES_raster/index.html) but CHR ROM allows far more than two banks to be used, as seen in a forward-scrolling shooter called Cosmic Epsilon.

Workaround for PRG/CHR divide
    A drawback of using CHR ROM is that the split between PRG ROM and CHR ROM fragments your data, but it can be worked around. If your PRG ROM is slightly bigger than a power of two, but you have a bit of spare CHR ROM left, you can stash the data in CHR ROM and read it out through [PPUADDR](PPU_registers.xhtml "PPUADDR")/[PPUDATA](PPU_registers.xhtml "PPUDATA"). For instance, _Super Mario Bros._ keeps its title screen map data at the end of CHR ROM and copies it into RAM to draw it. However, you can't read this data while rendering is turned on, and due to the [DMA glitch](APU_DMC.xhtml#Likely_internal_implementation_of_the_read "APU DMC"), reading [PPUDATA](PPU_registers.xhtml "PPUDATA") while playing sampled sound is unreliable.

## CHR RAM

### Advantages

  * Can switch tiles in small increments, and the granularity of switching does not depend on the mapper's complexity.
  * Tile data can be [compressed in ROM](Tile_compression.xhtml "Tile compression").
  * Tile data can be otherwise generated in real time.
  * Only one chip to rewire and put on the board when replicating your work on cartridge.
  * All data is stored in one address space, as opposed to a small amount being inaccessible when rendering is on and unreliable when DPCM is on.



### Applications

Compositing
    Sometimes you need to draw piles of things that aren't aligned to an 8x8 pixel grid, and there are more of them than will fit into the limit of 8 sprites per scanline. _Hatris_ , _Shanghai II_ , _Blockout_ , and its clone _3D Block_ have a large playfield containing large stacks of such objects. _Cocoron_ creates the player character's animation frames by piecing together several customizable body parts.
Text flexibility
    A font is a set of glyphs, or pictures that represent the characters of a script. Compositing these glyphs with CHR RAM allows drawing text in a proportional font (also called a variable-width font, or VWF). Not a lot of NES games used a VWF, but something like _Word Munchers_ or _Fraction Munchers_ might benefit. The Super NES version of _Mario Is Missing_ uses a VWF, as do a lot of Super NES RPGs, the [Action 53](Action_53.xhtml "Action 53") menu, and text boxes in _[User:Tepples/RHDE/RHDE](https://www.nesdev.org/w/index.php?title=User:Tepples/RHDE/RHDE&action=edit&redlink=1 "User:Tepples/RHDE/RHDE \(page does not exist\)")_ and _[Nova the Squirrel](User_NovaSquirrel_Nova_the_Squirrel.xhtml "User:NovaSquirrel/Nova the Squirrel")_. It's also the most practical way to display characters in connected scripts such as Arabic or heavily accented scripts such as Vietnamese.
Compression
    [Compression](Tile_compression.xhtml "Tile compression") refers to transforming a block of data to reduce its size in bits, so that another transformation on the compressed data can recover the original data. This is fairly common with tile data in games that use CHR RAM. The graphics in Konami games (_Blades of Steel_ , the US version of _Contra_ , and the Japanese version of _Simon's Quest_) and Codemasters games (such as _Bee 52_) are compressed using a run-length encoding scheme: Konami's codec [works on bytes](http://thefox.aspekt.fi/graveduck.py), while Codemasters' [works on pixels](http://forums.nesdev.org/viewtopic.php?p=48658#p48658). If your game has a lot of tile data, especially if it's just a shade over the power of two boundary between one [ROM](ROM.xhtml "ROM") size and the next larger power of 2, consider compression.
Drawing
    A few games allow the user to edit tiles. These include paint programs such as _Videomation_ and _Color a Dinosaur_ , or moddable titles such as the Japan-only shooter maker _Dezaemon_ (whose stock sprites were reused in _Recca_).
Vector graphics
    _Qix_ has horizontal lines, vertical lines, and filled areas that aren't aligned to a tile grid. Graphics in _Elite_ are wireframe 3D.
Juxtaposition
    Some CHR ROM games restrict which objects can be seen together because of what bank their CHR data is stored in. CHR RAM has no such problem because any object's tile data can be loaded at any position. This proves useful in a game like _Final Fantasy_ , where any player characters can meet any monsters, or in a game like _Dizzy_ or _RHDE_ or _Animal Crossing_ , where the player can have any of several dozen items in his inventory. The extreme of this is _Battletoads_ , which keeps only one frame of each player's animation loaded. To switch frames of animation, it copies them into CHR RAM. But then it has to turn off rendering at the top of the screen, creating a blank strip in the status bar, in order to fit the copy into blanking time. Whether you are using 8x8 or 8x16 pixel sprites, there is enough space in $1000-$1FFF to hold the current and next cel for all 64 sprites. This effect is used even more intensely in platforms with dual-ported VRAM (TurboGrafx-16, Game Boy Advance) and in platforms which have hardware-assisted memory copying to video ports other than OAM (Genesis, Super NES, Game Boy Color, Game Boy Advance). And it's _required_ if you want to display text in a language whose script has more than 250 or so glyphs, such as the logographic characters in some Chinese games and the Japanese versions of _Faxanadu_ and _Bionic Commando_.
One chip
    CHR RAM means you don't need to program a separate flash chip to act as the CHR ROM.
Battery RAM
    NES 2.0 supports CHR battery RAM. If not all of the memory is used for tiles, you can store save game data in there, which means you don't have to install CHR ROM or PRG RAM on the cartridge. (However not many mappers officially support this; as far as I know not many emulators do either.)

## Effects possible with both types

Tile animation. Think of the animated **?** blocks, coins, and conveyor belts in Super Mario Bros. 3 or the animated grass and quicksand in Super Mario Bros. 2, spinning fans or cogs in some games, or the independent parallax scrolling of distant repeating tile patterns in Batman: Return of the Joker, Crisis Force, and Metal Storm. 

With CHR ROM, you'd make a separate bank for each frame of animation that you want to display, or for each offset between the distant pattern's scroll position and the foreground pattern's scroll position. It works best on a mapper with CHR banks smaller than 4 KB, such as MMC3. 

With CHR RAM, you'd copy the tiles into VRAM as needed. Assuming moderately unrolled VRAM loading code, the NTSC vblank is long enough to copy about 160 bytes per frame plus the [OAM](PPU_OAM.xhtml "OAM") display list without turning rendering on late. This is enough for 10 tiles alone, or 8 tiles plus a nametable row, nametable column, or entire palette. 

In cases where player 1 and player 2 can select a character, and each character has his own frames of animation, the game needs to use either CHR RAM or a CHR ROM mapper with 1 KiB sprite banks so that both player 1's animation and player 2's animation can be loaded at once. According to an article in _Nintendo Power_ , character selection in Pro Wrestling was the driving force behind the invention of UNROM. 

Technically, juxtaposition isn't impossible with CHR ROM mappers, but almost no mappers support assigning a separate tile bank for each space in a nametable. [MMC5](MMC5.xhtml "MMC5") has an "ExGrafix" mode that allows this, regardless of page boundaries within the CHR ROM, as long as the game doesn't scroll vertically or uses the effect only in the playfield or only in the status bar. ([MMC4](MMC4.xhtml "MMC4") also supports automatically switching CHR ROM banks, although in a much more limited way than MMC5.) Thus only MMC5 can properly handle Chinese characters without needing each message to be rendered to a separate page. 

## Switching to CHR RAM

It's straightforward to change an existing project using [NROM](NROM.xhtml "NROM") to use CHR RAM. 

  1. Start with an NROM-256 project, and make sure at least 8300 bytes are free in the PRG ROM. We want to make sure CHR RAM works before dealing with mapper bankswitch.
  2. Remove the CHR ROM data from your build process, whether it be `.incbin "mytiles.chr"` after the IRQ vector or `copy /b game.prg+mytiles.chr game.nes` after assembly and linking.
  3. In your program's [iNES](INES.xhtml "INES") header, set the number of CHR banks to 0 (which signifies CHR RAM). (If you are using a [NES 2.0](NES_2_0.xhtml "NES 2.0") header, also set the CHR RAM size byte to $07, which denotes 64 × 27 bytes.)
  4. In your program's [init code](Init_code.xhtml "Init code"), after the PPU has stabilized but before you turn on rendering, `jsr copy_mytiles_chr` which is listed below (or you can inline the code).
  5. Rebuild your project. The size should end up as 32,784 bytes (16 bytes of header and 32,768 bytes of PRG ROM).


    
    
    ; for ca65
    PPUMASK = $2001
    PPUADDR = $2006
    PPUDATA = $2007
    
    .segment "CODE"
    .proc copy_mytiles_chr
    src = 0
      lda #<mytiles_chr  ; load the source address into a pointer in zero page
      sta src
      lda #>mytiles_chr
      sta src+1
    
      ldy #0       ; starting index into the first page
      sty PPUMASK  ; turn off rendering just in case
      sty PPUADDR  ; load the destination address into the PPU
      sty PPUADDR
      ldx #32      ; number of 256-byte pages to copy
    loop:
      lda (src),y  ; copy one byte
      sta PPUDATA
      iny
      bne loop  ; repeat until we finish the page
      inc src+1  ; go to the next page
      dex
      bne loop  ; repeat until we've copied enough pages
      rts
    .endproc
    
    .segment "RODATA"
    mytiles_chr: .incbin "mytiles.chr"
    

The original NROM-256 board is designed to take a [mask ROM or a JEDEC-pinout EPROM](Mask_ROM_pinout.xhtml "Mask ROM pinout") in the CHR ROM position, not a [6264 SRAM](6264_static_RAM.xhtml "6264 static RAM"). Because the board needs to be rewired slightly for CHR RAM, a few emulators do not emulate [iNES Mapper 000](NROM.xhtml "NROM") (NROM) with CHR RAM. For these, you'll need to use [iNES Mapper 034](INES_Mapper_034.xhtml "INES Mapper 034") (BNROM), which has CHR RAM and 32 KiB bank switching. 

The next step after this is to switch to a mapper that allows switching PRG banks. See [Programming UNROM](Programming_UNROM.xhtml "Programming UNROM") and [Programming MMC1](Programming_MMC1.xhtml "Programming MMC1"). 

## Notes

  * Based on forum posts: [42576](http://forums.nesdev.org/viewtopic.php?p=42576#p42576) and [61957](http://forums.nesdev.org/viewtopic.php?p=61957#p61957)


