# File:NTSC video ragged box.png

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/File%3ANTSC_video_ragged_box.png) | View [other pages](Special_AllPages.xhtml#File_NTSC_video_ragged_box_png)

[![Media as described below](../wiki-images/NTSC_video_ragged_box.png)](../wiki-images/NTSC_video_ragged_box.png)

How an [NTSC video](NTSC_video.xhtml "NTSC video") signal gets generated in the PPU and decoded by the TV 

Horizontal scale: 1 diagram pixel = 1 NTSC [master clock](Cycle_reference_chart.xhtml#Clock_rates "Cycle reference chart") (21.5 MHz) cycle; 4 diagram pixels = 1 NES pixel; 6 diagram pixels = 1 color subcarrier cycle 

Top row: what goes on in the NES PPU 

  1. Generate the subcarrier for a solid red screen (color $16).
  2. A shape drawn in this color, including a 2-pixel-wide vertical line.
  3. Multiply it by 0 outside of the shape and 1 inside the shape. Notice how the subcarrier protrudes into the shape.



Bottom row: what goes on in the TV when separating luma from chroma 

  1. Incoming picture signal on the composite
  2. Impulse response of the low-pass filter
  3. Picture signal convolved with the low-pass filter, used as luma. Notice the ragged left and right sides of the vertical line.



The filter in this diagram is an FIR filter [1 4 7 8 8 8 7 4 1]/48, which factors to [1 1][1 1][1 1][1 1 1][1 0 0 1]/48. A real TV might use a Bessel filter (near-linear-phase IIR filter), but the principle is the same: filter out anything above 3 MHz. 

Permission is granted to use this copyrighted illustration under the [WTFPL](http://sam.zoy.org/wtfpl/COPYING). 
