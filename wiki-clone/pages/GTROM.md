# GTROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/GTROM) | View [other pages](Special_AllPages.xhtml#GTROM)

**Cheapocabra (GTROM)**

**Company** | Membler Industries   
---|---  
**Boards** | GTROM   
**PRG ROM capacity** | 512K   
**PRG ROM window** | 32K   
**PRG RAM capacity** | None   
**CHR capacity** | 16K   
**CHR window** | 8K   
**Nametable[mirroring](Mirroring.xhtml "Mirroring")** | 4 fixed   
**[Bus conflicts](Bus_conflict.xhtml "Bus conflict")** | No   
**IRQ** | No   
**Audio** | No   
**iNES[mappers](Mapper.xhtml "Mapper")** | [111](GTROM.xhtml "INES Mapper 111")  
  
  
**Cheapocabra** or **GTROM** is a self-flashable cartridge board intended for homebrew releases with its own unique mapper assigned to **iNES Mapper 111**. The board and mapper were designed by Membler Industries which also manufactures the cartridges. 

Releases: 

  * _Anguna: Scourge of the Goblin King_
  * _Candelabra Estoscerro_ (also available as [UNROM512](UNROM_512.xhtml "UNROM512"))
  * _The Incident_
  * _NEScape_
  * _Project Blue_ (First US batch of the kickstarter fulfillment. Later and international batches use [BNROM](INES_Mapper_034.xhtml "BNROM"))
  * _Spook-o-tron_
  * _Storied Sword_



## Contents

  * 1 Overview
  * 2 Registers
    * 2.1 Bank select ($5000-$5FFF, others)
  * 3 Banks
    * 3.1 PRG ROM (CPU $8000-$FFFF)
      * 3.1.1 Self-flashing
    * 3.2 PPU RAM
      * 3.2.1 CHR RAM (PPU $0000-$1FFF)
      * 3.2.2 VRAM (PPU $2000-$3EFF)
        * 3.2.2.1 Bonus RAM
  * 4 Emulator Support
    * 4.1 Mesen
    * 4.2 FCEUX
    * 4.3 PowerPak
    * 4.4 EverDrive
  * 5 Hardware Teardown
  * 6 Variations
    * 6.1 GTMP3
    * 6.2 Famiclone-compatible mode
    * 6.3 Ninja Ryukenden Chinese Fan Translation
  * 7 See also



## Overview

The Cheapocabra board and mapper were developed in 2015 by Memblers to meet the needs of the lower end of the homebrew cartridge market. This includes games that would have historically used an [NROM](NROM.xhtml "NROM") board as well as other simpler boards which provide some kind of bank switching but lacked more advanced features like IRQs. The underlying philosophy is that stocking larger numbers of a single board design allows for lower costs by economies of scale when compared to stocking smaller individual quantities of a suite of lower end boards all with similar capabilities. 

The name Cheapocabra is a play on [Chupacabra](https://en.wikipedia.org/wiki/Chupacabra "wikipedia:Chupacabra"), a creature from American folklore, where the similar sounding first syllable is replaced with the word "cheap" to indicate the intended low cost of the hardware. The GTROM moniker, meaning "Goat ROM," is derived directly from this as Chupacabra translates literally as "goat-sucker" and the Chupacabras of legend are known for attacking goats and other livestock. 

## Registers

### Bank select ($5000-$5FFF, others)

The register is mapped to the CPU addresses **$5000-$5FFF** and **$7000-$7FFF** and writing a byte to any of these addresses all have the same result. Programs most commonly use **$5000** for writing to the register but any of the others work equally well. Note that reading from the register effectively writes the value of open bus, so bitwise manipulation will require keeping a copy of the mapper register's value in RAM. 
    
    
    7  bit  0
    ---- ----
    GRNC PPPP
    |||| ||||
    |||| ++++- Select 32 KB PRG ROM bank for CPU $8000-$FFFF
    |||+------ Select 8 KB CHR RAM bank for PPU $0000-$1FFF
    ||+------- Select 8 KB nametable for PPU $2000-$3EFF
    |+-------- Red LED - 0=On; 1=Off
    +--------- Green LED - 0=On; 1=Off
    

There are two LEDs on the board; one red and one green. They can be independently turned on and off using the register at any time. The LEDs are potentially useful as a simple means of communicating debug information or some kind of indicator for the player. Another approach is to turn the light off during processing and on during idle resulting in the brightness of the light being a rough indicator of available CPU time. 

## Banks

### PRG ROM (CPU $8000-$FFFF)

512 KB of flash memory is mapped to a 32 KB window located at **$8000-$FFFF** in CPU address space providing the PRG ROM for the cartridge. The full 512 KB is divided into sixteen 32 KB pages which can be selected by writing the index number of the desired page to the lower nybble of the control register located at $5000. No portion of the bank is fixed and no portion of the pages overlap and so switching pages results in all the addresses in the bank being redirected to another unique location on the flash chip. 

The lack of a fixed bank may make development more difficult since care must be taken to ensure that the next instruction of the program to execute after the register update will be located at the next address in the page that is being switched in. If the new page contains something else at that address then the program will likely crash since the CPU will still proceed to execute it anyway. The simplest way to handle this is to create a faux fixed bank by duplicating the same data at the same offset within all 16 pages. A common approach is to imitate a [UxROM](UxROM.xhtml "UxROM") style layout by duplicating the same 16 KB of data at $C000-$FFFF but this effectively reduces the size of the ROM to 272 KB (16 KB * 16 pages + 16 KB fixed). Memblers has released a command line utility called the [Fixed Bank Creator](https://forums.nesdev.org/viewtopic.php?f=2&t=18035) that can be used to duplicate data across multiples pages in a variety of configurations. 

#### Self-flashing

Like [UNROM 512](UNROM_512.xhtml "UNROM 512"), GTROM uses a SST39SF040 flash chip as the PRG ROM storage device. Both of these mappers lack PRG RAM for game saves but the configuration of both boards make it possible to erase and rewrite sections of the chip using NES software thus allowing games to store saved games and other variable information on the "ROM" chip. The saved data actually becomes part of the ROM image since its just stored alongside the rest of the program and game data on the flash chip. 

Rewriting is accomplished by writing specific sequences of bytes to specific addresses on the chip to unlock and perform a particular action such as erasing a sector or writing a byte value. The particular sequences are documented on the [data sheet](https://www.microchip.com/wwwproducts/Devices.aspx?product=SST39SF040) for the chip. The addresses given on the data sheet are in addresses on the chip and thus **5555h** corresponds to CPU **$D555** and **2AAAh** corresponds to CPU **$AAAA** regardless of the PRG page selected. 

NES programs must execute from RAM while erasing and writing to the chip since the CPU fetching instructions from it would interfere with the command sequence. Writing data to a sector is accomplished by first erasing the sector and then writing each byte in turn which requires writing the appropriate command sequence to the chip each time. This would be similar to the [example code](https://www.nesdev.org/w/images/default/f/fa/Flash.asm.txt "Flash.asm.txt") for UNROM except that the bank switches and addresses would need to be adjusted to match GTROM's requirements. 

### PPU RAM

The board contains a 32KiB RAM. This RAM is always enabled, and the NES's internal 2KiB CIRAM is always disabled. 

#### CHR RAM (PPU $0000-$1FFF)

The [pattern tables](PPU_pattern_tables.xhtml "PPU pattern tables") can select between the first two 8KiB of the PPU RAM, vaguely like [mapper 87](INES_Mapper_087.xhtml "INES Mapper 087"). The page is selected by setting or clearing bit 4 of the control register. 

All of the [normal advantages](CHR_ROM_vs__CHR_RAM.xhtml "CHR ROM vs. CHR RAM") of CHR RAM over CHR ROM apply to this board such as being able to dynamical update tiles between frames for animations and other effects. Since the RAM bank is paged it also has the same kind of "flip book" capabilities often associated with CHR ROM. 

#### VRAM (PPU $2000-$3EFF)

The [nametables](PPU_nametables.xhtml "PPU nametables") can select between the last two 8KiB of the PPU RAM. This provides unique memory in both banks for all 4 nametables. This means the mapper always uses 4-screen [mirroring](Mirroring.xhtml "Mirroring") and can neither use nor benefit from a switch to control the nametable arrangement. 

Use bit 5 of the control register to select the specific page. 

##### Bonus RAM

The nametable RAM on the board covers not only the full nametable [address range](PPU_memory_map.xhtml "PPU memory map"), but also the next 4KiB. Normally, PPU $3000-$3EFF is just a mirror of $2000-$2EFF and so is not usually useful. 

On this board these addresses are not mirrored and instead are wired to real, independent addresses on the RAM chip. Since these PPU addresses aren't actually used by the system this creates some additional RAM space that can be used for general purposes. Both pages in this bank contain their own copy of this bonus RAM so the total is almost 8 KB split evenly across the two pages. 

However, the last 256 bytes are unwriteable because [palette RAM](PPU_palettes.xhtml "PPU palettes") is mapped to $3F00-$3FFF. 

## Emulator Support

Emulator support for this mapper is currently spotty. Most were designed before GTROM and have been slow to add support but this may be a chicken-or-the-egg problem with the current lack of many freely distributed ROM images that use the mapper. Some of the implementations that are available are incomplete since they commonly lack support for the bonus RAM or self-flashing capabilities. The LED lights on the board are universally unsupported at this time. 

### Mesen

[Mesen](https://www.mesen.ca/) added full support for GTROM as of [version 0.9.0](https://www.mesen.ca/#Changelog) (released May 13, 2017). 

### FCEUX

Since version 2.3.0 FCEUX has officially supported GTROM. 

Prior to that release, it had been available in interim builds for several years. On Memblers' homepage is [an interim build](https://membler-industries.com/nes/fceux-r3293.zip) with GTROM. 

This implementation does not support the "bonus RAM" at PPU $3000-$3EFF because the architecture of the emulator makes implementing this infeasible. The [source code](https://github.com/TASVideos/fceux/blob/master/src/boards/cheapocabra.cpp) is available on GitHub. 

### PowerPak

[PowerPak](PowerPak.xhtml "PowerPak") has had a [3rd-party mapper](https://forums.nesdev.org/viewtopic.php?f=9&t=16631) made to support GTROM. The current implementation supports bonus RAM but does not support self-flashing. 

### EverDrive

[Everdrive N8](Everdrive_N8.xhtml "Everdrive N8") has had a [3rd-party mapper](https://forums.nesdev.org/viewtopic.php?f=9&t=18841) made to support GTROM, based on the PowerPak mapper. As such, the current implementation supports bonus RAM but does not support self-flashing. 

## Hardware Teardown

The specific hardware on this board is a [74377](74377.xhtml "74377"), a [7402](7402.xhtml "7402"), and a [7410](7410.xhtml "7410"). 

Memblers shared the [schematic here](https://forums.nesdev.org/viewtopic.php?p=226364#p226364). 

The functions evaluated by the logic include: 

  * CHRRAMA14 = **N** if (PPUA13==1) otherwise **C**
  * PRGROM/OE = NOT(R/W)
  * Latch on falling edge of M2, if /ROMSEL, A14, and A12 all high.



Note that reads from the latch will write the value of open bus to the latch, which includes dummy reads from indexed addressing. 

## Variations

### GTMP3

A new variant of this mapper known as [GTMP3](https://docs.google.com/document/d/108LuUaH3KgHwvCuID_jHYiMAcHVZ-3EHLE9EOahEzTk) was introduced in October 2019. The new version adds an audio expansion and is mostly backward compatible with the original. Memblers intends to continue to make the original GTROM board available for purchase since it's still less expensive to produce than the enhanced MP3 version. 

This adds a YX5200 coprocessor which can be instructed by NES code to play MP3 or WAV audio stored on a microSD card. It is controlled using asynchronous serial emitted on bit 7 of the mapper's register, instead of the green LED. Code designed to run from a classic GTROM board should otherwise be fully compatible with GTMP3. [Example code](https://github.com/Memblers/mp3player) is available on GitHub. 

### Famiclone-compatible mode

Many Famiclones don't permit the cartridge to replace the nametables, sometimes even using the relevant pins to detect cartridge insertion and replace the built-in games. A set of solder jumpers are available to reconfigure this to instead act like a self-flashable [GNROM](GxROM.xhtml "GNROM") board with four 8KB banks of CHR RAM and fixed nametable mirroring. Nothing has yet been released using this mode, and when found it should probably be allocated a new mapper number. 

### Ninja Ryukenden Chinese Fan Translation

Prior to the introduction of GTROM, Mapper 111 was assigned to a Chinese Fan Translation of Ninja Ryukenden (Japanese Ninja Gaiden). This translation uses a non-serialized version of MMC1 and supports 256KiB of CHR-ROM, whereas the official MMC1 is limited to 128KiB. This assignment can coexist with GTROM provided the emulator recognizes that the translation uses CHR-ROM and therefore must emulate the MMC1 variant instead of GTROM hardware. 

## See also

  * [Membler Industries in 2015](http://forums.nesdev.org/viewtopic.php?p=146039) \- forum post describing the board.
  * [BNROM](INES_Mapper_034.xhtml "BNROM") \- The easiest other mapper to convert to/from, so long as you don't use GTROM's unique features.
  * [UNROM 512](UNROM_512.xhtml "UNROM 512") \- A very similar homebrew mapper, but with a 16k + fixed PRG configuration.
  * [How I learned to play The Incident and other GTROM favorites sans the cartridge](https://bacteriamage.wordpress.com/2019/03/01/how-i-learned-to-play-the-incident-and-other-gtrom-favorites-sans-the-cartridge/) \- Blog post describing patching some popular GTROM games to run from other mappers ([patches](https://bacteriamage.wordpress.com/2019/03/10/patch-scripts-released-for-the-incident-scramble-and-khan-4-in-1/))



Categories: [GNROM-like mappers](Category_GNROM_like_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with flash save](Category_Mappers_with_flash_save.xhtml)
