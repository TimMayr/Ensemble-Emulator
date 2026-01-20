# User:PostNES

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3APostNES) | View [other pages](Special_AllPages.xhtml#User_PostNES)

[Mappers](Mapper.xhtml "Mappers")

* * *

### [Top 10 mappers used (NesCartDB)](http://bootgod.dyndns.org:7777/stats.php?page=6)

by usage   
---  
[![Mfr icon Nintendo.png](../wiki-images/Mfr_icon_Nintendo.png)](File_Mfr_icon_Nintendo_png.xhtml) [001](MMC1.xhtml "INES Mapper 001")   
[![Mfr icon Nintendo.png](../wiki-images/Mfr_icon_Nintendo.png)](File_Mfr_icon_Nintendo_png.xhtml) [004](MMC3.xhtml "INES Mapper 004")   
[![Mfr icon Nintendo.png](../wiki-images/Mfr_icon_Nintendo.png)](File_Mfr_icon_Nintendo_png.xhtml) [002](UxROM.xhtml "INES Mapper 002")   
[![Mfr icon Nintendo.png](../wiki-images/Mfr_icon_Nintendo.png)](File_Mfr_icon_Nintendo_png.xhtml) [000](NROM.xhtml "INES Mapper 000")   
[![Mfr icon Nintendo.png](../wiki-images/Mfr_icon_Nintendo.png)](File_Mfr_icon_Nintendo_png.xhtml) [003](CNROM.xhtml "INES Mapper 003")   
[![Mfr icon Rare.png](../wiki-images/Mfr_icon_Rare.png)](File_Mfr_icon_Rare_png.xhtml) [007](AxROM.xhtml "INES Mapper 007")   
[![Mfr icon Namco.png](../wiki-images/Mfr_icon_Namco.png)](File_Mfr_icon_Namco_png.xhtml) [206](INES_Mapper_206.xhtml "INES Mapper 206")   
[![Mfr icon Color Dreams.png](../wiki-images/Mfr_icon_Color_Dreams.png)](File_Mfr_icon_Color_Dreams_png.xhtml) [011](Color_Dreams.xhtml "INES Mapper 011")   
[![Mfr icon Nintendo.png](../wiki-images/Mfr_icon_Nintendo.png)](File_Mfr_icon_Nintendo_png.xhtml) [005](MMC5.xhtml "INES Mapper 005")   
[![Mfr icon Namco.png](../wiki-images/Mfr_icon_Namco.png)](File_Mfr_icon_Namco_png.xhtml) [019](INES_Mapper_019.xhtml "INES Mapper 019")   
  
ascending   
[![Mfr icon Nintendo.png](../wiki-images/Mfr_icon_Nintendo.png)](File_Mfr_icon_Nintendo_png.xhtml) [000](NROM.xhtml "INES Mapper 000")   
[![Mfr icon Nintendo.png](../wiki-images/Mfr_icon_Nintendo.png)](File_Mfr_icon_Nintendo_png.xhtml) [001](MMC1.xhtml "INES Mapper 001")   
[![Mfr icon Nintendo.png](../wiki-images/Mfr_icon_Nintendo.png)](File_Mfr_icon_Nintendo_png.xhtml) [002](UxROM.xhtml "INES Mapper 002")   
[![Mfr icon Nintendo.png](../wiki-images/Mfr_icon_Nintendo.png)](File_Mfr_icon_Nintendo_png.xhtml) [003](CNROM.xhtml "INES Mapper 003")   
[![Mfr icon Nintendo.png](../wiki-images/Mfr_icon_Nintendo.png)](File_Mfr_icon_Nintendo_png.xhtml) [004](MMC3.xhtml "INES Mapper 004")   
[![Mfr icon Nintendo.png](../wiki-images/Mfr_icon_Nintendo.png)](File_Mfr_icon_Nintendo_png.xhtml) [005](MMC5.xhtml "INES Mapper 005")   
[![Mfr icon Rare.png](../wiki-images/Mfr_icon_Rare.png)](File_Mfr_icon_Rare_png.xhtml) [007](AxROM.xhtml "INES Mapper 007")   
[![Mfr icon Color Dreams.png](../wiki-images/Mfr_icon_Color_Dreams.png)](File_Mfr_icon_Color_Dreams_png.xhtml) [011](Color_Dreams.xhtml "INES Mapper 011")   
[![Mfr icon Namco.png](../wiki-images/Mfr_icon_Namco.png)](File_Mfr_icon_Namco_png.xhtml) [019](INES_Mapper_019.xhtml "INES Mapper 019")   
[![Mfr icon Namco.png](../wiki-images/Mfr_icon_Namco.png)](File_Mfr_icon_Namco_png.xhtml) [206](INES_Mapper_206.xhtml "INES Mapper 206")   
  
  
## Mapper ideas

  * CHR hardware scrolling unit 
    * Scrolls portions of pattern table with wrapping for hardware accelerated parallax effects 
      * implementation: two CHR ROMs with alternately stored tiles (VRAM address `%0AAAAA AAAcBBBB` -> ROM `%c`, address in ROM `%AAAAAAAABBBB`) for parallel fetching of pixel rows


  * CHR color remapping unit 
    * ExGraphix-like, with each byte in ExRAM storing a color index (0-3) to be remapped to in a character, for every source color index (0-3). This would allow to conserve some palette/CHR-ROM space in some cases.   
Example:


    
    
    ExRAM byte = %01010000, or 1100 in quaternary system: remap 0 to 0, 1 to 0, 2 to 1, 3 to 1
    
    Original character:
    .defchr $00000000,
            $02233200,
            $02100120,
            $03000030,
            $03333310,
            $03000030,
            $03000030,
            $03222210
    
    Remapped:
    .defchr $00000000,
            $01111100,
            $01000010,
            $01000010,
            $01111100,
            $01000010,
            $01000010,
            $01111100
    

  * DPCM converter 
    * Converts any on-cartridge audio to DPCM and returns it to CPU on sample reads so that there's audio expansion on NES! 
      * DMC rate control would coarsely alter pitch if the conversion rate is constant


  * DPCM hardware mixer 
    * Plays back (by returning to CPU on sample read) multiple samples at different speeds


