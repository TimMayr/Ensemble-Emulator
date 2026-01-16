# Talk:APU Noise

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AAPU_Noise) | View [other pages](Special_AllPages.xhtml#Talk_APU_Noise)

Curiously, the noise channel's frequency counter [on the die](http://uxul.org/~noname/chips/cpu-2/no-metal/stitched/final/) isn't an ordinary counter - it's an **11-bit linear feedback shift register** (with taps at the 9th and 11th bits); when I take the counter values from the on-die ROM and run the LFSR until the result is '10000000000', the cycle counts (for NTSC) match once multiplied by 2 (note that this means the PAL frequency for $1 is almost definitely wrong). --[Quietust](User_Quietust.xhtml "User:Quietust") 04:58, 23 January 2011 (UTC) 

    ...and it's been discovered that the "letterless" RP2A03 only had a **10-bit** LFSR (with taps at the 7th and 10th bits) for its frequency counter, with the longest period only being 2046 cycles. I was only able to read the odd columns from the frequency table, but aside from the last one (for $F), they all seem to produce the same frequencies as in the 2A03G. --[Quietust](User_Quietust.xhtml "User:Quietust") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Quietust&action=edit&redlink=1 "User talk:Quietust \(page does not exist\)")) 15:01, 1 April 2025 (UTC)

"randomly 93 or 31 steps long otherwise. (The particular 31- or 93-step sequence depends on where in the 32767-step sequence the shift register was when Mode flag was set" \- seems worth mentioning that, while random, there is only one 31 step pattern [1101000100101011000011100110111], and it occurs 1/1057 of the time. This compares to the 352 different 93 step patterns that occur the other 1056/1057 of the time. --GreyRogue 

    From a musical point of view, the problem is that the harmonic content of the waveform varies widely, with no ability to control or even appreciate correlations between the harmonics, and the question whether the specific sequence has 31 or 93 steps is irrelevant.. Far more relevant is the relative rarity of the LFSR waveforms that have a loud 31st harmonic.
    From an emulation accuracy point of view, the relative probability seems like a distraction. What is the purpose of mentioning this?
    Here's the relevant thread that resulted in me adding that to the wiki: <https://forums.nesdev.org/viewtopic.php?t=11535> —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 00:23, 18 April 2021 (MDT) 

    Having read that thread, I agree with everything stated. I hadn't thought of the various harmonics being so noticeably loud depending on which of the 93-step patterns was current. Can this link be added to the page in a References section? -- GreyRogue
