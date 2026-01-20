# File:NTSC video ragged box animated.gif

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/File%3ANTSC_video_ragged_box_animated.gif) | View [other pages](Special_AllPages.xhtml#File_NTSC_video_ragged_box_animated_gif)

[![Media as described below](../wiki-images/NTSC_video_ragged_box_animated.gif)](../wiki-images/NTSC_video_ragged_box_animated.gif)

## Summary

Animated version of [https://www.nesdev.org/wiki/File:NTSC_video_ragged_box.png](File_NTSC_video_ragged_box_png.xhtml). 

The second frame is made by offsetting the frame by 1 "scanline". 

The filter is replicated as two Convolution Matrix passes in linear light space using GIMP 2.10.32. The kernels used are [1 1 0 1 1] and [1 3 4 3 1] respectively. 

After filtering the resulting kernels, an approximate 2.4 gamma curve is applied using the Levels tool, setting the middle slider to 1/2.4 (approx. 0.42). 

This derivative work is also licensed under the [WTFPL](http://sam.zoy.org/wtfpl/COPYING). 
