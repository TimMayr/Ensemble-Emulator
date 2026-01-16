# APU Noise

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/APU_Noise) | View [other pages](Special_AllPages.xhtml#APU_Noise)

The [NES APU](APU.xhtml "APU") noise channel generates pseudo-random 1-bit noise at 16 different frequencies. 

The noise channel contains the following: [envelope generator](APU_Envelope.xhtml "APU Envelope"), [timer](APU.xhtml "APU Misc"), [Linear Feedback Shift Register](https://en.wikipedia.org/wiki/Linear_feedback_shift_register "wikipedia:Linear feedback shift register"), [length counter](APU_Length_Counter.xhtml "APU Length Counter"). 
    
    
       Timer --> Shift Register   Length Counter
                       |                |
                       v                v
    Envelope -------> Gate ----------> Gate --> (to mixer)
    

**$400C** | `--lc.vvvv` | **[Length counter halt](APU_Length_Counter.xhtml "APU Length Counter")** , constant volume/envelope flag, and volume/**[envelope](APU_Envelope.xhtml "APU Envelope")** divider period (write)   
---|---|---  
**$400E** | `M---.PPPP` | **Mode and period** (write)   
bit 7 | `M--- ----` | Mode flag   
bits 3-0 | `---- PPPP` | The timer period is set to entry P of the following:  

    
    
    Rate  $0 $1  $2  $3  $4  $5   $6   $7   $8   $9   $A   $B   $C    $D    $E    $F
          --------------------------------------------------------------------------
    NTSC   4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068 (2046 in old)
    PAL    4, 8, 14, 30, 60, 88, 118, 148, 188, 236, 354, 472, 708,  944, 1890, 3778
    

The period determines how many CPU cycles happen between shift register clocks.  
These periods are all even numbers because there are 2 CPU cycles in an APU cycle.   
**$400F** | `llll.l---` | **[Length counter load](APU_Length_Counter.xhtml "APU Length Counter")** and **[envelope restart](APU_Envelope.xhtml "APU Envelope")** (write)   
  
The shift register is 15 bits wide, with bits numbered  
14 - 13 - 12 - 11 - 10 - 9 - 8 - 7 - 6 - 5 - 4 - 3 - 2 - 1 - 0 

When the timer clocks the shift register, the following actions occur in order: 

  1. Feedback is calculated as the exclusive-OR of bit 0 and one other bit: bit 6 if Mode flag is set, otherwise bit 1.
  2. The shift register is shifted right by one bit.
  3. Bit 14, the leftmost bit, is set to the feedback calculated earlier.



This results in a pseudo-random bit sequence, 32767 steps long when Mode flag is clear, and randomly 93 or 31 steps long otherwise. (The particular 31- or 93-step sequence depends on where in the 32767-step sequence the shift register was when Mode flag was set). 

The [mixer](APU_Mixer.xhtml "APU Mixer") receives the current [envelope volume](APU_Envelope.xhtml "APU Envelope") except when 

  * Bit 0 of the shift register is set, or
  * The [length counter](APU_Length_Counter.xhtml "APU Length Counter") is zero



Within the mixer, the DMC level has a noticeable effect on the noise's level. 

On power-up, the shift register is loaded with the value 1. 

In the earliest revisions of the 2A03 CPU, the Mode flag was nonexistent: the shift register always used bits 0 and 1 for feedback, and the lowest period (rate $F) lasts 2046 M2 cycles instead of 4068 (e.g. Bowser's flames in _Super Mario Bros._). These CPUs were used in the first batch of Famicom consoles (which were recalled), in Vs. System boards, and in the arcade games that used the 2A03 as a sound coprocessor.[[1]](http://forums.nesdev.org/viewtopic.php?p=58046#p58046) [[2]](https://bsky.app/profile/plgdavid.bsky.social/post/3llhf3ocbx22d)

The 93-step sequence is about a quarter tone (50 cents) sharp relative to A440 tuning. The approximate frequencies and pitches (in [LilyPond's variant of Helmholtz notation](https://en.wikipedia.org/wiki/Helmholtz_pitch_notation#Variations "wikipedia:Helmholtz pitch notation")) are as follows: 

Pitches of 93-step noise  | NTSC | PAL   
---|---|---  
Register value | Sample rate | Repeat rate | MIDI note | Pitch | Sample rate | Repeat rate | MIDI note | Pitch   
$80 | 447443.2 Hz | 4811.2 Hz | 110.41 | d''''' + 41¢ | 415651.8 Hz | 4469.4 Hz | 109.13 | c#''''' + 13¢   
$81 | 223721.6 Hz | 2405.6 Hz | 98.41 | d'''' + 41¢ | 207825.9 Hz | 2234.7 Hz | 97.13 | c#'''' + 13¢   
$82 | 111860.8 Hz | 1202.8 Hz | 86.41 | d''' + 41¢ | 118757.6 Hz | 1277.0 Hz | 87.45 | d#''' + 45¢   
$83 | 55930.4 Hz | 601.4 Hz | 74.41 | d'' + 41¢ | 55420.2 Hz | 595.9 Hz | 74.25 | d'' + 25¢   
$84 | 27965.2 Hz | 300.7 Hz | 62.41 | d' + 41¢ | 27710.1 Hz | 298.0 Hz | 62.25 | d' + 25¢   
$85 | 18643.5 Hz | 200.5 Hz | 55.39 | g + 39¢ | 18893.3 Hz | 203.2 Hz | 55.62 | g + 62¢   
$86 | 13982.6 Hz | 150.4 Hz | 50.41 | d + 41¢ | 14089.9 Hz | 151.5 Hz | 50.54 | d + 54¢   
$87 | 11186.1 Hz | 120.3 Hz | 46.55 | a#, + 55¢ | 11233.8 Hz | 120.8 Hz | 46.62 | a#, + 62¢   
$88 | 8860.3 Hz | 95.3 Hz | 42.51 | f#, + 51¢ | 8843.7 Hz | 95.1 Hz | 42.48 | f#, + 48¢   
$89 | 7046.3 Hz | 75.8 Hz | 38.55 | d, + 55¢ | 7044.9 Hz | 75.8 Hz | 38.54 | d, + 54¢   
$8A | 4709.9 Hz | 50.6 Hz | 31.57 | g,, + 57¢ | 4696.6 Hz | 50.5 Hz | 31.52 | g,, + 52¢   
$8B | 3523.2 Hz | 37.9 Hz | 26.55 | d,, + 55¢ | 3522.5 Hz | 37.9 Hz | 26.54 | d,, + 54¢   
$8C | 2348.8 Hz | 25.3 Hz | 19.53 | g,,, + 53¢ | 2348.3 Hz | 25.3 Hz | 19.52 | g,,, + 52¢   
$8D | 1761.6 Hz | 18.9 Hz | 14.55 | d,,, + 55¢ | 1761.2 Hz | 18.9 Hz | 14.54 | d,,, + 54¢   
$8E | 879.9 Hz | 9.5 Hz | 2.53 | d,,,, + 53¢ | 879.7 Hz | 9.5 Hz | 2.52 | d,,,, + 52¢   
$8F | 440.0 Hz | 4.7 Hz | -9.47 | d,,,,, + 53¢ | 440.1 Hz | 4.7 Hz | -9.47 | d,,,,, + 53¢   
  
Categories: [APU](Category_APU.xhtml)
