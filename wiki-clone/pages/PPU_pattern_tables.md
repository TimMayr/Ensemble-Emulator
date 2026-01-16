# PPU pattern tables

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/PPU_pattern_tables) | View [other pages](Special_AllPages.xhtml#PPU_pattern_tables)

The **pattern table** is an area of memory connected to the PPU that defines the shapes of tiles that make up backgrounds and sprites. This data is also known as **CHR** , and the memory attached to the PPU which contains it may either be [CHR-ROM or CHR-RAM](CHR_ROM_vs__CHR_RAM.xhtml "CHR ROM vs. CHR RAM"). _CHR_ comes from "character", as related to computer text displays where each tile might represent a single letter character. 

[![](../wiki-images/One_half_fraction_CHR.png)](File_One_half_fraction_CHR_png.xhtml)

Encoding of a ½ tile

Each tile in the pattern table is 16 bytes, made of two planes. Each bit in the first plane controls bit 0 of a pixel's color index; the corresponding bit in the second plane controls bit 1. 

  * If neither bit is set to 1: The pixel is background/transparent.
  * If only the bit in the first plane is set to 1: The pixel's color index is 1.
  * If only the bit in the second plane is set to 1: The pixel's color index is 2.
  * If both bits are set to 1: The pixel's color index is 3.



This diagram depicts how a tile for ½ (one-half fraction) is encoded, with `.` representing a transparent pixel. 
    
    
    Bit Planes            Pixel Pattern
    $0xx0=$41  01000001
    $0xx1=$C2  11000010
    $0xx2=$44  01000100
    $0xx3=$48  01001000
    $0xx4=$10  00010000
    $0xx5=$20  00100000         .1.....3
    $0xx6=$40  01000000         11....3.
    $0xx7=$80  10000000  =====  .1...3..
                                .1..3...
    $0xx8=$01  00000001  =====  ...3.22.
    $0xx9=$02  00000010         ..3....2
    $0xxA=$04  00000100         .3....2.
    $0xxB=$08  00001000         3....222
    $0xxC=$16  00010110
    $0xxD=$21  00100001
    $0xxE=$42  01000010
    $0xxF=$87  10000111
    

The pattern table is divided into two 256-tile sections: a first pattern table at $0000-$0FFF and a second pattern table at $1000-$1FFF. Occasionally, these are nicknamed the "left" and "right" pattern tables based on how emulators with a debugger display them. (See #Display convention below.) 

An important aspect of a [mapper](Mapper.xhtml "Mapper")'s capability is how finely it allows bank switching parts of the pattern table. 

## Addressing

PPU addresses within the pattern tables can be decoded as follows: 
    
    
    DCBA98 76543210
    ---------------
    0HNNNN NNNNPyyy
    |||||| |||||+++- T: Fine Y offset, the row number within a tile
    |||||| ||||+---- P: Bit plane (0: less significant bit; 1: more significant bit)
    ||++++-++++----- N: Tile number from name table
    |+-------------- H: Half of pattern table (0: "left"; 1: "right")
    +--------------- 0: Pattern table is at $0000-$1FFF
    

The value written to [PPUCTRL](PPU_registers.xhtml "PPUCTRL") ($2000) controls whether the background and sprites use the first pattern table ($0000-$0FFF) or the second pattern table ($1000-$1FFF). PPUCTRL bit 4 applies to backgrounds, bit 3 applies to 8x8 sprites, and bit 0 of each OAM entry's tile number applies to 8x16 sprites. 

For example, if rows of a tile are numbered 0 through 7, row 1 of tile $69 in the left pattern table is stored with plane 0 in $0691 and plane 1 in $0699. 

## Display convention

[![](../wiki-images/Thwaite_pattern_tables.png)](File_Thwaite_pattern_tables_png.xhtml)

_Thwaite_ CHR ROM in a pattern table viewer

It is conventional for debugging emulators' video memory viewers to display the pattern table as two 16x16-tile grids side by side. They draw the pattern table at $0000-$0FFF on the left and the pattern table at $1000-$1FFF on the right. Each pattern table is commonly represented as a 128 by 128 pixel square, with 16 rows of 16 tiles. Usually the tiles are shown left to right, top to bottom, in Western reading order: $00 in the top left, $01 to the right of that, through $0F at the top right, then $10 through $1F on the second row, all the way through $FF at the bottom right. Some emulators have an option to rearrange the view for 8x16 sprites, where the first two rows are $00, $02, $04, ..., $1E, and $01, $03, $05, ..., $1F, and then each pair of rows below that shows another 16 pairs of tiles. 
