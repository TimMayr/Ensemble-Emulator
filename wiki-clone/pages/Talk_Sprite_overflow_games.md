# Talk:Sprite overflow games

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ASprite_overflow_games) | View [other pages](Special_AllPages.xhtml#Talk_Sprite_overflow_games)

## Splitting Commercial Games

I have a question, Is it better to split the Commercial Games between Official and Unofficial Releases? (Example: Bee 52 is an Unlicenced cartridge, so this goes under Unofficial) --[Hamtaro126](User_Hamtaro126.xhtml "User:Hamtaro126") ([talk](User_talk_Hamtaro126.xhtml "User talk:Hamtaro126")) 14:38, 11 November 2014 (MST) 

    I might consider it once there is more than one unlicensed game in the list. Having more than one unlicensed game in the list may help toward an explanation of how unlicensed games on the whole use it more, less, or in a different manner compared to licensed games. But having only one doesn't accomplish much other than shaming Codemasters for not having the Seal. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 15:19, 12 November 2014 (MST)

## Zelda 1?

The page claims that Zelda 1 uses this trick when Link is entering a vertical doorway. I took this to refer to the effect where Link appears to be going down stairs, such as when he enters the cave on the first screen of the game. But if I enable more than 8 sprites per scanline in FCEUX or Nestopia, nothing changes; Link still sinks behind the background like he's supposed to, suggesting this is done by some other means such as flipping his priority bit. Am I mistaken in understanding what the page is referring to, or is the page itself mistaken? - [Furrykef](User_Furrykef.xhtml "User:Furrykef") ([talk](https://www.nesdev.org/w/index.php?title=User_talk:Furrykef&action=edit&redlink=1 "User talk:Furrykef \(page does not exist\)")) 21:37, 24 October 2015 (MDT) 

    (Note: According to the page, the technique in question is to "intentionally place multiple blank sprites early in the OAM at the same Y position so that other sprites on those scanlines are hidden." This is a different thing than "flipping the priority bit". See [PPU sprite priority](PPU_sprite_priority.xhtml "PPU sprite priority").)

    [I made a previous statement here about the "more than 8 sprites per scanline" setting, but I think I misunderstood the setting and removed my comments.]

    It looks like the page is talking about **dungeon** doors (not overworld doors) on the top and bottom of the screen. When the "more than 8 sprites per scanline" setting is on, Link walks over the wall instead of under it. I clarified the description on the page. --[Bavi H](User_Bavi_H.xhtml "User:Bavi H") ([talk](User_talk_Bavi_H.xhtml "User talk:Bavi H")) 05:26, 25 October 2015 (MDT)

## Ninja Gaiden Games - 8 sprite masking notes

Ninja Gaiden 1,2,3 sprite masking with 8 sprites: 

Always done to keep sprites inside the black borders. Other masking effects not listed here are usually done with priority (e.g. Ryu falling through trap door in Ninja Gaiden 1). 

Ninja Gaiden Intro \- Zoomed in faces running (palette swaps of Ryu for his Father and Malth) Between Act 2/3, \- Ryu's face bouncing up and down when running \- The ninja stealing the statue Between Act 3/4, \- Ryu's face bouncing up and down when running is masked \- Same as above, but not running (in Foster's office) \- Turned on during the parachute scene. Not sure if this affects anything Between 6.3 and 6.4 \- Same Ryu face zoom when talking to his father 

Ninja Gaiden 2 \- Intro Ryu's sideways face \- In between Act 1 and 2, same face \- In between Act 2 and 3, same face \- In between Act 3 and 4, same face \- In between Act 5.1 and 5.2, same face \- In between Act 5.2 and 5.3, same face \- In between Act 5.2 and 5.3, lower part of Robert's body \- In between Act 5 and 6, same face \- In between Act 6 and 7, lower part of Robert's body \- In between Act 6 and 7, same face \- In between Act 7.1 and 7.2, same face \- In between Act 7.2 and 7.3, same face \- In between Act 7.3 and 7.4, same face \- In the ending, some of the reused scenes use 8 sprite masks here, where they used masking that was done with priority before. 

Ninja Gaiden 3 \- Intro Ninja jumping \- Intro Sideways Ninja face \- Different face between Acts 3 and 4 \- Jumping ninjas between Acts 3 and 4 \- Jumping ninjas between Acts 5 and 6 \- Irene between Acts 5.2 and 5.3 \- Mutated bionoid between Acts 5.2 and 5.3 \- Teleporting effects between Acts 5 and 6 \- Between 6 and 7 is different. Doesn't use the same tile to block Ryu's legs \- Between 6 and 7 blocks explosion \- Between 6 and 7 blocks Ryu's head when panning down \- Between 7.3 and 7.4 - blocks lower part of Ryu. Doesn't use 8 copies of the same tile. \- Ending reused Sideways face \- Ending reused mutated bionoid. Doesn't use 8 copies of the same tile. 

\--This list seems too detailed for the main page. It's simpler just to say all sprites should be confined inside the black borders. 
