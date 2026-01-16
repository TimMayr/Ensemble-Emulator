# 4-in-1 multicart

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/4-in-1_multicart) | View [other pages](Special_AllPages.xhtml#4_in_1_multicart)

4-in-1 multicart 

[![](../wiki-images/4-in-1_menu.png)](File_4_in_1_menu_png.xhtml)

[](File_4_in_1_menu_png.xhtml "Enlarge")

4-in-1 menu

[![](../wiki-images/4-in-1_top.jpg)](File_4_in_1_top_jpg.xhtml)

[](File_4_in_1_top_jpg.xhtml "Enlarge")

4-in-1 top

[![](../wiki-images/4-in-1_bottom.jpg)](File_4_in_1_bottom_jpg.xhtml)

[](File_4_in_1_bottom_jpg.xhtml "Enlarge")

4-in-1 bottom

[![](../wiki-images/4-in-1_schematic.png)](File_4_in_1_schematic_png.xhtml)

[](File_4_in_1_schematic_png.xhtml "Enlarge")

4-in-1 schematic

This cartridge contains 4 CNROM games: 

  * Puzzle
  * F-15 City War
  * Volley Ball
  * PokeBlock



## Contents

  * 1 Banks
  * 2 Mirroring
  * 3 Subject to bus conflicts
  * 4 Registers
  * 5 Quirks



## Banks

  * CPU $8000-$ffff: 32 KiB switchable PRG ROM bank
  * PPU $0000-$1fff: 8 KiB switchable CHR ROM bank



## Mirroring

H/V (selected by software) 

## Subject to bus conflicts

Yes 

## Registers

Mask: $8000 
    
    
    $8000: [..pP CCCC]
              || ||||
              || ++++- select 8kB CHR-ROM bank at $0000-$1fff
              ++------ select 32 kB PRG-ROM bank at $8000-$ffff
              +------- select mirroring (0=H, 1=V)
    

## Quirks

  * Multicart register overlaps CNROM register and multicart register is not protected from writes after game is selected, so all games' bankswitching routines had to be patched
  * The register is not initialized on powerup or reset, so all games' reset vector had to be patched
  * DIP28 1 Mibit mask roms are used both for PRG and CHR (pin 22, which is normally used as !OE in DIP28 EEPROMS is utilized as A16)
  * This is basically identical to [GxROM](GxROM.xhtml "GxROM") but one bit also determines nametable mirroring as well as PRG bank



Categories: [Mappers](Category_Mappers.xhtml)
