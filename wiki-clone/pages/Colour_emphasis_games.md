# Colour-emphasis games

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Colour-emphasis_games) | View [other pages](Special_AllPages.xhtml#Colour_emphasis_games)

The following is a list of games which make use of [PPU register $2001](PPU_registers.xhtml#Mask_\(%242001\)_>_write "PPU registers")'s colour emphasis bits (D7 to D5): 

## Contents

  * 1 Commercial
  * 2 Homebrew
    * 2.1 Commercial homebrew
  * 3 Multicarts' menus
  * 4 Romhacks
  * 5 References



## Commercial

  * _Bubble Bobble_ for the Famicom Disk System uses red emphasis to tint the screen throughout the game, which is absent in the cartridge releases.
  * _The Fantastic Adventures of Dizzy_ uses it in the beginning when the scroll is unfurled.
  * _Final Fantasy_ and _Final Fantasy II_ rapidly sets and clears all 3 emphasis bits to flash the screen when going into a battle screen, also rotates colors when entering the status screen.
  * _[Super Game's Aladdin](http://community.fandom.com/wiki/c:bootleggames:Aladdin_\(Super_Game\) "wikia:c:bootleggames:Aladdin \(Super Game\)")_ uses red emphasis to tint the screen during gameplay, and all 3 emphasis bits to dim when paused. The official version of _Aladdin_ , released only in Europe, also uses the emphasis bits to tint the screen green during gameplay (as the PAL console flips the red and green emphasis bits), but does not dim the screen when paused.



The following games set all 3 emphasis bits at once to dim the screen: 

  * _Cliffhanger_
  * _Cybernoid - The Fighting Machine_ on the top part of the difficulty select and better luck next time screens. Also uses red emphasis when the screen changes in-game.
  * _Dragon's Lair_
  * _Felix the Cat_
  * _Fun House_ for fading out
  * _The Immortal_
  * _Indiana Jones and the Last Crusade (Taito)_ , story scenes and some action scenes.
  * _James Bond Jr_
  * _The Jungle Book_
  * _Just Breed_
  * _Lagrange Point_ uses colour emphasis, in some frames combined with grayscale bit to do a very cheap fade-out effect when switching to and from the menu on the playfield, and also when entering into battle
  * _Legend of Prince Valiant, The (E)_ uses full emphasis on the title screen, green emphasis for gameplay and red emphasis for status bar
  * _Lethal Weapon_
  * _The Lion King (E)_
  * _Magician_
  * _Mickey's Adventures in Numberland_
  * _Muppet Adventure: Chaos at the Carnival_
  * _Noah's Ark (E)_ uses blue emphasis combined with grayscale mode to put part of the level underwater and all 3 emphasis bits in other cases
  * _Qix_
  * _Rampart_ uses color emphasis on the player select screen, the options screen and some of the scrolling text. Blue emphasis is used for the score tally bar.
  * _R.B.I. Baseball 3_
  * _Space Shuttle Project_
  * _Super Spy Hunter_ , when paused. The Japanese version _Battle Formula_ doesn't do this.
  * _Xiao Ma Li_



## Homebrew

  * [Wall](http://nesdev.org/wall.zip) by Chris Covell uses a blue emphasis to simulate the water flooding the wall.
  * [Graphics editor](http://pineight.com/nes/#editor) by Damian Yerrick allows applying a SCREEN TINT to the picture.
  * [Munchie Attack](http://nesdev.org/Munchie_Attack.zip) by Memblers dims the screen upon Game Over.
  * [Super Bat Puncher Demo](http://morphcat.de/superbatpuncher/) dims the screen during pause.
  * Battle Kid and Battle Kid 2 dim the screen during pause.
  * [Waddles the Duck](https://cpprograms.net/classic-gaming/waddles-the-duck/) dims the screen when on a warp, and RG-emphasis when in ice-dimension, among others.



### Commercial homebrew

  * [Full Quiet](https://www.retrotainmentgames.com/collections/video-games/products/copy-of-full-quiet-regular-edition-nes-game-green-glow-cartridge-complete-in-box-cib) applies various tints over the course of its day/night cycle.



## Multicarts' menus

  * _[400 in 1](https://bootleggames.fandom.com/wiki/400-in-1_Multicart) sets all 3 bits _



## Romhacks

  * _[Rockman 4 Minus Infinity](https://sites.google.com/site/rockman4mi/)_ (during Brightman's stage) darkens areas farther away from Mega Man.
  * _[Bisqwit's Castlevania II retranslation](http://bisqwit.iki.fi/cv2fin/)_ uses the de-emphasis bits to highlight a section of a screen when scrolling the list of savegame slots.



## References

  * [BBS topic](http://forums.nesdev.org/viewtopic.php?t=8623)
  * [BBS topic #2](http://forums.nesdev.org/viewtopic.php?f=16&t=11995)


