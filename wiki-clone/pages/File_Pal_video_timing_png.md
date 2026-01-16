# File:Pal video timing.png

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/File%3APal_video_timing.png) | View [other pages](Special_AllPages.xhtml#File_Pal_video_timing_png)

[![Media as described below](../wiki-images/Pal_video_timing.png)](../wiki-images/Pal_video_timing.png)

## Summary

A visualization of the PAL PPU's video output signal, using different colors to denote each part of the signal. Based on [Ntsc_video_timing.png](File_Ntsc_video_timing_png.xhtml). 

The video output of the PPU is delayed by 1 pixel clock in relation to the working PPU memory. In addition, the active video portion is delayed by another 1 pixel clock. Cycle 0, scanline 0 according to the [PPU Frame Timing Diagram](File_Ppu_svg.xhtml) is marked by the black pixel. 

  * Cyan is the horizontal blanking pulse
  * Dark blue is the "back porch" after HBlank 
    * Yellow is the colorburst signal
  * Red is the visible output region, including the left and right border 
    * Bright red is where the background and sprites are actually visible
    * Dark red is where the border crops into the active video output
  * Orange is the "front porch" before HBlank
  * Magenta is the vertical blanking pulse
  * Dark green is the surrounding black border, vertical blanking region, and the sync separator


