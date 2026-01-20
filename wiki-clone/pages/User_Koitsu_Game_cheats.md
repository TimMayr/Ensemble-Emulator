# User:Koitsu/Game cheats

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AKoitsu/Game_cheats) | View [other pages](Special_AllPages.xhtml#User_Koitsu_Game_cheats)

## Contents

  * 1 Game cheats
    * 1.1 1943 - The Battle of Midway
    * 1.2 Adventures of Dino Riki, The
    * 1.3 Code Name: Viper
    * 1.4 Legend of Zelda, The
    * 1.5 Mega Man 2
    * 1.6 River City Ransom



# Game cheats

Throughout the years I've reverse-engineered a lot of games, as well as Game Genie or Pro Action Replay/Rocky codes, to figure out the internals. In most cases I do this because I don't want have to "grind" \-- given enough time, I'd be able to accomplish the task at hand, so why not skip all that nonsense? A good example is gold/currency. I paid my dues sitting in front of a TV for hours as a kid, so I think I've earned the right. :-) 

Anyway, I wanted to make a list of games and memory locations locations of such things. I tend to focus on just zero page/RAM data and not stuff in ROM, but technically any memory location can be used (the tricky part with NES games is being precise about which PRG page has the cheat applied to it -- that's one of the reasons many emulators have a "Compare" field alongside "Location" and "Value"!). 

All below games are assumed to be the NTSC versions. I'll try to denote differences between game versions (e.g. PRG0 vs. PRG1) if it applies. 

## 1943 - The Battle of Midway

Location | Description   
---|---  
$0410 | Current energy, ones place. Range $00 to $09   
$0411 | Current energy, tenths place. Range $00 to $09   
$0412 | Current energy, hundredths place. Range $00 to $09   
  
  * For infinite energy, setting $0410, $0411, and $0412 all to $09 is needed -- just $0412 by itself is insufficient: you CAN die in some situations!



## Adventures of Dino Riki, The

Location | Description   
---|---  
$0089 | Current health (energy) value. Range $00 to $04   
$009D | Active weapon type: $00 = rock, $01 = axe, $02 = boomerang, $03 = torch. Other values result in glitches and/or game crashing   
$009E | Wings status: $00 = player lacks wings, $01 or greater = player has wings   
  
## Code Name: Viper

Location | Description   
---|---  
$06E0 | Handgun bullet count. Range $00 to $80   
$06E2 | Machine gun bullet count. Range $00 to $80   
$06F8 | Current health (energy) value. Range $00 to $04   
  
  * The current health (energy) value can exceed the number of health containers (ex. if health container count is 2 and $06F8 = $04, then player will actually have 4 health)



## Legend of Zelda, The

Location | Description   
---|---  
$04F0 | Invincibility timer value. Range $00 to $FF. See below notes   
$0658 | Current bombs. Range $00 to $10, though game core will work with values up to $FF   
$066D | Current rupies. Range $00 to FF   
$066E | Current keys. Range $00 to $09, but game core will work with values up to $FF   
  
  * Invincibility timer: the game core decrements this by 1 fairly quickly (a value of $FF will reach $00 in approximately 8 seconds). For "visually accurate" permanent invincibility, use a value $FF with a compare value of $00 (i.e. timer resets to $FF once reaching $00)
  * [Details of how the bomb/rupie/key printing code works](http://forums.nesdev.org/viewtopic.php?p=173537#p173537)



## Mega Man 2

Location | Description   
---|---  
$004B | Invincibility timer value. Range $00 to $FF. See below notes   
  
  * Invincibility timer: the game core decrements this by 1 fairly quickly (a value of $FF will reach $00 in approximately 4 seconds). For "visually accurate" permanent invincibility, use a value $FF with a compare value of $00 (i.e. timer resets to $FF once reaching $00)



## River City Ransom

Location | Description   
---|---  
$04C7 | Player 1 money, cents value. BCD encoded. Range $00 to $99   
$04C8 | Player 1 money, dollars value. BCD encoded. Range $00 to $99   
$04C9 | Player 1 money, 100-dollars value. BCD encoded. Range $00 to $09   
  
  * Money example: for player 1 to have $367.49, you'd need $04C7 = $49, $04C8 = $67, $04C9 = $03


