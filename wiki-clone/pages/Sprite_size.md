# Sprite size

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Sprite_size) | View [other pages](Special_AllPages.xhtml#Sprite_size)

The NES PPU offers the choice of 8x8 pixel or 8x16 pixel sprites. Each size has its advantages. 

## Advantages of 8x8

If the majority of your objects fit in an 8x8 pixel sprite, choose 8x8. These might include tiny bullets, puffs of smoke, or puzzle pieces. Drawing, say, a 4x4 pixel bullet with an 8x16 pixel sprite would waste pattern table space and increase potential for dropout or flicker on adjacent scanlines. 

Some very detailed sprite animations are easier to do in 8x8. For example, 8x8 is more amenable to animating just the legs in an RPG character's walk cycle while reusing the head tiles. An overlay to add more colors to a small area, as in _Mega Man_ series, causes flicker on fewer lines. And it's possible to simulate small amounts of rotation by shearing the sprite, moving individual 8-pixel chunks 1 pixel at a time. 

The NES has no way to put a sprite half off the top of the screen, other than by using a top status bar and hiding sprites in [$2001](PPU_registers.xhtml "PPU registers") while the status bar is being displayed. Sprites entering or leaving have to enter or leave all at once, and this is especially visible on a PAL NES. So 8x8 sprites help diminish this pop-on effect. 

_Super Mario Bros. 3_ uses 8x16 sprites, and some of the enemies inherited from the original _Super Mario Bros._ had to be modified to fit this. Blooper (the squid), for example, is 24 pixels tall in the original but had to be redrawn smaller for _SMB3_. 

## Advantages of 8x16

The NES supports 64 8x8 sprites or 64 8x16 sprites. This means 8x16 sprites can cover a larger area of the screen. 

Using 8x16 pixel sprites can sometimes save CPU time. Say a game has four characters, each 32x16 pixels in size. It takes more time to write 32 entries to a display list than to write 16. 

Some games, such as _Crystal Mines_ (and its retreads _Exodus_ and _Joshua_), repeatedly switch game objects from being part of the background to being sprites and back so that they can temporarily leave the tile grid. _Super Mario Bros. 2_ likewise does this for the mushroom blocks, keys, and the like. Because 8x16 sprites can use both pattern tables, an object can use the same tiles whether it is rendered as background or as sprites. This causes a problem, however, for games using a scanline counter clocked by PA12 like that of the [MMC3](MMC3.xhtml "MMC3") because fetching from both pattern tables causes extra rises in PA12, which confuses the counter circuit. 

The NES supports 4 KiB for the background and 4 KiB for sprites. MMC5, however, has a 12K CHR mode that replaces background patterns with a third pattern table during [sprite fetch time in horizontal blanking](PPU_rendering.xhtml "PPU rendering"). This mode works only with 8x16 sprites because 8x8 sprites can use only one pattern table at a time. 

_Alfred Chicken_ , _Incredible Crash Dummies_ , _Teenage Mutant Ninja Turtles_ , and _Zelda II_ use a trick to completely hide attribute glitches when scrolling horizontally with [horizontally mirrored nametables](Mirroring.xhtml "Mirroring") that involves placing a column of black sprites at x=248. This is practical only with 8x16 sprites, as it needs 15 8x16 sprites or 30 8x8 sprites to cover the entire screen height. 

## External links

  * BBS discussion: 
    * Topics [1473](http://forums.nesdev.org/viewtopic.php?f=10&t=1473), [3649](http://forums.nesdev.org/viewtopic.php?f=10&t=3649), [4622](http://forums.nesdev.org/viewtopic.php?p=40787#p40787), [6194](http://forums.nesdev.org/viewtopic.php?f=10&t=6194), [6223](http://forums.nesdev.org/viewtopic.php?f=2&t=6223), and [10324](http://forums.nesdev.org/viewtopic.php?p=115918#p115918)
    * [Sprite shearing](http://forums.nesdev.org/viewtopic.php?f=21&t=9678)


