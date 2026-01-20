# Talk:Namco 163 audio

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANamco_163_audio) | View [other pages](Special_AllPages.xhtml#Talk_Namco_163_audio)

## Contents

  * 1 Point of terminology: Namco 106 vs. Namco 163
  * 2 Mechanism
  * 3 Mixing noise approximation
  * 4 Found resistor
  * 5 N163 incorrect terminology?



## Point of terminology: Namco 106 vs. Namco 163

A regular on another wiki, who might not qualify for trusted on wiki.nesdev.org because he isn't active on [the BBS](http://nesdev.org/bbs/), sent me a [message on my talk page at that wiki](http://pineight.com/mw/?title=User_talk:Tepples&oldid=2041#Nesdev_move_request) that [Namco 106 audio](Namco_163_audio.xhtml "Namco 106 audio") might need a rename. --[Tepples](User_Tepples.xhtml "User:Tepples") 14:52, 15 December 2010 (UTC) 

## Mechanism

Is the cycle-by-cycle behavior known? I can guess it's something like "ld_7f ld_8 add_9 st_9 ld_a add_b st_b ld+mask_c add_d bound_c st_d add_e ld_indirect mult_f st_dac"—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:41, 11 February 2013 (MST) 

    I think that would require a decap; it would be hard to determine this externally. The pseudocode I put as an example seems to give very accurate results though. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:16, 11 February 2013 (MST)

    

    Should be able to figure out which cycle 9, b, and d are updated. But otherwise, yeah.—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:45, 11 February 2013 (MST)

    

    

    How do you intend to do that? They're write-only registers. You can sort of read them by setting the waveform position to sit on the registers and recording the audio out (this is how the contents were determined), but the output only changes every 15 cycles, so there's no way I can think of to see anything getting updated at a finer granularity than that. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 02:05, 12 February 2013 (MST)

    

    

    

    The same way blargg's tests test the timing of write-only PPU registers: very precisely timed writes. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 09:50, 12 February 2013 (MST)

    

    

    

    

    Hmm, so you're saying to devise a test for each of the control bytes, to be executed at each of the 15 cycle alignments, and then deduce from the signal collected what happened in each of these tests? I suppose you could mark your starting alignment with a $4011 write as well. Anyhow... I can see how it would be possible in theory. The real question I suppose is whether anyone will do it? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:31, 12 February 2013 (MST)

    

    

    

    I'm not convinced they're write-only. Certainly the interface as a whole isn't (e.g. games use it as save memory). In order for the control registers to be write-only, they'd have to be shadowed from the 128b actually presented...—[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:21, 12 February 2013 (MST)

    

    

    

    

    Ah, yeah I hadn't thought about how they are down in the $4000 range. Reading might be possible then. Well, if you test it out, let me know what you find. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:31, 12 February 2013 (MST)

## Mixing noise approximation

A reasonable approximation of the switching noise, and cheaper than full resampling, is to sum the channels as though there were no switching noise, resample to the soundcard's native rate, and then multiply the output by `(1+sin(k·n))`, where k is chosen to produce a sine wave at the switching rate. (In the case of rendering audio at a rate higher than 48kHz, you may also add sine waves at every multiple of the switching rate that's below the nyquist rate. But none of those should be audible) 

Justification: if the sound through each channel is uncorrelated, then it can be treated as though each channel is sounding while all others are silent. In which case, this is standard DSP [upsampling](https://en.wikipedia.org/wiki/Upsampling "wikipedia:Upsampling"), which produces modulated copies of the baseband signal at every multiple of the switching rate. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 15:30, 17 July 2013 (MDT) 

    Honestly, I don't think most people will want to listen to the switching noise, so a performance-saving approximation of it probably isn't that desirable. People who want the switching noise are probably interested in accuracy at the expense of performance. Though, in my NSFPlay implementation, it seems that it's not really a performance problem either way, emulating and mixing the N163 is faster/simpler than most of the other chips. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 12:11, 18 July 2013 (MDT)

## Found resistor

In their current state, these seem a bit misleading. The values for Rolling Thunder and King of Kings seem especially dubious if there is a direct relationship between the resistance values found and the output level. Is there only one resistor involved? (What is the mixing circuit like? Shouldn't the N163 output have its own resistor?) I notice the King of Kings board on Bootgod has 3 surface mount resistors on it... are we sure we have the right one here? 

If all the values are correct, then we need to consider whether other factors affect the mix and include them (otherwise the resistance alone is useless). Either this or the carts observed are very inconsistent (I don't want to presume this unless we can be sure). If they are incorrect, we need to fix them before they can be of use. 

I like the idea that they can add useful information, but I kinda think we should remove it for now until it can be verified better. 

4-Channel Game | Volume | Samples | Resistor on cart   
---|---|---|---  
Final Lap | 3.6x | 1 | 4.7k or 15k   
Mappy Kids | ? | 0 | 10k   
Megami Tensei II | 4.0x | 1 | 4.7k   
Namco Classic 2 | ? | 0 | ?   
Rolling Thunder | 6.5x | 2 | 22k   
Sangokushi | ? | 0 | 4.7k   
Sangokushi 2 | ? | 0 | ?   
8-Channel Game | Volume | Samples |   
Erika to Satoru no Yumebouken | 8.5x | 1 | 10k   
King of Kings | 7.3x | 1 | 4.7k   
  
\-- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:34, 18 July 2013 (MDT) 

    Wow, Famicom carts are not easy to open. My Rolling Thunder has a 22k resistor connected to pin 45. My Erika to Satoru has a 10k resistor connected to pin 45. In my tests of these carts, Erika is definitely ~25% louder than Rolling Thunder (N163 relative to 2A03), so I don't think resistance alone is going to be a good enough indicator. I'll take some board pictures if you want to study them. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 14:55, 18 July 2013 (MDT)

    Boards of Erika and Rolling Thunder: 

  * <https://dl.dropboxusercontent.com/u/883356/n163_erika.jpg>
  * <https://dl.dropboxusercontent.com/u/883356/n163_rolling_thunder.jpg>


    -[Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 15:18, 18 July 2013 (MDT)

    

    I found the information on a 2chan thread and copied it to [iNES Mapper 210](INES_Mapper_210.xhtml "INES Mapper 210") (with citation). The mixing circuit on N163 carts seems to always be:
    
    
     AudioFrom2A03 --- Resistor ---+--- AudioToRFModulator
                                   |
                              AudioFrom163
    

    

    So there's got to be some intrinsic resistance on the 163's output to mix properly. I'm not clear how to resolve the resistors with the volumes you found either; it should be a reciprocal function of the resistance. There's also the photos on NesCartDB of the boards, so that you don't need to open them (Or, since all the games seem to use the above circuit, you should be able to just measure the resistance using an ohmmeter without opening the cartridge)

    

    But in NesCartDB, his picture of Rolling Thunder shows a 15kΩ resistor instead. And there's other evidence that different production runs of the same game may well have different resistors, and so different volumes.

    

    It really seems like your musing on a heuristic is a better approach. I'm mostly trying to point out that the mixes are not even fixed per game —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 16:32, 18 July 2013 (MDT)

    

    

    If Rolling Thunder uses 22k and Erika to Satoru no Yumebouken uses 10k, it's clear that there's another factor here. Presumably different revisions of the N163 have different internal resistance? If we can't measure that, or identify N163 revisions (good luck with these globs), I don't imagine we can really glean anything useful from just the 2A03 mix resistor. As far as games being inconsistent, I do expect changes with different runs, but I wouldn't assume any wide variation in the volume unless we can actually measure it. It seems plausible that we could find 2 runs of Rolling Thunder that have different N163 revisions, and different resistors that compensate for that, keeping the volume consistent? It's equally plausible that lack of care with this could lead to wide variation, but I don't want to assume that until it's actually found in the field. The Rolling Thunder on BootGod looks like mine except it's missing the cap (to ground?) on the audio-- are you sure it says 15k? there don't seem to be enough pixels to read it. I measured a recording of a second Rolling Thunder cart taken by jrlepage (this is what is meant by the Samples column), and it was really close (~6.4x vs ~6.6x). Maybe I can put a call out for recordings on the forums. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 18:16, 18 July 2013 (MDT)

    

    

    

    If you're logged into NEScartDB, you can click on the image to get an original-resolution copy. (Things I didn't know until he told me). ... of course, now I notice that that's actually the back for Final Lap...
    Anyway, as far as we know there's only one revision of the 163; certainly all the QFP versions are labelled the same. I guess who knows what's going on under the epoxy? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 23:14, 18 July 2013 (MDT)

    

    

    

    

    Yes, I know about the original resolution images, just the one for Rolling Thunder is unreadable even at the original resolution (unless there's more than one board image available?). Anyhow, I put up a survey request on the forum. Hopefully I can compile some useful information from it. Also, I did test pins 45-46 with an ohmmeter to verify that works, and I got 22.2k and 9.9k for my two carts, so that's good to know of an easier way to verify than opening it up and reading the resistors. I damaged my Erika cart's casing trying to open it, there was a centre pin that had a death grip on the front of the cart. I'm currently gluing it back together! - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 23:39, 18 July 2013 (MDT)
    [http://forums.nesdev.org/viewtopic.php?f=3&t=10294](http://forums.nesdev.org/viewtopic.php?f=3&t=10294)

    

    

    

    

    

    This is entirely academic, but to make sure we're looking at the same thing: <http://bootgod.dyndns.org:7777/profile.php?id=3339> then image.php?ImageID=4097 then getoriginalimg.php?ImageID=4097 ... I see two resistors, labelled 473 and 153 (or 47k and 15k) —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 02:16, 19 July 2013 (MDT)

    

    

    

    

    

    

    Okay, that's weird. I would go to the profile, and then click the image (going to image.php), which is what I had presumed was the original size of the image, and that's what I was looking at [(looks like this, below is my own picture scaled to similar size)](https://dl.dropboxusercontent.com/u/883356/compare_rolling_thunder.png). How do you get to the "getoriginalimg.php" from there? I tried typing it in the link and I had to log in, but even after logging in I can't find anything to click on to get from image.php to getoriginalimg.php. Typing it in manually downloads a file that is named Final Lap, though, so... that's strange, but at least I can see that it's 15k now! - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:26, 19 July 2013 (MDT)

    

    

    

    

    

    

    

    Once you're looking at the 600px image, if you're logged in, you click on that image _again_ to get to getoriginalimg.php —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 12:47, 19 July 2013 (MDT)

    

    

    

    

    

    

    

    

    Hmm, I dunno why that doesn't work for me, but at least I can type in the URL now that I know about it! - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 17:55, 19 July 2013 (MDT)

## N163 incorrect terminology?

Wondering what the rationale is for calling N163 incorrect terminology. I mean, the chip appears to just have the number 163 on it, but N163 is just a straightforward abbreviation of Namco 163 (or Namcot 163). It's a popular expansion chip and lots of Famitracker users I talk to us the term N163. It's perfectly clear to me why the number 106 was wrong, but I don't understand the new prescription to eliminate N163. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 17:31, 8 April 2014 (MDT) 

    That'd be news to the MAME developers who have been calling the combination of a 6502 without decimal mode and the NES APU the "N2A03". --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 19:55, 8 April 2014 (MDT)
