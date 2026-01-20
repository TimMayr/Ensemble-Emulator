# Talk:CPU power up state

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ACPU_power_up_state) | View [other pages](Special_AllPages.xhtml#Talk_CPU_power_up_state)

Parts of the APU which appear to be affected by reset (determined by tracing the reset signal throughout the chip): 

  * Square channel volume/envelope timer (**not** the reload value at $4000/$4004 bits 0-3) and counter (the actual volume output)
  * Noise channel volume/envelope timer (**not** the reload value at $4008 bits 0-3) and counter (the actual volume output)
  * Square channel frequency counter, including the 3 duty cycle bits
  * Triangle channel frequency counter
  * Noise channel frequency counter, sort of - it forces the LFSR input to 0, so a **very** short reset pulse might only partially clear it
  * DPCM channel frequency counter, sort of - it forces the LFSR input to 0, so a **very** short reset pulse might only partially clear it
  * Noise channel frequency ($400E bits 0-3)
  * Square channel sweep counter
  * All 4 length counters
  * All 5 channel enables ($4015)
  * Triangle channel linear counter (**not** the reload value at $4008)
  * Triangle channel Position
  * DPCM channel Sample Length counter (**not** the reload value)
  * DPCM channel Position ($4011)
  * DPCM channel Sample Buffer and Bit Counter
  * DPCM channel Sample Address counter (**not** the reload value)
  * DPCM channel state machine
  * Sprite DMA address counter and state machine
  * Part of the Frame Counter (notably, **not** the two writable bits of $4017)
  * Debug register $401A bit 7



\--[Quietust](User_Quietust.xhtml "User:Quietust") 16:40, 8 June 2011 (UTC) 

## Detecting User Reset

Is there way using common mappers to check if power on or reset button pushed? --[Zzo38](User_Zzo38.xhtml "User:Zzo38") 18:35, 10 September 2012 (MDT) 

    Common practice seems to be "put a canary in memory, if it's still there assume warm (re)boot instead of cold boot. e.g., Galaxian puts the programmer's name in $0100-$010F, and if it's there on boot, it doesn't clear high scores or the easter egg counter.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") 19:47, 10 September 2012 (MDT)

## Consistent RAM power-up state?

I tested my NES and Famicom today with a [simple ROM that just displays the contents of RAM](http://forums.nesdev.org/viewtopic.php?f=3&t=13334). I get widely varying results at power-on. I tried leaving the system off for varying amounts of time between powerings, but I don't see much consistency at all. Here's some examples of what I've seen: 

  1. Mostly $FF.
  2. Mostly some specific byte, e.g. $E7.
  3. Repeating patterns like $00 $FF $00 FF, often in consistent lines of 16 or 32 bytes.
  4. Often I get groups with a specific byte value, especially in columns (using rows of 16).



All of these patterns are heavily subject to noise. Sometimes a lot of noise, sometimes only a little. Really I don't see much consistent behaviour at all on my NES. 

My Famicom usually powers up with mostly a repeating 16 byte pattern of 00FF00FF00FF00FFFF00FF00FF00FF00, often with one or many columns of bytes (using rows of 16) that look completely random in contrast to that pattern. Occasionally the Famicom powers up with different patterns than this too. 

    I finally got an opportunity to test this myself. My Front loading NES powers on with a 32 byte base pattern that's briefly described as "data = ((addr>>0 ^ addr>>2 ^ addr>>4 ^ addr>>5) & 1) * 0xff" with some pattern of toggled bits I have not discerned yet. [Ns43110](https://www.nesdev.org/w/index.php?title=User:Ns43110&action=edit&redlink=1 "User:Ns43110 \(page does not exist\)") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Ns43110&action=edit&redlink=1 "User talk:Ns43110 \(page does not exist\)")) 11:58, 21 January 2016 (MST)

The recommendation for specific values at $0008/9/A/F seems bizarre to me. Why is this in the article at all? These bytes are not particularly consistent on either of my machines, and _certainly_ don't match these magic values. A few quick power cycles sometimes seems to leave some bytes alone, but I don't see anything special about 8/9/A/F? 

    Nowhere does it say that it's a recommendation. Those are simply results from a test (I believe from blargg). --[Thefox](User_Thefox.xhtml "User:Thefox") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Thefox&action=edit&redlink=1 "User talk:Thefox \(page does not exist\)")) 16:24, 5 October 2015 (MDT)

    

    Two emulator authors seemed to have taken it as some kind of recommendation (NEStopia and puNES), so I think at the very least I think we need a better description of what should be expected. I don't see any value at all in describing these 4 magic values here, personally. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:25, 5 October 2015 (MDT)

I'm not certain the idea that it should be "mostly FF" is particularly good either. On my Famicom, it looks like 50/50 for a randomly selected bit (though with a high probability of fitting the pattern listed for any specific bit). On my NES, I'd guess there's might be a slight bias toward 1 bits, but I really don't think that's a very useful piece of information given how random it is, and how often there are repetitive (and different) patterns in the bits. 

I'm curious what other people might see on their machines. 

\- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 16:07, 5 October 2015 (MDT) 

    My NES (RAM is a LH5116) is mostly $FF, although random bits are randomly clear. It's often patterned with a stride of 32 bytes. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 18:39, 5 October 2015 (MDT)
