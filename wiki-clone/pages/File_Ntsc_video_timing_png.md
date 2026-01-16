# File:Ntsc video timing.png

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/File%3ANtsc_video_timing.png) | View [other pages](Special_AllPages.xhtml#File_Ntsc_video_timing_png)

[![Media as described below](../wiki-images/Ntsc_video_timing.png)](../wiki-images/Ntsc_video_timing.png)

A visualization of the NTSC PPU's video output signal, using different colors to denote each part of the signal. 

The video output of the PPU is delayed by 1 pixel clock; this means that cycle 0, scanline 0 according to the [PPU Frame Timing Diagram](File_Ppu_svg.xhtml) is marked by the black pixel. 

  * Cyan is the horizontal blanking pulse
  * Dark blue is the "back porch" after HBlank 
    * Yellow is the colorburst signal
  * Red is the visible output region, including the left and right border 
    * Bright green is the grayscale pulse (at cycle 326)
    * Bright red is where the background and sprites are actually visible
    * The orange dot is skipped on odd frames (340, 261)
  * Orange is the "front porch" before HBlank
  * Magenta is the vertical blanking pulse
  * Dark green is the surrounding vertical blanking region (and the sync separator)


