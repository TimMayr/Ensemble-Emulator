# User:Zzo38/Attribute Zone

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Attribute_Zone) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Attribute_Zone)

This game is a puzzle game designed based around the limitation of the [PPU](PPU.xhtml "PPU"), such as 2x2 blocks of tiles have a single attribute, and that no more than eight sprites per scanline are possible. 

It is possible to save levels to ROM if using an emulator that writes RAM images such that the 2K RAM is contiguous and unencoded in the save file (although I do not expect any emulator to save RAM images in any other way). It is **not** possible to save levels to ROM if running the cartridge on a real Famicom hardware (although, you can save them to tape instead in that case). 

Warning: Game uses several [unofficial opcodes](CPU_unofficial_opcodes.xhtml "CPU unofficial opcodes"). Also, it probably will not work on an actual Color Dreams board, since this program does not do anything with the lockout defeat. 

Note: This program is designed to work whether or not there are bus conflicts. Therefore, if someone wishes to implement this game in a Famicom cartridge, they can decide whether or not to implement bus conflict avoidance. 

## Contents

  * 1 Overview
  * 2 Instructions
  * 3 Compression
  * 4 External



## Overview

  * Status: Editor done, gameplay done, partially music done, partially levels done
  * Project owner: [User:Zzo38](User_Zzo38.xhtml "User:Zzo38")
  * Developer: [User:Zzo38](User_Zzo38.xhtml "User:Zzo38")
  * Compiler: [Unofficial MagicKit](https://www.nesdev.org/w/index.php?title=Unofficial_MagicKit&action=edit&redlink=1 "Unofficial MagicKit \(page does not exist\)")
  * Other tools: Csound, CsoundMML, "lvlcopy.c", "huffer.c", "mkperiod.bas"
  * Sound driver: Custom, using CsoundMML
  * Soundtrack by: [User:Zzo38](User_Zzo38.xhtml "User:Zzo38")
  * Graphics by: [User:Zzo38](User_Zzo38.xhtml "User:Zzo38")
  * Started on: December, 2013
  * Cart made: No
  * Will make cart: It is intended to be possible although I probably won't do it by myself
  * Mapper: iNES Mapper #11: [Color Dreams](Color_Dreams.xhtml "Color Dreams")
  * PRG ROM: 32K
  * CHR ROM: Unknown
  * PRG RAM: 0
  * CHR RAM: 0
  * Players: 1
  * Input devices: [Family BASIC Keyboard](Family_BASIC_Keyboard.xhtml "Family BASIC Keyboard") is supported in all modes; [standard controller](Standard_controller.xhtml "Standard controller") can play ROM levels only
  * License: Public domain
  * Tested on official hardware: No (I am unable to)



## Instructions

There is tiles and sprites. Tiles can be white, blue, green, red, or yellow. Each 2x2 block (which are clearly marked as such) cannot contain non-white tiles of more than one color. It is also impossible for more than eight sprites in one row. Any condition resulting in one of these impossible conditions, instead causes the objects to stay where it is, instead of being moved/changed. 

Square brackets can be used for previous/next level, and F1 will go back to title screen. If you push RETURN, the current level is reset, so that you can try again. 

Tiles: 

  * Empty: Objects can pass through.
  * Wall: Objects cannot pass through.
  * Key: Indirectly pushable. Can be used to open door of the same color of a key.
  * Door: If you have the key, you can open the door.
  * Box: Pushable in any direction. Otherwise, just blocks movement.
  * Slider: Pushable, but can move only in direction of arrow is pointing at.
  * One Step: If you step on here, it becomes a wall next time, you you cannot step on a second time (or any subsequent time).
  * Ball: Move a far distance. Can splash in water, destroying both a ball and water.
  * Gem: Indirectly pushable. Collect the remaining gem on the level to win.
  * Paintbrush: Move it to touch object you want to change the color, and become a same color of paint.
  * Gate: The player can step on here, but other objects mostly don't.
  * Bomb: Can be moved around. Explodes when the "X" key is pushed, or if non-player sprite objects run into it. Does not explode a white wall, and also does not explode any waters regardless of the colors.
  * Ghost Block: Can be moved around, and prevent a ghost from passing through.
  * Puller: If a player moves directly away, the puller attempts to follow (possibly blocking the path to go back).
  * Water: Blocks movement, but some things can remove it.



Sprites: 

  * Player: Move by arrow keys or "H", "J", "K", "L" keys.
  * Other Player: Upsidedown, but can be made rightsideup (and normal player upsidedown) by pushing a "Z" key.
  * Pusher: Move in direction of pointing. Will try to push objects which are in the way.
  * Ghost: Chase player. You lose if the player is touched by a ghost. Move through nearly any tiles, except ghost blocks.
  * Chaser: Chase player. You lose if the player is touched by a chaser.
  * Runner: Run back and forth. It can cause bombs to explode, and you also lose if it touches a player.
  * Rock: Can be moved around, and can fall on water, which removes both the water and rock.
  * Bubble: Does nothing by itself, but can be removed simply by player touched it.
  * Timer: Moves and wraps around the screen to next line/column. If it is stuck, you lose.



## Compression

Here is a description of the compression used for level data in this program, in case it helps anyone. 

Compressed level data is stored in CHR ROM, using Huffman coding. The program "huffer.c" will Huffman encode the data and generate the corresponding Huffman decoder in assembly language. Controlled by an external file, it will also generate Huffman encoding for RLE for some kinds of tiles. 

Pointers into CHR ROM are then stored in PRG ROM. Some of the bits are used for bankswitching and are written to the bankswitching register. Since the mapper has four bits for CHR bankswitching but there is only three bits available in the table (since it is mapped to PPU address space $0000-$1FFF), there is one additional value recorded which decides which level number starts with the high bit of the bank register set. 

Storing compressed data in CHR ROM means that the software does not need to keep track of the current address and to perform address increments, since that is done by the hardware, therefore speeding up loading of compressed level data, which can therefore cross 256 byte pages as much as you want. 

## External

  * [Forum](http://forums.nesdev.org/viewtopic.php?f=22&t=10913)
  * [Download](http://zzo38computer.org/nes_program/attrzone.zip) (includes source-codes, iNES ROM image, NSF, INI, and a few other things)



Categories: [Homebrew games](Category_Homebrew_games.xhtml)
