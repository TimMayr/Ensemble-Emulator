# .pal

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/.pal) | View [other pages](Special_AllPages.xhtml#_pal)

**.pal** is an extension used by palette files for NES emulators. Palette files are required for emulation of the RGB PPUs of the VS Unisystem, PlayChoice 10, and Famicom Titler systems. Before [NTSC video](NTSC_video.xhtml "NTSC video") was fully understood, emulators also used palette files to translate the [PPU's color values](PPU_palettes.xhtml "PPU palettes") into sRGB values for the PC display. 

Most palette files used by NES emulators are 192 bytes in size, consisting of 64 3-byte entries, one for each of 64 NES color values. 

Each entry consists of three unsigned 8-bit bytes, in this order: 

  1. red intensity
  2. green intensity
  3. blue intensity



Intensity is most commonly ranged from 0 (black) to 255 (full intensity), presuming the [sRGB](https://en.wikipedia.org/wiki/sRGB "wikipedia:sRGB") color space used by most monitors. 

Most emulators will ignore any entries past the first 64, but an optional extension of the .pal format (currently only known to be supported by Nintendulator 0.965 and later, and FCEUX 2.3.0) lengthens the file to 1536 bytes in order to store each palette value when colour (de-)emphasis bits are applied. This allows customization of the emphasis behaviour, and is particularly useful for representing the alternative emphasis behaviour of the RGB PPU used in the [Vs. System](Vs__System.xhtml "Vs. System") and PlayChoice. 

## See Also

  * [PPU palettes](PPU_palettes.xhtml "PPU palettes")



## External links

  * [VGA Palette on ModdingWiki](http://www.shikadi.net/moddingwiki/VGA_Palette)
  * [Editing custom NES palettes in Linux](https://forums.libretro.com/t/editing-custom-nes-palettes-in-linux/6733/6) instructions on editing .pal files in GIMP



Categories: [File formats](Category_File_formats.xhtml)
