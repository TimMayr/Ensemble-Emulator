# APU registers

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/APU_registers) | View [other pages](Special_AllPages.xhtml#APU_registers)

The following memory-mapped registers are used by the [NES APU](APU.xhtml "APU"). They are write-only except $4015 which is read/write. Unused registers aren't listed. 

_For Famicom expansion audio registers, see also:[Expansion audio](Category_Expansion_audio.xhtml "Category:Expansion audio")_

Address | `7654.3210` | Function   
---|---|---  
| **[Pulse 1 channel](APU_Pulse.xhtml "APU Pulse")** (write)   
**$4000** | `DDLC NNNN` | [Duty](APU_Pulse.xhtml "APU Pulse"), [loop envelope](APU_Envelope.xhtml "APU Envelope")/[disable length counter](APU_Length_Counter.xhtml "APU Length Counter"), [constant volume](APU_Envelope.xhtml "APU Envelope"), [envelope period/volume](APU_Envelope.xhtml "APU Envelope")  
**$4001** | `EPPP NSSS` | [Sweep unit](APU_Sweep.xhtml "APU Sweep"): enabled, period, negative, shift count   
**$4002** | `LLLL LLLL` | [Timer low](APU_Pulse.xhtml "APU Pulse")  
**$4003** | `LLLL LHHH` | [Length counter load](APU_Length_Counter.xhtml "APU Length Counter"), [timer high](APU_Pulse.xhtml "APU Pulse") (also resets [duty](APU_Pulse.xhtml "APU Pulse") and starts [envelope](APU_Envelope.xhtml "APU Envelope"))   
| **[Pulse 2 channel](APU_Pulse.xhtml "APU Pulse")** (write)   
**$4004** | `DDLC NNNN` | [Duty](APU_Pulse.xhtml "APU Pulse"), [loop envelope](APU_Envelope.xhtml "APU Envelope")/[disable length counter](APU_Length_Counter.xhtml "APU Length Counter"), [constant volume](APU_Envelope.xhtml "APU Envelope"), [envelope period/volume](APU_Envelope.xhtml "APU Envelope")  
**$4005** | `EPPP NSSS` | [Sweep unit](APU_Sweep.xhtml "APU Sweep"): enabled, period, negative, shift count   
**$4006** | `LLLL LLLL` | [Timer low](APU_Pulse.xhtml "APU Pulse")  
**$4007** | `LLLL LHHH` | [Length counter load](APU_Length_Counter.xhtml "APU Length Counter"), [timer high](APU_Pulse.xhtml "APU Pulse") (also resets [duty](APU_Pulse.xhtml "APU Pulse") and starts [envelope](APU_Envelope.xhtml "APU Envelope"))   
| **[Triangle channel](APU_Triangle.xhtml "APU Triangle")** (write)   
**$4008** | `CRRR RRRR` | [Length counter disable](APU_Length_Counter.xhtml "APU Length Counter")/[linear counter control](APU_Triangle.xhtml "APU Triangle"), [linear counter reload value](APU_Triangle.xhtml "APU Triangle")  
**$400A** | `LLLL LLLL` | [Timer low](APU_Triangle.xhtml "APU Triangle")  
**$400B** | `LLLL LHHH` | [Length counter load](APU_Length_Counter.xhtml "APU Length Counter"), [timer high](APU_Triangle.xhtml "APU Triangle") (also reloads [linear counter](APU_Triangle.xhtml "APU Triangle"))   
| **[Noise channel](APU_Noise.xhtml "APU Noise")** (write)   
**$400C** | `--LC NNNN` | [Loop envelope](APU_Envelope.xhtml "APU Envelope")/[disable length counter](APU_Length_Counter.xhtml "APU Length Counter"), [constant volume](APU_Envelope.xhtml "APU Envelope"), [envelope period/volume](APU_Envelope.xhtml "APU Envelope")  
**$400E** | `L--- PPPP` | [Loop noise](APU_Noise.xhtml "APU Noise"), [noise period](APU_Noise.xhtml "APU Noise")  
**$400F** | `LLLL L---` | [Length counter load](APU_Length_Counter.xhtml "APU Length Counter") (also starts [envelope](APU_Envelope.xhtml "APU Envelope"))   
| **[DMC channel](APU_DMC.xhtml "APU DMC")** (write)   
**$4010** | `IL-- FFFF` | IRQ enable, loop sample, frequency index   
**$4011** | `-DDD DDDD` | Direct load   
**$4012** | `AAAA AAAA` | Sample address `%11AAAAAA.AA000000`  
**$4013** | `LLLL LLLL` | Sample length `%0000LLLL.LLLL0001`  
**$4015** | `---D NT21` | **[Control](APU.xhtml "APU Status")** : [DMC enable](APU_DMC.xhtml "APU DMC"), [length counter enables](APU_Length_Counter.xhtml "APU Length Counter"): [noise](APU_Noise.xhtml "APU Noise"), [triangle](APU_Triangle.xhtml "APU Triangle"), [pulse 2](APU_Pulse.xhtml "APU Pulse"), [pulse 1](APU_Pulse.xhtml "APU Pulse") (write)   
**$4015** | `IF-D NT21` | **[Status](APU.xhtml "APU Status")** : [DMC interrupt](APU_DMC.xhtml "APU DMC"), [frame interrupt](APU_Frame_Counter.xhtml "APU Frame Counter"), [length counter status](APU_Length_Counter.xhtml "APU Length Counter"): [noise](APU_Noise.xhtml "APU Noise"), [triangle](APU_Triangle.xhtml "APU Triangle"), [pulse 2](APU_Pulse.xhtml "APU Pulse"), [pulse 1](APU_Pulse.xhtml "APU Pulse") (read)   
**$4017** | `SD-- ----` | **[Frame counter](APU_Frame_Counter.xhtml "APU Frame Counter")** : 5-frame sequence, disable frame interrupt (write)   
  
Categories: [APU](Category_APU.xhtml)
