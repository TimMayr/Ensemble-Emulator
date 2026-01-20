# Color $0D games

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Color_%240D_games) | View [other pages](Special_AllPages.xhtml#Color__0D_games)

On an NES, the palette color $0D causes the signal to drop below the normal black level. This low voltage signal is sometimes mistaken by televisions for blanking signals, which can cause an unstable picture, or total picture loss on some devices. Other devices seem to process with signal without problem. 

## Contents

  * 1 Games
  * 2 Effects
  * 3 Workarounds
  * 4 Tests
  * 5 See also



## Games

Game  | Notes   
---|---  
_Contra 100 in 1_ | Uses both for background color and one of the sprites' color; to apply a patch, change values at those offsets in ROM from $0D to $0F: $388B, $388F, $3893, $3897, $389B, $389C, $389F, $38A3, $38A7   
_Contra 168 in 1_ | Uses both for background color and one of the sprites' color; to apply a patch, change values at those offsets in ROM from $0D to $0F: $2451, $2455, $2459, $245D, $2461, $2462, 2465, $2469, $246D   
_Bee 52_ |   
_Castelian_ |   
_Cybernoid_ | $0D is used as the background color.   
_The Fantastic Adventures of Dizzy_ |   
_FireHawk_ (USA) | Used in both the background and sprites. The European release (with NTSC video detection) corrects this.   
_Game Genie_ | The code entry screen uses it for its background.   
_The Immortal_ | Also uses all three de-emphasis bits to compensate for the the user cranking up the TV set's brightness so that regular black ($xE/$xF) can be used [as a first darker shade of gray, and color $1D as an even darker shadw of gray, while color $0D is used as a black background color](https://youtu.be/IMFoci7S2hs).   
_Indiana Jones and the Last Crusade_ (Taito) | Used as the background color in the motorcycle level.   
_Indiana Jones and the Last Crusade_ (Ubisoft) |   
_Maniac Mansion_ (USA) |   
_Micro Machines_ |   
_MIG-29 Soviet Fighter_ |   
_Quattro Sports_ |   
_Quattro Adventures_ |   
_Skate or Die 2_ | Used as the background color during the introduction cutscene sequence.   
_The Super Shinobi_ | Unlicensed clone of Shinobi III.   
_Teenage Mutant Ninja Turtles_ | Uses it for black outlines on sprites, the lack of large areas of this color mitigates the problem.   
_The Three Stooges_ | Uses $xD colors that are turned to $0D during fades.   
  
## Effects

[![](../wiki-images/Color_0d_-_contra_168.jpg)](File_Color_0d___contra_168_jpg.xhtml)

[](File_Color_0d___contra_168_jpg.xhtml "Enlarge")

Modern LCD TV and modern famiclone (probably RS-35) showing distorded image in _Contra 168-in-1_ cartridge

[![](../wiki-images/Color_0d_-_super_robin_hood.jpg)](File_Color_0d___super_robin_hood_jpg.xhtml)

[](File_Color_0d___super_robin_hood_jpg.xhtml "Enlarge")

Different modern LCD TV with digital famiclone (connected via HDMI) shows $0D color to be displayed as gray in _Super Robin Hood_ game

[![](../wiki-images/Color_0d_-_fantblue.gif)](File_Color_0d___fantblue_gif.xhtml)

Modern 29" CRT TV with IQ502 rev 2 (UM6561 nes-on-chip) famiclone shows periodic loss of v-sync when the paper unrolls at the beginning of _Fantastic Adventures of Dizzy_ game. Note that this TV has no problems with displaying $0D colors with different famiclones, and even using this famiclone - this is the only moment in this game that the problem manifests, though this game uses this color all the time

[![](../wiki-images/Color_0d_-_ultimate_stuntman.gif)](File_Color_0d___ultimate_stuntman_gif.xhtml)

Same TV and console as above, No problems in the beginning screenshots, where $0d is used as universal background colors, while permanent v-sync loss during game, where $0d is used as blinking car's shadow color in _Ultimate Stuntman_ game.

  
Because the signal created by $0D is outside the specifications for the video signal, there is a lot of variation in how display devices handle it. Here are some possible effects that may be seen when using $0D: 

  * $0D appears the same black as the other black colors (e.g. $0F).
  * $0D appears slightly darker than other blacks.
  * $0D appears as gray.
  * The device renormalizes the range when $0D appears, slightly brightening all other colours while it is onscreen.
  * Wobbly or distorted image from loss of horizontal blanking stability (either permanent or periodic)
  * Loss of vertical blanking stability.
  * Total loss of picture.



These effects are more likely to occur when color $0D is used with the de-emphasis bits enabled, such as in _The Immortal_ , as seen in these [example](https://youtu.be/3fhyX3HdVcg) [videos](https://youtu.be/4Kli09cSGOU). 

On consoles with an RGB PPU like the Sharp Famicom Titler or C1 TV, Color $0D is simply a "normal" black palette entry identical to $0F, so systems with an RGB PPU are immune to causing video output problems. 

## Workarounds

  * Patch the game code to replace all the $0D writes with $0F. This likely requires changes to many bytes in the ROM, so a Game Genie (limited to 3 changes) would not be sufficient. Instead, updated ROMs or a more capable pass-through patching device would likely be required
  * If your TV has digital inputs (for example - HDMI), use RCA to HDMI adapter, whose analog to digital converter might cope better with the out-of-spec video signal
  * Modify the console to use a different video output stage that adjusts the voltages so that the TV may accept it, such as by omitting the NPN transistor follower part of the video amplifier. However, the TV may ignore this voltage offset
  * Change your TV, console and/or video adapter (if you are using one)



## Tests

  * [Palette test ROM](http://forums.nesdev.org/viewtopic.php?f=3&t=13264) \- Displays NES palette, and can toggle $0D display.
  * [NESPix](http://forums.nesdev.org/viewtopic.php?f=22&t=11255) \- Native graphics editor that allows use of $0D, and can test it in various visual arrangements.
  * [240p test suite](http://forums.nesdev.org/viewtopic.php?f=22&t=13394) \- TV testing program. Test cards with $0D include PLUGE, SMPTE color bars, Solid color screen, and IRE. PLUGE also includes emphasized $0D.



## See also

  * [NTSC video](NTSC_video.xhtml "NTSC video")
  * [PPU palettes](PPU_palettes.xhtml "PPU palettes")
  * [Colour-emphasis games](Colour_emphasis_games.xhtml "Colour-emphasis games")


