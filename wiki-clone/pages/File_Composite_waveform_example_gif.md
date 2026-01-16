# File:Composite waveform example.gif

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/File%3AComposite_waveform_example.gif) | View [other pages](Special_AllPages.xhtml#File_Composite_waveform_example_gif)

[![Media as described below](../wiki-images/Composite_waveform_example.gif)](../wiki-images/Composite_waveform_example.gif)

## Summary

This waveform steps through various grays and then stops on a color. 

Based on the original ASCII art of the example composite waveform. [https://www.nesdev.org/wiki/NTSC_video#Example_Waveform](NTSC_video.xhtml#Example_Waveform)

The composite signal steps through 6 gray colors ($0D, $0F, $2D, $00, $10, $30) then continues through with color $11. 

This plot was generated using [PPUPlayer's](https://github.com/emu-russia/breaknes/tree/main/BreaksPPU/PPUPlayer) video signal dump, plotted by Matplotlib via a python script and a batch file. 

[Source code for the underlying demo rom](https://github.com/Gumball2415/nes-scribbles/tree/main/nrom-composite-waveform), and this work is licensed under the [MIT License](https://mit-license.org). 
