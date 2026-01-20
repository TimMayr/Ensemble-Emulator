# Talk:NSFe

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ANSFe) | View [other pages](Special_AllPages.xhtml#Talk_NSFe)

## chunk suggestions

Suggestion for "auth" chunk: Add extra fields for composer of original and composer of cover. Another suggestion, for "vrc7" chunk: Specify the VRC7 built-in instrument settings which this file is meant to be played (there are a few different patch sets due to inaccuracy, and some might intend certain ones to sound better as they were written). Also suggestion a "JF13" chunk, which if it exists, contains ADPCM encoded audio, which is controlled by a register at $7FFF. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 03:22, 25 January 2013 (MST) 

    Many people who make NSF covers already put both in the existing artist field, or use the Copyright field with it, often prefixing the cover artist with "cv:". Since in NSFe there is no length limit on the strings in the "auth" chunk there's plenty of room for multiple names in there already. People have suggested a lot of extra fields to add, but I personally just think this makes UI implementation for it annoying, which is why I just added a "text" field instead where any arbitrary information could be added and displayed at the author likes. Currently only NSFPlay will show it, and only in the infobox window, though I plan to make a breakout window for it so you don't have to scroll through that tiny box to see it. As for the Jaleco-JF-13 sound board, that's been kinda low priority to me for pretty obvious reasons (i.e. only used in one game, not for music, only for baseball gameplay sounds, lack of implementation details, scarcity of ROM, unrippable ROM, etc.). - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 08:05, 27 January 2013 (MST)

    

    But, what about "vrc7" chunk? Different emulators will play the built-in VRC7 patches differently, and some files which are tested with older emulators may be made to sound best with certain patch sets, which is what it is for, which is why it is lowercase. (If someone wants to use it to make up completely their own instrument sets, you could make up a uppercase "VRC7" chunk for that but then it won't be really VRC7, and it won't really be the expansion audio which is actually used with Famicom.) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 22:01, 2 February 2013 (MST)

    I never did get an explanation as to how providing the ability to customize the built-in VRC7 instruments is a good thing. Would you humor me? —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 17:36, 2 February 2013 (MST)

Furthermore, another suggestion for "mixe" chunk which tells the volume and panning of each channel, specifying the ratio of the master volume with the maximum volume that the channel can have. Another chunk is "MULT" which combines multiple files into one; each record is an address and length of the ROM data (within the "DATA" block) corresponding to each one. The "INFO" and "BANK" data are copied the same number of times each within their chunks, so if the first block specifies 5 songs and the next 2 songs, then songs 0 to 4 are using the first ROM address/data and INFO and BANK headers, while the songs 5 and 6 use the second one and subtracts 5 to write the accumulator initially. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 16:21, 2 February 2013 (MST) 

    re: "mixe" I'm a little wary of providing features that can't be implemented in hardware: as long as we assume a 2A03 as the sound generator (and if not, why does this belong in a **N** SF specification?), grouping mixing by "both square waves", "triangle, noise, and DPCM", and "expansion" would be something I could defend, but more fine-grained panning/volume control seems untrue. (I guess the sole exception would be I also see an argument for "individually pan-able and volume-control-able channels of the 5B", because we know it's identical to a YM2149F.)

    While your NSF combiner has evidently been greatly useful, I find myself a little troubled "MULT" turning NSFe into a almost-general archive format… —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 17:36, 2 February 2013 (MST)

    

    I know it cannot be implemented in standard 2A03 hardware which is why it is lowercased (my idea about the $4009 register for stereo (which does the grouping like you said) could be implemented in a hardware modification, though?); it is not required, and is supposed to play in mono as well. NSFplay can do volume/panning of individual channels in the setting menu but cannot load it from the NSFe file. --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 22:01, 2 February 2013 (MST)

I think these have come up before but here's my thoughts on those features: 

  1. I don't think custom VRC7 patches is worth doing. It has nothing to do with Famicom hardware, or any existing hardware, and I don't consider the NSF/NSFe format to be a dumping ground for fantasy hardware features.
  2. A panning/mix option is similar; has no relevance to the actual hardware, ripping games, preserving emulation, etc. the only purpose of this is for people to compose music that requires something the hardware does not have, and I don't see the point of this. Just write music for hardware that doesn't have this limitation.
  3. I also don't particularly see the advantage of packing multiple NSFs into one. It isn't relevant to ripping any game, and would make it more complicated to use with something like the PowerPak. I would recommend .zip/.rar/.7z if you need a way to put multiple NSFs in one file.



\- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 23:14, 3 February 2013 (MST) 
