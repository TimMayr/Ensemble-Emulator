# APU Mixer

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/APU_Mixer) | View [other pages](Special_AllPages.xhtml#APU_Mixer)

The [NES APU](APU.xhtml "APU") mixer takes the channel outputs and converts them to an analog audio signal. Each channel has its own internal digital-to-analog convertor (DAC), implemented in a way that causes non-linearity and interaction between channels, so calculation of the resulting amplitude is somewhat involved. In particular, games such as _Super Mario Bros._ and _StarTropics_ use the DMC level ($4011) as a crude volume control for the triangle and noise channels. 

The following formula[1] calculates the approximate audio output level within the range of 0.0 to 1.0. It is the sum of two sub-groupings of the channels: 
    
    
    output = pulse_out + tnd_out
    
                                95.88
    pulse_out = ------------------------------------
                 (8128 / (pulse1 + pulse2)) + 100
    
                                           159.79
    tnd_out = -------------------------------------------------------------
                                        1
               ----------------------------------------------------- + 100
                (triangle / 8227) + (noise / 12241) + (dmc / 22638)
    

The values for [pulse1](APU_Pulse.xhtml "APU Pulse"), [pulse2](APU_Pulse.xhtml "APU Pulse"), [triangle](APU_Triangle.xhtml "APU Triangle"), [noise](APU_Noise.xhtml "APU Noise"), and [dmc](APU_DMC.xhtml "APU DMC") are the output values for the corresponding channel. The dmc value ranges from 0 to 127 and the others range from 0 to 15. When the values for one of the groups are all zero, the result for that group should be treated as zero rather than undefined due to the division by 0 that otherwise results. 

Faster but less accurate approximations are also possible: using an efficient lookup table, or even rougher with a linear formula. 

The NES hardware follows the DACs with a [surprisingly involved circuit](https://archive.nesdev.org/NESAudio.gif) that adds several low-pass and high-pass filters: 

  * A first-order high-pass filter at 90 Hz
  * Another first-order high-pass filter at 440 Hz
  * A first-order low-pass filter at 14 kHz



See also: 

  * [blargg's data and analysis, and lidnariq's matching analog components to filters](http://forums.nesdev.org/viewtopic.php?p=44255#p44255)



The Famicom hardware instead ONLY specifies a first-order high-pass filter at 37 Hz, followed by the unknown (and varying) properties of the RF modulator and demodulator. 

## Contents

  * 1 Emulation
    * 1.1 Lookup Table
    * 1.2 Linear Approximation
  * 2 References



## Emulation

The NES APU Mixer can be efficiently emulated using a lookup table or a less-accurate linear approximation. 

### Lookup Table

The APU mixer formulas can be efficiently implemented using two lookup tables: a 31-entry table for the two [pulse channels](APU_Pulse.xhtml "APU Pulse") and a 203-entry table for the remaining channels (due to the approximation of tnd_out, the numerators are adjusted slightly to preserve the normalized output range). 
    
    
        output = pulse_out + tnd_out
    
        pulse_table [n] = 95.52 / (8128.0 / n + 100)
    
        pulse_out = pulse_table [pulse1 + pulse2]
    

The tnd_out table is approximated (within 4%) by using a base unit close to the [DMC's](APU_DMC.xhtml "APU DMC") DAC. 
    
    
        tnd_table [n] = 163.67 / (24329.0 / n + 100)
    
        tnd_out = tnd_table [3 * triangle + 2 * noise + dmc]
    

### Linear Approximation

A linear approximation can also be used, which results in slightly louder DMC samples, but otherwise fairly accurate operation since the wave channels use a small portion of the transfer curve. The overall volume will be reduced due to the headroom required by the DMC approximation. 
    
    
        output = pulse_out + tnd_out
        
        pulse_out = 0.00752 * (pulse1 + pulse2)
        
        tnd_out = 0.00851 * triangle + 0.00494 * noise + 0.00335 * dmc
    

## References

  1. ↑ [apu_ref.txt](https://www.nesdev.org/apu_ref.txt) by blargg



Categories: [APU](Category_APU.xhtml)
