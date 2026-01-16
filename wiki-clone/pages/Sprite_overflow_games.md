# Sprite overflow games

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Sprite_overflow_games) | View [other pages](Special_AllPages.xhtml#Sprite_overflow_games)

The following is a list of games which rely on putting more than 8 sprites on a scanline. 

## Contents

  * 1 Use of sprite overflow flag
    * 1.1 Commercial
    * 1.2 Homebrew
  * 2 Use of excess sprites for masking effects
    * 2.1 Commercial
    * 2.2 Homebrew
    * 2.3 Detecting masking effects
  * 3 Misc
    * 3.1 Commercial
  * 4 References



## Use of sprite overflow flag

The sprite overflow flag is rarely used, mainly due to bugs when exactly 8 sprites are present on a scanline. No games rely on the buggy behavior. See [sprite overflow bug](PPU_sprite_evaluation.xhtml#Sprite_overflow_bug "PPU sprite evaluation") for more details. 

Nonetheless, games can intentionally place 9 or more sprites in a scanline to trigger the overflow flag consistently, as long as no previous scanlines have exactly 8 sprites. 

### Commercial

  * _Bee 52_ : At the title screen, the game splits the screen with sprite overflow (at scanline 165), then splits the screen with a sprite 0 hit (at scanline 207). If sprite overflow is not emulated, the game will crash at a solid blue-purple screen.



### Homebrew

  * blargg's [sprite overflow test ROMs](https://github.com/christopherpow/nes-test-roms/tree/master/sprite_overflow_tests): tests behavior of sprite overflow, including the buggy behavior.
  * _City Trouble_ uses both the sprite overflow flag and the sprite 0 flag to make two scroll splits.



## Use of excess sprites for masking effects

Some games intentionally place multiple blank sprites early in the [OAM](PPU_OAM.xhtml "OAM") at the same Y position so that other sprites on those scanlines are hidden. 

### Commercial

  * _Castlevania II: Simon's Quest_ (a.k.a. _Dracula 2_): When Simon enters a swamp, the lower half of his body should be hidden.[1]
  * _Felix the Cat_ : When entering or exiting a bag.
  * _Gimmick!_ : When entering a level. Also used to keep extra sprites out of the status bar.
  * _Gremlins 2 - The New Batch_ : Uses multiple blank sprites to mask rows during cutscenes.
  * _Majou Densetsu II: Daimashikyou Galious_ : When entering a doorway, Popolon's body should gradually disappear (to imitate walking down stairs).[2]
  * _Ninja Gaiden 1, 2 and 3_ : All sprites in all cutscenes should be confined inside the black background borders.
  * _The Legend of Zelda_ (a.k.a. _Zelda 1_ , _Zeruda no Densetsu_): On the top or bottom of dungeon screens.



### Homebrew

  * _Lizard_ hides sprites overlapping the pause overlay while the game is paused.



### Detecting masking effects

Games will place 8 consecutive sprites with the same Y coordinate and same tile number. If you see this, then that is a sign that the game is using a masking effect, and the 8-sprite limit should be enforced for that area. 

## Misc

### Commercial

  * _Solstice_ : during the intro cutscene, there are stray sprites on the screen beyond the 8 per scanline, but the NES won't display the excess sprites. This is not a masking effect, it is merely the hardware covering up a mistake that wasn't caught by the original programmers.



## References

  1. ↑ [BBS topic](http://forums.nesdev.org/viewtopic.php?p=136304#p136304) with screenshots of _Castlevania II: Simon's Quest_.
  2. ↑ [Youtube video](https://www.youtube.com/watch?v=9XHXYhb_gDo) demonstrating the effect on _Majou Densetsu II: Daimashikyou Galious_.


