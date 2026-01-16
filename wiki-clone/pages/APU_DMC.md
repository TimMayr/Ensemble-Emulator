# APU DMC

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/APU_DMC) | View [other pages](Special_AllPages.xhtml#APU_DMC)

  
The [NES APU's](APU.xhtml "APU") delta modulation channel (DMC) can output 1-bit [delta-encoded samples](https://en.wikipedia.org/wiki/Delta_modulation "wikipedia:Delta modulation") or can have its 7-bit counter directly loaded, allowing flexible manual sample playback. 

## Contents

  * 1 Overview
    * 1.1 Pitch table
    * 1.2 Memory reader
    * 1.3 Output unit
  * 2 Conflict with controller and PPU read
    * 2.1 Likely internal implementation of the read
  * 3 Usage of DMC for syncing to video
    * 3.1 Concept
    * 3.2 Timing table
    * 3.3 Number of scanlines to wait table
  * 4 References



## Overview

The DMC channel contains the following: memory reader, interrupt flag, sample buffer, [timer](APU.xhtml "APU Misc"), output unit, 7-bit output level with up and down counter. 
    
    
                             Timer
                               |
                               v
    Reader ---> Buffer ---> Shifter ---> Output level ---> (to the mixer)
    

**$4010** | `IL--.RRRR` | **Flags and Rate** (write)   
---|---|---  
bit 7 | `I---.----` | IRQ enabled flag. If clear, the interrupt flag is cleared.   
bit 6 | `-L--.----` | Loop flag   
bits 3-0 | `----.RRRR` | Rate index  

    
    
    Rate   $0   $1   $2   $3   $4   $5   $6   $7   $8   $9   $A   $B   $C   $D   $E   $F
          ------------------------------------------------------------------------------
    NTSC  428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106,  84,  72,  54
    PAL   398, 354, 316, 298, 276, 236, 210, 198, 176, 148, 132, 118,  98,  78,  66,  50
    

The rate determines for how many CPU cycles happen between changes in the output level during automatic delta-encoded sample playback. For example, on NTSC (1.789773 MHz), a rate of 428 gives a frequency of 1789773/428 Hz = 4181.71 Hz. These periods are all even numbers because there are 2 CPU cycles in an APU cycle. A rate of 428 means the output level changes every 214 APU cycles.   
**$4011** | `-DDD.DDDD` | **Direct load** (write)   
bits 6-0 | `-DDD.DDDD` | The DMC output level is set to D, an unsigned value. If the timer is outputting a clock at the same time, the output level is occasionally not changed properly.[[1]](http://forums.nesdev.org/viewtopic.php?p=104491#p104491)  
**$4012** | `AAAA.AAAA` | **Sample address** (write)   
bits 7-0 | `AAAA.AAAA` | Sample address = `%11AAAAAA.AA000000` = `$C000 + (A * 64)`  
**$4013** | `LLLL.LLLL` | **Sample length** (write)   
bits 7-0 | `LLLL.LLLL` | Sample length = `%LLLL.LLLL0001` = `(L * 16) + 1 bytes`  
  
The output level is sent to the [mixer](APU_Mixer.xhtml "APU Mixer") whether the channel is enabled or not. It is loaded with 0 on power-up, and can be updated by $4011 writes and delta-encoded sample playback. 

Automatic 1-bit [delta-encoded sample](https://en.wikipedia.org/wiki/Delta_modulation "wikipedia:Delta modulation") playback is carried out by a combination of three units. The _memory reader_ fills the 8-bit _sample buffer_ whenever it is emptied by the sample _output unit_. The [status register](APU.xhtml "APU Status") is used to start and stop automatic sample playback. 

The **sample buffer** either holds a single 8-bit sample byte or is empty. It is filled by the reader and can only be emptied by the output unit; once loaded with a sample byte it will be played back. 

### Pitch table

| NTSC  | PAL   
---|---|---  
$4010 | Period | Frequency | Note (raw) | Note (1-byte loop) | Note (17-byte loop) | Note (33-byte loop) | Period | Frequency | Note (raw) | Note (1-byte loop) | Note (17-byte loop) | Note (33-byte loop)   
$0  | $1AC | 4181.71 Hz | C-8 -2c | C-5 -2c | B-0 -7c | infrasound  | $18E | 4177.40 Hz | C-8 -4c | C-5 -4c | B-0 -9c | infrasound   
$1  | $17C | 4709.93 Hz | D-8 +4c | C-5 +4c | C#1 -1c | infrasound  | $162 | 4696.63 Hz | D-8 -1c | D-5 -1c | C#1 -6c | infrasound   
$2  | $154 | 5264.04 Hz | E-8 -3c | E-5 -3c | Eb1 -8c | infrasound  | $13C | 5261.41 Hz | E-8 -4c | E-5 -4c | Eb1 -9c | infrasound   
$3  | $140 | 5593.04 Hz | F-8 +2c | F-5 +2c | E-1 -3c | E-0 +48c  | $12A | 5579.22 Hz | F-8 -3c | F-5 -3c | E-1 -8c | E-0 +44c   
$4  | $11E | 6257.95 Hz | G-8 -4c | G-5 -4c | F#1 -9c | F#0 +43c  | $114 | 6023.94 Hz | G-8 -70c | G-5 -70c | F#1 -75c | F#0 -23c   
$5  | $0FE | 7046.35 Hz | A-8 +2c | A-5 +2c | Ab1 -3c | Ab0 +48c  | $0EC | 7044.94 Hz | A-8 +1c | A-5 +1c | Ab1 -4c | Ab0 +48c   
$6  | $0E2 | 7919.35 Hz | B-8 +4c | B-5 +4c | Bb1 -1c | Bb0 +50c  | $0D2 | 7917.18 Hz | B-8 +3c | B-5 +3c | Bb1 -2c | Bb0 +50c   
$7  | $0D6 | 8363.42 Hz | C-9 -2c | C-6 -2c | B-1 -7c | B-0 +45c  | $0C6 | 8397.01 Hz | C-9 +5c | C-6 +5c | B-1 +0c | B-0 +52c   
$8  | $0BE | 9419.86 Hz | D-9 +4c | D-6 +4c | C#2 -1c | C#1 +51c  | $0B0 | 9446.63 Hz | D-9 +9c | D-9 +9c | C#2 +4c | C#1 +56c   
$9  | $0A0 | 11186.1 Hz | F-9 +2c | F-6 +2c | E-2 -3c | E-1 +48c  | $094 | 11233.8 Hz | F-9 +9c | F-6 +9c | E-2 +4c | E-1 +56c   
$A  | $08E | 12604.0 Hz | G-9 +8c | G-6 +8c | F#2 +3c | F#1 +55c  | $084 | 12595.5 Hz | G-9 +7c | G-6 +7c | F#2 -2c | G-1 +54c   
$B  | $080 | 13982.6 Hz | A-9 -12c | A-6 -12c | Ab2 -17c | Ab1 +35c  | $076 | 14089.9 Hz | A-9 +1c | A-6 +1c | Ab2 -4c | Ab1 +48c   
$C  | $06A | 16884.6 Hz | C-10 +14c | C-7 +14c | B-2 +10c | B-1 +61c  | $062 | 16965.4 Hz | C-10 +23c | C-7 +23c | B-2 +18c | B-1 +69c   
$D  | $054 | 21306.8 Hz | E-10 +17c | E-7 +17c | Eb3 +12c | Eb2 +64c  | $04E | 21315.5 Hz | E-10 +18c | E-7 +18c | Eb3 +13c | Eb2 +65c   
$E  | $048 | 24858.0 Hz | G-10 -16c | G-7 -16c | F#3 -21c | F#2 +31c  | $042 | 25191.0 Hz | G-10 +7c | G-7 +7c | F#3 +2c | F#2 +54c   
$F  | $036 | 33143.9 Hz | C-11 -18c | C-8 -18c | B-3 -23c | B-2 +29c  | $032 | 33252.1 Hz | C-11 -12c | C-8 -12c | B-3 -17c | B-2 +34c   
  
(Deviation from note is given in cents, which are defined as 1/100th of a semitone.) 

Note that on PAL systems, the pitches at $4 and $C appear to be incorrect with respect to their intended A-440 tuning scheme[1]. 

### Memory reader

When the sample buffer is emptied, the memory reader fills the sample buffer with the next byte from the currently playing sample. It has an address counter and a bytes remaining counter. 

When a sample is (re)started, the current address is set to the sample address, and bytes remaining is set to the sample length. 

Any time the sample buffer is in an empty state and bytes remaining is not zero (including just after a write to $4015 that enables the channel, regardless of where that write occurs relative to the bit counter mentioned below), the following occur: 

  * The [CPU](CPU.xhtml "CPU") is stalled for 1-4 CPU cycles to read a sample byte. The exact cycle count depends on many factors and is described in detail in the [DMA](DMA.xhtml "DMA") article.
  * The sample buffer is filled with the next sample byte read from the current address, subject to whatever [mapping hardware](Mapper.xhtml "MMC") is present.
  * The address is incremented; if it exceeds $FFFF, it is wrapped around to $8000.
  * The bytes remaining counter is decremented; if it becomes zero and the loop flag is set, the sample is restarted (see above); otherwise, if the bytes remaining counter becomes zero and the IRQ enabled flag is set, the interrupt flag is set.



At any time, if the interrupt flag is set, the [CPU's IRQ line](CPU.xhtml "CPU") is _continuously_ asserted until the interrupt flag is cleared. The processor will continue on from where it was stalled. 

### Output unit

The output unit continuously outputs a 7-bit value to the [mixer](APU_Mixer.xhtml "APU Mixer"). It contains an 8-bit right shift register, a bits-remaining counter, a 7-bit output level (the same one that can be loaded directly via $4011), and a silence flag. 

The bits-remaining counter is updated whenever the [timer](APU.xhtml "APU Misc") outputs a clock, regardless of whether a sample is currently playing. When this counter reaches zero, we say that the output cycle ends. The DPCM unit can only transition from silent to playing at the end of an output cycle. 

When an output cycle ends, a new cycle is started as follows: 

  * The bits-remaining counter is loaded with 8.
  * If the sample buffer is empty, then the silence flag is set; otherwise, the silence flag is cleared and the sample buffer is emptied into the shift register.



When the timer outputs a clock, the following actions occur in order: 

  1. If the silence flag is clear, the output level changes based on bit 0 of the shift register. If the bit is 1, add 2; otherwise, subtract 2. But if adding or subtracting 2 would cause the output level to leave the 0-127 range, leave the output level unchanged. This means subtract 2 only if the current level is at least 2, or add 2 only if the current level is at most 125.
  2. The right shift register is clocked.
  3. As stated above, the bits-remaining counter is decremented. If it becomes zero, a new output cycle is started.



_Nothing can interrupt a cycle; every cycle runs to completion before a new cycle is started._

## Conflict with controller and PPU read

On the NTSC NES and Famicom, if a new sample byte is fetched from memory at the same time the program is reading the [controller](Standard_controller.xhtml "Standard controller") through $4016/4017, a conflict occurs corrupting the data read from the controller. Programs which use DPCM sample playback will normally use a redundant [controller read](Controller_reading.xhtml "Controller Reading") routine to work around this defect. 

A similar problem occurs when reading data from the PPU through $2007, or polling $2002 for vblank. 

### Likely internal implementation of the read

The following is speculation, and thus not necessarily 100% accurate, but it does accurately predict observed behavior. 

The 6502 cannot be pulled off of the bus normally. The 2A03 DMC gets around this by pulling RDY low internally. This causes the CPU to pause during the next read cycle, until RDY goes high again. The DMC unit holds RDY low for 4 cycles. The first three cycles it idles, as the CPU could have just started an interrupt cycle, and thus be writing for 3 consecutive cycles (and thus ignoring RDY). On the fourth cycle, the DMC unit drives the next sample address onto the address lines, and reads that byte from memory. It then drives RDY high again, and the CPU picks up where it left off. 

This matters because on NTSC NES and Famicom, it can interfere with the expected operation of any register where reads have a side effect: the controller registers ($4016 and $4017), reads of the PPU status register ($2002), and reads of VRAM/VROM data ($2007) if they happen to occur in the same cycle that the DMC unit pulls RDY low. 

For the controller registers, this can cause an extra rising clock edge to occur, and thus shift an extra bit out. For the others, the PPU will see multiple reads, which will cause extra increments of the address latches, or clear the vblank flag. 

This problem has been fixed on the 2A07 and PAL NES is exempt of this bug. 

## Usage of DMC for syncing to video

DMC IRQs can be used for timed video operations. The following method was discussed on the forum in 2010.[2]

### Concept

The NES hardware only has limited tools for syncing the code with video rendering. The VBlank NMI and sprite 0 hit are the only two reasonably reliable flags that can be used, so only 2 synchronizations per frame can be done easily. In addition, only the VBlank NMI can trigger an interrupt; the sprite 0 hit flag has to be polled, potentially wasting a lot of CPU cycles. 

However, the DMC channel can hypothetically be used for syncing with video instead of using it for sound. Unfortunately it's a bit complicated, but used correctly, it can function as a crude scanline counter, eliminating the need for an advanced mapper. 

The DMC's timing is completely separate from the video. The DMC's timer is always running, and samples can only start every 8 clock cycles. However, because the DMC's timer isn't synchronized to the PPU in any way, these 8-clock boundaries occur on different scanlines each frame. 

Here are the steps to achieve stable timing: 

  * At a fixed point in video rendering (we'll use the start of vblank as an example), a dummy single-byte sample at rate $F is started. Due to a hardware quirk†, the sample needs to be started three times in a row like this:


    
    
    sei
    lda #$10 
    sta $4015 
    sta $4015 
    sta $4015 
    cli
    

  * The amount of cycles before a DMC IRQ happens is then measured (either using an actual IRQ, or by polling $4015). 
    * At rate $F, there are 54 CPU cycles between clocks, so there are 432 CPU cycles (432 × 3 ÷ 341 = about 3.8 scanlines) between boundaries.
  * The main sample that will be used for the timing is then started (please refer to the table below to have sample lengths for various waiting times)
  * When the main IRQ happens, the measurement from before is retrieved, and a timing loop with variable delay is used. In order to synchronize with vblank, after a DMC IRQ we should wait 432 CPU cycles minus the time we measured.



†**Note:** The hardware quirk mentioned above deals with how DMC IRQs are generated. Basically, the IRQ is generated when the last **byte** of the sample is **read** , **not** when the last _sample_ of the sample _plays_. The sample buffer sometimes has enough time to empty itself between writes to $4015, meaning your next write to $4015 will trigger an immediate IRQ. Fortunately, writing to $4015 three times will avoid this issue. 

Still using vblank as an example, the measurement tells how far into the 8-clock boundary vblank occurred, and by delaying after a DMC IRQ, we perform a raster effect at the same point within the 8-clock boundary, aligning it with vblank. By performing this same method each frame, the raster effect will have a reasonably stable timing to it. As a bonus, since mostly using IRQs are being used, the CPU is free to do something else, instead of waiting in a timed loop. 

It's possible to use more than one IRQ per frame - but the _measurement_ part needs to be done at the _same time_ within each frame, before the usage of any IRQ. 

Only a single split-point per IRQ is possible, with the shortest IRQ being 3.8 scanlines. For split points closer than this amount, timed code has to be used. 

In order to remain silent, samples should be made up of all $00 bytes, and $00 should have been previously written to $4011. Otherwise, audio will unintentionally be created. This _is_ a sound channel, after all. 

### Timing table

This table converts sample length in scanline length (all values are rounded to the higher integer). 
    
    
    NTSC               Rate 
    Length              $0    $1   $2   $3   $4   $5   $6   $7   $8   $9   $a   $b   $c   $d   $e   $f 
    ---------------------------------------------------------------------------------------------------- 
    1-byte (8 bits)     31    27   24   23   21   18   16   16   14   12   10   10   8    6    6    4 
    17-byte (136 bits)  **    **   **   **   **   **   **   **   228  192  170  154  127  101  87   65 
    33-byte (264 bits)  **    **   **   **   **   **   **   **   **   **   **   **   **   196  168  126 
    49-byte (392 bits)  **    **   **   **   **   **   **   **   **   **   **   **   **   **   **   187 
    
    PAL                Rate 
    Length              $0    $1   $2   $3   $4   $5   $6   $7   $8   $9   $a   $b   $c   $d   $e   $f 
    ---------------------------------------------------------------------------------------------------- 
    1-byte (8 bits)     30    27   24   23   21   18   16   15   14   12   10   9    8    6    5    4 
    17-byte (136 bits)  **    **   **   **   **   **   **   **   225  189  169  151  126  100  85   64 
    33-byte (264 bits)  **    **   **   **   **   **   **   **   **   **   **   **   **   194  164  124 
    49-byte (392 bits)  **    **   **   **   **   **   **   **   **   **   **   **   **   **   **   184
    

### Number of scanlines to wait table

This table gives the best sample length and frequency combinations for all possible scanlines interval to wait. They are best because they are where the CPU will have to kill the least time. However, it's still possible to use options to wait for fewer lines and kill more time during the interrupt before the video effect. 

Because a PAL interrupt will always happen about the same time or a bit sooner than a NTSC interrupt, the NTSC table will be used to set the "best" setting here : 
    
    
    Scanlines  Best opt. for IRQ 
     
    1-3        Timed code 
    4-5        Length $0, rate $f 
    6-7        Length $0, rate $d 
    8-9        Length $0, rate $c 
    10-11      Length $0, rate $a 
    12-13      Length $0, rate $9 
    14-15      Length $0, rate $8 
    16-17      Length $0, rate $6 
    18-20      Length $0, rate $5 
    21-22      Length $0, rate $4 
    23         Length $0, rate $3 
    24-26      Length $0, rate $2 
    27-30      Length $0, rate $1 
    31-64      Length $0, rate $0 
    65-86      Length $1, rate $f 
    87-100     Length $1, rate $e 
    101-125    Length $1, rate $d 
    126        Length $2, rate $f 
    127-153    Length $1, rate $c 
    154-167    Length $1, rate $b 
    168-169    Length $2, rate $e 
    170-186    Length $1, rate $a 
    187-191    Length $3, rate $f 
    192-195    Length $1, rate $9 
    196-227    Length $2, rate $d 
    228-239    Length $1, rate $8
    

## References

  1. ↑ [Forum post](http://forums.nesdev.org/viewtopic.php?p=94079#p94079): PAL DPCM frequency table contains 2 errors.
  2. ↑ [Forum thread](http://forums.nesdev.org/viewtopic.php?t=6521): DMC IRQ as a video timer.



Categories: [APU](Category_APU.xhtml)
