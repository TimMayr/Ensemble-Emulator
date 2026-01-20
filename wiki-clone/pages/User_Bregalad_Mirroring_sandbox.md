# User:Bregalad/Mirroring sandbox

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ABregalad/Mirroring_sandbox) | View [other pages](Special_AllPages.xhtml#User_Bregalad_Mirroring_sandbox)

This is my proposal to replace the current [mirroring](Mirroring.xhtml "Mirroring") page. Most information about scrolling is removed from here and would be eventually merged with [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling") page to create a page dedicated to level scrolling techniques. On the other hand, the advanced loopy split scrolling is moved to its own page. Pretty much the entierety of this page was significantly altered by myself (Bregalad), except the end of the _Other_ section. 

Note : I wish the images could be made about 50% smaller but I don't know how to do that unfortunately. 

## Contents

  * 1 Memory Mirroring
  * 2 Nametable Mirroring
    * 2.1 Hardwired vs mapper controlled mirroring
    * 2.2 Horizontal mirroring
    * 2.3 Vertical mirroring
    * 2.4 Single-Screen
    * 2.5 4-Screen mirroring
    * 2.6 Other
  * 3 See also



## Memory Mirroring

Memory mirroring refers to the appearance of memory or I/O registers at more than one range of addresses, with the same byte being accessible at more than one address. This occurs when the full address isn't completely _decoded_ , that is, when a chip ignores one or more address lines. Because completely decoding an address usually takes a lot more pins on a chip, incomplete decoding is used to reduce the hardware required. 

Within the NES, many things are mirrored: 

  * System memory at $0000-$07FF is mirrored at $0800-$0FFF, $1000-$17FF, and $1800-$1FFF - attempting to access memory at, for example, $0173 is the same as accessing memory at $0973, $1173, or $1973.
  * PPU I/O registers at $2000-$2007 are mirrored at $2008-$200F, and so forth, all the way up to $3FF8-$3FFF.
  * Nametable mirroring, as described below, happens due to memory mirroring within PPU $2000-$2FFF (hence its name). However, in this case the memory mirroring is intentional.
  * In [NROM](NROM.xhtml "NROM")-128, the 16k PRG ROM is mirrored into both $8000-$BFFF and $C000-$FFFF.



In addition to this, the single registers on most [simple mappers](Category_Discrete_logic_mappers.xhtml "Category:Discrete logic mappers") are mirrored throughout $8000-$FFFF. * Registers on many common ASIC mappers (such as the MMC1 and MMC3) are also mirrored, in groups, throughout $8000-$FFFF. 

PRG and CHR banks when using a size smaller than the maximum supported by the mapper are mirrored, because not all bankswitching bits are used. 

## Nametable Mirroring

The PPU fetches data from 4 [nametables](PPU_nametables.xhtml "Nametables") and their corresponding [attribute tables](https://www.nesdev.org/w/index.php?title=Attribute_tables&action=edit&redlink=1 "Attribute tables \(page does not exist\)") at adresses $2000-$23ff, $2400-$27ff, $2800-$2bfff and $2c00-$2fff. This totalizes an adress range of 4 KB, however, the NES itself only contains a VRAM chip of 2 KB for this purpose. The way the memory is mapped and is mirrored in the nametable adress ranges will affect scrolling function. In typical cases the 2 KB of internal VRAM (also called CIRAM) is used, however using RAM or ROM which is present on the cartridge is also possible. 

The term "mirroring" is a confusing way to describe nametable memory layouts. It is unknown why this terminology popular, very likely this is related to the creation of the [iNES](INES.xhtml "INES") format. 

How mirroring is used in combination with scrolling techniques is developped in the [User:Bregalad/Scrolling sandbox](User_Bregalad_Scrolling_sandbox.xhtml "User:Bregalad/Scrolling sandbox") article 

### Hardwired vs mapper controlled mirroring

Hardwired mirroring is used when the VRAM is mapped into the 4 nametable by the phisical wiring of the cartridge. It cannot be changed at runtime. On [iNES](INES.xhtml "INES") ROM dumps, the header determines the type of hardwired mirroring, and offers 3 possibilities, although one is very rarely used by commerical games. 

Many [mappers](Mapper.xhtml "Mappers") allows control of nametable mirroring. When a mapper with such a feature is used, the H/V bit in the [iNES](INES.xhtml "INES") header is ignored. 

### Horizontal mirroring

[![Horizontal mirroring diagram.png](../wiki-images/Horizontal_mirroring_diagram.png)](File_Horizontal_mirroring_diagram_png.xhtml)

This results in a **vertical arrangement** of the nametables, resulting in a non-contigous **32x60 tilemap**. 

Horizontal mirroring is created when the pin CIRAM A10 is connected to PPU A11, either by being wired directly or through a mapper chip via a multiplexer. On cartridge boards made by Nintendo, this is selected by shorting the "V" solder pad (for "vertical arrangement"). 

### Vertical mirroring

[![Vertical mirroring diagram.png](../wiki-images/Vertical_mirroring_diagram.png)](File_Vertical_mirroring_diagram_png.xhtml)

Vertical mirroring is created when the pin CIRAM A10 is connected to PPU A10, either by being wired directly or through a mapper chip via a multiplexer. This results in an **horizontal arrangement** of the nametables, resulting in a non-contigous **64x30 tilemap**. 

On cartridge boards made by Nintendo, this is selected by shorting the "H" solder pad (for "horizontal arrangement"). 

### Single-Screen

[![Single screen mirroring diagram overlaid.png](../wiki-images/Single_screen_mirroring_diagram_overlaid.png)](File_Single_screen_mirroring_diagram_overlaid_png.xhtml)

Single-screen mirroring is used when CIRAM A10 is controlled directly by a register. This results in simulating **two 32x30 tilemaps**. 

A register controls directly which of the two nametable is accessible, only one at a time. This works exactly like PRG or CHR [bankswitching](https://www.nesdev.org/w/index.php?title=Bankswitching&action=edit&redlink=1 "Bankswitching \(page does not exist\)"), except it switches nametable instead. This is most useful for doing split-scroll effects. 

Rarely, game cartridge can put half of CIRAM to other use or simply decide to not use it, resulting in only a single 32x30 tilemap being accesssible, by extension the term 'Single-Screen mirroring' is also used in this case. For example board that implements [iNES Mapper 218](INES_Mapper_218.xhtml "INES Mapper 218") wires CIRAM A10 to PPU13, so that the second tilemap is sacrified for 1 KB of CHR-RAM stored in CIRAM. There's then no need for CHR ROM nor CHR RAM in the cartridge. 

### 4-Screen mirroring

[![Four nametables diagram.png](../wiki-images/Four_nametables_diagram.png)](File_Four_nametables_diagram_png.xhtml)

With additional VRAM present on the cartridge, 4 unique nametables can be addressed through the PPU bus, resulting in a non-contigous **64x60 tilemap**. Even though very few games made use of this type of nametable layout, the [iNES](INES.xhtml "INES") format can enable it in the header, allowing 4-screen RAM nametables to be applied to any mapper that doesn't structurally conflict with this. When this is enabled, this overwrites both the iNES H/V mirroring bit and the mapper-controlled mirroring. 

There are several ways to implement extra nametable RAM on a cartridge board: 

  * Add an extra 2 KiB of RAM on the board and combine it with the CIRAM already present in the console with a decoder chip in order to create linear accessible 4k block of RAM at $2000-$2FFF.
  * Add a [6264](6264_static_RAM.xhtml "6264") 8 KiB RAM on the board, replacing the CIRAM present in the console. This effectively "wastes" the 2 KB CIRAM chip as a whole plus 4 KB of (normally unused) extra memory at $3000-$3EFF. However this leads to a simpler, single-chip solution.
  * Add a larger RAM on the board and map it to the entire PPU address space. This allows the same chip to be used for extra nametable RAM and for CHR-RAM simultaneously.



4-screen is a complete lack of "mirroring", in the hardware sense, but over time we seem to have ended up using the term "4-screen mirroring" due to [iNES](INES.xhtml "INES") terminology. 

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




More complex layouts can be made by treating each nametable as switchable banks just like CHR-ROM/RAM banks. As such there is no limit to how many nametables are avialable nor to how they are arranged, as long as the corresponding VRAM/VROM and logic is on the cartridge. 

Arbitrary arrangements are available with the [TxSROM](INES_Mapper_119.xhtml "INES Mapper 119") variations of the MMC3, the [MMC5](MMC5.xhtml "MMC5") and the [Namco 163](INES_Mapper_019.xhtml "Namco 163"). [MMC5](MMC5.xhtml "MMC5") has a graphic mode where 3 nametables can be used instead of the usual 2. 

Some mappers allow [ROM nametables](Category_Mappers_with_ROM_nametables.xhtml "Category:Mappers with ROM nametables"). 

Examples (that would facilitate changes in scrolling direction without having to flip between Horizontal and Vertical mirroring) : 

**X-shaped mirroring** (CIRAM A10 = PA11 XOR PA10). **L-shaped mirroring** (CIRAM A10 = PA11 OR PA10), seen in Sachen's [374N](INES_Mapper_243.xhtml "INES Mapper 243") and [8259 family](Sachen_8259.xhtml "Sachen 8259"). 

Unusual cases seen in games: 

  * _Castlevania 3_ uses the third nametable RAM available on the [MMC5](MMC5.xhtml "MMC5") ([illustration](File_Castlevania_III_3_Screen_Mirroring_png.xhtml "File:Castlevania III 3-Screen Mirroring.png"))
  * _Laser Invasion_ uses the third nametable RAM available on the [MMC5](MMC5.xhtml "MMC5")
  * The [Sunsoft-4](INES_Mapper_068.xhtml "INES Mapper 068") mapper used by _After Burner_ allows ROM nametables.
  * _Mighty Morphin Power Rangers III, IV (JY Company)_ uses ROM nametables in a 3-Screen horizontal configuration, though the lower screen is never shown (But, lower screen has been used in the status bar).



## See also

  * [PPU scrolling](PPU_scrolling.xhtml "PPU scrolling")


