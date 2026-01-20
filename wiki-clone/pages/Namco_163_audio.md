# Namco 163 audio

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Namco_163_audio) | View [other pages](Special_AllPages.xhtml#Namco_163_audio)

The [Namco 163](INES_Mapper_019.xhtml "Namco 163") offers up to 8 additional sound channels that play wavetable samples while the [175 and 340](INES_Mapper_210.xhtml "INES Mapper 210") do not. Each waveform can be of a configurable length, and each channel has linear volume control. It has $80 bytes of sound RAM shared by channel registers and wavetable samples; at least $40 bytes are dedicated to samples, with more available if not all channels are used. 

The chip is unable to clock every channel at once, so it cycles though channels, updating one every 15 CPU cycles. Because of this, the chip allows the game to configure the number of enabled channels. When fewer channels are enabled, the channels are clocked more often, allowing for higher tones with longer, more detailed waveforms. When more channels are enabled, clocking slows down since each channel has to wait its turn, resulting in lower tones and a high-pitched whining noise at the switching frequency. Most games using this IC used only 4 channels. 

## Contents

  * 1 Registers
    * 1.1 Sound Enable ($E000-E7FF)
    * 1.2 Address Port ($F800-$FFFF)
    * 1.3 Data Port ($4800-$4FFF)
      * 1.3.1 Sound RAM $78 - Low Frequency
      * 1.3.2 Sound RAM $79 - Low Phase
      * 1.3.3 Sound RAM $7A - Mid Frequency
      * 1.3.4 Sound RAM $7B - Mid Phase
      * 1.3.5 Sound RAM $7C - High Frequency and Wave Length
      * 1.3.6 Sound RAM $7D - High Phase
      * 1.3.7 Sound RAM $7E - Wave Address
      * 1.3.8 Sound RAM $7F - Volume
      * 1.3.9 Other Channels
  * 2 Waveform
  * 3 Channel Update
  * 4 Frequency
  * 5 Mixing
  * 6 References



## Registers

### Sound Enable ($E000-E7FF)
    
    
    7  bit  0
    ---- ----
    .SPP PPPP
     ||| ||||
     |++-++++- Select 8KB page of PRG-ROM at $8000-$9FFF
     +-------- Disables sound if set.
    

Sound is enabled on the 163 by writing a clear bit 6 to this register (0 is sufficient). 

### Address Port ($F800-$FFFF)
    
    
    7  bit  0   (write only)
    ---- ----
    IAAA AAAA
    |||| ||||
    |+++-++++- Address
    +--------- Auto-increment
    

Writing to this register sets the internal address to write to or read from. If the 'I' bit is set, the address will increment on writes and reads to the Data Port ($4800), wrapping $7F->$00 (address is 7 bits wide). 

### Data Port ($4800-$4FFF)
    
    
    7  bit  0   (read / write)
    ---- ----
    DDDD DDDD
    |||| ||||
    ++++-++++- Data
    

This Port accesses the 163's internal $80 bytes of sound RAM. Which of the $80 bytes is determined by the Address register ($F800). When read, the appropriate byte is returned. When written, the appropriate byte is stored. 

**Caution:** Do not read from sound RAM while auto-increment is enabled and [DMC](APU_DMC.xhtml "APU DMC") is playing. Otherwise, you are likely to miss bytes because of the [DMA multiple-read bug](DMA.xhtml#Register_conflicts "DMA"). Writing with auto-increment while DMC is playing behaves as expected. 

This RAM is used primarily for wavetables. The sound channel control registers are also set by writing to certain addresses in sound RAM: 

#### Sound RAM $78 - Low Frequency
    
    
    7  bit  0
    ---------
    FFFF FFFF
    |||| ||||
    ++++-++++- Low 8 bits of Frequency
    

#### Sound RAM $79 - Low Phase
    
    
    7  bit  0
    ---------
    PPPP PPPP
    |||| ||||
    ++++-++++- Low 8 bits of Phase
    

#### Sound RAM $7A - Mid Frequency
    
    
    7  bit  0
    ---------
    FFFF FFFF
    |||| ||||
    ++++-++++- Middle 8 bits of Frequency
    

#### Sound RAM $7B - Mid Phase
    
    
    7  bit  0
    ---------
    PPPP PPPP
    |||| ||||
    ++++-++++- Middle 8 bits of Phase
    

#### Sound RAM $7C - High Frequency and Wave Length
    
    
    7  bit  0
    ---------
    LLLL LLFF
    |||| ||||
    |||| ||++- High 2 bits of Frequency
    ++++-++--- Length of waveform ((64-L)*4 4-bit samples)
    

Equivalently, the waveform length = `256 - %LLLLLL00` samples. 

The Namco 129 was never released with a game that used audio. However, on the Namco 129 this register instead only encodes length, supporting any length of waveform and not only multiple of 4 samples. In exchange, the high 2 bits of frequency can't be configured and are effectively always 0. 

#### Sound RAM $7D - High Phase
    
    
    7  bit  0
    ---------
    PPPP PPPP
    |||| ||||
    ++++-++++- High 8 bits of Phase
    

The high byte of the 24-bit phase value directly determines the current sample position of the channel. Every time a channel is updated, the 18-bit frequency value is added to the 24-bit phase accumulator, which is stored in these three registers. 

The phase registers may be written to immediately set the phase of the wave. It is good practice to set the frequency to 0 before doing so. 

#### Sound RAM $7E - Wave Address
    
    
    7  bit  0
    ---------
    AAAA AAAA
    |||| ||||
    ++++-++++- Address of waveform (in 4-bit samples)
    

#### Sound RAM $7F - Volume
    
    
    7  bit  0
    ---------
    .CCC VVVV
     ||| ||||
     ||| ++++- Linear Volume
     +++------ Enabled Channels (1+C)
    

Note 'C' is available on register $7F ONLY. Those bits have no effect in other registers. 

  * When C=0, only channel 8 enabled
  * When C=1, channels 8 and 7 enabled
  * When C=2, channels 8, 7, 6 enabled
  * etc...



#### Other Channels

Above Sound RAM register descriptions ($78-$7F) are for the 8th channel. The other 7 channels are accessed via the same pattern, but each 8 bytes before the last: 
    
    
    Channel 8:  $78-$7F
    Channel 7:  $70-$77
    Channel 6:  $68-$6F
    Channel 5:  $60-$67
    Channel 4:  $58-$5F
    Channel 3:  $50-$57
    Channel 2:  $48-$4F
    Channel 1:  $40-$47
    

Again note that the 'C' bits in the final register is only available at address $7F. 

When channels are disabled, their registers are unused, and can be used for waveform data instead. 

## Waveform

Each enabled channel cycles through its waveform at a rate determined by the 18-bit frequency value 'F'. Each step in the waveform is 4-bits wide, and the number of steps is determined by the 'L' bits ((64-L)*4). Two samples are stored to a byte, which is little-endian (unlike the Game Boy's wavetable channel). 

The 'A' bits dictate where in the internal sound RAM the waveform starts. 'A' is the address in 4-bit samples, therefore a value of $02 would be the low 4 bits of address $01. A value of $03 would be the high 4 bits of address $01. 

For a visual example, assume you have the following sound RAM: 
    
    
    $00:    00 00 00 A8 DC EE FF FF EF DE AC 58 23 11 00 00
    $10:    10 21 53 00 00 00 00 00 00 00 00 00 00 00 00 00
    

And assume a channel has an 'A' value of $06, and an 'L' value of $38. That channel's waveform would be a sine wave, looking like the following: 
    
    
    F -       *****
    E -     **     **
    D -    *         *
    C -   *           *
    B -
    A -  *             *
    9 - 
    8 - *               *
    7 - 
    6 -
    5 -                  *             *
    4 -
    3 -                   *           *
    2 -                    *         *
    1 -                     **     **
    0 -                       *****
    

## Channel Update

Namco's 163 does not internally mix its channels. Instead, each channel is output one at a time. It takes exactly 15 CPU cycles to update and output one channel. When multiple channels are used it will cycle between them. With 6 or fewer channels, the time to update all channels is a rate faster than any audible frequency, and the difference between this serial output and mixing cannot be heard, but for 8 channels it creates a very loud and apparent noise at the update rate. For a Famicom through RF output, this noise is attenuated during demodulation (which performs a lowpass filter), but through A/V output to a TV that does not filter high frequencies, it can be very unpleasant. Only two games used all 8 channels: _King of Kings_ and _Erika to Satoru no Yume Bouken_. 

Channels  
Enabled | Update Rate  
(NTSC) | Update Rate  
(PAL)   
---|---|---  
1 | 119.318 kHz | 110.840 kHz   
2 | 59.659 kHz | 55.420 kHz   
3 | 39.773 kHz | 36.947 kHz   
4 | 29.830 kHz | 27.710 kHz   
5 | 23.864 kHz | 22.168 kHz   
6 | 19.886 kHz | 18.473 kHz   
7 | 17.045 kHz | 15.834 kHz   
8 | 14.915 kHz | 13.855 kHz   
  
The following is a speculative version of a single channel update, occurring every 15 CPU cycles: 
    
    
    * _w_[$80] = the 163's internal memory
    * _sample_(x) = (_w_[x/2] >> ((x&1)*4)) & $0F
    * _phase_ = (_w_[$7D] << 16) + (_w_[$7B] << 8) + _w_[$79]
    * _freq_ = ((_w_[$7C] & $03) << 16) + (_w_[$7A] << 8) + _w_[$78]
    * _length_ = 256 - (_w_[$7C] & $FC)
    * _offset_ = _w_[$7E]
    * _volume_ = _w_[$7F] & $0F
    
    
    
    _phase_ = (_phase_ + _freq_) % (_length_ << 16)
    _**output**_ = (_sample_(((_phase_ >> 16) + _offset_) & $FF) - 8) * _volume_
    

The _**output**_ will be held until the next channel update. The 24-bit _phase_ value will be stored back into _w_[$7D/$7B/$79]. 

The sample value is biased by -8, meaning that a waveform value of 8 represents the centre voltage. This means that volume changes have no effect on a sample of 8, will tend negative if <8 and positive if >8\. 

## Frequency

The wave position is driven by the high 8 bits of a 24-bit accumulator. Every 15 CPU clocks, one channel will add its 18-bit frequency to the accumulator. Because only one channel is updated per tick, the output frequency is thus divided by the number of channels enabled. 
    
    
    f = wave frequency
    l = wave length
    c = number of channels
    p = 18-bit frequency value
    n = CPU clock rate (≈1789773 Hz)
    
    f = (n * p) / (15 * 65536 * l * c)
    

## Mixing

The relative volume of the IC varies from game to game, unfortunately. The following samples have been recorded using various test programs. 

Difference between loudest APU square and loudest N163 square in 1-channel mode (dB)   
---  
Game  | [NewRisingSun](http://forums.nesdev.org/viewtopic.php?f=2&t=16910#p222765) | [jrlepage](http://forums.nesdev.org/viewtopic.php?f=2&t=16910&start=30#p222880) | [Rainwarrior](http://forums.nesdev.org/viewtopic.php?f=2&t=16910&start=15#p222873)  
Final Lap | 12.7 | 11.2 |   
Sangokushi II: Haou no Tairiku | 12.9 |  |   
Megami Tensei II | 13.0 | 11.9 |   
Rolling Thunder | 16.9 | 16.0 | 16.9   
King of Kings | 18.0 | 17.3 |   
Mappy Kids | 18.6 |  |   
Erika to Satoru no Yume Bouken | 18.8 |  | 18.9   
Youkai Douchuu-ki | 18.9 |  |   
Sangokushi: Chuugen no Hasha | 19.5 |  |   
  
Based on these measurements, the following submappers were allocated: 

[INES Mapper 019 submapper table](INES_Mapper_019_Submapper_table.xhtml "INES Mapper 019/Submapper table")  
---  
Submapper # | Meaning | Note   
0 | Default | Expansion sound volume unspecified   
_1_ | _Deprecated_ | Internal 128b RAM is battery backed, no external PRG-RAM is present. No expansion sound. (Equivalent to submapper 2 with 0 in PRG-NVRAM field.)   
2 | No expansion sound |   
3 | N163 expansion sound: **11.0-13.0 dB** louder than NES APU |   
4 | N163 expansion sound: **16.0-17.0 dB** louder than NES APU |   
5 | N163 expansion sound: **18.0-19.5 dB** louder than NES APU |   
  
  
Because the high frequency generated by the channel cycling can be unpleasant, and emulation of high frequency audio can be difficult, it is often preferred to simply sum the channel outputs, and divide the output volume by the number of active channels. For 6 channels or more, where the switching frequency crosses the threshold of audibility, this approximation will become slightly too loud as it fails to compensate for the transferred energy. 

## References

  * [Namcot 163 family pinout](Namcot_163_family_pinout.xhtml "Namcot 163 family pinout") \- diagram of the chip
  * [Namcot 106 Mapper Information](http://nesdev.org/namco106.txt) by Goroh, ZW4, nori
  * [NES Music Format Spec](http://kevtris.org/nes/nsfspec.txt) by Kevin Horton, N106 info by Mamiya



Categories: [Expansion audio](Category_Expansion_audio.xhtml)
