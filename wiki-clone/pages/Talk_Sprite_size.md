# Talk:Sprite size

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3ASprite_size) | View [other pages](Special_AllPages.xhtml#Talk_Sprite_size)

## "Natural fit" and/or strange size logic

I've removed these two statements which I don't really follow/agree with: 

    **(8x8)** If your game's characters are 21-24 pixels tall, 8x8 pixel sprites may be the best choice.
    For example, this is true of _Mario Bros._ (1983), _Balloon Fight_ , the enemies in the original _Super Mario Bros._ , and the hero in the _Mega Man_ series.
    And in _[Thwaite](User_Tepples_Thwaite.xhtml "User:Tepples/Thwaite")_ , everything is either 8x8 (missiles, smoke, crosshair) or 24x24 (explosions) by nature, so 8x8 is a natural fit.

    **(8x16)** So games without many objects that are smaller than 12 pixels or 17-24 pixels in height can benefit from 8x16 sprites.
    These include fighting games, vertical shooters, or platformers without guns.

I don't understand this very peculiar 21-24 pixel range for 8x8. Why is 17-20 exempt? Similarly for 8x16 I don't understand the "smaller than 12 pixels" statement either, or 17-24... why isn't 32 mentioned??? This is very difficult for me to grasp the logic of. 

Secondly, I don't believe _any_ of these genres are a natural fit for either sprite mode, so I don't think it's a good idea to dictate an opinion like this. 

Thread where this came up: <https://forums.nesdev.org/viewtopic.php?p=200585#p200585>

\- [Rainwarrior](User_Rainwarrior.xhtml "User:Rainwarrior") ([talk](User_talk_Rainwarrior.xhtml "User talk:Rainwarrior")) 09:11, 21 July 2017 (MDT) 

    I may not be able to reverse-engineer the entire thought process behind the rules of thumb that I wrote almost four years ago, but I'll do my best.
    "Why is 17-20 exempt?"
    The "21-24" may have been a mistake. Or it may have been an attempt to imply that if something is 17 to 20 pixels tall, the part at the top or bottom that exceeds 16 pixels is likely not to be as wide as the part that occupies more of the width of the cel. Thus it occupies fewer of the 8 allowed sprites per scanline. Or sprites that are just barely bigger than 16 pixels can be redrawn to be 16 pixels or smaller. I could state it as follows: 

    If your game's characters are 17-24 pixels tall, and they cannot be practically redrawn to be 16 pixels tall or shorter possibly with one 8-pixel-wide part jutting out of the top or bottom, 8x8 pixel sprites may be the best choice.
    But wouldn't this complexity confuse especially novice designers?
    "Similarly for 8x16 I don't understand the 'smaller than 12 pixels' statement either
    That was probably similar logic to the "21-24": something with a narrow top or bottom or redrawable to fit in 8 would occupy less space.
    "why isn't 32 mentioned???"
    Because neither mode has much of an advantage flicker-wise with heights 25 to 32, at least not compared to the advantage that 8x8 has with 21 to 24.
    "Secondly, I don't believe any of these genres are a natural fit for either sprite mode"
    I mentioned fighting games because games with large characters are more likely to hit the 64 sprites per scene limit than games with small characters. I mentioned vertical shooters because they're less likely to hit the 8 sprites per scanline limit than horizontal shooters, and many player ships either are 16 pixels tall or are 16 pixels tall except for a narrow part at the front. I mentioned guns because of the flicker that many nearly horizontally aligned projectiles add.
    I mentioned _Thwaite_ because I imagine that if I sat down to try to prove it, I would discover that 8x16 causes much more flicker for the particular sprite sizes used in that game. If you want, I could try branching _Thwaite_ and modifying it to use 8x16 pixel sprites.
    \--[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 10:34, 21 July 2017 (MDT)
