# Overscan

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Overscan) | View [other pages](Special_AllPages.xhtml#Overscan)

In television, **overscan** occurs when a video monitor draws the picture slightly larger than the screen, with some of the picture hidden by the border. Consumer TV monitors are generally configured with some amount of overscan to accommodate component tolerances in CRTs.[[1]](http://www.mastersofcinema.org/reviews/03lookingbeyond.htm)[(Overscan)](https://en.wikipedia.org/wiki/Overscan "wikipedia:Overscan") Broadcasting standards generally prescribe a **safe area** , or part of the picture unlikely to be covered by overscan. This often consists of a minimum visible area, the **title safe area** , and a maximum, called the **action safe area**. For example, the BBC recommends keeping a 5 percent margin inside the screen and putting all important information, such as text, within the center 90 percent of the width and height of the screen.[[2]](http://www.bbc.co.uk/guidelines/dq/pdf/tv/tv_standards_london.pdf)

The NES [PPU](PPU.xhtml "PPU") always generates a 256x240 pixel picture. But not all of these pixels are visible on an NTSC TV, especially the top and bottom few scanlines. Because very few NTSC TVs display much if any of the picture outside the center 256x224 pixels, games can treat this area as an action safe area and arrange their video memory updates to keep glitches caused by nametable [mirroring](Mirroring.xhtml "Mirroring") or use of ["negative" Y scroll](PPU_scrolling.xhtml#Y_increment "PPU scrolling")[1] outside this action safe area. In addition, to accommodate TVs with larger overscan, they keep important information 16 pixels inside that, forming a 224x192 pixel title safe area. 

## Contents

  * 1 NTSC
    * 1.1 For game developers
    * 1.2 For emulator developers
    * 1.3 Statistics
  * 2 PAL
  * 3 References



## NTSC

[BT.601](https://en.wikipedia.org/wiki/Rec._601 "wikipedia:Rec. 601") (formerly CCIR 601) is a standard for digital processing of component video signals, describing a way to sample [NTSC video](NTSC_video.xhtml "NTSC video") at exactly 13.5 MHz with 720 samples per line.[2] This includes the active portion of the line, as defined by SMPTE 170M,[3] plus some extra pixels at the sides for [nominal analog blanking](https://en.wikipedia.org/wiki/Nominal_analogue_blanking "wikipedia:Nominal analogue blanking"), to capture signals slightly before and after the clean aperture in case of a sync shift, allowing the signal to be recentered before broadcast. The convention is that the center 704 samples of this represent picture information within the 4:3 frame, for 704x480 non-square pixels. (ATSC codified this 704-pixel width by the mid-1990s.[4]) Equivalently, one could sample at 135/11 MHz ≈ 12.273 MHz to produce 640x480 square pixels in the "clean aperture".[5] This means the signal associated with one scanline is 640/(135/11) μs = 704/13.5 μs ≈ 52.148 microseconds long. 

The NTSC color subcarrier is defined as 227.5/286 times the 4.5 MHz audio subcarrier, or 39,375,000/11 Hz = 315/88 MHz ≈ 3.5795 MHz. The NES master clock is always 6 times the color subcarrier frequency because there are 12 hues on the NES PPU, and the [color generator](NTSC_video.xhtml "NTSC video") generates hues with a set of ring counters clocked on the rising and falling edges of the master clock. So the PPU's pixel rate is 1/4 of the master clock, or 6/4 of the color subcarrier, or 315/88*6/4 ≈ 5.3693 million pixels per second. (This pixel rate appears to have originated in the TMS9918 VDP used in the ColecoVision. The NTSC Sega Master System VDP and Super NES PPU use this same rate, as does the Sega Genesis VDP in the 256-pixel mode that several multiplatform titles used.) 

Multiplying the pixel rate by the scanline length gives 39,375,000*6/4/11*640/(135,000,000/11) = 280 pixels per scanline. The PPU puts signal in 256 of these and a border at the left and right sides. The color of this border is the same as the backdrop color (usually the value in [$3F00](PPU_palettes.xhtml "PPU palettes")). This makes the [pixel aspect ratio](https://en.wikipedia.org/wiki/pixel_aspect_ratio "wikipedia:pixel aspect ratio") on a 4:3 TV to be 240/280*4/3 = exactly 8:7, or about 1.143:1. (The other 61 pixels in each 341-pixel scanline period are the horizontal blanking period. Side border is counted in the 4:3 frame; horizontal blanking is not.) 

In NTSC video sampled at 13.5 MHz, there are 481.5 non-square pixels from the start of the horizontal sync pulse to the center of the picture. This equals 191.5 NES pixels. But there are 65 pixels from the start of hsync to the left side of the active picture, or 193 to the center. So the NES picture is in theory a pixel and a half to the right of center. 

### For game developers

For a 280x240 pixel picture, the 90 percent safe area is 252x216 pixels. Some CRT TVs have rounded corners; it's a good idea to keep text away from the corners if possible. 

This, however, doesn't give developers a free pass to ignore glitches caused by background mirroring. Actual TVs show about 224 lines of the signal, hence the commonly reported 256x224 resolution. But the vertical position may be slightly off center. Emulators and LCD HDTVs tend to use lines 8 to 231, but some TVs have been seen to show lines 12 to 235. In fact, even some CRT SDTVs made in the 2000s show more of the bottom than the top; this may be so that tickers on cable news channels aren't cut off. 

As seen in [an interview with the _Zelda_ team](https://forums.nesdev.org/viewtopic.php?t=5991), Nintendo's official [background planning sheets](http://www.nintendo.co.uk/games/oms/mario-maker-3ds/_downloads/super_mario_bros_sheets.pdf) had a conservative 224x192 pixel [title safe area](https://en.wikipedia.org/wiki/Safe_area#Title_safe_area "wikipedia:Safe area"), covering 80% of the width and height of the overall picture. It reserved 24 lines at the top and bottom and 16 pixels at the left and right as potential overscan. 

Later in the NES's commercial life, as TVs became manufactured to tighter tolerances, some developers expected more of the picture to be visible. The scaled mode of PocketNES, an NES emulator for Game Boy Advance, hides 8 pixels of overscan on the left and right, 16 on the top, and 11 on the bottom. This happens to match the "bad case" of how TVs from the 1980s and later were calibrated. If all your graphics are visible in PocketNES, they should be visible on TVs made since the 1980s. 

[![](../wiki-images/Safe_areas.png)](File_Safe_areas_png.xhtml)

Diagram of action, post-1985, and title safe areas within the 4:3 frame. (Transcript behind link)

You can preview a game's overscan compliance by pasting a border template image over a screenshot from an emulator. 

  * [![Border template for action, post-1985, and title safe areas](../wiki-images/Safe_area_overlay.png)](File_Safe_area_overlay_png.xhtml "Border template for action, post-1985, and title safe areas")

Border template for action, post-1985, and title safe areas 

  * [![Matt Hughson's CRT TV displaying the safe areas pattern](../wiki-images/Matt_Hughson_TV_20221003_112626.jpg)](File_Matt_Hughson_TV_20221003_112626_jpg.xhtml "Matt Hughson's CRT TV displaying the safe areas pattern")

Matt Hughson's CRT TV displaying the safe areas pattern 




Since 2007, most new TVs have been 16:9. To prepare your game for zoomed-in mode, assume a 160-line safe area surrounded by an overscan of 40 lines on the top and bottom. This is close to the unscaled mode of PocketNES without sprite following. Side borders are more likely to be visible on 16:9 sets. 

### For emulator developers

There are two ways to emulate the pixel aspect ratio of the NES: scale before padding and pad before scaling. The NES PPU hardware performs the padding first, adding 24 pixels of border to form a 280x240 pixel picture that can be resized to 320x240, 640x480, or 960x720 square pixels, or to 352x240 or 704x480 if your SDTV output circuit produces non-square pixels at 13.5 MHz (Rec. 601/DVD dot clock, 132/35*colorburst) or 13.423 MHz (PlayStation dot clock, 15/4*colorburst). But as a slight optimization, you can scale first (256 * 8/7 = 292) and then pad: stretch the 256x240 pixels to 292x240, 584x480, 876x720, or 1168x960 square pixels or 320x240 or 640x480 non-square pixels. Or when enlarging 4.5x to fit a 1080p screen, output 256*(1080/240)*(8/7) = 1316 pixels. Then you can emulate the overscan by drawing a TV bezel on top of the edges of the emulated picture. 

### Statistics

Here are some numbers of overscan lines from actual TVs, determined using a homebrew paint program on an NTSC NES + [PowerPak](PowerPak.xhtml "PowerPak"). Ranges denote fewer hidden lines at top and bottom center (x = 120-135) than at the corners (x = 16-31 and 223-239) due to CRT curvature. 

TV | Description | Mode | Top lines | Bottom lines   
---|---|---|---|---  
Vizio VX32L | LCD HDTV | Normal | 8 | 6   
Vizio VX32L | LCD HDTV | Zoom | 36 | 35   
Magnavox MS2530 C225 | CRT SDTV |  | 12-14 | 8-10   
Sylvania 6420FF | CRT SDTV |  | 13-15 | 8-11   
  
## PAL

When sampled at 14.75 MHz (768x576) or 7.375 MHz (384x288), PAL video has square pixels. The width of a scanline is 768 pixels, or 768/14750000 seconds, or 52.068 microseconds.[5]

The PAL color subcarrier is defined as 4,433,618.75 Hz. The PAL NES master clock is six times that, and the PPU generates one pixel for every 5 master clock cycles, or 5320342.5 Hz. This makes the width of a scanline 5320342.5*768/14750000 = 277 pixels, and the pixel aspect ratio 7375000/5320342.5 = (59*125000)/(165*64489/2) ≈ 1.3862:1. This fraction doesn't simplify nearly as nicely as NTSC because of the large prime factor 64489,[6] but 7:5 (1% too wide) and 18:13 (0.11% too narrow) are close. 

The width of the picture is nearly the same as on NTSC, so we need not reconsider horizontal placement. The border is always black, and it extends into the leftmost and rightmost 2 pixels[7] and the top scanline, the one that never shows sprites.[8] However, the total picture is 288 lines tall, making the safe area roughly 260 lines tall. This means every TV shows blank bars at the top and bottom of the 240-line active area that the NES generates. So for a PAL-only title, the developer need not concern himself with TVs cutting off the status bar, but minimizing artifacts caused by nametable mirroring and sprite pop-on at the top becomes paramount. Both the PAL NES and PAL famiclones (such as Dendy) behave this way.[9]

Emulator developers can simulate this system's pixel aspect ratio by stretching the picture to a multiple of 355×240, such as 1065×720 or 1420×960. 

## References

  1. ↑ Konami's _Teenage Mutant Ninja Turtles_ title screen and horizontal scrolling segments use negative Y.
  2. ↑ [Recommendation BT.601](https://www.itu.int/dms_pubrec/itu-r/rec/bt/R-REC-BT.601-7-201103-I!!PDF-E.pdf) Accessed 2022-05-14.
  3. ↑ [SMPTE-170M and BT.1700](https://www.itu.int/rec/R-REC-BT.1700-0-200502-I/en)
  4. ↑ [ATSC spec](https://prdatsc.wpenginepowered.com/wp-content/uploads/2021/04/a_53-Part-1-6-2007.pdf)
  5. ↑ 5.0 5.1 Chris Pirazzi. "[Programmer's Guide to Video Systems](http://lurkertech.com/lg/video-systems/)". _Lurkertech_. Accessed 2015-11-17.
  6. ↑ "[Colour Subcarrier - an overview](https://www.sciencedirect.com/topics/computer-science/colour-subcarrier)". From Charles Poynton, ed. _Digital Video and HD: Algorithms and Interfaces_ (2nd ed.) Morgan Kaufmann, 2012.
  7. ↑ BBS topic: [Looking for PAL display that displays ALL pixels](http://forums.nesdev.org/viewtopic.php?f=2&t=6132)
  8. ↑ BBS topic: [240p test suite](http://forums.nesdev.org/viewtopic.php?p=159304#p159304)
  9. ↑ BBS topic: [240p test suite](http://forums.nesdev.org/viewtopic.php?p=173764#p173764)


