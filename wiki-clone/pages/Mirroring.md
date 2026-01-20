# Mirroring

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Mirroring) | View [other pages](Special_AllPages.xhtml#Mirroring)

There are two types of **mirroring** on the NES: 

  * **Memory Mirroring** is when the same memory may be accessed at multiple addresses, causing an apparent duplication.
  * **Nametable Mirroring** describes the layout of the NES' 2x2 [background nametable](PPU_nametables.xhtml "PPU nametables") graphics, usually achieved by mirrored memory.



  


## Contents

  * 1 Memory Mirroring
  * 2 Nametable Mirroring
    * 2.1 Horizontal
    * 2.2 Vertical
    * 2.3 Single-Screen
    * 2.4 4-Screen
    * 2.5 Other
    * 2.6 Mirroring chart
  * 3 See also



## Memory Mirroring

Memory mirroring refers to the appearance of memory or I/O registers at more than one range of addresses, with the same byte being accessible at more than one address. This occurs when the full address isn't completely _decoded_ , that is, when a chip ignores one or more address lines. Because completely decoding an address usually takes a lot more pins on a chip, incomplete decoding is used to reduce the hardware required; if the mirror occupies otherwise unused address space, it poses no problems. 

Within the NES, many things are mirrored: 

  * System memory at $0000-$07FF is mirrored at $0800-$0FFF, $1000-$17FF, and $1800-$1FFF - attempting to access memory at, for example, $0173 is the same as accessing memory at $0973, $1173, or $1973.
  * PPU I/O registers at $2000-$2007 are mirrored at $2008-$200F, $2010-$2017, $2018-$201F, and so forth, all the way up to $3FF8-$3FFF.
  * The single registers on most [simple mappers](Category_Discrete_logic_mappers.xhtml "Category:Discrete logic mappers") are mirrored throughout $8000-$FFFF.
  * Registers on many common ASIC mappers (such as the MMC1 and MMC3) are also mirrored, in groups, throughout $8000-$FFFF.
  * Nametable mirroring, as described below, happens due to memory mirroring within PPU $2000-$2FFF (hence its name). However, in this case the memory mirroring is intentional and necessary.
  * In [NROM](NROM.xhtml "NROM")-128, the 16k PRG ROM is mirrored into both $8000-$BFFF and $C000-$FFFF.
  * In most [mappers](Mapper.xhtml "Mapper"), banks past the end of PRG or CHR ROM show up as mirrors of earlier banks. For example, [UNROM](UxROM.xhtml "UNROM") PRG banks 8-15 are duplicates of banks 0-7 respectively. [Non-power-of-two ROM size](Non_power_of_two_ROM_size.xhtml "Non-power-of-two ROM size") may imply more complicated mirroring.



## Nametable Mirroring

Nametable mirroring affects what is shown past the right and bottom edges of the current nametable. When mirroring is enabled for a particular axis (horizontal and/or vertical), the coordinates simply wrap around on the current nametable. A background "mirrored" in this way is repeated, _not_ flipped. When mirroring is disabled, a second nametable is used. There are four common combinations of mirroring: 

### Horizontal

[![Horizontal mirroring diagram.png](../wiki-images/Horizontal_mirroring_diagram.png)](File_Horizontal_mirroring_diagram_png.xhtml)

A **vertical arrangement** of the nametables results in **horizontal mirroring** , which makes a **32x60 tilemap**. 

This is most commonly used for games which only scroll vertically or in all directions. 

Doing any horizontal scrolling using horizontal mirroring is hard to do smoothly because the data on the right of the screen is immediately show on the left due to mirroring. Clever use of hardware left-side screen clipping will hide all name table glitches, but because the attribute tables have a resolution of 2x2 tiles, there will always be attribute glitches on the left and/or the right side of the screen. The best possible way to hide it is to have 4 pixels with potentially wrong attributes on both sides, but most commercial games did worse than that having usually 8 or even more glitchy pixels, so that is why so many NES games have color glitches on the border of the screen. 

Some televisions [overscan](Overscan.xhtml "Overscan") up to 8 pixels on both left and right border, but most don't. Perfectionist programmers could use solid black sprites on the right border to hide attribute glitches and make the screen look symmetrical and hide absolutely all attribute glitches, as in the game _Alfred Chicken_ , but very few games do this because it reduces the number of sprites per scanline to 7 and wastes a lot of [OAM](PPU_OAM.xhtml "PPU OAM") space (roughly 1/4 in 8x16 pixel sprite mode). 

To configure a cartridge board for horizontal mirroring, connect PPU A11 to CIRAM A10. On cartridge boards made by Nintendo, this is selected by shorting the "V" solder pad (for "vertical arrangement"). 

### Vertical

[![Vertical mirroring diagram.png](../wiki-images/Vertical_mirroring_diagram.png)](File_Vertical_mirroring_diagram_png.xhtml)

A **horizontal arrangement** of the nametables results in **vertical mirroring** , which makes a **64x30 tilemap**. 

This is most commonly used for games which only scroll horizontally. Games that scroll vertically (by any amount and without status bar) and that never scroll horizontally by more than one screen would use this mirroring (e.g. Lode Runner, Bomberman, Fire Emblem, Crystal Mines), so that they don't have to load anything when scrolling horizontally. 

Of course it is also used for games which scroll in both directions without a status bar. Because data that is on the top/bottom of the screen will immediately show up on the other side, a clever use of NTSC [overscan](Overscan.xhtml "Overscan") can make it glitch-less multidirectional scrolling, but glitches will appear on PAL televisions (and NTSC televisions with a overscan range which is a little off). The best possible way to hide glitches is to make 4 pixels with wrong tiles and 4 additional pixels with wrong color on both sides, but most commercial games did much worse than this, that's why they look so bad if overscan is disabled. 

Perfectionist programmers could use raster split to hide glitches (and possibly also provide more blanking time to update VRAM) as in the games _Jurassic Park_ and _M.C. Kids_ , but it was rarely done because it complicates the code a lot for little benefits. 

To configure a cartridge board for vertical mirroring, connect PPU A10 to CIRAM A10. On cartridge boards made by Nintendo, this is selected by shorting the "H" solder pad (for "horizontal arrangement"). 

### Single-Screen

[![Single screen mirroring diagram overlaid.png](../wiki-images/Single_screen_mirroring_diagram_overlaid.png)](File_Single_screen_mirroring_diagram_overlaid_png.xhtml)

Single-screen mirroring is only available with certain mappers, such as the [AxROM](AxROM.xhtml "AxROM"), [SxROM](MMC1.xhtml "SxROM"), and [TLSROM](TLSROM.xhtml "TLSROM") boards, resulting in **two 32x30 tilemaps**. 

Its main advantage is that it allows using a status bar at the top or bottom of the screen while also allowing the playfield to extend equally in any direction - this can be done by storing the status bar in one nametable, rendering the playfield in the other nametable, and switching mirroring (and scrolling parameters) at the appropriate screen location during rendering. 

There are also a lot of other things that can be drastically simplified when using 1-screen mirroring: The formulas used to calculate PPU address of data to be updated to the screen are also significantly simpler, and if the status bar have a variable size or is scrolling, all this would be a headache without 1-screen mirroring. 

When this mirroring is used to scroll horizontally, similar glitches and scrolling problems that those of horizontal mirroring will happen. However, as long as there is a status bar, no glitches will happen vertically since the data that falls off the bottom (or the top) of the screen will come in the area that is "hidden" by the status bar, regardless of overscan factors. 

There are several different ways to configure a cartridge board for single-screen mirroring. Some boards have single screen mirroring which is mapper controlled like the MMC1 and AxROM. For AxROM connect the output of a register (e.g. [74HC161](74161.xhtml "74HC161")) to CIRAM A10 (AxROM latches PRG D4). A simpler way to create one screen mirroring that's fixed (non-changable by software) is to simply connect CIRAM A10 to Gnd or Vcc. The board that implements [iNES Mapper 218](INES_Mapper_218.xhtml "INES Mapper 218") wires CIRAM A10 to PPU A10, A11, A12, or A13, so that the game can store tiles in CIRAM without having any CHR ROM or CHR RAM in the cartridge. 

### 4-Screen

[![Four nametables diagram.png](../wiki-images/Four_nametables_diagram.png)](File_Four_nametables_diagram_png.xhtml)

With additional RAM and/or PPU address mapping present on the cartridge, 4 unique nametables can be addressed through the PPU bus, creating a **64x60 tilemap** , allowing for more flexible screen layouts. Very few games made use of this lack of mirroring. 

Games known to use 4-screen RAM nametables: 

  * _Rad Racer II_
  * _Gauntlet_
  * _Napoleon Senki_
  * _Rocman X (Sachen)_
  * Every [Vs. System](Vs__System.xhtml "Vs. System") game



Example games using 4-screen with [ROM nametables](Category_Mappers_with_ROM_nametables.xhtml "Category:Mappers with ROM nametables"): 

  * _Final Lap_
  * _King of Kings_



Mappers known to be used with 4 screens of nametables: 

  * [MMC3](MMC3.xhtml "MMC3") (_Rad Racer II_).
  * [iNES 206](INES_Mapper_206.xhtml "INES Mapper 206") implements a subset of MMC3 features, and can use 4-screen RAM (_Gauntlet_).
  * [iNES 77](INES_Mapper_077.xhtml "INES Mapper 077") maps RAM across the PPU memory space, combining with internal VRAM to provide 4 RAM nametables, and a combination of CHR-RAM and ROM for pattern tables (_Napoleon Senki_).
  * [Vs. System](Vs__System.xhtml "Vs. System") had twice as much VRAM as the NES, giving a permanent 4-screen setup. This is most visible as [iNES 99](INES_Mapper_099.xhtml "INES Mapper 099"), but [several other mappers](Vs__System.xhtml#See_also "Vs. System") were also used on this hardware.
  * [Namco 163](INES_Mapper_019.xhtml "Namco 163") allows 1k CHR-ROM pages to be arbitrarily mapped into the 4 nametable screens (_Final Lap_ , _King of Kings_).
  * The [JY Company](J_Y__Company_ASIC.xhtml "JY Company") mapper allows 1k CHR-ROM pages to be arbitrarily mapped into the 4 nametable screens.
  * [UNROM 512](UNROM_512.xhtml "UNROM 512") and [GTROM](GTROM.xhtml "GTROM") are homebrew mappers with 4-screen configurations.



Other mappers capable of uncommon 4-screen layouts: 

  * [VRC6](VRC6.xhtml "VRC6") allows 1k CHR-ROM pages to be arbitrarily mapped into the 4 nametable screens.
  * [MMC5](MMC5.xhtml "MMC5") can use its internal RAM to create a 3rd nametable, while procedurally generating a blank data page for a 4th, allowing (just barely) 4 different screens to be mapped at once.



The [iNES](INES.xhtml "INES") format can specify 4 nametables in the header, allowing 4-screen RAM nametables to be applied to any mapper that doesn't structurally conflict with this (if supported by the emulator). 

There are several ways to implement extra nametable RAM on a cartridge board: 

  * Add an extra 2 KiB of RAM on the board and combine it with the CIRAM already present in the console with a decoder chip in order to create linear accessible 4k block of RAM at $2000-$2FFF.
  * Add a [6264](6264_static_RAM.xhtml "6264 static RAM") 8 KiB RAM on the board, replacing the CIRAM present in the console. This effectively "wastes" the CIRAM chip as a whole, 4 KB of (normally unused) extra memory at $3000-$3EFF. However this leads to a simpler, single-chip solution (in addition, 8 KiB RAM chips are today more common and less expensive than 2 KiB ones).
  * Add a larger RAM on the board and map it to the entire PPU address space. This allows 8 KiB of pattern tables at $0000-$1FFF and 4 KiB of nametables at $2000-$2FFF sharing the same RAM chip.



### Other

  * [![Diagonal](../wiki-images/Diagonal_mirroring_diagram.png)](File_Diagonal_mirroring_diagram_png.xhtml "Diagonal")

Diagonal 

  * [![L-shaped](../wiki-images/L-shaped_mirroring_diagram.png)](File_L_shaped_mirroring_diagram_png.xhtml "L-shaped")

L-shaped 

  * [![3-screen vertical](../wiki-images/ACBC_mirroring_diagram.png)](File_ACBC_mirroring_diagram_png.xhtml "3-screen vertical")

3-screen vertical 

  * [![3-screen horizontal](../wiki-images/ABCC_mirroring_diagram.png)](File_ABCC_mirroring_diagram_png.xhtml "3-screen horizontal")

3-screen horizontal 

  * [![3-screen diagonal](../wiki-images/ABBC_mirroring_diagram.png)](File_ABBC_mirroring_diagram_png.xhtml "3-screen diagonal")

3-screen diagonal 

  * [![1-screen fixed](../wiki-images/Single_screen_2000_mirroring_diagram.png)](File_Single_screen_2000_mirroring_diagram_png.xhtml "1-screen fixed")

1-screen fixed 




Other uncommon types of mirroring are available in other boards, such as [TxSROM](INES_Mapper_119.xhtml "INES Mapper 119") variations of the MMC3, extended techniques available to the [MMC5](MMC5.xhtml "MMC5"), arbitrary VRAM mirroring arrangements by the [Namco 163](INES_Mapper_019.xhtml "Namco 163"), or ROM mirroring arrangements using mappers that allow [ROM nametables](Category_Mappers_with_ROM_nametables.xhtml "Category:Mappers with ROM nametables"). 

**Diagonal mirroring** (CIRAM A10 = PA11 XOR PA10) would facilitate changes in scrolling direction without having to flip between Horizontal and Vertical mirroring. 

**L-shaped mirroring** (CIRAM A10 = PA11 OR PA10), seen in Sachen's [374N](INES_Mapper_243.xhtml "INES Mapper 243") and [8259 family](Sachen_8259.xhtml "Sachen 8259"), allows scrolling in four directions as long as scrolling changes directions only at screen boundaries. 

Unusual cases: 

  * _Castlevania 3_ uses the third nametable RAM available on the [MMC5](MMC5.xhtml "MMC5") ([illustration](File_Castlevania_III_3_Screen_Mirroring_png.xhtml "File:Castlevania III 3-Screen Mirroring.png"))
  * _Laser Invasion_ uses the third nametable RAM available on the [MMC5](MMC5.xhtml "MMC5")
  * _After Burner_ uses the ROM nametables available on the [Sunsoft-4](INES_Mapper_068.xhtml "INES Mapper 068") (iNES 68), but it only supports 1/H/V arrangements.
  * _Mighty Morphin Power Rangers III, IV (JY Company)_ uses ROM nametables in a 3-Screen horizontal configuration, though the lower screen is never shown (But, lower screen has been used in the status bar).



### Mirroring chart

This table lists the more simple and easy to understand mirroring and scrolling techniques. There are a huge variety of more complicated techniques. For a more comprehensive survey, see: [List of games by mirroring technique](List_of_games_by_mirroring_technique.xhtml "List of games by mirroring technique")

Scrolling Type | Mirroring | Example Games | Comment   
---|---|---|---  
None  | Any  | _Donkey Kong_ , _Tennis_ | With only a single fixed screen, any mirroring type can be used.   
Horizontal Only  | Vertical  | _Super Mario Bros._ , _Gimmick!_ | A [status bar](https://www.nesdev.org/w/index.php?title=Status_bar&action=edit&redlink=1 "Status bar \(page does not exist\)") at the top is easy to accomplish with a [sprite-0 hit](PPU_OAM.xhtml "Sprite-0 hit") (see _Super Mario Bros._).   
Vertical Only  | Horizontal  | _Ice Climber_ , _Gun.Smoke_ | Without a status bar, horizontal mirroring is the best choice for vertical-only scrolling. With a status bar, vertical or single-screen mirroring give you a place in the nametable to render the status bar, and the scrolling seam should be hidden under the bar.   
Alternating Horizontal/Vertical  | Mapper switches H/V  | _Metroid_ , _Air Fortress_ | Motion is limited to a single axis at any given time, and the direction can only change when a new screen is reached.   
Limited Bidirectional  | Horizontal/Vertical  | _Super Mario Bros. 3_ , _Fire Emblem_ | By limiting one of the scrolling axes to only 2-screens wide, this makes unlimited scrolling in the other axis simple. With unlimited horizontal scrolling there will be unavoidable attribute glitches at one side of the screen (see _Super Mario Bros. 3_), but with unlimited vertical scrolling this can be hidden by [overscan](Overscan.xhtml "Overscan") in NTSC regions (see _Fire Emblem_).   
Unlimited Bidirectional  | Various  | _Castlevania II_ , _Battletoads_ , _Crystalis_ , _Final Fantasy_ | Unlimited scrolling in both axes at once is an advanced technique requiring a game-specific solution.   
  
The best way to understand the mirroring techniques used in a game, use a debugging emulator to look at the nametables. [Status bars](https://www.nesdev.org/w/index.php?title=Status_bar&action=edit&redlink=1 "Status bar \(page does not exist\)") typically require a scrolling split at a timed location on the screen. This can be done most easily with a mapper based [IRQ](IRQ.xhtml "IRQ"), but can also be accomplished with a [sprite-0 hit](PPU_OAM.xhtml "Sprite-0 hit") or other techniques. 

## See also

  * [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling")


