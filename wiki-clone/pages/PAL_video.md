# PAL video

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/PAL_video) | View [other pages](Special_AllPages.xhtml#PAL_video)

The 2C07 PPU in the PAL NES and the 6538 PPU in PAL famiclones generate composite **PAL video** in much the same way that the 2C02 generates [NTSC video](NTSC_video.xhtml "NTSC video"): as a square wave between "high" and "low" levels, offset by 0 to 5.5 master clock cycles from the phase of color burst. 

PAL NES video has several key differences from NTSC NES video: 

[Phase Alternating Line](https://en.wikipedia.org/wiki/PAL "wikipedia:PAL")
    The phase of the V subcarrier inverts between each scanline and the next. The color burst is ideally at 135 degrees relative to the [UV plane](https://en.wikipedia.org/wiki/YUV "wikipedia:YUV"), not 180. This allows the TV to detect the phase of the V subcarrier by comparing the phase of color burst to that of the previous line. The PAL NES outputs a somewhat nonstandard 120 degree color burst.
Higher color subcarrier frequency
    PAL-B uses a color subcarrier frequency of 4.43361875 MHz. The 2C07 and 6538 still use the same design of 12 oscillators, so the master clock crystal is now six times that, or 26.6017125 MHz. Scanlines still consist of 341 pixels, but in order for scanlines to happen at approximately the correct frequency, each pixel is 5 master clocks (5/6 of a pixel) instead of 4 (4/6). This reduces the amount of luma-chroma crosstalk. 

    The rarer PAL-N (Argentina) and PAL-M (Brazil) Famiclones use a lower subcarrier frequency. Scanlines are still 341 pixels, but their pixels are only 4 master clocks.

50 Hz
    Each field contains 312 lines, not 262, producing vertical [underscan](Overscan.xhtml "Overscan") and a pixel aspect ratio close to 1.386:1, which is wider than 1.143:1 of NTSC.
No short line
    The dot at the end of pre-render scanline is never skipped. This combined with the fact that 312 is a multiple of 6 causes the chroma dot pattern not to vary from one field to the next, producing dirty stills but OK movement.
Phase per line
    The PAL NES scanline has 284⅙ chroma cycles, rather than the standard 283¾. This results in a 6-line pattern of color artifacts, as the phase changes by 60° on each line. NTSC by comparison has 227⅓ chroma cycles per line, giving a 3-line pattern.
Larger border
    The border, including the area outside the picture, is always black rather than using the [backdrop](Glossary.xhtml#B "Glossary"), and it covers the top scanline and the left and right 2 pixels of each remaining scanline.
Hue shift
    The color hue is slightly different relative to NTSC. There is a 15 degree shift due to the PAL's definition of color burst being rotated by 45 degrees relative to NTSC, but this difference may be mitigated for brighter colors by PAL's cancellation of differential phase distortion effects (see below), which on NTSC appear to cause a similar shift but only for brighter colors.
Differential phase distortion
    The output is subject to differential phase distortion like the NTSC PPU, and on individual scanlines the effect may be even more severe on PAL[1], but because the alternating-line mechanism causes the hue distortion to be opposite on consecutive lines, the color error can cancel out vertically when it's not on isolated rows. (See: [NTSC video: Color phases](NTSC_video.xhtml#Color_phases "NTSC video"))

The 2C07 and 6538 additionally have minor timing differences related to post-render length and OAM refresh; see [Clock rate](Cycle_reference_chart.xhtml#Clock_rates "Cycle reference chart"). 

There are two different ways that a TV can _decode_ PAL video. The simple way treats PAL as if it were NTSC, modulo the five differences above. If the television signal is subject to phase distortion, this produces an artifact called [Hanover bars](https://en.wikipedia.org/wiki/Hanover_bars "wikipedia:Hanover bars"). The other way is a comb filter: average the received scanline's chroma (U and V channels) with the chroma from the received scanline immediately above it in the same field and use that. This method, invented by an engineer at Telefunken, eliminates Hanover bars at the expense of a 64 microsecond quartz delay line, a royalty payable to Telefunken, and blurred vertical color detail. Less expensive PAL TVs used the simple method and relied on the change in color burst phase over time to cancel out the Hanover bars, though this technique may not be effective with the slightly nonstandard scanline length in the PAL signal from an NES. 

Emulator developers planning to simulate PAL video decoding can use a signal captured from a 6538 to test the decoder: 

  * [250 MHz capture](File_PAL_signal_6538_250MHz_png.xhtml "File:PAL signal 6538 250MHz.png")
  * [Same capture downsampled to twelve times the subcarrier frequency (53.2 MHz)](File_PAL_signal_6538_53_2MHz_png.xhtml "File:PAL signal 6538 53.2MHz.png"), with one pixel per rise or fall of the master clock



## Contents

  * 1 Scanline Timing
  * 2 Color Phases
  * 3 Libraries
  * 4 See Also
  * 5 References



## Scanline Timing

Values in PPU pixels (341 total per scanline). 

The video output of the PPU is delayed by 1 pixel clock in relation to the working PPU memory. In addition, the active video portion is delayed by another 1 pixel clock. Cycle 0, scanline 0 according to the [PPU Frame Timing Diagram](File_Ppu_svg.xhtml) is marked by the black pixel. 

The start times of each entry are thus relative to cycle 0, taking into account the delay. Timings based on Breaking NES Wiki reverse engineered horizontal[2] and vertical [3] decoder functions of the 2C07-0. 

[![](../wiki-images/Pal_video_timing.png)](File_Pal_video_timing_png.xhtml)

A visualization of the tables to the left, starts with cyan for horizontal sync

Rendering scanlines (n=240): 

■ | name | start | duration | row | notes   
---|---|---|---|---|---  
■ | horizontal sync | 277 | 25 | 0-239   
■ | back porch (black) | 302 | 4 | 0-239   
■ | colorburst | 306 | 15 | 0-239   
■ | back porch, continued (black) | 321 | 5 | 0-239   
■ | left border (black) | 326 | 18 | 0-239   
■ | left border, cropping active video (black) | 3 | 2 | 0-239   
■ | active | 5 | 252 | 0-239 | line 0 is entirely cropped by the black border   
■ | right border, cropping active video (black) | 257 | 2 | 0-239   
■ | right border (black) | 259 | 9 | 0-239   
■ | front porch (black) | 268 | 9 | 0-239   
  
Post-render blanking scanlines (n=30): 

■ | name | start | duration | row | notes   
---|---|---|---|---|---  
■ | horizontal sync | 277 | 25 | 240-269   
■ | back porch (black) | 303 | 4 | 240-269   
■ | colorburst | 306 | 15 | 240-269   
■ | back porch, continued (black) | 321 | 5 | 240-269   
■ | vertical blanking region (black) | 326 | 283 | 240-269 | in the 2C07, VBlank flag is set on scanline 241   
■ | front porch (black) | 268 | 9 | 240-269   
  
Vertical sync scanlines (n=3): 

■ | name | start | duration | row   
---|---|---|---|---  
■ | vertical blanking pulse | 277 | 320 | 270-272   
■ | vertical sync separator (black) | 256 | 12 | 270-272   
■ | vertical sync separator (front porch, black) | 268 | 9 | 270-272   
  
Pre-render blanking scanlines (n=39): 

■ | name | start | duration | row | notes   
---|---|---|---|---|---  
■ | horizontal sync | 277 | 25 | 273-311   
■ | back porch (black) | 303 | 4 | 273-311   
■ | colorburst | 306 | 15 | 273-311   
■ | back porch, continued (black) | 321 | 5 | 273-311   
■ | vertical blanking region (black) | 326 | 283 | 273-311 | in the UMC UA6538, VBlank flag is set on scanline 291; VBlank (both 2C07 and UA6538) is cleared on scanline 311   
■ | front porch (black) | 268 | 9 | 273-311   
  
This amounts to a total of 312 scanlines. 

## Color Phases

Like NTSC, there are still 12 color square waves spaced at regular phases. However, because each scanline must encode the [V](https://en.wikipedia.org/wiki/YUV "wikipedia:YUV") component with opposite sign, there isn't a direct mapping of "encoded color in palette RAM" to "specific square wave". 

PAL colorburst (pure shade -U+V) is the same phase as phase 7. The same two square wave oscillators are used for pairs of hues: 3 and 2, 4 and 1, 5 and C, 6 and B, 7 and A, 8 and 9. 

The PAL PPUs are still susceptible to the same [differential phase distortion](NTSC_video.xhtml#Differential_Phase_Distortion "NTSC video") – in fact, it's often a little worse due to PAL-B's higher chroma frequency. However, delay-line chroma decoding will compensate for this effect. 

## Libraries

  * [LMP88959 (EMMIR)'s PAL decoder](https://github.com/LMP88959/PAL-CRT/)



## See Also

  * [NTSC video](NTSC_video.xhtml "NTSC video")



## References

  1. ↑ [Re: PAL chroma merging?](https://forums.nesdev.org/viewtopic.php?p=133640#p133640) TheFox observes a ~60° hue difference between even and odd scanlines.
  2. ↑ [Breaking NES Wiki article on H counter decoder in 2C07-0](https://github.com/emu-russia/breaks/blob/master/BreakingNESWiki_DeepL/PPU/pal.md#h-decoder)
  3. ↑ [Breaking NES Wiki article on V counter decoder in 2C07-0](https://github.com/emu-russia/breaks/blob/master/BreakingNESWiki_DeepL/PPU/pal.md#v-decoder)



  * [What is the cause of these three effects?](https://forums.nesdev.org/viewtopic.php?t=18406) \- Some strange decoding effects observed on a PAL television.


