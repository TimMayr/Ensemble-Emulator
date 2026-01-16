# Family BASIC Data Recorder

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Family_BASIC_Data_Recorder) | View [other pages](Special_AllPages.xhtml#Family_BASIC_Data_Recorder)

[![](../wiki-images/Nintendo-Family-BASIC-Data-Recorder.jpg)](File_Nintendo_Family_BASIC_Data_Recorder_jpg.xhtml)

[](File_Nintendo_Family_BASIC_Data_Recorder_jpg.xhtml "Enlarge")

The Family BASIC Data Recorder device

The **Family BASIC Data Recorder** (HVC-008) is a device manufactured by Matsushita/Panasonic for Nintendo and was released sometime in 1984 only in Japan. The device serves as an addition to the [Family BASIC Keyboard](Family_BASIC_Keyboard.xhtml "Family BASIC Keyboard") to read and write data from BASIC programs created by users. Several Famicom titles also used it to save its own data. The device itself can be substituted with any compact cassette player device or other audio capable devices that plug into the 3.5mm monaural phone jacks. 

Example Titles: 

  * _Family BASIC_
  * _Family BASIC V3_
  * _Arkanoid II_
  * _Castle Excellent_
  * _Excitebike_
  * _Lode Runner_
  * _Mach Rider_
  * _Nuts & Milk_
  * _Wrecking Crew_



## Contents

  * 1 Hardware interface
    * 1.1 Input ($4016 write)
    * 1.2 Output ($4016 read)
  * 2 Software
  * 3 References



## Hardware interface

The [Family BASIC Keyboard](Family_BASIC_Keyboard.xhtml "Family BASIC Keyboard") provides two ⅛" (3.5mm) monaural phone jacks which can be plugged into the data recorder. [This circuit](http://nesdev.org/tapedrv.PNG) will provide the same interface on the NES. 

### Input ($4016 write)
    
    
    7  bit  0
    ---- ----
    xxxx xExS
          | |
          | +- 1-bit DAC audio to audio cassette
          +--- When 0, force audio readback to always read as binary 0 (5V)
    

The audio to the cassette recorder goes through a first-order highpass at 100Hz and is attenuated to 5mVPP at the input to the recorder. 

### Output ($4016 read)
    
    
    7  bit  0
    ---- ----
    xxxx xxAx
           |
           +-- 1-bit ADC audio from audio cassette
    

Because of how magnetic tape works, playing back the tape will produce a signal that is the lowpassed derivative of the original. Then this audio from the cassette recorder goes through a highpass with corner frequency of 800Hz before being discretized. In simulation, square waves of frequency 600 Hz up to the bandwidth of the tape appear to be recovered by this processing. 

For an emulator, just play back the same bit stream as it was emitted. 

## Software

It is not known whether Family BASIC uses [Kansas City Standard](https://en.wikipedia.org/wiki/Kansas_City_Standard "wikipedia:Kansas City Standard") encoding, [Bell 103](https://en.wikipedia.org/wiki/Bell_103_modem "wikipedia:Bell 103 modem") or [202](https://en.wikipedia.org/wiki/Bell_202_modem "wikipedia:Bell 202 modem"), or some other arbitrary home-grown convention for encoding the audio on the tape. Castle Excellent's recorder handling code mostly exists between $8000 and $80FE, and provides save games using the 1200 baud ('CUTS') and _bit-reversed_ variant of Kansas City Standard. Wrecking Crew uses the [tape format](https://original.sharpmz.org/tapeprob01.htm) from the [Sharp MZ](https://en.wikipedia.org/wiki/Sharp_MZ "wikipedia:Sharp MZ") personal computer. 

For homebrew use, Chris Covell wrote a set of KCS [-generating](http://www.chrismcovell.com/TapeDump_Controls.html) and [-decoding](http://forums.nesdev.org/viewtopic.php?p=111232#p111232) routines, which could easily be adapted to work with the data recorder. 

## References

Reverse-engineered schematics by [Enri](http://cmpslv3.stars.ne.jp/): 

  * <http://cmpslv3.stars.ne.jp/Famic/Fambas.htm>
  * Also available [here](http://atariusa.com/Famicom_Schematics/FC%20Family%20Basic%20Keyboard%20and%20Sound%20Circuit%20Schematic.png) and [here](http://atariusa.com/Famicom_Schematics/FC%20Keyboard%20Key%20Schematic.png)
  * [ccovell identified Wrecking Crew's encoding](http://forums.nesdev.org/viewtopic.php?p=109242#p109242)



Categories: [File formats](Category_File_formats.xhtml), [Hardware](Category_Hardware.xhtml)
