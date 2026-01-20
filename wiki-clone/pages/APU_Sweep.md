# APU Sweep

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/APU_Sweep) | View [other pages](Special_AllPages.xhtml#APU_Sweep)

An [NES APU](APU.xhtml "APU") sweep unit can be made to periodically adjust a [pulse channel's](APU_Pulse.xhtml "APU Pulse") period up or down. 

Each sweep unit contains the following: 

  * [divider](APU.xhtml "APU Misc")
  * reload flag



## Contents

  * 1 Registers
  * 2 Calculating the target period
  * 3 Muting
  * 4 Updating the period



## Registers

**$4001** | `EPPP.NSSS` | **[Pulse channel 1](APU_Pulse.xhtml "APU Pulse")** sweep setup (write)   
---|---|---  
**$4005** | `EPPP.NSSS` | **[Pulse channel 2](APU_Pulse.xhtml "APU Pulse")** sweep setup (write)   
bit 7 | `E--- ----` | Enabled flag   
bits 6-4 | `-PPP ----` | The divider's period is P + 1 half-frames   
bit 3 | `---- N---` | Negate flag  
0: add to period, sweeping toward lower frequencies  
1: subtract from period, sweeping toward higher frequencies   
bits 2-0 | `---- -SSS` | Shift count (number of bits).  
If SSS is 0, then behaves like E=0.   
Side effects | Sets the reload flag   
  
## Calculating the target period

The sweep unit continuously calculates each pulse channel's **target period** in this way: 

  1. A barrel shifter shifts the pulse channel's 11-bit [raw timer period](APU_Pulse.xhtml "APU Pulse") right by the shift count, producing the change amount.
  2. If the negate flag is true, the change amount is made negative.
  3. The target period is the sum of the current period and the change amount, clamped to zero if this sum is negative.



For example, if the negate flag is false and the shift amount is zero, the change amount equals the current period, making the target period equal to twice the current period. 

The two pulse channels have their adders' carry inputs wired differently, which produces different results when each channel's change amount is made negative: 

  * Pulse 1 adds the [ones' complement](https://en.wikipedia.org/wiki/Ones%27_complement "wikipedia:Ones' complement") (_−c − 1_). Making 20 negative produces a change amount of −21.
  * Pulse 2 adds the [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement "wikipedia:Two's complement") (_−c_). Making 20 negative produces a change amount of −20.



Whenever the current period or sweep setting changes, whether by $400x writes or by sweep updating the period, the target period also changes. 

## Muting

**Muting** means that the pulse channel sends 0 to the [mixer](APU_Mixer.xhtml "APU Mixer") instead of the current volume. Muting happens regardless of whether the sweep unit is disabled (because either the Enabled flag or the Shift count are zero) and regardless of whether the sweep divider is outputting a clock signal. 

Two conditions cause the sweep unit to mute the channel until the condition ends: 

  1. If the _current_ period is less than 8, the sweep unit mutes the channel.
  2. If at any time the _target_ period is greater than $7FF, the sweep unit mutes the channel.



In particular, if the negate flag is false, the shift count is zero, and the current period is at least $400, the target period will be large enough to mute the channel. (This is why several publishers' NES games never seem to use the bottom octave of the pulse waves.) 

Because the target period is computed continuously, a target period overflow from the sweep unit's adder can silence a channel _even when the sweep unit is disabled_ and even when the sweep divider is not outputting a clock signal. Thus to fully disable the sweep unit, a program must additionally turn on the Negate flag, such as by writing $08. This ensures that the target period is not greater than the current period and therefore not greater than $7FF. 

## Updating the period

When the [frame counter](APU_Frame_Counter.xhtml "APU Frame Counter") sends a half-frame clock (at 120 or 96 Hz), two things happen: 

  1. If the divider's counter is zero, the sweep is enabled, the shift count is nonzero, 
     1. and the sweep unit is _not_ muting the channel: The pulse's period is set to the target period.
     2. and the sweep unit _is_ muting the channel: the pulse's period remains unchanged, but the sweep unit's divider continues to count down and reload the divider's period as normal.
  2. If the divider's counter is zero _or_ the reload flag is true: The divider counter is set to P and the reload flag is cleared. Otherwise, the divider counter is decremented.



If the sweep unit is disabled including because the shift count is zero, the pulse channel's period is never updated, but muting logic still applies. 

Categories: [APU](Category_APU.xhtml)
