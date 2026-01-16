# APU Envelope

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/APU_Envelope) | View [other pages](Special_AllPages.xhtml#APU_Envelope)

In a synthesizer, an [envelope](https://en.wikipedia.org/wiki/ADSR_envelope "wikipedia:ADSR envelope") is the way a sound's parameter changes over time. The NES [APU](APU.xhtml "APU") has an envelope generator that controls the volume in one of two ways: it can generate a decreasing saw envelope (like a decay phase of an [ADSR](http://en.wikipedia.org/wiki/Synthesizer#ADSR_envelope)) with optional looping, or it can generate a constant volume that a more sophisticated software envelope generator can manipulate. Volume values are practically linear (see: [APU Mixer](APU_Mixer.xhtml "APU Mixer")). 

Each volume envelope unit contains the following: start flag, [divider](APU.xhtml "APU Misc"), and decay level counter. 
    
    
                                       Loop flag
                                            |
                   Start flag  +--------.   |   Constant volume
                               |        |   |        flag
                               v        v   v          |
    Quarter frame clock --> Divider --> Decay --> |    | 
                               ^        level     |    v
                               |                  | Select --> Envelope output
                               |                  |
            Envelope parameter +----------------> |                   
    

Address | Bitfield | Description   
---|---|---  
**$4000** | `ddLC.VVVV` | **[Pulse channel 1](APU_Pulse.xhtml "APU Pulse")** duty and volume/envelope (write)   
**$4004** | `ddLC.VVVV` | **[Pulse channel 2](APU_Pulse.xhtml "APU Pulse")** duty and volume/envelope (write)   
**$400C** | `--LC.VVVV` | **[Noise channel](APU_Noise.xhtml "APU Noise")** volume/envelope (write)   
bit 5 | `--L- ----` | [APU Length Counter](APU_Length_Counter.xhtml "APU Length Counter") halt flag/envelope loop flag   
bit 4 | `---C ----` | Constant volume flag (0: use volume from envelope; 1: use constant volume)   
bits 3-0 | `---- VVVV` | Used as the volume in constant volume (C set) mode. Also used as the reload value for the envelope's divider (the period becomes V + 1 quarter frames).   
**$4003** | `LLLL.Lttt` | **[Pulse channel 1](APU_Pulse.xhtml "APU Pulse")** [length counter load](APU_Length_Counter.xhtml "APU Length Counter") and [timer](APU.xhtml "APU Misc") (write)   
**$4007** | `LLLL.Lttt` | **[Pulse channel 2](APU_Pulse.xhtml "APU Pulse")** [length counter load](APU_Length_Counter.xhtml "APU Length Counter") and [timer](APU.xhtml "APU Misc") (write)   
**$400F** | `LLLL.L---` | **[Noise channel](APU_Noise.xhtml "APU Noise")** [length counter load](APU_Length_Counter.xhtml "APU Length Counter") (write)   
Side effects | Sets start flag   
  
When clocked by the [frame counter](APU_Frame_Counter.xhtml "APU Frame Counter"), one of two actions occurs: if the start flag is clear, the divider is clocked, otherwise the start flag is cleared, the decay level counter is loaded with 15, and the divider's period is immediately reloaded. 

When the divider is clocked while at 0, it is loaded with V and clocks the decay level counter. Then one of two actions occurs: If the counter is non-zero, it is decremented, otherwise if the loop flag is set, the decay level counter is loaded with 15. 

The envelope unit's volume output depends on the constant volume flag: if set, the envelope parameter directly sets the volume, otherwise the decay level is the current volume. The constant volume flag has no effect besides selecting the volume source; the decay level will still be updated when constant volume is selected. 

Each of the three envelope units' output is fed through additional gates at the [sweep unit](APU_Sweep.xhtml "APU Sweep") (pulse only), waveform generator ([sequencer](APU.xhtml#Glossary "APU") or LFSR), and [length counter](APU_Length_Counter.xhtml "APU Length Counter"). 

Categories: [APU](Category_APU.xhtml)
