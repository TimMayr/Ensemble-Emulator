# RP2A03 Programmable Interval Timer

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/RP2A03_Programmable_Interval_Timer) | View [other pages](Special_AllPages.xhtml#RP2A03_Programmable_Interval_Timer)

The original RP2A03 contains an unfinished and inaccessible 24-bit **programmable interval timer** , capable of generating an IRQ. Most of the traces connecting it to the rest of the chip have been cut, physically disconnecting it and preventing its use. This timer was discovered through visual inspection of the RP2A03 die and its behavior was reverse engineered visually and through simulation. It was removed in subsequent revisions of the CPU (RP2A03E and onward). 

## Contents

  * 1 Inferred operation
  * 2 Inferred register interface
    * 2.1 Counter reload value ($401C, $401D, $401E write)
    * 2.2 Current counter value ($401C, $401D, $401E read)
    * 2.3 Timer settings ($401F write)
    * 2.4 Timer status ($401F read)
  * 3 Observations
  * 4 Errata
  * 5 References



## Inferred operation

The timer uses a 24-bit counter and counts up or down by one each clock, setting a timer flag approximately when the counter reaches 0. If the timer flag is set and the timer IRQ is enabled, the CPU's /IRQ input is asserted until the timer flag is read, clearing the flag. The counter can be configured to reload automatically upon expiration and can also be reloaded manually. 

The input clock is selectable between two prescaled M2 clocks and one of either edge of joypad 2 /OE. The M2 prescaler is reset when the counter is manually reloaded. Joypad 1 /OE toggles when the timer expires. 

## Inferred register interface

Because the timer is unfinished, there appear to be numerous bugs in its register interface. The register definitions below include corrections for these, representing how it is believed the registers were intended to function. The bugs are discussed in the Errata section. 

### Counter reload value ($401C, $401D, $401E write)
    
    
      $401E       $401D       $401C
    7  bit  0   7  bit  0   7  bit  0
    ---- ----   ---- ----   ---- ----
    HHHH HHHH   MMMM MMMM   LLLL LLLL
    |||| ||||   |||| ||||   |||| ||||
    ++++-++++---++++-++++---++++-++++- 24-bit counter reload value
    

### Current counter value ($401C, $401D, $401E read)
    
    
      $401E       $401D       $401C
    7  bit  0   7  bit  0   7  bit  0
    ---- ----   ---- ----   ---- ----
    HHHH HHHH   MMMM MMMM   LLLL LLLL
    |||| ||||   |||| ||||   |||| ||||
    ++++-++++---++++-++++---++++-++++- Current 24-bit counter value
    

### Timer settings ($401F write)
    
    
      $401F
    7  bit  0
    ---- ----
    ELAC DTZC
    |||| ||||
    |||+----+- Clock source:
    |||  |||    00: M2÷16
    |||  |||    01: M2÷256
    |||  |||    10: Rising edges of joypad 2 /OE
    |||  |||    11: Falling edges of joypad 2 /OE
    |||  ||+-- Unused. Latched value toggles on timer expiration.
    |||  |+--- Value to output on joypad 1 /OE. Toggles on timer expiration. (0 = low, 1 = high)
    |||  +---- Counter direction (0 = down, 1 = up)
    ||+------- Enable automatic reload on expiration (1 = enable)
    |+-------- Trigger immediate counter reload and M2 prescaler reset (1 = trigger)
    +--------- Enable timer IRQ (1 = enable)
    

### Timer status ($401F read)
    
    
      $401F
    7  bit  0
    ---- ----
    Ixxx xxxT
    |||| ||||
    |||| |||+- Timer flag (1 = timer expired)
    |+++-+++-- (Open bus)
    +--------- Timer IRQ enabled (1 = enabled)
    
    On read: T = 0
    

The timer expired flag becomes set when the timer expires (reaches 0). The timer asserts an IRQ while that flag and the timer IRQ enable are both 1. Reading this register clears the flag, acknowledging the interrupt. 

## Observations

It's not clear what the intended purpose of this timer was. 

  * A 24-bit counter with the maximum M2 prescaler can support times as long as 40 minutes, which is exceptionally long in terms of gameplay.
  * Joypad 2 /OE can be used as a clock source and has a history of being used for watchdogs (such as on the [Vs. System](Vs__System.xhtml "Vs. System") and [FamicomBox](FamicomBox.xhtml "FamicomBox")), but as a watchdog, it would need to be used as a counter reset, not a clock source. It's also not clear why the clock can be selected between rising or falling edges on joypad 2 /OE, as both edges occur in close proximity and should be roughly equivalent.
  * Joypad 1 /OE acts as a timer-controlled clock, but it's not clear why you would want this outputted to a joypad, and you can't use the timer without causing at least one toggle on this output. Perhaps the joypad 1 /OE pin would no longer go to the joypad and would serve some other purpose.



## Errata

  * The counter reload bit is stored as dynamic memory and will decay. The approximate time until decay and direction of decay aren't known.
  * The condition for whether the counter should count is backward, causing the counter to count when it is 0 rather than non-zero.
  * It looks intended that the timer would expire a clock after becoming 0, but a missing via complicates this behavior, likely making it expire when becoming 0 (and possibly at other times, too).
  * The M2 prescalers appear to only divide by 8 and 128; in simulation, the first stage is nonfunctional.
  * All 4 readable registers are activated by writes instead of reads. The IRQ acknowledge on $401F works on reads as it should, though.
  * Joypad 1 /OE as a timer expiration output is not compatible with the use of a standard controller.



## References

  * Forum: [Memory map and 2A03 register map / 2A03 cutting-room floormetal](http://forums.nesdev.org/viewtopic.php?f=9&t=14421)
  * See: [CPU pin out and signal description](CPU_pinout.xhtml "CPU pin out and signal description")


