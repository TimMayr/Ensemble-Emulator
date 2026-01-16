# Talk:Clock rate

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AClock_rate) | View [other pages](Special_AllPages.xhtml#Talk_Clock_rate)

## Divider

Ok, ok. I went to the visual2A03, saw that Quietust had implemented a "microstep" function which drives the 2A03's clock input (instead of the inside 6502's φ0 input), and so tested this. [In the _2A03_](http://www.qmtpro.com/~nes/chipimages/visual2a03/?nosim=t&find=11292&panx=553.7&pany=344.0&zoom=8.6), the input clock is implemented using a [Johnson counter](https://en.wikipedia.org/wiki/Ring_counter "wikipedia:Ring counter"), which intrinsically divides by an even number. They couldn't use the same design to divide by 15, so they instead divided by 16. 

Interestingly, there is even two more unused bits on this divider (off the bottom), so maybe by the time they made the 2A03G's die they had unified the 2A07 and 2A03 designs sufficiently to just be a "move this tap from here to here". 

Dividing by 15 would have required an entire different design, and I suspect that the smallest ÷15 circuit (4 bit binary counter plus 4-input AND gate) would have taken more silicon die space than the 8-stage Johnson counter they used. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 00:05, 4 September 2012 (MDT) 

    The CIC uses a polynomial counter for the program counter to save die space. How hard would it have been to use a 4-bit [linear feedback shift register](https://en.wikipedia.org/wiki/Linear_feedback_shift_register "wikipedia:Linear feedback shift register") to divide by 15? I guess there would have to be some post-decoding of the LFSR's state to produce a binary clock signal, and that might have taken a lot of space too. --[Tepples](User_Tepples.xhtml "User:Tepples") 08:13, 4 September 2012 (MDT)

    

    Right, that would have required at least a 2-input XOR gate. I'm not clear on whether 4 bits of shift register plus the smallest 2-input XOR gate (implementated as an and-or-invert and an inverter, no P-type MOSFETs so no transmission gates yet) is bigger, but regardless none of the internal bits of state are even close to a 50% duty cycle at ≈1.8MHz, and converting that back would definitely have been more silicon space. Meanwhile, a design with a synchronous 4-bit counter that skipped state 0b1111 would have required 4 JK registers, and one each of a 2-, 3-, and 4- input AND gate.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 13:25, 4 September 2012 (MDT)

    

    I don't know if it's a requirement of the output, but a johnson counter very conveniently has each bit output equal time high and low, doesn't it? If you an effective 15x divide with equal time high/low, would you actually need a 30x divider that is clocked twice as fast? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 12:20, 4 September 2012 (MDT)

    

    

    While other microcontroller datasheets specify fairly strict requirements on duty cycle, the 6502 datasheet only specifies that φ0 high and low times both need to be at least 240ns (2MHz binned parts). A nonuniform division should have been ok as long as it complied with that part; for the PAL CPU they probably could have divided it into 6.5 (240ns×26.6MHz=6.4) and 8.5 safely but no further. To answer the other half: yes, so you'd need a design that reacted to both rising and falling edges from the master clock. Possible, but bigger, and probably easier to use a binary counter. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 13:25, 4 September 2012 (MDT) 

    I slightly misspoke: In the 2A0x, **two** Johnson counters are used, each clocked on opposite edges, and a NOR gate (output is node 11200) combines the two phases. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 16:47, 17 January 2013 (MST)
    Kev tells me the "In the _2A03_ " link is broken. Did you mean [this](http://www.qmtpro.com/~nes/chipimages/visual2a03/?nosim=t&find=11292&panx=553.7&pany=344.0&zoom=8.6)? Perhaps die shots from a Dendy CPU might help. --[Tepples](User_Tepples.xhtml "User:Tepples") 15:50, 17 January 2013 (MST) 

    Looks like Q replaced the traditional view with the expert one. Fixed my link. Dollars to doughnuts that the Dendy implements its ÷15 with an arithmetic up-counter and a 4-input AND gate. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 16:47, 17 January 2013 (MST)

## NTSC frequency

Where did 1.789772**67** MHz come from? Why is it not 1.789772**73**? It's only 30ppb error, insignificant by NTSC's 3ppm requirement, but it's odd. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:56, 14 November 2014 (MST) 

    Probably the master clock (21.477272 MHz), rounded to the nearest Hz, then divided by 12. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 17:16, 14 November 2014 (MST)
