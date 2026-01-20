# Talk:CPU pinout

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ACPU_pinout) | View [other pages](Special_AllPages.xhtml#Talk_CPU_pinout)

## Pin 30

The signal from pin 30 goes through the exact same "processing" as the chip's /RESET signal (which includes a Schmitt trigger), at which point it goes to the output logic for the M2 pin (don't know exactly what it does) and then through an inverter which goes to the enables for $4018-$401A and to some other spots (seemingly related to the data buffer used for Sprite DMA). Bit 7 of the writable register at $401A seems to propagate to numerous locations within the sound channels and thus might be responsible for the observation of all writable registers disappearing. --[Quietust](User_Quietust.xhtml "User:Quietust") 16:49, 29 April 2011 (UTC) 

    When pin 30's input is "processed" it gets inverted, just like with the /RESET pin (since transistors themselves are active-high) - that signal then gets inverted again (so it's now equal to pin 30's input, normalized) and NORed together with the inverted +RESET signal (so it's also the actual state of the input signal), and that result goes to the M2 pin's output logic; thus, if pin 30 is pulled high, then M2 will **not** be tri-stated during RESET. --[Quietust](User_Quietust.xhtml "User:Quietust") 20:33, 7 May 2011 (UTC) 

    The area where pin 30 goes which I thought was part of the Sprite DMA buffer is actually the logic that controls the R/W pin and the direction of the data pins (it puts the data pins in output mode to handle reading from internal registers), and pulling pin 30 high forces all of $4000-$401F to become internal registers (instead of just $4015). This means that pulling pin 30 high will effectively disable the joypad read ports (but **not** the write port) - the read enables will still go low, but the CPU won't get any of the data. I'm guessing the designers figured it'd be simpler to do it that way than to combine the 3 read enable signals for $4018-$401A. --[Quietust](User_Quietust.xhtml "User:Quietust") 15:21, 9 May 2011 (UTC) 

    Given Lidnariq's edits to the main article, has this actually been tested and verified against actual hardware? --[Quietust](User_Quietust.xhtml "User:Quietust") 19:58, 14 September 2011 (UTC) 

    I've just performed a test with my CopyNES (removed the CPU, carefully bent pin 30 up, then plugged it in and clipped a wire between it and VCC) and I can confirm that registers $4018-$401A **do** behave as described [here](File_Apu_address_jpg.xhtml "File:Apu address.jpg") \- reading $4018-$401A returns the output values of each channel, writing $401A bits 0-4 sets the triangle channel position, and setting $401A bit 7 freezes all of the channels. I can also confirm that the joypad ports no longer work, though they still seem to return open bus - presumably, it's actually coming from inside the chip. Additional confirmation would be highly desirable. --[Quietust](User_Quietust.xhtml "User:Quietust") 18:28, 16 September 2011 (UTC)
    Will it work OK if pin 30 and pin 7 are wired together? Is someone able to try this to see what happens? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 16:24, 1 January 2014 (MST) 

    If by "work OK" you mean "permit access to $4018-$401A without breaking $4016-$4017 by making debug mode only activate when accessing $4008-$400F/$4018-$401F/etc.", then the answer is probably "yes", but I don't have the means to test that right now. --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 09:50, 2 January 2014 (MST)

Pin 30 is actually used on the Playchoice, in the schematic it is named /SPECIAL. Maybe it is used to disable controller input? [Memblers](https://www.nesdev.org/w/index.php?title=User:Memblers&action=edit&redlink=1 "User:Memblers \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Memblers&action=edit&redlink=1 "User talk:Memblers \(page does not exist\)")) 04:51, 21 July 2018 (MDT) 

## M2

M2 goes high 1.5 clock cycles (i.e. three clock _edges_ from the 21.477272MHz master clock) **before** Φ2, but it goes low at the same time as Φ2. 

Would it be okay to still mention the 6502 input clock (Φ0) in conjunction with M2? It helped me a lot when I realized they're "basically" the same (i.e. when reading [[1]](http://forums.nesdev.org/viewtopic.php?f=3&t=9999)), and it's pretty easy to miss still scanning through the description. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 19:05, 9 April 2013 (MDT) 

    I feel like that point belongs with CLK instead of M2; does my last edit satisfy?—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 20:18, 9 April 2013 (MDT) 

    Stating that M2 is (roughly) the CPU clock cuts down the chain of inference a bit, and it's handy knowledge even for code monkeys like me when trying to figure out timing diagrams and the like. I'll see if I can come up with some reasonable middle ground. :) -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 20:24, 9 April 2013 (MDT) 

    I guess the "(φ0 is in turn inverted to form φ1, which is then inverted to form φ2.)" part could be removed if you feel it's getting too verbose/redundant. -[Ulfalizer](User_Ulfalizer.xhtml "User:Ulfalizer") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ulfalizer&action=edit&redlink=1 "User talk:Ulfalizer \(page does not exist\)")) 20:51, 9 April 2013 (MDT) 

    LGTM. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 20:58, 9 April 2013 (MDT)
