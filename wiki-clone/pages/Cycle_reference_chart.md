# Cycle reference chart

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Cycle_reference_chart) | View [other pages](Special_AllPages.xhtml#Cycle_reference_chart)

## Contents

  * 1 Clock rates
  * 2 CPU cycle counts
  * 3 See also
  * 4 References



## Clock rates

The **clock rate** of various components in the NES differs between consoles in the USA and Europe due to the different television standards used (NTSC M vs. PAL B). The color encoding method used by the NES (see [NTSC video](NTSC_video.xhtml "NTSC video")) requires that the master clock frequency be six times that of the color subcarrier, but this frequency is about 24% higher on PAL than on NTSC. In addition, PAL has more scanlines per field and fewer fields per second than NTSC. Furthermore, the PAL CPU's master clock could have been divided by 15 to preserve the ratio between CPU and PPU speeds, but Nintendo chose to keep the [Johnson counter](https://en.wikipedia.org/wiki/Ring_counter "wikipedia:Ring counter") structure, which always has an even period, and divide by 16 instead. 

So the main differences between the NTSC and PAL PPUs are depicted in the following table: 

Property  | NTSC (2C02)  | PAL (2C07)  | "Dendy" PAL Famiclone  | RGB (2C03/4/5)  | Brazil Famiclone  | Argentina Famiclone   
---|---|---|---|---|---|---  
Master clock speed  | 21.477272 MHz ± 40 Hz  
236.25 MHz ÷ 11 by definition  | 26.601712 MHz ± 50 Hz  
26.6017125 MHz by definition  | Like PAL  | Like NTSC  | 21.453671 MHz  
3.067875 GHz ÷ 143 by definition  | 21.492338 MHz  
42.984675 MHz ÷ 2 by definition   
CPU  | Ricoh 2A03  | Ricoh 2A07  | UMC UA6527P  | Ricoh 2A03  | UMC UA6527   
CPU clock speed  | 21.47~ MHz ÷ 12 = 1.789773 MHz  | 26.60~ MHz ÷ 16 = 1.662607 MHz  | 26.60~ MHz ÷ 15 = 1.773448 MHz  | Like NTSC  | 21.45~ MHz ÷ 12 = 1.787806 MHz  | 21.49~ MHz ÷ 12 = 1.791028 MHz   
[APU Frame Counter](APU_Frame_Counter.xhtml "APU Frame Counter") rate  | 60 Hz  | 50 Hz[1] | 59 Hz[2] | Like NTSC  | Like NTSC   
PPU  | Ricoh 2C02  | Ricoh 2C07  | UMC UA6538  | Ricoh 2C03, 2C04, 2C05  | UMC UA6548  | UMC UA6528P   
Master clocks per PPU dot  | 4  | 5  | 5  | 4  | 4   
PPU dots per CPU cycle  | 3  | 3.2  | Like NTSC   
CPU cycles per scanline  | 341 × 4÷12 = 1132⁄3 | 341 × 5÷16 = 1069⁄16 | 341 × 5÷15 = 1132⁄3 | Like NTSC   
Height of picture  | 240 scanlines  | 239 scanlines  | Like PAL  | Like NTSC  | thought to be like NTSC  | thought to be like PAL   
Nominal visible picture height (see [Overscan](Overscan.xhtml "Overscan"))  | 224 scanlines  | 268 scanlines  | Like PAL  | Like NTSC  | Like NTSC  | Like PAL   
"Post-render" blanking lines between end of picture and [NMI](NMI.xhtml "NMI") | 1 scanline  | 1 scanline  | 51 scanlines  | 1 scanline  | 1 scanline  | 51 scanlines   
Length of vertical blanking after NMI  | 20 scanlines (≈ 2273 CPU cycles)  | 70 scanlines (≈ 7459 CPU cycles)  | Like NTSC   
Time during which [OAM](PPU_OAM.xhtml "OAM") can be written  | Vertical or forced blanking  | Only during first 24 scanlines after NMI (≈2557 CPU cycles)  | Like NTSC  | Like NTSC   
"Pre-render" lines between vertical blanking and next picture  | 1 scanline   
Total number of dots per frame  | 341 × 261 + 340.5 = 89341.5  
(pre-render line is one dot shorter in every odd frame)  | 341 × 312 = 106392  | Like PAL  | 341 × 262 = 89342  | unknown  
either like NTSC or RGB  | Like PAL   
Total number of CPU cycles per frame  | 89341.5 ÷ 3 = 29780.5  | 106392 ÷ 3.2 = 33247.5  | 106392 ÷ 3 = 35464  | 89342 ÷ 3 = 297802⁄3 | unknown  
either like NTSC or RGB  | Like PAL famiclone   
Frame rate (vertical scan rate)  | 60.0988 Hz  | 50.0070 Hz  | Like PAL  | 60.0985 Hz  | 60.03 Hz  | 50.5027 Hz   
Color of top border  | Always black ($0E)   
[Side and bottom borders](NTSC_video.xhtml "NTSC video") | [Palette](PPU_palettes.xhtml "PPU palettes") entry at $3F00  | Always black ($0E), [intruding on left and right 2 pixels and top 1 pixel of picture](Overscan.xhtml#PAL "Overscan") | Like PAL[3] | Like NTSC[4] | Unknown  | Unknown, probably like PAL   
Color emphasis  
(with correlating bit in [PPUMASK](PPU_registers.xhtml "PPUMASK"))  | Blue (D7), green (D6), red (D5)  | Blue (D7), red (D6), green (D5)  | Like PAL  | Blue, green, red (full scale)  | Unknown   
Other quirks  | Early revisions cannot read back sprite or palette memory  |  |  | Many  | poorly-researched   
  
Some frequencies in the above table are rounded. 

The 2C03, 2C04, and 2C05 PPUs were all found in Nintendo's [Vs. System](https://en.wikipedia.org/wiki/Nintendo_VS._System "wikipedia:Nintendo VS. System") and [PlayChoice-10](https://en.wikipedia.org/wiki/PlayChoice-10 "wikipedia:PlayChoice-10") (a.k.a. PC10 or PC-10) arcade systems. Famicom Titler, Famicom TVs, and RGB-modded NES consoles would use either the 2C03 or a 2C05 with glue logic to unswap $2000 and $2001. (Later RGB mods used a 2C02 in output mode and faked out all palette logic.) 

The color emphasis bits on the PAL NES have their red and green bits in [PPUMASK](PPU_registers.xhtml "PPUMASK") swapped 

The authentic NES sold in Brazil is an NTSC NES with an adapter board to turn the NTSC video into [PAL-M video](https://en.wikipedia.org/wiki/PAL-M "wikipedia:PAL-M"), a variant of PAL using NTSC frequencies but PAL's color modulation. 

Micro Genius is a clone of the Famicom, manufactured by TXC Corporation of Taiwan and sold under various brand names in the 50 Hz market.[5] Among the best known brands is [Dendy](https://en.wikipedia.org/wiki/Dendy_\(console\) "wikipedia:Dendy \(console\)"), distributed in Russia by Steepler, and the attention given by Russian reverse engineers to this clone has led to "Dendy" becoming a common name for all PAL Micro Genius-type famiclones. Its chipset (UA6527P+UA6538) is designed for compatibility with Famicom games, including games with CPU cycle counting mappers (e.g. [VRC4](VRC2_and_VRC4.xhtml "VRC4")) and games that use a cycle-timed NMI handler (e.g. _Balloon Fight_). This explains the faster CPU divider and longer post-render period vs. the authentic PAL NES. 

To compensate for these differences, you can [detect the TV system](Detect_TV_system.xhtml "Detect TV system") at power-on. 

## CPU cycle counts

To make things easier for those programming on the NES, the below chart provides the number of CPU cycles that a particular PPU-oriented trait takes. 

Property  | NTSC  | PAL  | Dendy   
---|---|---|---  
Scanline (341 pixels)  | 1132⁄3 | 1069⁄16 | 1132⁄3  
HBlank (85 pixels)  | 281⁄3 | 269⁄16 | 281⁄3  
NMI to start of rendering  | 22731⁄3 | 74593⁄8 | 22731⁄3  
Frame  | 29780.5*  | 33247.5  | 35464   
PPU dots ÷ CPU cycles  | 3  | 3.2  | 3   
OAM DMA  | 513 (+1 if starting on CPU [get cycles](DMA.xhtml#Cadence "DMA"))   
  
* NTSC frame timing is 29780.5 cycles if rendering is enabled during the 20th scanline; 297802⁄3 otherwise. 

## See also

  * [PPU frame timing](PPU_frame_timing.xhtml "PPU frame timing")
  * [PPU rendering](PPU_rendering.xhtml "PPU rendering")
  * [Cycle counting](Cycle_counting.xhtml "Cycle counting")



## References

  1. ↑ nesdev forum post by thefox: <http://forums.nesdev.org/viewtopic.php?p=160349#p160349>
  2. ↑ nesdev forum post by Eugene.S: <http://forums.nesdev.org/viewtopic.php?p=174970#p174970>
  3. ↑ nesdev forum post by Eugene.S: <https://forums.nesdev.org/viewtopic.php?p=173764#p173764>
  4. ↑ nesdev forum post by lidnariq: <https://forums.nesdev.org/viewtopic.php?p=179705#p179705>
  5. ↑ Post by feos at [TASVideos](http://tasvideos.org/forum/viewtopic.php?p=467169#467169) and [NESdev](https://forums.nesdev.org/viewtopic.php?p=216078#p216078)


