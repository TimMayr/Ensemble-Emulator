# User:Blargg/Blargg PPU

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ABlargg/Blargg_PPU) | View [other pages](Special_AllPages.xhtml#User_Blargg_Blargg_PPU)

_Just a rough attempt at striking the right detail level for the main PPU documentation. Posting here (rather than just previewing a local copy) in case someone wants to get an idea of what I'm trying._

## Pattern tables

The background and sprites are drawn using small 8x8 pixel images called tiles. There are 512 tiles total, arranged in two pattern tables, each holding 256 tiles. A tile takes 16 bytes of memory, with 2 bits per pixel. The pixels are stored as two bit planes, each 8 bytes. Each byte of a plane represents one row of the tile, with the most significant bit representing the left-hand pixel. The first plane specifies the least significant bit of each pixel. A pixel value of 0 is transparent, while 1, 2, and 3 specify a color from one of the sub-palettes. 

## Palette

The colors of bckground and sprite pixels are determined by the palette. It consists of 8 sub-palettes; the first four for the background and the remaining four for the sprites. Each sub-palette specifies the colors for tile pixel values of 1 to 3. 

The palette uses 32 bytes of VRAM starting at $3F00, with each sub-palette using 4 bytes. Normally the first entry of each sub-palette is ignored, except for the first entry of the first sub-palette, which specifies the backdrop color. 

$3F00  
Backdrop | $3F01  
Color 1 | $3F02  
Color 2 | $3F03  
Color 3 | Background sub-palette 0   
---|---|---|---|---  
$3F04  
Unused | $3F05  
Color 1 | $3F06  
Color 2 | $3F07  
Color 3 | Background sub-palette 1   
$3F08  
Unused | $3F09  
Color 1 | $3F0A  
Color 2 | $3F0B  
Color 3 | Background sub-palette 2   
$3F0C  
Unused | $3F0D  
Color 1 | $3F0E  
Color 2 | $3F0F  
Color 3 | Background sub-palette 3   
$3F10  
Backdrop mirror | $3F11  
Color 1 | $3F12  
Color 1 | $3F13  
Color 1 | Sprite sub-palette 0   
$3F14  
Unused | $3F15  
Color 1 | $3F16  
Color 2 | $3F17  
Color 3 | Sprite sub-palette 1   
$3F18  
Unused | $3F19  
Color 1 | $3F1A  
Color 2 | $3F1B  
Color 3 | Sprite sub-palette 2   
$3F1C  
Unused | $3F1D  
Color 1 | $3F1E  
Color 2 | $3F1F  
Color 3 | Sprite sub-palette 3   
  
Note that $3F00 and $3F10 both set the backdrop color; these two memory locations actually access the same byte of memory. This must be kept in mind when writing to sprite sub-palette 0. 

Each palette entry is only 6 bits. The upper 2 bits specify brightness, and the lower 4 hue: 

Palette entry  | `--BB HHHH` | Brightness and Hue   
---|---|---  
bits 0-3 | `---- HHHH` | Hue   
bits 4-5 | `--BB ----` | Brightness   
  
[palette goes here, which shows palette and hex values for each color] 

Note that a value of $0D should not be used (brightness=0, hue=13), as this results in a "blacker than black" color that causes stability problems on some televisions. 

## Background

The background is composed of 64x60 tiles, divided into four 32x30 nametables. Each nametable is 960 bytes, with 8-bit tile indicies into one of the two pattern tables as selected by PPUCTRL. Each nametable also has a corresponding 64-byte attribute table, which breaks the tiles into blocks of 2x2 and specifies which background sub-palette to use for each. 

Normally there is only enough VRAM for two nametables and attribute tables; the other two are duplicates of these. Some mappers support other schemes, like having a single nametable duplicated in all four positions, or extra VRAM to allow four unique nametables. In the following table, A, B, C, and D refer to the underlying memory the nametables are stored in. 

Arrangement | Name   
---|---  
A A  
B B  | Horizontal   
A B  
A B  | Vertical   
A A  
A A  | One-screen   
A B  
C D  | Four-screen 
