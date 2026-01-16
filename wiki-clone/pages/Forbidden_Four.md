# Forbidden Four

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Forbidden_Four) | View [other pages](Special_AllPages.xhtml#Forbidden_Four)

[![](../wiki-images/ForbiddenFour.png)](File_ForbiddenFour_png.xhtml)

[](File_ForbiddenFour_png.xhtml "Enlarge")

Screenshot of menu

**Double Crossing: The Forbidden Four** is a multicart engine based on _The Legend of Zelda_ and designed to accommodate the four hardest-to-obtain NES games in _[Animal Crossing](https://en.wikipedia.org/wiki/Animal_Crossing_\(video_game\) "wikipedia:Animal Crossing \(video game\)")_. 

## Contents

  * 1 Background
  * 2 Instructions
  * 3 How it works
  * 4 Further developments
  * 5 References



## Background

_Animal Crossing_ , a social simulator for Nintendo GameCube, included NES consoles with built-in games as collectible furniture items. The player could play these games through _AC'_ s built-in emulator. There were several ways to receive games: 

  * Common games 
    * Buy them from Tom Nook or Redd when he has them in stock
    * Enter a "universal code" created with a code generator in Tom Nook's store
  * Uncommon games 
    * A neighbor announces on the bulletin board that he has buried one of these in some part of town; use a shovel, find the star mark denoting a buried item, and dig it up
    * Enter a "universal code" created with a code generator in Tom Nook's store
  * Rare games* 
    * Enter a code in the letter and send it to a neighbor
    * Connect a Game Boy Advance system and do a particular trading sequence on the island
    * Enter a trade code, given to a specific player name in a specific town name, in Tom Nook's store as the right player in the right town (most commonly for _[Punch-Out!!](MMC2.xhtml "MMC2")_)
    * Enter a "universal code" created with a code generator in Tom Nook's store
  * Forbidden games 
    * Receive one of these games at a Nintendo trade show
    * Scan a particular e-Reader card (for Mario Bros and Ice Climber)
    * Use Action Replay to change the ID number of a furni in your inventory



The forbidden games are _Mario Bros._ , _Ice Climber_ , _Super Mario Bros._ , and _The Legend of Zelda_. The code for Tom Nook's store is specifically designed to reject universal codes that refer to the ID numbers of these games.[1]

_Animal Crossing: Wild World_ didn't have NES games because it was too soon after the release of Famicom Mini/Classic NES Series. Later games on Nintendo platforms (_City Folk_ and _New Leaf_) didn't have NES games because they would compete with Virtual Console games from Wii Shop Channel. _Pocket Camp_ runs on hardware whose input device differs too much from that of the NES. 

* None of them from [the developer of _Battletoads_](Rare.xhtml "Rare")

## Instructions

  1. Install 7-Zip. (Windows version on 7-Zip.org; under Ubuntu, `sudo apt-get install p7zip` to add .7z support to the File Roller archive manager.)
  2. Download and unpack [Forbidden Four](http://pics.pineight.com/nes/ForbiddenFour.7z).
  3. Put two [NROM](NROM.xhtml "NROM")-128 games in [iNES](INES.xhtml "INES") format named "bros.nes" and "ice.nes" (not included) in the same folder as "menu.nes".
  4. Put one NROM-256 game named "smb1.nes" (not included) in the same folder.
  5. (Optional) Use a nametable editor to edit the names of the games in dxinglogo.nam and use CC65 to recompile the menu binary.
  6. Put The _Legend of Zelda_ (or possibly other select games) named "loz.nes" in the same folder.
  7. (On systems other than Windows) Recompile the ROM builder 'buildf4'.
  8. Run "Build Multicart.bat", or on systems other than Windows, run tools/buildf4.



## How it works

Forbidden Four is a multicart engine that can take any four NES ROMs that fit the following criteria: 

  * Any NROM-128 game, called "Bros"
  * A second NROM-128 game, called "Ice"
  * Any NROM-256 game, called "SMB1"
  * The _Legend of Zelda_ , called "LoZ", or any other 1 Mbit [SNROM](SxROM.xhtml "SNROM") game with the same init code



Two further restrictions: The NROM code must not write to ROM $8000-$9FFF or $E000-$FFFF, as this will trigger an unexpected bankswitch on the [MMC1](MMC1.xhtml "MMC1") and probably crash the game. The NROM-128 games also must not depend on values read from the ROM [mirror](Mirroring.xhtml "Mirroring") at $8000-$BFFF because this ROM mirror does not exist in the multicart. 

This SNROM has 256 KiB of PRG ROM, 8 KiB of CHR RAM, and 8 KiB of battery-backed PRG RAM. The ROM of any MMC1 based game is divided into 16 KiB banks. There are sixteen such banks in Forbidden Four: 
    
    
     0  LoZ PRG 0
     1  LoZ PRG 1
     2  LoZ PRG 2
     3  LoZ PRG 3
     4  LoZ PRG 4
     5  LoZ PRG 5
     6  LoZ PRG 6
     7  LoZ PRG 7
     8  SMB1 PRG 0
     9  SMB1 PRG 1
    10  Bros CHR and SMB1 CHR
    11  Bros PRG
    12  Ice CHR and menu CHR
    13  Ice PRG
    14  Menu PRG
    15  LoZ PRG 7 (duplicate for vectors)
    

The last bank of LoZ PRG is duplicated because the game expects to find it in both bank 7 and the MMC1's fixed bank. 

The first 256 bytes of Menu contain 

  1. a jump table with four entries,
  2. code to copy a CHR RAM loader and a jump table manager into RAM at $0300, and
  3. the CHR RAM loader and jump table manager.



The four entries in the jump table specify which CHR page to load, which PRG page to load, how to set up the memory mapping and the mirroring, and where to put the program counter. 

From the mapper init code of LoZ: 
    
    
    setMMC1CTRL = $FF98
    setMMC1PRG = $FFAC
    $FF76:A9 0F     LDA #$0F
    $FF78:20 98 FF  JSR setMMC1CTRL
    ; Omitted code: set CHR bank to 0
    $FF90:A9 07     LDA #$07
    $FF92:20 AC FF  JSR setMMC1PRG
    $FF95:4C 40 E4  JMP $E440
    

So in order to intercept Zelda's boot sequence, we need to change the start bank and start address to point to our boot sequence at $8020, and make sure it ends up at ctrl=$0F bank=$07 PC=$E440 by the time Zelda starts. 
    
    
    $FF91: 0E
    $FF96: 20
    $FF97: 80
    

This passes control to the first 256 bytes of Menu, which sets up CPU RAM and starts the menu using a mechanism similar to the jump table. This is done because the first 256 bytes of Menu are designed to run from $8000-$80FF, so that LoZ can pass control to it, while the rest of Menu runs from $C100-$FFFF, so that it can take over the vertical blank NMI for display timing. 

## Further developments

If you are building this on top of an SNROM base game other than LoZ, you will need to change the addresses a bit in buildf4.c and then use GCC to rebuild buildf4.exe. If you have a patch to buildf4.c to support hooking other 1 Mbit SNROM games' init code, [talk](https://www.nesdev.org/w/index.php?title=Talk:Forbidden_Four&action=edit&redlink=1 "Talk:Forbidden Four \(page does not exist\)") about it. 

## References

  1. ↑ "[Forbidden Four](https://nookipedia.com/wiki/Forbidden_Four)" on Nookipedia



Categories: [Homebrew](Category_Homebrew.xhtml)
