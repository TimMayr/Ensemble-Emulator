# Talk:APU

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AAPU) | View [other pages](Special_AllPages.xhtml#Talk_APU)

This page has ad-hoc protection on it to deter spambots. To reply to a thread or start a new thread:  
  
**[Unlock this page](Talk_APU_current.xhtml "Talk:APU/current")**

## Contents

  * 1 Function of $4017
  * 2 Channel disable doesn't affect length counter
  * 3 Glossary entry for "timer"
  * 4 Lengthcounter and Framecounter inconsistency



## Function of $4017

Isn't $4017 joy2? — Preceding unsigned comment added by [69.20.131.234](User_69_20_131_234.xhtml "User:69.20.131.234") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:69.20.131.234&action=edit&redlink=1 "User talk:69.20.131.234 \(page does not exist\)") • ~~contribs~~) 3 June 2017‎

    Yes but that's the read function. The write function of $4017 is to control the APU. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:28, 5 June 2017 (MDT)

## Channel disable doesn't affect length counter

Looking at Visual 2A03 and e.g. the sq0_on and sq0_len nodes, it seems that disabling a channel doesn't touch the length counter at all. Rather, there are internal channel enable flags (eg. sq0_on) that get set when the length counter is initialized (presumably only for non-zero values, though I haven't checked) and cleared when either the length counter reaches zero or the channel is disabled. These are the flags you get back from $4015/read as well. Even if this page only describes "abstract APU operation", that's not any more complicated than the current version, so maybe the descriptions should be updated. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 23:30, 28 April 2013 (MDT) 

## Glossary entry for "timer"

The glossary entry for timer says 

> The triangle channel's timer is clocked on every CPU cycle, but the pulse, noise, and DMC timers are clocked only on every second CPU cycle and thus produce only even periods.

But my understanding is the triangle, noise, and DMC timers are clocked on every CPU cycle, and only the pulse channel timers are clocked on every second CPU cycle. Have I understood correctly? --[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 21:29, 19 February 2014 (MST) 

    Then why do noise and DMC timers produce only even periods? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 07:22, 20 February 2014 (MST)

    

    The [noise](APU_Noise.xhtml "APU Noise") and [DMC](APU_DMC.xhtml "APU DMC") channels use lookup tables to set the period of their timers. It looks like all of the lookup table entries happen to be even.

    

    I've been studying the NES sound capabilities as a hobby, let me know if I've gotten something wrong. Here's what I understand.

    

    From the References section, I read [Blargg's NES APU Reference](http://nesdev.org/apu_ref.txt). In that document, the "Timer" section says "All channels use a timer which is a divider driven by the ~1.79 MHz clock." I interpret this to mean all sound channels have a timer clocked by the [CPU clock](Cycle_reference_chart.xhtml "Clock rate"). In the same document, the "Square Channel" section block diagram has a block called "Timer / 2", whereas the block diagrams in all other channels have a block called "Timer".

    

    If I understand correctly, a "Timer / 2" clocked by every CPU clock would be the same as a "Timer" clocked by every other CPU clock. I'm not sure which one it really is implemented as. My main concern was it seems the statement in the glossary may be wrong regarding the noise and DMC timers, if I understand correctly, they are clocked every CPU clock. --[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 19:02, 20 February 2014 (MST)

    

    

    The terminology here is problematic. The APU clock, which is used by all audio hardware except the triangle channel, is formed by dividing the CPU clock by two. In practice, the formulæ on this page already include this division, and any differences in implementation will likely only show up on the most pedantic of tests. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 19:41, 20 February 2014 (MST)

    

    

    

    I see. Blargg's document states it's just an functional model that may differ from actual implementation, so I guess it's not usable as proof of how the timers are actually implemented.

    

    

    

    To make sure I understand: The lookup tables at [APU Noise](APU_Noise.xhtml "APU Noise") and [APU DMC](APU_DMC.xhtml "APU DMC") are in terms of CPU clocks, correct? But you're saying the hardware implementation of the lookup tables are actually in terms of APU clocks (using values that are half those shown in the documents), and the actual noise and DMC timers in hardware are clocked by APU clocks?--[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 21:11, 20 February 2014 (MST)

    

    

    

    

    You understand correctly. If you go to [visual2a03](http://www.qmtpro.com/~nes/chipimages/visual2a03/) you can find the signals _apu_clk1_ , _apu_/clk2_ , _apu_clk2a_ , …b, …c, …d, which control this. This is briefly mentioned in [Visual circuit tutorial#APU clock signals](Visual_circuit_tutorial.xhtml#APU_clock_signals "Visual circuit tutorial"). —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 23:28, 20 February 2014 (MST)

    

    

    

    

    

    From a different approach, the DPCM tunings are all evidently a closest match to an A440 tuning scheme (notwithstanding 2 mistakes in the PAL table implementation), but _only_ if we're using half-clocks. If the DPCM tables could have "odd" values in it, the DPCM tuning scheme would have used them to better fit it. More info here: [[1]](http://forums.nesdev.org/viewtopic.php?f=6&t=5473) \- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 07:44, 22 February 2014 (MST)

## Lengthcounter and Framecounter inconsistency

The page about the APU Framecounter suggests that the Lenghtcounter is clocked twice per sequence (regardless of the Framecounter mode flag). Oddly the documentation for the lengthcunter assumes that it is clocked only once. So what is right? If it is really clocked twice per sequence, the bpm values would be 150 bpm and 180 bpm instead of 75 bpm and 90 bpm. In this case the document would also completely miss that the notes are slower if the framecounter mode flag is set. This would provide two alternative paces of 120 bpm (MSB set) and 144 bpm (MSB clear). 

The resource document (Blargg's NES APU Reference) also says that the lengthcounter is clocked twice per sequence. 

    The length counter is clocked twice per sequence. Where on the Wiki does it say otherwise? (This would need to be corrected.) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:32, 19 September 2014 (MDT)
