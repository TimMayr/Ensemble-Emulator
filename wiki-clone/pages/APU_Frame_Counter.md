# APU Frame Counter

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/APU_Frame_Counter) | View [other pages](Special_AllPages.xhtml#APU_Frame_Counter)

The **[NES APU](APU.xhtml "APU") frame counter** (or **frame sequencer**) generates low-frequency clocks for the channels and an optional 60 Hz interrupt. The name "frame counter" might be slightly misleading because the clocks have nothing to do with the video signal. 

The frame counter contains the following: [divider](APU.xhtml "APU Misc"), looping clock [sequencer](APU.xhtml "APU Misc"), frame interrupt flag. 

The sequencer is clocked on every other CPU cycle, so 2 CPU cycles = 1 APU cycle. The sequencer keeps track of how many APU cycles have elapsed in total, and each step of the sequence will occur once that total has reached the indicated amount (with an additional delay of one CPU cycle for the quarter and half frame signals). Once the last step has executed, the count resets to 0 on the next APU cycle. 

Address | Bitfield | Description   
---|---|---  
**$4017** | `MI--.----` | **Set mode and interrupt** (write)   
Bit 7 | `M--- ----` | Sequencer mode: 0 selects 4-step sequence, 1 selects 5-step sequence   
Bit 6 | `-I-- ----` | Interrupt inhibit flag. If set, the frame interrupt flag is cleared, otherwise it is unaffected.   
Side effects | After 3 or 4 CPU clock cycles*, the timer is reset.  
If the mode flag is set, then both "quarter frame" and "half frame" signals are also generated.   
  
* If the write occurs _during_ an APU cycle, the effects occur 3 CPU cycles after the **$4017** write cycle, and if the write occurs _between_ APU cycles, the effects occurs 4 CPU cycles after the write cycle. 

PAL behavior is currently assumed to be the same. 

The frame interrupt flag is connected to the [CPU](CPU.xhtml "CPU")'s IRQ line. It is set at a particular point in the 4-step sequence (see below) provided the interrupt inhibit flag in $4017 is clear, and can be cleared either by reading $4015 (which also returns its old status) or by setting the interrupt inhibit flag. 

### Mode 0: 4-Step Sequence (bit 7 of $4017 clear)

Step  | APU cycles  | [Envelopes](APU_Envelope.xhtml "APU Envelope") & [triangle's linear counter](APU_Triangle.xhtml "APU Triangle")  
(Quarter frame)  | [Length counters](APU_Length_Counter.xhtml "APU Length Counter") & [sweep units](APU_Sweep.xhtml "APU Sweep")  
(Half frame)  | Frame interrupt flag   
---|---|---|---|---  
NTSC | PAL   
1 | 3728.5 | 4156.5 | Clock |  |   
2 | 7456.5 | 8313.5 | Clock | Clock |   
3 | 11185.5 | 12469.5 | Clock |  |   
4 | 14914 | 16626 |  |  | Set if interrupt inhibit is clear   
14914.5 | 16626.5 | Clock | Clock | Set if interrupt inhibit is clear   
0 (14915) | 0 (16627) |  |  | Set if interrupt inhibit is clear   
|  | NTSC: _240 Hz (approx.)_  
PAL: _200 Hz (approx)_ | NTSC: _120 Hz (approx.)_  
PAL: _100 Hz (approx)_ | NTSC: _60 Hz (approx.)_  
PAL: _50 Hz (approx)_  
  
In this mode, the interrupt flag is set every 29830 CPU cycles, which is slightly (0.166%) slower than the 29780.5 CPU cycles per NTSC PPU frame. 

On the PAL RP2A07, this is every 33254 CPU cycles, still slower than the 33247.5 CPU cycles per PAL PPU frame but much closer (only off by 0.0196%). 

Some Nintendo arcade boards, even those not directly based on the NES, use the 2A03 CPU as a sound processor. Examples include _[Punch-Out!!](https://en.wikipedia.org/wiki/Punch-Out!!_\(arcade_game\) "wikipedia:Punch-Out!! \(arcade game\)")_ and _[Donkey Kong 3](https://en.wikipedia.org/wiki/Donkey_Kong_3 "wikipedia:Donkey Kong 3")_. This IRQ allows the CPU to keep time even if no PPU is connected to the bus. 

### Mode 1: 5-Step Sequence (bit 7 of $4017 set)

Step  | APU cycles  | [Envelopes](APU_Envelope.xhtml "APU Envelope") & [triangle's linear counter](APU_Triangle.xhtml "APU Triangle")  
(Quarter frame)  | [Length counters](APU_Length_Counter.xhtml "APU Length Counter") & [sweep units](APU_Sweep.xhtml "APU Sweep")  
(Half frame)   
---|---|---|---  
NTSC | PAL   
1 | 3728.5 | 4156.5 | Clock |   
2 | 7456.5 | 8313.5 | Clock | Clock   
3 | 11185.5 | 12469.5 | Clock |   
4 | 14914.5 | 16626.5 |  |   
5 | 18640.5 | 20782.5 | Clock | Clock   
0 (18641) | 0 (20783) |  |   
|  | NTSC: _192 Hz (approx.), uneven timing_  
PAL: _160 Hz (approx.), uneven timing_ | NTSC: _96 Hz (approx.), uneven timing_  
PAL: _80 Hz (approx.), uneven timing_  
  
In this mode, the frame interrupt flag is never set.  


Categories: [APU](Category_APU.xhtml)
