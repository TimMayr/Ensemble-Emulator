# Talk:PPU sprite evaluation

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3APPU_sprite_evaluation) | View [other pages](Special_AllPages.xhtml#Talk_PPU_sprite_evaluation)

## Sprite zero with sprites disabled

The article says: "According to the blargg's sprite zero hit tests, sprites are NOT evaluated in the pre-render scanline. Plus, the evaluation occurs if sprites OR background are enabled."

I find the second sentence problematic. I took it to mean that the sprite zero flag will still be set even if sprites are disabled in PPUMASK. But in fact it will not. I was pretty annoyed to find that out the hard way! So what is this trying to say, then? - [Furrykef](User_Furrykef.xhtml "User:Furrykef") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Furrykef&action=edit&redlink=1 "User talk:Furrykef \(page does not exist\)")) 01:10, 30 August 2016 (MDT) 

    I think this is worded very confusingly. Sprite "evaluation" is a vague and unclear term, especially since you've determined that sprite 0 hit won't be caused when sprites are disabled. I think it means that the OAM is accessed and refreshed as long as either the sprites or background are enabled. This is important for OAM decay, at least, but I don't know about other factors. Does sprite overflow still happen is sprites are disabled, but the background is? (Is that what "sprite evaluation" means?) Does sprite 0 hit still happen? (No. Right?) I think the whole sentence is a bit worthless as-is, to be honest. I think I'll delete it. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 01:36, 30 August 2016 (MDT)

    

    Man, you must be kidding now. It's not my fault if "sprite evaluation" is a vague term to you, sorry. Anyway, the sprite zero test FAILS (plain and simple) if the PPU sprite evaluation starts at scanline -1 (pre-render scanline). It should start at scanline 0, the first visible scanline. --[Zepper](User_Zepper.xhtml "User:Zepper") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Zepper&action=edit&redlink=1 "User talk:Zepper \(page does not exist\)")) 19:39, 5 September 2016 (MDT)
    <http://wiki.nesdev.org/w/index.php/PPU_sprite_evaluation>

Sprite evaluation *is* vague. It could mean fetch OAM data into temporary OAM or fetch data from VRAM based on temporary OAM data, and also the setting of sprite #0 and overflow flags. 3 different things. Also if might be mistaken, but sprite zero hit won't work either if sprites are enabled but not BG. You rally need both enabled for it to work. Disabling only sprites or only BG just replace them by blank patterns insternally, but still fetches the pattern in VRAM and internally replace them with all 0s, I guess. (Bregalad) ~~128.178.126.68~~ 07:21, 6 September 2016 (MDT) 

    I added a brief definition at the top of the page, and I tried to break up the removed sentence into a couple of ideas that I hope are more clear. ([revision 12889](http://wiki.nesdev.org/w/index.php?title=PPU_sprite_evaluation&diff=12889&oldid=12382)) - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 11:27, 6 September 2016 (MDT)

    

    Cool. Again, let me rephrase it. If the emulator (PPU) evaluates sprites at the pre-render scanline (-1), the sprite zero hit test will give an error. Simple. People that don't understand the PPU sprite evaluation proccess should read about it more and more, ever and ever... until it's clear. Crying here isn't good.--[Zepper](User_Zepper.xhtml "User:Zepper") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Zepper&action=edit&redlink=1 "User talk:Zepper \(page does not exist\)")) 15:38, 6 September 2016 (MDT)

    

    

    I reverted [this change](http://wiki.nesdev.org/w/index.php?title=PPU_sprite_evaluation&diff=12898&oldid=12897) because the difference between sprite 0 hit and sprite evaluation was _precisely_ what was causing confusion. You removed my statement clarifying that, and then added something that says evaluation will cause sprite 0 test failure. This is not good! Sprite 0 hit does not happen in sprite evaluation! That's the whole problem here! - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 16:13, 6 September 2016 (MDT)

    

    

    I sort of understand your statement as "I implemented evaluation a line early by mistake, and it caused _blargg's sprite 0 test ROM_ to fail (whatever that test is, you still need to clarify this), but if you really want to include that information you have to be more explicit about this. I don't really think it's a good idea to try to enumerate all the ways people might get emulation wrong (there are endless ways). The lack of evaluation on line 0 is clearly mentioned, is it not? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 16:15, 6 September 2016 (MDT)

    

    

    

    "I implemented evaluation a line early by mistake, and it caused _blargg's sprite 0 test ROM_ to fail". **Correct**. Actually, it's a hint _JUST IN CASE_ someone is getting errors in the sprite #0 hit test ROM. Well, the lack of evaluation on line 0 was **NOT** clear, I asked a few times in the forum without success... until I got the sprite #0 test error and located the problem in my emulator. You see, it's a hint, not a rule. And I'm sorry for the trouble and language barrier, but I must discuss things at anyway.--[Zepper](User_Zepper.xhtml "User:Zepper") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Zepper&action=edit&redlink=1 "User talk:Zepper \(page does not exist\)")) 19:04, 6 September 2016 (MDT)

## Note about sprite address not zero

    _If the sprite address ($2003) is not zero (usually, $2003 AND $F8) at the beginning of the pre-render scanline, then copy the contents of its 8-byte page into the first 8 bytes._

What does this mean? Is there a forum thread where this was discussed? Does this cause sprites to be rendered on scanline 0? - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 16:34, 6 September 2016 (MDT) 

    

    I found some relevant forum posts explaining it. What was very unclear to me was where these bytes were copied to. It sounded like it was to affect PPU sprite evaluation (because it's on the PPU sprite evaluation page), but it's actually a corruption of OAM that happens before evaluation begins. I rewrote the line hopefully making this clearer, and added links to the forum posts explaining it. - [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 16:56, 6 September 2016 (MDT)
