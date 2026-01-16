# Talk:APU DMC

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AAPU_DMC) | View [other pages](Special_AllPages.xhtml#Talk_APU_DMC)

Similarly to the [noise channel](APU_Noise.xhtml "APU Noise"), the DPCM channel's frequency counter [on the die](http://uxul.org/~noname/chips/cpu-2/no-metal/stitched/final/) is a **9-bit linear feedback shift register** (with taps at the 5th and 9th bits); when I take the counter values from the on-die ROM and run the LFSR until the result is '100000000', the cycle counts (for NTSC) match once multiplied by 2. --[Quietust](User_Quietust.xhtml "User:Quietust") 05:00, 23 January 2011 (UTC) 

I suspect there's no "address increment", but _another_ shift register. Just pay attention to the address wrap - if we had a counter, it would wrap to zero, but instead, it wraps to $8000, like ($10000 >> 1). \--[Zepper](User_Zepper.xhtml "User:Zepper") 01:49, 6 June 2011 (UTC) 

    The generated addresses are linear, and I don't think a shift register can do that. So I'm conjecturing an up-counter here. --[Tepples](User_Tepples.xhtml "User:Tepples") 02:30, 6 June 2011 (UTC) 

    The DPCM address register is only 15 bits wide (with A15 being connected to the VCC rail @ 2850,5450), and the upper bit (A14) gets initialized to 1 (@ 2565,5365) instead of the last value written to $4012 (and the bottom 6 bits get initialized to 0, but we already know this). --[Quietust](User_Quietust.xhtml "User:Quietust") 03:03, 6 June 2011 (UTC)

## Contents

  * 1 DMC find
  * 2 PAL frequency table
  * 3 Pitch table
  * 4 Diagrams
  * 5 DPCM as alternate name



## DMC find

Looks like the DMC silent flag makes difference for $4011 raw output. If this flag is non-zero, the $4011 value written is ignored by the raw output _only_. I could test a few NSFs and it works flawlessly. --[Zepper](User_Zepper.xhtml "User:Zepper") 14:53, 14 November 2011 (UTC) 

    By "the DMC silent flag" do you mean bit 4 of $4015? And what exactly do you mean by "ignored by the raw output only"? Do you mean that values are queued up and take effect after the sample period ends? --[Tepples](User_Tepples.xhtml "User:Tepples") 15:10, 14 November 2011 (UTC)

Well, the silent flag is described in the APU DMC section, "_The output unit continuously outputs a 7-bit value to the mixer. It contains an 8-bit right shift register, a bits-remaining counter, a 7-bit delta-counter, and**a silence flag**_ ". Just read the rest of the APU DMC output unit. About the _raw output_ , I mean the timed writes to $4011 register, as listen in Battletoads (drums) for example. --[Zepper](User_Zepper.xhtml "User:Zepper") 15:31, 14 November 2011 (UTC) 

    So as I understand it, you're saying that in effect, $4011 writes don't take effect if a sample is not playing. Interesting; is there a test ROM for me to run on my PowerPak? --[Tepples](User_Tepples.xhtml "User:Tepples") 17:20, 14 November 2011 (UTC) 

    Could you point out the location (within the [Visual 2A03](http://www.qmtpro.com/~nes/chipimages/visual2a03/expert.html)) of the hardware responsible for this hypothetical behavior? Because if it's there, I can't see it - the internal signal for "write $4011" comes solely from the address+R/W+CLK lines, and when it's active, it _immediately_ updates the position register (of which only the upper 6 bits are a "delta-counter" \- the bottom-most bit is just a simple latch) to match the state of D0-D6. An example, for D6: during a write to $4011, t13375 activates and connects node 13255 to node 10616 (D6), then that updates t13885 and causes node 10957 to be set to the inverse of node 13255, then that updates t13872 and causes node 10881 (PCM_OUT6) to be set to the inverse of node 10957, which works out to be the value of D6. --[Quietust](User_Quietust.xhtml "User:Quietust") 22:55, 14 November 2011 (UTC)

Thanks for verifying it. I suspected it's empirical. It works only when **emulating** $4011, as such behavior do not exist in the hardware. In short words, it's a trick that can be used for emulation only, avoiding sound pops. Sorry for the inconvenience, if any. --[Zepper](User_Zepper.xhtml "User:Zepper") 00:02, 15 November 2011 (UTC) 

    I mentioned something similar in the [enhancement](Enhancement.xhtml "Enhancement") article. But there's still one problem: What happens when the DMC tries to write back to the position register on the same cycle as a $4011 write? --[Tepples](User_Tepples.xhtml "User:Tepples") 00:23, 15 November 2011 (UTC) 

    Most of the APU "runs" when M2 is low, while writes to registers happen when M2 is high. The logic that updates the DMC position register appears to be an exception, though - I'm not sure what will happen if a $4011 write happens at the same time as an increment/decrement, but I'm guessing the resulting state would be (position +/- 1) AND (data bits). --[Quietust](User_Quietust.xhtml "User:Quietust") 00:45, 15 November 2011 (UTC)

Well, it's the _$4011 position register_ value (7 bit delta-counter) being treated as wave data (raw) too. Since the _silence flag_ is not zero, the value is ignored as wave data, working as it should be. --[Zepper](User_Zepper.xhtml "User:Zepper") 02:24, 15 November 2011 (UTC) 

## PAL frequency table

Is this table correct? There are two entries I find really strange, for reasons which I outlined here: <http://forums.nesdev.org/viewtopic.php?p=94079#p94079>

The two values in question: 
    
    
    $4 : $114 = 276
    $C : $062 = 98
    

I think these should have been: 
    
    
    $4 : $10A = 266
    $C : $064 = 100
    

I don't have a PAL NES to test, but can someone confirm if the table in this article is correct or not? The NTSC table represents a scheme that consistently uses the best match to a tone in an A440 tuning. The PAL table does the same thing except these two pitches which are significantly detuned. Either our table is wrong, or the chip was designed "wrong", and I'm open to either possibility, but as I said I cannot verify for myself. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 22:50, 18 May 2012 (PDT) 

    With some help from jrlepage, who has a PAL NES, I have verified that the two frequencies are indeed 276 and 98. I guess they implemented the chip incorrectly. Weird. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 17:30, 25 July 2012 (PDT)

    Just for reference, the table I verified is: 398, 354, 316, 298, 276, 236, 210, 198, 176, 148, 132, 118, 98, 78, 66, 50
    \- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") 17:42, 25 July 2012 (PDT)

## Pitch table

I removed the pitch names from the table. 

The "pitch table" was really a sample rate table. In the table, the frequencies in Hz are sample points per second, or deltas per second. (When talking about pitch, frequencies in Hz are wave periods per second.) In other words, having pitch names in the table isn't useful. It's as useless as saying CD audio that plays at 44100 samples per second is pitch F11 - 23 cents. 

If I've made a mistake, feel free to revert. --[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 20:53, 28 December 2013 (MST) 

    The important part of the table is _relative_ pitch. Maybe my table at [User:Lidnariq/DPCM_mistuning](User_Lidnariq_DPCM_mistuning.xhtml "User:Lidnariq/DPCM mistuning") would be a better presentation of it, I dunno. While it's definitely not useful to say that "4181.71 Hz is C-8 -1.78c", it is useful to point out that "4709.93 Hz is to 4181.71Hz as C-8 -1.78c is to D-8 +4.16c". —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 21:56, 28 December 2013 (MST)

    (edit conflict:) I reverted the change. After thinking about it, the table is trying to show the rates were chosen to get particular intervals or a particular scale (not specific pitches). I'm still not comfortable with the absolute pitches in the table (C8 to C11 is at the upper end to above human hearing), but I think I'll leave it there and play around with alternatives before changing it again. --[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 22:00, 28 December 2013 (MST)

    

    It's not as irrelevant as it may seem. The organization of the DPCM samplerates clearly demonstrates that they were designed to be tuned to a scale. (See [this thread](http://forums.nesdev.org/viewtopic.php?f=6&t=5473).) They happen to be nearly useless for this purpose, especially because of the +1 modifier on the sample length that looks like an accident of the implementation, but the design intent is clear. I agree that it's unfortunately misleading to label them with pitches, but I can't think of a simpler way to express their tuning relationships to each other. You can have a matrix with cents and relative intervals, maybe, but I think using letter pitch names gets the idea across quicker and more easily. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 02:05, 29 December 2013 (MST)

    

    As an aside, I've found a use for these pitches in the past when wanting to play a DPCM ramp to adjust triangle volume. By knowing the pitch of the samplerate, I can tune the ramp to a pitch that will be hidden by the existing harmony of the music. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 02:06, 29 December 2013 (MST)

## Diagrams

The part about using DMC for syncing badly needs visual representation of what's going on. Describing it in pure text form is by essence incomplete.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 05:54, 21 September 2018 (MDT) 

## DPCM as alternate name

The fact this sound channel is also known as "DPCM" should be mentionned, and appropriate page redirection should be introduced.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 13:39, 29 April 2019 (MDT) 
