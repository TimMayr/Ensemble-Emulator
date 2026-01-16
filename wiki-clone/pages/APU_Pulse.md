# APU Pulse

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/APU_Pulse) | View [other pages](Special_AllPages.xhtml#APU_Pulse)

Each of the two [NES APU](APU.xhtml "APU") pulse (square) wave channels generate a pulse wave with variable duty. 

Each pulse channel contains the following: 

  * [envelope generator](APU_Envelope.xhtml "APU Envelope")
  * [sweep unit](APU_Sweep.xhtml "APU Sweep")
  * [timer](APU.xhtml "APU Misc")
  * 8-step [sequencer](APU.xhtml "APU Misc")
  * [length counter](APU_Length_Counter.xhtml "APU Length Counter")


    
    
                         Sweep -----> Timer
                           |            |
                           |            |
                           |            v 
                           |        Sequencer   Length Counter
                           |            |             |
                           |            |             |
                           v            v             v
        Envelope -------> Gate -----> Gate -------> Gate --->(to mixer)
    

## Contents

  * 1 Registers
  * 2 Sequencer behavior
  * 3 Pulse channel output to mixer
  * 4 Pulse channel 1 vs Pulse channel 2 behavior
  * 5 See Also



## Registers

**Note** : the addresses below are _write-only!_ Reading from these addresses exhibits [open-bus behavior](Open_bus_behavior.xhtml "Open bus behavior"). 

Address | Bitfield | Description   
---|---|---  
**$4000** | `DDlc.vvvv` | **Pulse 1** **D** uty cycle, [length counter halt](APU_Length_Counter.xhtml "APU Length Counter"), **c** onstant volume/[envelope](APU_Envelope.xhtml "APU Envelope") flag, and **v** olume/envelope divider period   
**$4004** | `DDlc.vvvv` | **Pulse 2** **D** uty cycle, [length counter halt](APU_Length_Counter.xhtml "APU Length Counter"), **c** onstant volume/[envelope](APU_Envelope.xhtml "APU Envelope") flag, and **v** olume/envelope divider period   
Side effects | The duty cycle is changed (see table below), but the sequencer's current position isn't affected.   
**$4001** | `EPPP.NSSS` | See [APU Sweep](APU_Sweep.xhtml "APU Sweep")  
**$4005** | `EPPP.NSSS` | See [APU Sweep](APU_Sweep.xhtml "APU Sweep")  
**$4002** | `LLLL.LLLL` | **Pulse 1** timer **L** ow 8 bits   
**$4006** | `LLLL.LLLL` | **Pulse 2** timer **L** ow 8 bits   
**$4003** | `llll.lHHH` | **Pulse 1** [length counter load](APU_Length_Counter.xhtml "APU Length Counter") and timer **H** igh 3 bits   
**$4007** | `llll.lHHH` | **Pulse 2** [length counter load](APU_Length_Counter.xhtml "APU Length Counter") and timer **H** igh 3 bits   
Side effects | The sequencer is immediately restarted at the first value of the current sequence. The [envelope](APU_Envelope.xhtml "APU Envelope") is also restarted. The period divider is _not_ reset.[[1]](https://forums.nesdev.org/viewtopic.php?p=186129#p186129)  
  
## Sequencer behavior

The sequencer is clocked by an 11-bit [timer](APU.xhtml "APU Misc"). Given the timer value _t = HHHLLLLLLLL_ formed by timer high and timer low, this timer is updated every APU cycle (i.e., every second CPU cycle), and counts _t, t-1, ..., 0, t, t-1, ..._ , clocking the waveform generator when it goes from 0 to t. Since the period of the timer is _t+1_ APU cycles and the sequencer has 8 steps, the period of the waveform is _8*(t+1)_ APU cycles, or equivalently _16*(t+1)_ CPU cycles. 

Hence 

  * fpulse = fCPU/(16*(t+1)) (where fCPU is 1.789773 MHz for NTSC, 1.662607 MHz for PAL, and 1.773448 MHz for Dendy)
  * t = fCPU/(16*fpulse) - 1



  
**Note:** A period of _t < 8_, either set explicitly or via a sweep period update, **silences the corresponding pulse channel**. The highest frequency a pulse channel can output is hence about 12.4 kHz for NTSC. (**TODO:** PAL behavior?) 

  
**Duty Cycle Sequences**

Duty | Output waveform   
---|---  
0 | 0 1 0 0 0 0 0 0 (12.5%)   
1 | 0 1 1 0 0 0 0 0 (25%)   
2 | 0 1 1 1 1 0 0 0 (50%)   
3 | 1 0 0 1 1 1 1 1 (25% negated)   
  
Notice that a few Famiclone units have swapped APU duty cycles, as 12.5 [0], 50 [1], 25 [2] and 25 negated [3] instead. 

  
**Implementation details**

The reason for the odd output from the sequencer is that the counter is initialized to zero but counts _downward_ rather than upward. Thus it reads the sequence lookup table in the order 0, 7, 6, 5, 4, 3, 2, 1. 

Duty | Sequence lookup table | Output waveform   
---|---|---  
0 | `0 0 0 0 0 0 0 1` | 0 1 0 0 0 0 0 0 (12.5%)   
1 | `0 0 0 0 0 0 1 1` | 0 1 1 0 0 0 0 0 (25%)   
2 | `0 0 0 0 1 1 1 1` | 0 1 1 1 1 0 0 0 (50%)   
3 | `1 1 1 1 1 1 0 0` | 1 0 0 1 1 1 1 1 (25% negated)   
  
## Pulse channel output to mixer

The [mixer](APU_Mixer.xhtml "APU Mixer") receives the pulse channel's current [envelope volume](APU_Envelope.xhtml "APU Envelope") (lower 4 bits from $4000 or $4004) except when 

  * The sequencer output is zero, or
  * overflow from the [sweep](APU_Sweep.xhtml "APU Sweep") unit's adder is silencing the channel, or
  * the [length counter](APU_Length_Counter.xhtml "APU Length Counter") is zero, or
  * the timer has a value less than eight (_t <8_, noted above).



If any of the above are true, then the pulse channel sends zero (silence) to the mixer. 

## Pulse channel 1 vs Pulse channel 2 behavior

The behavior of the two pulse channels differs only in the effect of the negate mode of their [sweep units](APU_Sweep.xhtml "APU Sweep"). 

## See Also

  * [Pulse Channel frequency chart](Pulse_Channel_frequency_chart.xhtml "Pulse Channel frequency chart")



Categories: [APU](Category_APU.xhtml)
