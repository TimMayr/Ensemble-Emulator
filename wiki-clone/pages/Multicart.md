# Multicart

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Multicart) | View [other pages](Special_AllPages.xhtml#Multicart)

A **multicart** is a ROM cartridge containing several independent games. Although a small number of multicarts from official copyright holders exist, most multicarts are made by third parties for-profit. Most multicarts include a menu from which the individual games are selected; based on the choice made there, the menu code sets up the multicart's banking registers to switch in the desired game and transfers control to it. 

## Contents

  * 1 Multicart mapper requirements
    * 1.1 NROM-based multicarts
    * 1.2 Inner and outer banks
  * 2 Example multicarts
    * 2.1 Official
    * 2.2 Pirate
    * 2.3 Homebrew
  * 3 See also
  * 4 External links



# Multicart mapper requirements

The typical multicart modifies the code of individual games as little as possible for the purpose of multicart inclusion. Instead, the multicart's circuit board switches between games in such a way that each game's code can run in the same memory environment as its original single-game cartridge. To make this happen, a multicart mapper must be able to switch ... 

  * the PRG ROM area at CPU $8000-$FFFF, in particular, the reset and interrupt vectors at $FFFA-$FFFF;
  * the CHR area at PPU $0000-$1FFF;
  * the nametable mirroring type between Horizontal, Vertical, or Single-Screen, as needed by each game.



## NROM-based multicarts

These tasks are simple to implement when only [NROM](NROM.xhtml "NROM") games are to be included. A multicart-based mapper for NROM games only must be able to switch ... 

  * between several 16 KiB PRG banks if NROM-128 games are to be switched;
  * between several 32 KiB PRG banks if NROM-256 games are to be switched;
  * be able to switch between NROM-128 and NROM-256 mode _if_ both types of games are to be included;
  * be able to switch between several 8 KiB CHR banks;
  * be able to switch between mirroring types _if_ games of different mirroring are to be included, otherwise the multicart can use hard-wired mirroring just like the original individual games.



Switching between NROM-128 and NROM-256 PRG modes means in technical terms that the PRG ROM chip's A14 line can be selected to be driven 'either' from a PRG bank bit in the multicart's banking register (NROM-128 mode) or from the CPU A14 line (NROM-256 mode), as the CPU A14 line is the address that changes between the $8000-$BFFF and $C000-$FFFF CPU address range. This is a fundamental principle in almost all multicart mappers: PRG ROM address lines ("PRG A0" to "PRG A _xx_ ") almost always either relayed directly from the cartridge connector (called "CPU A0" to "CPU A14") or from mapper logic such as a bank register. 

The simplest NROM-based multicart mappers use the same register bits to select the 16/32 KiB PRG and the 8 KiB CHR bank; see [INES Mapper 200](INES_Mapper_200.xhtml "INES Mapper 200") as an example as that. Slightly more involved multicart mappers use different register bits to select the 16/32 KiB PRG and the 8 KiB CHR banks; doing so will allow the mapper to include one or more [CNROM](CNROM.xhtml "CNROM") games; a common example of this is [INES Mapper 058](INES_Mapper_058.xhtml "INES Mapper 058"). Depending on how the CHR bank switch is implemented, the CNROM games' bankswitching routines will require appropriate modification of the bankswitching mechanism and the bank number "offset", meaning the number of the first of their several CHR banks. 

## Inner and outer banks

To reduce the burden of modifying individual CNROM games for multicart inclusion, a multicart mapper switching CNROM-based games can separate the CHR bank number into an "inner bank", meaning the CHR bank that is modified by the individual game's code, and an "outer bank" that is only switched by the multicart menu's code. These two bank numbers will then occupy separate hardware registers, with the inner bank typically occupying the same address range ($8000-$FFFF) as an original [CNROM](CNROM.xhtml "CNROM") circuit board to minimize the need for modifying the game's code. The "inner CHR bank" in such cases typically selects between four 8 KiB CHR banks per game (meaning "it selects CHR A13 to A14"), while the "outer CHR bank" typically selects between 32 KiB CHR "super banks" from game to game (meaning "it selects CHR A15 and above"). Some multicart mappers allow switching between NROM, 16 KIB CNROM and 32 KIB CNROM modes; in such cases, both the inner and outer bank registers will have bits for CHR A13 (the "8 KiB bank") and CHR A14 (the "16 KiB bank"), with the banking mode bit(s) selecting whether CHR A13/A14 will be decided by the respective bit in the inner or outer bank register. See [NES 2.0 Mapper 332](NES_2_0_Mapper_332.xhtml "NES 2.0 Mapper 332") for an example of such functionality. 

The separation of inner and outer bank then allows for the inclusion of games that switch PRG banks in-game as well. A common example are [UNROM](UxROM.xhtml "UNROM")-based games: they switch between eight 16 KiB banks in the CPU $8000-$BFFF range (i.e. when CPU A14=0) while having a fixed last bank in the $C000-$FFFF range (i.e. when CPU A14=1) that includes the important interrupt vectors. Putting several UNROM games on one multicart without modification of the individual games' code therefore requires two bank registers: an UNROM latch that responds to writes in the CPU $8000-$FFFF range that switches PRG A13..A16 when CPU A14=0 and forces CPU A13..A16 to 1 if CPU A14=1; this UNROM latch becomes the "inner bank register" in the multicart context, and an "outer bank register" that switches PRG A17 and higher for the entire $8000-$FFFF address range so that the "fixed" bank of each game with its interrupt vectors is switched as well. 

# Example multicarts

## Official

Multicarts authorized by the games' copyright owners have been released on these boards: 

  * [CNROM](CNROM.xhtml "CNROM") (Donkey Kong Classics)
  * [MHROM](GxROM.xhtml "MHROM") (Super Mario Bros./Duck Hunt)
  * [NES-EVENT](NES_EVENT.xhtml "NES-EVENT") (Nintendo World Championships)
  * [SFROM](MMC1.xhtml "SxROM") (Super Mario Bros./Duck Hunt/World Class Track Meet)
  * [Super Mario Bros./Tetris/Nintendo World Cup](INES_Mapper_037.xhtml "INES Mapper 037")
  * [Nintendo World Cup/Super Spike V'Ball](INES_Mapper_047.xhtml "INES Mapper 047")
  * [Color Dreams](Color_Dreams.xhtml "Color Dreams") board (_Sunday Funday_ , etc.)
  * [Codemasters Quattro series](INES_Mapper_232.xhtml "INES Mapper 232")
  * [Caltron 6-in-1](INES_Mapper_041.xhtml "INES Mapper 041")
  * [Maxi 15](INES_Mapper_234.xhtml "INES Mapper 234")



## Pirate

Most pirate multicarts have several [NROM](NROM.xhtml "NROM") games because it's really easy to switch them, even in mappers as simple as [GNROM](GxROM.xhtml "GNROM") or the [Color Dreams](Color_Dreams.xhtml "Color Dreams") board. 

  * [iNES Mapper 015](INES_Mapper_015.xhtml "INES Mapper 015") (100-in-1 Contra Function 16)
  * SNROM ([Forbidden Four](Forbidden_Four.xhtml "Forbidden Four"))



## Homebrew

After the NES's commercial era, some homebrew games were released in limited quantities on multicarts. 

  * [2-in-1 Geminim/Siamond](User_Sivak_2_in_1_Geminim_Siamond.xhtml "User:Sivak/2-in-1 Geminim/Siamond")
  * [Garage Cart](http://nerdlypleasures.blogspot.com/2016/02/the-nes-garage-cart-father-of-nes.html) series
  * [Action 53](Action_53.xhtml "Action 53") a series of homebrew compilation releases



# See also

  * [Category:Multicart mappers](Category_Multicart_mappers.xhtml "Category:Multicart mappers")



# External links

  * [Multicart on BootlegGames](http://community.fandom.com/wiki/c:bootleggames:Multicarts "wikia:c:bootleggames:Multicarts")


