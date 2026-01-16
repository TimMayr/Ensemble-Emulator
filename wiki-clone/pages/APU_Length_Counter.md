# APU Length Counter

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/APU_Length_Counter) | View [other pages](Special_AllPages.xhtml#APU_Length_Counter)

The length counter provides automatic duration control for the [NES APU](APU.xhtml "APU") waveform channels. Once loaded with a value, it can optionally count down (when the length counter halt flag is clear). Once it reaches zero, the corresponding channel is silenced. 

Address | Bitfield | Description   
---|---|---  
**$4015** | `---d.nt21` | **[DMC](APU_DMC.xhtml "APU DMC") control and length counter [enabled flags](APU.xhtml "APU Status")** (write)   
**$4000** | `ssHc.vvvv` | **[Pulse channel 1](APU_Pulse.xhtml "APU Pulse") duty cycle, length counter halt, constant volume flag, and volume/[envelope](APU_Envelope.xhtml "APU Envelope")** (write)   
**$4004** | `ssHc.vvvv` | **[Pulse channel 2](APU_Pulse.xhtml "APU Pulse") duty cycle, length counter halt, constant volume flag, and volume/[envelope](APU_Envelope.xhtml "APU Envelope")** (write)   
**$400C** | `--Hc.vvvv` | **[Noise channel](APU_Noise.xhtml "APU Noise") length counter halt, constant volume flag, and volume/[envelope](APU_Envelope.xhtml "APU Envelope")** (write)   
bit 5 | `--H- ----` | Halt length counter (this bit is also the [envelope's loop flag](APU_Envelope.xhtml "APU Envelope"))   
**$4008** | `Hlll.llll` | **[Triangle channel](APU_Triangle.xhtml "APU Triangle") length counter halt and linear counter load** (write)   
bit 7 | `H--- ----` | Halt length counter (this bit is also the [linear counter's control flag](APU_Triangle.xhtml "APU Triangle"))   
**$4003** | `LLLL.Lttt` | **[Pulse channel 1](APU_Pulse.xhtml "APU Pulse") length counter load and [timer](APU_Pulse.xhtml "APU Pulse")** (write)   
**$4007** | `LLLL.Lttt` | **[Pulse channel 2](APU_Pulse.xhtml "APU Pulse") length counter load and [timer](APU_Pulse.xhtml "APU Pulse")** (write)   
**$400B** | `LLLL.Lttt` | **[Triangle channel](APU_Triangle.xhtml "APU Triangle") length counter load and [timer](APU_Triangle.xhtml "APU Triangle")** (write)   
**$400F** | `LLLL.L---` | **[Noise channel](APU_Noise.xhtml "APU Noise") length counter load** (write)   
bits 7-3 | `LLLL L---` | If the [enabled flag](APU.xhtml "APU Status") is set, the length counter is loaded with entry L of the length table: 
    
    
         |  0   1   2   3   4   5   6   7    8   9   A   B   C   D   E   F
    -----+----------------------------------------------------------------
    00-0F  10,254, 20,  2, 40,  4, 80,  6, 160,  8, 60, 10, 14, 12, 26, 14,
    10-1F  12, 16, 24, 18, 48, 20, 96, 22, 192, 24, 72, 26, 16, 28, 32, 30
      
  
Side effects | The [envelope is restarted](APU_Envelope.xhtml "APU Envelope"), for pulse channels phase is reset, for triangle the linear counter reload flag is set.   
  
## Clocking

When the [enabled](APU.xhtml "APU Status") bit is cleared (via **$4015**), the length counter is forced to 0 and cannot be changed until enabled is set again (the length counter's previous value is lost). There is no immediate effect when enabled is set. 

When clocked by the [frame counter](APU_Frame_Counter.xhtml "APU Frame Counter"), the length counter is decremented _except_ when: 

  * The length counter is 0, or
  * The halt flag is set



## Table structure

The structure of the length table becomes clearer when rearranged like in the following index to length map (which corresponds to the order in which the values appear in the [internal APU lookup table](Visual_circuit_tutorial.xhtml#Decoders_and_mask_ROMs "Visual circuit tutorial")): 
    
    
    Legend:
    <bit pattern> (<value of bit pattern>) => <note length>
    
    Linear length values:
    1 1111 (1F) => 30
    1 1101 (1D) => 28
    1 1011 (1B) => 26
    1 1001 (19) => 24
    1 0111 (17) => 22
    1 0101 (15) => 20
    1 0011 (13) => 18
    1 0001 (11) => 16
    0 1111 (0F) => 14
    0 1101 (0D) => 12
    0 1011 (0B) => 10
    0 1001 (09) => 8
    0 0111 (07) => 6
    0 0101 (05) => 4
    0 0011 (03) => 2
    0 0001 (01) => 254
    
    Notes with base length 12 (4/4 at 75 bpm):
    1 1110 (1E) => 32  (96 times 1/3, quarter note triplet)
    1 1100 (1C) => 16  (48 times 1/3, eighth note triplet)
    1 1010 (1A) => 72  (48 times 1 1/2, dotted quarter)
    1 1000 (18) => 192 (Whole note)
    1 0110 (16) => 96  (Half note)
    1 0100 (14) => 48  (Quarter note)
    1 0010 (12) => 24  (Eighth note)
    1 0000 (10) => 12  (Sixteenth)
    
    Notes with base length 10 (4/4 at 90 bpm, with relative durations being the same as above):
    0 1110 (0E) => 26  (Approx. 80 times 1/3, quarter note triplet)
    0 1100 (0C) => 14  (Approx. 40 times 1/3, eighth note triplet)
    0 1010 (0A) => 60  (40 times 1 1/2, dotted quarter)
    0 1000 (08) => 160 (Whole note)
    0 0110 (06) => 80  (Half note)
    0 0100 (04) => 40  (Quarter note)
    0 0010 (02) => 20  (Eighth note)
    0 0000 (00) => 10  (Sixteenth)
    

With the least significant bit set, the remaining bits select a linear length (with the exception of the 0 entry). Otherwise, we get note lengths based on a base length of 10 (MSB clear) or 12 (MSB set). 

## Length counter internals

In the actual APU, the length counter silences the channel when clocked _while already zero_ (provided the length counter halt flag isn't set). The values in the above table are the actual values the length counter gets loaded with _plus one_ , to allow us to use a model where the channel is silenced when the length counter _becomes_ zero. 

The triangle's linear counter works differently, and does silence the channel when it _reaches_ zero. 

Categories: [APU](Category_APU.xhtml)
