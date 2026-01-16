# File:Safe areas.png

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/File%3ASafe_areas.png) | View [other pages](Special_AllPages.xhtml#File_Safe_areas_png)

[![Media as described below](../wiki-images/Safe_areas.png)](../wiki-images/Safe_areas.png)

Diagram of [safe area](Overscan.xhtml "Overscan") on 60 Hz NES PPU (and other 5.37 MHz NTSC VDPs) 

Red
    Danger Zone, top and bottom 8 lines
    Most 60 Hz TVs hide all of this. Keep [scroll seam artifacts](Mirroring.xhtml "Mirroring") here if possible. (Visible at 50 Hz)
Yellow
    Action Safe Area, 256x224, (0, 8)-(255, 231)
    Most TVs show some of this.
Blue
    Post-1985 Safe Area, 240x212, (8, 16)-(247, 227)
    Practically all TVs show this. Platforms, pits, and indicators are fine.
Gray
    Title Safe Area, 224x192, (16, 24)-(239, 215)
    Marked on Nintendo background planning sheet. Keep menus, score, dialogue, and legal notices here.

The diagram applies to any signal source using TMS9918's 5.37 MHz dot clock rate, not specifically only the NES. This includes ColecoVision, SG-1000, Sega Master System, TurboGrafx-16 H32 mode, Genesis H32 mode, and Super NES. 

Made by [Damian Yerrick](User_Tepples.xhtml "User:Tepples") in February 2017. Revised in January 2023. Dedicated to public domain using Creative Commons Zero (CC0). 

This version is scaled to correct NTSC pixel aspect ratio (8:7). [File:Safe areas (pixel perfect).png](File_Safe_areas__pixel_perfect__png.xhtml "File:Safe areas \(pixel perfect\).png") is unscaled. 
