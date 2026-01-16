# VRC6 audio

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/VRC6_audio) | View [other pages](Special_AllPages.xhtml#VRC6_audio)

Konami's [VRC6](VRC6.xhtml "VRC6") mapper provided 3 extra channels for sound: two pulse waves, and one sawtooth. All channels operate similarly to the native channels in the [NES APU](APU.xhtml "APU"). 

On some boards, the A0 and A1 lines were switched, so for those boards, registers will need adjustment when emulating ($x001 will become $x002 and vice versa). Registers listed here are how they are for 悪魔城伝説 (_Akumajou Densetsu_ , iNES mapper 024). For _Madara_ and _Esper Dream 2_ (iNES mapper 026), you will need to adjust the registers. 

## Contents

  * 1 Registers
    * 1.1 Frequency Control ($9003)
    * 1.2 Pulse Control ($9000,$A000)
    * 1.3 Saw Accum Rate ($B000)
    * 1.4 Freq Low ($9001,$A001,$B001)
    * 1.5 Freq High ($9002,$A002,$B002)
  * 2 Pulse Channels
  * 3 Sawtooth Channel
  * 4 Output
  * 5 References



## Registers

Overall map of VRC6 audio registers  Base | +0 | +1 | +2 | +3   
---|---|---|---|---  
$9000 | Pulse 1 duty and volume | Pulse 1 period low | Pulse 1 period high | Frequency scaling   
$A000 | Pulse 2 duty and volume | Pulse 2 period low | Pulse 2 period high |   
$B000 | Saw volume | Saw period low | Saw period high | ([mirroring control](VRC6.xhtml#PPU_Banking_Style_.28.24B003.29 "VRC6"))   
  
### Frequency Control ($9003)

Normally you should write $00 to this register on startup to initialize it, and not make any further writes to it. This is what all three original VRC6 games do. 

    $9003 controls the overall frequency of the VRC6 audio.
    
    
    7  bit  0
    ---- ----
    .... .ABH
          |||
          ||+- Halt
          |+-- 16x frequency (4 octaves up)
          +--- 256x frequency (8 octaves up)
    
    
    
    H - halts all oscillators, stopping them in their current state
    B - 16x frequency, all oscillators (4 octave increase)
    A - 256x frequency, all oscillators (8 octave increase)
    

  * The halt flag overrides the other flags.


  * The 256x flag overrides the 16x flag.


  * The 16x/256x flags effectively control a 4-bit and 8-bit right shift of the 12-bit period registers.



### Pulse Control ($9000,$A000)

    $9000 controls Pulse 1
    $A000 controls Pulse 2
    
    
    7  bit  0
    ---- ----
    MDDD VVVV
    |||| ||||
    |||| ++++- Volume
    |+++------ Duty Cycle
    +--------- Mode (1: ignore duty)
    

### Saw Accum Rate ($B000)
    
    
    7  bit  0
    ---- ----
    ..AA AAAA
      ++-++++- Accumulator Rate (controls volume)
    

### Freq Low ($9001,$A001,$B001)

    $9001 controls Pulse 1
    $A001 controls Pulse 2
    $B001 controls Saw
    
    
    7  bit  0
    ---- ----
    FFFF FFFF
    |||| ||||
    ++++-++++- Low 8 bits of frequency
    

  


### Freq High ($9002,$A002,$B002)

    $9002 controls Pulse 1
    $A002 controls Pulse 2
    $B002 controls Saw
    
    
    7  bit  0
    ---- ----
    E... FFFF
    |    ||||
    |    ++++- High 4 bits of frequency
    +--------- Enable (0 = channel disabled)
    

## Pulse Channels

The VRC6 pulse channels operate similarly to the NES's own pulse channels. The CPU clock rate (1.79 MHz) drives the 12-bit divider **F**. Every cycle the divider counts down until it reaches zero, at which point the divider resets and the duty cycle generator is clocked. 

The duty cycle generator takes 16 steps, counting down from 15 to 0. When the current step is less than or equal to the given duty cycle **D** , the channel volume **V** is output, otherwise 0 is output. 

When the mode bit **M** is true, the channel ignores the duty cycle generator and outputs the current volume regardless of the current duty. 

Therefore, **D** and **M** values provide the following duty cycles: 

D value | Duty (percent)   
---|---  
0 | 1/16 | 6.25%   
1 | 2/16 | 12.5%   
2 | 3/16 | 18.75%   
3 | 4/16 | 25%   
4 | 5/16 | 31.25%   
5 | 6/16 | 37.5%   
6 | 7/16 | 43.75%   
7 | 8/16 | 50%   
M | 16/16 | 100%   
  
When the channel is disabled by clearing the **E** bit, output is forced to 0, and the duty cycle is immediately reset and halted; it will resume from the beginning when **E** is once again set. Thus it is possible to reset phase by clearing and immediately setting **E**. 

When using equivalent duty cycle settings for the VRC6 pulse channels, they will appear inverted compared to their 2A03 counterparts. 

The frequency of the pulse channels is very similar to the APU pulse channels. It is a division of the [CPU Clock](Cycle_reference_chart.xhtml#Clock_rates "Cycle reference chart") (1.789773MHz NTSC). The output frequency (f) of the generator can be determined by the 12-bit period value (t) written to $9001-9002/$A001-A002. 
    
    
    f = CPU / (16 * (t + 1))
    t = (CPU / (16 * f)) - 1
    

## Sawtooth Channel

For the sawtooth, the CPU clock rate drives a 12-bit divider **F**. Every cycle, the divider counts down until it reaches zero, at which point it reloads and clocks the accumulator. However, it seems that the accumulator only reacts on every 2 clocks. 

When clocked, the rate value **A** is added to an internal 8-bit accumulator. The high 5 bits of the accumulator are then output (provided the channel is enabled by having the **E** bit set). After **A** has been added 6 times, on the 7th clock, instead of **A** being added, the internal accumulator is reset to zero. 

For an example, assume an **A** value of $08 

Step | Accumulator | Output | Comment   
---|---|---|---  
0 | $00 | $00 |   
1 | $00 | $00 | (odd step, do nothing)   
2 | $08 | $01 | (even step, add **A** to accumulator)   
3 | $08 | $01 |   
4 | $10 | $02 |   
5 | $10 | $02 |   
6 | $18 | $03 |   
7 | $18 | $03 |   
8 | $20 | $04 |   
9 | $20 | $04 |   
10 | $28 | $05 |   
11 | $28 | $05 |   
12 | $30 | $06 |   
13 | $30 | $06 |   
0 | $00 | $00 | (14th step, reset accumulator)   
1 | $00 | $00 |   
2 | $08 | $01 |   
  
If **A** is more than 42 (floor(255 / 6)), the accumulator will wrap, resulting in distorted sound. 

If **E** is clear, the accumulator is forced to zero until **E** is again set. The phase of the saw generator can be mostly reset by clearing and immediately setting **E**. Clearing **E** does not reset the frequency divider, however, so the first step of the reset saw may appear shortened. 

The frequency of the saw is a division of the [CPU Clock](Cycle_reference_chart.xhtml#Clock_rates "Cycle reference chart") (1.789773MHz NTSC). The output frequency (f) of the generator can be determined by the 12-bit period value (t) written to $B001-B002. Note that it divides the clock by 14 instead of 16, unlike the pulse channels. 
    
    
    f = CPU / (14 * (t + 1))
    t = (CPU / (14 * f)) - 1
    

## Output

At maximum volume, the pulse channels of the VRC6 are roughly equivalent to the pulse channels of the 2A03 (except inverted). The DAC of the VRC6, unlike the 2A03, appears to be linear. 

The final mix is a 6-bit DAC summing the two 4-bit pulse outputs and the high 5 bits of the saw accumulator. 

## References

  * [VRCVI Chip Info](https://nesdev.org/vrcvi.txt) by Kevin Horton
  * [Forum post](https://forums.nesdev.org/viewtopic.php?f=3&t=9207): VRC6 $9003 audio enable register?



Categories: [Expansion audio](Category_Expansion_audio.xhtml)
