# Talk:APU Frame Counter

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AAPU_Frame_Counter) | View [other pages](Special_AllPages.xhtml#Talk_APU_Frame_Counter)

The frame counter [on the die](http://uxul.org/~noname/chips/cpu-2/no-metal/stitched/final/) (upper-right, just below the joypad strobe pin) seems to be a 15-bit linear feedback shift register (with taps at the 14th and 15th bits); when the LFSR is initialized with all 1s and clocked every other CPU cycle, a decoder appears to generate signals at 7456, 14912 (+7456), 22370 (+7458), 29828 (+7458), and 37280 (+7452) cycles. I have not yet determined how these relate to the 2 different sequence modes. --[Quietust](User_Quietust.xhtml "User:Quietust") 17:09, 23 January 2011 (UTC) 

    Both the 4th and 5th stages (29828 and 32780) trigger the "quarter" and "half" frame signals and reset the LFSR, but the 4th stage is skipped entirely if it's in the 5-step sequence mode. The decode table also has a special case - when all of the bits are 0, it forces 1 into the LFSR's input, presumably to prevent it from stalling (likely triggered on power-up). I can also see some logic that looks like it'll forcefully clock the "quarter" and "half" frame signals upon writing $4017 if D7 is set, which would probably explain the behavior observed (but also result in slightly different timing between the first loop and subsequent loops). --[Quietust](User_Quietust.xhtml "User:Quietust") 15:46, 10 May 2011 (UTC) 

    When writing to $4017, the frame counter reset and the quarter/half frame triggers happen simultaneously, but only on "odd" cycles (and only after the first "even" cycle after the write occurs) - thus, it happens either 2 or 3 cycles after the write (i.e. on the 2nd or 3rd cycle of the next instruction). --[Quietust](User_Quietust.xhtml "User:Quietust") 18:41, 29 June 2011 (UTC)

With your permission, Quietust, I'd like to incorporate this information into the article. --[Drag](User_Drag.xhtml "User:Drag") 22:38, 11 July 2011 (UTC) 

    Most of it already has been added. --[Quietust](User_Quietust.xhtml "User:Quietust") 13:07, 12 July 2011 (UTC)

My emu doesn't pass in the blargg's frame IRQ flag test if the IRQ is set at 29828, which _should be_ at 29829. Plus, the step 4 reset _should be_ 29831 _minus_ 29828, and not a wrap to zero. What do you think? --[Zepper](User_Zepper.xhtml "User:Zepper") 17:39, 12 July 2011 (UTC) 

    Perhaps your emulator has issues with interrupt latency? Also, once your counter reaches 29828, it should reset to zero after two cycles - the actual counter in the APU is only updated every other CPU cycle, and when it reaches 14914 (step 4) it resets to 0 **instead of incrementing to 14915** (but it isn't really incrementing, since it's a linear feedback shift register as mentioned above). --[Quietust](User_Quietust.xhtml "User:Quietust") 19:33, 12 July 2011 (UTC)

It should also be noted that APU cycles happen when M2 is low - since CPU reads and writes happen when M2 is high, this means that APU cycles and APU writes can never happen at exactly the same time. --[Quietust](User_Quietust.xhtml "User:Quietust") 03:20, 13 July 2011 (UTC) 

I disagree about taking a lower level for emulation purposes. What's wrong about using a CPU cycle based counter? Plus, how does such _APU_ LFSR work? How each value (?) is checked for the APU operations, like the next step? The article in the wikipedia brings various types of shift register. Perhaps not everyone that has such level of knowledge, from an _hardware circuit level_.--[Zepper](User_Zepper.xhtml "User:Zepper") 04:05, 13 July 2011 (UTC) 

    I'm under the assumption that a 16-bit LFSR will generate a string of perfectly-unique 16-bit numbers before the string wraps around to the beginning again. A regular up-counter does the exact same thing. In both cases, you'd just be looking at the contents of the register to see if it matches one of the 5 (I'm guessing) steps. As for why it's a LFSR specifically, I can only speculate that it was cheaper/simpler than a counter, whilst providing the same functionality (since it just needs to count, it doesn't matter if the numbers are in pseudorandom order, just as long as they're in the same pseudorandom order every time).
    I don't see any reason why you _couldn't_ use a simple CPU cycle counter for the sequencer in an emulator; as long as your timings are correct, you can completely disregard that the hardware uses an LFSR. --[Drag](User_Drag.xhtml "User:Drag") 06:43, 13 July 2011 (UTC) 

    If you really want to see how it works, look [here](http://www.qmtpro.com/~nes/visual2a03/expert.html) and ask it to find "frm_t0". Also, in case you're curious, the noise and DPCM frequency counters are also LFSRs (since they get loaded from lookup tables). Interestingly, the NES lockout chip even uses an LFSR for its **program counter**. --[Quietust](User_Quietust.xhtml "User:Quietust") 12:37, 13 July 2011 (UTC)

Fact: there is a $4015 read at CPU 29828 **and** a frame IRQ being requested. Well, it appears in every APU test; that's why the frame IRQ _should_ take effect at 2982**9** , possibly because of the CPU/APU clock ratio (2 CPU clocks = 1 APU clock): it take effect on other CPU cycle. Unless, of course, I'm messing up the things. --[Zepper](User_Zepper.xhtml "User:Zepper") 16:38, 30 July 2011 (UTC) 

There's no _$4015 read_ with a _frame IRQ flag rise_. It was a timing bug in my pAPU driver. Because of my software architecture, the modules are clocked in this order: APU,PPU,CPU. So, the length counter clock is 1 cycle late, but it was fixed with an easy flag set/clear at each pAPU clock. Sorry for the trouble. Such pAPU debugging was a nice exercise, very additive. --[Zepper](User_Zepper.xhtml "User:Zepper") 02:35, 5 August 2011 (UTC) 

## Some times off by .5?

The IRQ firing at 14914 seems correct (when looking at the circuitry too), but blargg's apu_tests seem to assume that the length counters (and possibly other stuff too) update at 7456.5, etc. Would be easy to confirm if that's the case in Visual 2A03, but it always crashes before I can simulate to the first frame counter clock in both Firefox and Chrome (perhaps due to a memory leak). :/ -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 05:50, 13 August 2013 (MDT) 

    Through repeated getState()/setState() and reloading in Visual 2A03 I managed to get to APU tick 7456. The log around that point also suggests that the correct update tick is 7456.5. Each row is half a CPU cycle, and time flows from bottom to top. You see the frm_half and frm_quarter signals being generated one CPU cycle after frm_t (the LFSR that implements the frame counter) changes, and at the same time sq0_len is decremented:
    
    
    sq0_len  frm_t   frm_half   frm_quarter
    1c       580e    1          1
    1c       580e    1          1
    1c       6c07    1          1
    1c       6c07    1          1
    1c       6c07    1          1
    1c       6c07    1          1
    1c       3603    1          1
    1c       3603    0          0
    1d       3603    1          1
    1d       3603    1          1
    1d       5b01    1          1
    1d       5b01    1          1
    1d       5b01    1          1
    1d       5b01    1          1
    1d       2d80    1          1
    1d       2d80    1          1
    

    I'll wait a few days before I update the wiki to see if anyone screams. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 08:28, 13 August 2013 (MDT)

## PAL

What are the cycle counts for the 2A07 (PAL NES CPU)? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 15:22, 18 August 2014 (MDT) 

    Once an RP2A07 is depackaged and delayered, we'll find out. --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 07:41, 3 November 2014 (MST)

## meaning of table rows that aren't "steps"

So the frame counter steps at the particular cycle counts shown for each step (or not), but what's with the empty rows for 4-step mode that include "clock"? 
