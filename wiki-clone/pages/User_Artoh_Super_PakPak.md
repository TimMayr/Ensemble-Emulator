# User:Artoh/Super PakPak

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AArtoh/Super_PakPak) | View [other pages](Special_AllPages.xhtml#User_Artoh_Super_PakPak)

SUPER PAKPAK is a 2-4 player [Spacewar!](https://en.wikipedia.org/wiki/Spacewar! "wikipedia:Spacewar!")/[Thrust](https://en.wikipedia.org/wiki/Thrust_\(video_game\) "wikipedia:Thrust \(video game\)")/[Gravity Force](https://en.wikipedia.org/wiki/Gravity_Force "wikipedia:Gravity Force") style game for the NES and Famicom. 

It was programmed in a few weeks during the year 2010, and was submitted as an official entry for the [Assembly](https://en.wikipedia.org/wiki/Assembly_\(demoparty\) "wikipedia:Assembly \(demoparty\)") Summer 2010 Game Development competition in Helsinki, Finland. It ranked 7th out of 19. 

## Contents

  * 1 Overview
  * 2 Gameplay and rules
  * 3 Multi-player game
  * 4 Analog controller
  * 5 External links



## Overview

[![](../wiki-images/Superpakpak.png)](File_Superpakpak_png.xhtml)

Screenshot from Super PakPak

  * Project owner: fuzb (artoh @ the forum)
  * Programmer: fuzb (artoh @ the forum)
  * Logo and label artwork: lowhiz
  * Soundtrack by: quarren
  * Compiler: NESASM
  * Sound driver: Famitracker player
  * Started on: December, 2010
  * Release date: July 28, 2010
  * Cart made: No
  * Mapper: iNES Mapper #2: [UNROM](UxROM.xhtml "UxROM")
  * PRG Memory: 64 KB
  * CHR Memory: 0 KB
  * MD5 checksum of original ROM: 9f4c7dab70be2af37cadc5be0166023d



## Gameplay and rules

At least two players are needed to start the game. _Hold_ button A at the start menu to begin a game. The game will begin after the counter counts to zero. In two player games, you need to get two kills in a round before the level ends. In three or four player games, the level ends when there is only one ship remaining. 

During the game you can use the following keys: 

  * Start - Pause (during a pause you can return to the main menu by pressing the buttons down, select, A and B at the same time)
  * Left, right - Turn the ship
  * A - Throttle; increases the speed. When the ship has suffered enough damage, it bursts into flames and can not gain enough speed, hence it becomes harder to maneuver.
  * B - Fires the basic weapon.
  * Down - Launches the secondary weapon. If you are landed on a base, this button changes the secondary weapon. Note that you can't fire the secondary weapon while touching the base, and it is momentarily inactive right after you leave the base or right after launching it. The weapon icon is shown on the statusbar, and it will flash when the weapon is unavailable.



The secondary weapons are the following: 

  * Bomb - Drops down from the ship, explodes and emits 4 bullets upwards. Be careful not to launch this while you are near the ground or the enemy. The bomb itself does not do much direct damage (twice a bullet's).
  * Spray Cannon - Emits a 4-bullet spray
  * Mine - Could be planted anywhere. Does very high damage (3x a bullet's) and disappears after a while.
  * Vortex Cannon - Shoots thru any object, except the impenetrable walls. This does twice a bullet's damage.
  * Teleport - Transports the ship to a random starting location. This is useful when trying to avoid a tight spot.



You get a point by destroying another ship. If a ship gets destroyed indirectly (lava or crashing into a wall), the ship that damaged it the last time during the round will get a point. If a ship shoots itself or gets blown by an explosive in the level triggered by the ship itself, no one gets a point. There are 12 scenes in the game; after the 12th scene the game is reverted back to the first scene. After any player gets 50 points, the game ends and an end screen will appear. 

## Multi-player game

You can play with a NES Four Score/NES Satellite adapter, or if you have a Famicom System, you can use regular Famicom multi-player controllers. 

If you have a regular NES, and you don't have a Four Score/Satellite, it is possible to hack two more controllers into the expansion port at the bottom of the NES (this actually adds Famicom multi-player feature to your NES, which is supported by some Japanese games). 

## Analog controller

SUPER PAKPAK supports an analog controller! Too bad this analog controller does not exist yet. The analog controller _does not_ work with a Four Score, only with the Famicom style multitap. 

If you want to try to make one, here are the shift register bits: 
    
    
    +--------------------------------------------------------+
    |Read# 1 | 2 |   3    |   4   | 5  |  6   |  7   |   8   |
    +----+---------------------------------------------------+
         |                     Joy#1/Joy#2                   |
         +---------------------------------------------------+
         | A | B | Select | Start | Up | Down | Left | Right |
         +---------------------------------------------------+
    
    +-----------------------------------------------+
    |  9  | 10  | 11  | 12  | 13  | 14  | 15  | 16  | 
    +-----------------------------------------------+
    |                     Analog                    |
    +-----------------------------------------------+
    | D7  | D6  | D5  | D4  | D3  | D2  | D1  | D0  |
    +-----------------------------------------------+
    
    +---------------------------------------+
    | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 |
    +---------------------------------------+
    |             Adapter info              |
    +---------------------------------------+
    | 1  | 1  | 0  | 0  | 1  | 0  | 1  | 1  |
    +---------------------------------------+
    

A good candidate would be a microcontroller with AD and a potentiometer with the stopper inside removed, so it rotates 360 degrees. You could also implement it in pure TTL logic and use an old AD080x AD-converter! 

## External links

  * [Download from ftp.scene.org](ftp://ftp.scene.org/pub/parties/2010/assembly10/gamedev/super_pakpak_by_aoh_games.zip)
  * [Download from Assembly homepage](http://media.assembly.org/gamedev/2010/super_pakpak_by_aoh_games.zip)
  * [Super PakPak at Pouet](http://www.pouet.net/prod.php?which=55543)
  * [Gameplay video - Retrospelsmässan @ Kulturkalaset 2010](http://www.youtube.com/watch?v=TnYQN7FUVLU)
  * [Gameplay video - Assembly Summer 2010 Compo Video (has sound sync problems)](http://www.youtube.com/watch?v=o5kFEW5MqIM)



Categories: [Homebrew games](Category_Homebrew_games.xhtml)
