# Sprite cel streaming

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Sprite_cel_streaming) | View [other pages](Special_AllPages.xhtml#Sprite_cel_streaming)

**Sprite cel streaming** is a method of animating sprites in games using CHR RAM. When the game wants to change to a new cel (frame of animation), the game copies the cel from PRG ROM to CHR RAM during vertical blanking. Normally, about 8 tiles (128 bytes) can be copied this way in a single NTSC vblank. 

Games known to use streaming: 

  * _Battletoads_ streams the player 1 and 2 sprites and turns rendering on late to increase bandwidth to CHR RAM.
  * _Solstice_ not only streams the sprites of Shadax but also clips them against the shape of the background in software.
  * _Haunted: Halloween '85_ reserves two 16-tile buffers for each of six actors, one for the current frame and one for the next. Whenever the video memory transfer buffer isn't in use, the game predicts the next frame for each sprite based on the current one. For example, the most common frame in a walk is the next in the same walk cycle, and the most common frame after a punch or other attack is the next attack in the combo. Thus if most changes are predicted accurately, there's less of a bottleneck in loading the frame when prediction occasionally fails.
  * _Astérix_
  * _The Lion King_


