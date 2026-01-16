# Talk:NTSC video

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANTSC_video) | View [other pages](Special_AllPages.xhtml#Talk_NTSC_video)

The part of the PPU that generates the video signal (immediately to the right of the NMI and CLK pins) appears to consist of a very long resistor between VCC and GND with taps at various points, and one of them (closest to VCC) is a switchable pulldown to GND (no doubt for color emphasis). The logic itself can select 12 different voltage levels (00/10/20/30, 0D/1D/2D/3D, synch, colorburst L/H, and GND), and they appear to correspond to 10 distinct voltage levels - one of the duplicates is at the highest voltage (likely 20/30), and the other one is the 3rd-lowest [non-GND] voltage (not sure which one). --[Quietust](User_Quietust.xhtml "User:Quietust") 17:59, 28 September 2011 (UTC) 

  * With the help of ChipSim, I've traced down exactly what the 12 voltage levels correspond to - Synch (GND), Colorburst L, Color 0D, Color xE/xF and Color 1D, Colorburst H, Color 2D, Color 00, Color 10, color 3D, Color 20 and Color 30 ("and" == two signals which select the same actual voltage). --[Quietust](User_Quietust.xhtml "User:Quietust") 11:17, 29 October 2012 (MDT)



## Contents

  * 1 Absolute Video Signal Timing
  * 2 C++ code?
  * 3 Differential phase distortion and black/white stroke widening/narrowing?
  * 4 Citation broken link



## Absolute Video Signal Timing

Some tests with Visual2C02 managed to confirm all of the contents of the Scanline Timing table, plus one more interesting bit of information: the _offset_. Specifically, the "active" region begins at cycle **4** , which is when the PPU is just finishing the attribute byte fetch for the 3rd tile in the scanline (the first 2 are fetched at the end of the previous scanline). --[Quietust](User_Quietust.xhtml "User:Quietust") 08:50, 1 November 2012 (MDT) 

    But during which cycles is sync output? We need that too to align the NES picture center relative to the NTSC standard picture center (see [Overscan](Overscan.xhtml "Overscan")). --[Tepples](User_Tepples.xhtml "User:Tepples") 09:41, 1 November 2012 (MDT) 

    It's probably been mentioned elsewhere, but for completeness: cycles 280 through 304, inclusive, which happens to correspond to when the VRAM address gets reloaded during the prerender scanline. --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 07:24, 14 February 2013 (MST)

## C++ code?

Is this really C++ code? Can we call it what it is, if it's not? 
    
    
    auto InColorPhase = [=](int color) { return (color + phase) % 12 < 6; }; // Inline function
    

\-- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 07:26, 13 August 2014 (MDT) 

    It appears to be C++11 code. Not all C++ compilers understand C++11 yet. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 19:42, 13 August 2014 (MDT)

    

    Am I just out of touch? Is C++11 the accepted standard now? I didn't know it was safe to call it "C++" now, but maybe it is? :S - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 21:08, 13 August 2014 (MDT) 

    Yes, it's the standard. The draft was commonly referred to as "C++0x" because they aimed to get it accepted by the end of 2009. It became [C++11](https://en.wikipedia.org/wiki/C%2B%2B11 "wikipedia:C++11") when the changes were officially added to ISO C++ in 2011. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 06:56, 14 August 2014 (MDT) 

    It was an official standard in 2011, but that's different than it being the primary flavour of C++, which is really what I meant. After looking around, I've noted that as usual Microsoft has done a pick-and-choose with which features to implement, but the C++11 features they did implement have been enabled by default since VS2010. GCC on the other hand much more fully supports it, but requires a compiler switch to enable. Clang is in the same situation as GCC. Intel's compiler looks like partial support, off by default. I imagine other compilers are in similar states. As such, I'd say we're in a period of transition, but C++11 will surely become the default before long. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:56, 14 August 2014 (MDT)

## Differential phase distortion and black/white stroke widening/narrowing?

I removed this, because it seemed to be debated in a discord discussion. I think the effect may be real, but it might not be the differential phase distortion effect? 
    
    
    This can also be seen to slightly widen black strokes on a white background and narrow white strokes on a black background.
    

So, it might belong elsewhere in the article. Keeping it here as a reminder to discuss/resolve. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 01:10, 1 April 2023 (UTC) 

## Citation broken link

<https://www.maximintegrated.com/en/app-notes/index.mvp/id/1184>

This link is broken now. 

    The tutorial appears to have moved to analog.com. I've updated the link. --[Fiskbit](User_Fiskbit.xhtml "User:Fiskbit") ([talk](User_talk_Fiskbit.xhtml "User talk:Fiskbit")) 07:58, 6 April 2024 (UTC)
