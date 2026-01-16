# VT01 STN Palette

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VT01_STN_Palette) | View [other pages](Special_AllPages.xhtml#VT01_STN_Palette)

The V.R. Technology VT01 is a regular Famiclone whose only enhancement is the ability to drive particular types of STN displays directly. One of the supported STN displays uses red and cyan color components. When used with such a display, the six-bit color numbers that are written to CGRAM (PPU $3F00-$3F1F) have a different meaning compared to the [normal RP2C02 palette](PPU_palettes.xhtml "PPU palettes"): 
    
    
    7654 3210
    ---- ----
    ..RR CC..
      || ++--- Cyan color component level, 0=darkest, 3=brightest
      ++------ Red color component level, 0=darkest, 3=brightest
    

Note that when displaying such a color palette on a normal computer screen, differences in contrast range and gamma should be taken into account. There are several ROM images available with the name prefix "Portable FC-LCD" that use color numbers according to this format. As most of the games on them are palette hacks of existing games, the unused bits 0 and 1 may occasionally be set as well. 

# References

  * [VT01 Data Sheet](http://www.vrt.com.tw/old_site/admin/upload/datasheet/VT01%20Data%20Sheet%20RevisionA2_ENG_.pdf) from on V.R. Technology's website, in particular pages 2 and 10.
  * [Discussion](http://forums.nesdev.org/viewtopic.php?f=3&t=16868)


