# User:Dr. Floppy/Apex

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3ADr._Floppy/Apex) | View [other pages](Special_AllPages.xhtml#User_Dr__Floppy_Apex)

Apex is a side-scrolling adventure/exploration game. It serves as a testament to the "golden age" of console gaming (circa early '90's)- a celebration of what was, what is and what might have been. 

## Contents

  * 1 Overview
  * 2 Story
  * 3 Gameplay
  * 4 RAM Map
  * 5 ROM Map



## Overview

  * _Project Owner_ : Retrometheus
  * _Developer_ : Retrometheus
  * _Compiler_ : None (hex coding)
  * _Sound Driver_ : Retrodriver™
  * _Soundtrack by_ : Retrometheus
  * _Started on_ : 5/2011
  * _Status_ : Preproduction
  * _Mapper_ : MMC5 (unique submapper, tentatively dubbed "EJROM")
  * _PRG-ROM_ : 1024 KB
  * _CHR-ROM_ : 1024 KB
  * _PRG-RAM_ : 64 KB



## Story

Bioterrorists are cultivating superviruses in labs hidden at the bottom of the ocean. 

## Gameplay

Apex is a rather nondescript purple angelfish with two basic attacks: a stationary somersault and high-velocity bubble projectiles. 

## RAM Map

**RAM** | **Function** | **Details**  
---|---|---  
0x0000-0D | Music Channel pointers | P2-P1-T-N-D-P3-P5   
0x000E-0F | Rhythm Array pointer |   
0x0010-1D | Repeat Coda pointers | P2-P1-T-N-D-P3-P5   
0x001E | Song Data bank#   
0x001F | DPCM sample data Bank#   
0x0020-2C | Rhythmic countdowns (even slots only) | P2-P1-T-N-D-P3-P5   
0x0021-2D | Selective repeats # remaining (odd slots only) | P2-P1-T-N-D-P3-P5   
0x002E-2F | Header pointer   
0x0030-32 | Timbre/volume data | P2-P1-T   
0x0033 | Song ID# jukebox | Writing a value here triggers a new song to play.   
0x0034 | Current song ID#   
0x0035-36 | Timbre/volume data | P3-P5   
0x0037 | Reserved for SFX.   
0x0038-3E | Most recent rhythmic countdown value | Used to reload even-numbered slots in the $20-2C range.   
0x003F | Reserved for SFX.   
0x0040-5B | Unused   
0x005C | Current macromode   
0x005D | Macromode transition destination   
0x005E | Reserved   
0x005F | Transition status | #00 = Null; #01 = Activate; #FF = Transitioning   
0x0060-AF | Unused   
0x00B0-B2 | Controller 1 buttons | old-regular-new   
0x00B3-B7 | Reserved   
0x00B8-BA | Controller 2 buttons | old-regular-new   
0x00BB-BF | Reserved   
0x00C0 | Current spritesheet   
0x00C1 | Current BG tilesheet   
0x00C2-DF | Unused?   
0x00E0 | Game Paused? | #00 = No; #01 = Yes   
0x00E1-F7 | Unused?   
0x00F8 | SPR-OAM transfer on next NMI? | #00 = yes   
0x00FC | Frame counter | See what we did there?   
0x00FF | Used in terminal MPL loop   
0x0100-1FF | Stack   
0x0200-2FF | Sprites   
  
## ROM Map

$00010-0400F: Default music data bank  
$04010-1000F: Alternate music data banks  
$E0010-F000F: PRG-RAM banks (8 x 8 kB)  
$F0010-FE00F: Seven 8kB DPCM banks  
$FE010: NMI Routine  
$FF010: Init. Routine  
$FF090: Title Screen draw routine  
$FF0F0-FF10F: Title Screen palettes  
$FF210: Main Program Loop  
$FFC10: Music frequency table  
$FFD10: Tempo Offsets  
$FFF10: Music/Tempo/DPCM Bank index  
$100000: Waiting for NMI  


Categories: [Unfinished homebrew games](Category_Unfinished_homebrew_games.xhtml)
