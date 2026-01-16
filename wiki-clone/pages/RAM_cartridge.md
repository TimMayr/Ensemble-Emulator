# RAM cartridge

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/RAM_cartridge) | View [other pages](Special_AllPages.xhtml#RAM_cartridge)

Although the NES/Famicom was originally designed to only accept cartridges containing game code and data in read-only memory chips ([ROM](ROM.xhtml "ROM")), both Nintendo and unlicensed third parties released cartridges that allowed games to be played from RAM chips, with data loaded from floppy disks or solid-state flash cards controlled by a small ROM-based BIOS. 

## Contents

  * 1 Licensed RAM cartridges
  * 2 Unlicensed RAM cartridges
    * 2.1 First generation
    * 2.2 Second generation
    * 2.3 Third generation
    * 2.4 Fourth generation
  * 3 See also



# Licensed RAM cartridges

The [Famicom Disk System](Family_Computer_Disk_System.xhtml "Famicom Disk System")'s RAM Adapter provides 32 KiB of PRG-RAM and 8 KiB of CHR-RAM. Data is loaded from Mitsumi QuickDisk-format 2.8 inch floppy disks. Games have to be specifically written for the FDS RAM Adapter's memory map, for saving game progress to floppy disk, and if more than 32 KiB of PRG or 8 KiB of CHR data are to be used, for loading additional data from disk. 

# Unlicensed RAM cartridges

All unlicensed RAM cartridges are primarily designed to run games that originally were released on ROM cartridges. As a consequence, they closely mimic the memory map of ROM cartridges, load the entire game code and data at boot and do not load any data afterwards. 

## First generation

Early unlicensed RAM cartridges had 128 or 256 KiB of PRG-RAM and 32 KiB of CHR-RAM. They could directly mimic discrete mappers such as [UNROM](UxROM.xhtml "UNROM"), [CNROM](CNROM.xhtml "CNROM") and [GNROM](GxROM.xhtml "GNROM"). Games that originally ran on ASIC mappers such as the Nintendo [MMC1](MMC1.xhtml "MMC1") or [MMC3](MMC3.xhtml "MMC3") had to be modified to run on these RAM cartridges. If the games used more than 32 KiB of CHR-ROM, or banked it in amounts smaller than 8 KiB, then further modification was necessary, using a technique of _caching_ the most recently-used 32 KiB of CHR-ROM. 

These devices have been called "copiers" in the early NES emulation scene, even though none of them actually have the capability to copy a ROM cartridge game to floppy disk.[1]. Instead, users had to purchase (or copy) suitable disks from the manufacturer of the RAM cartridge or some other company close to it. Disks were usually specific to a particular RAM cartridge manufacturer and model, with later devices being backwards-compatible only to models by the same manufacturer and the original _Bung Game Doctor_ , often containing detection code to specifically lock-out competing RAM cartridges. 

All of these devices are connected _between_ the Famicom main unit and the FDS RAM adapter. The loading process is based on the process of loading a normal FDS game: the original FDS BIOS starts loading a game once a disk has been inserted. Disks targeting these RAM cartridges will contain a file that is loaded by the FDS BIOS to a CPU address unused by the FDS RAM adapter, but that when written to switches in the RAM cartridges's own BIOS and memory map. 

  * _Bung Game Doctor_ (in 128 KiB and 256 KiB PRG-RAM versions)
  * _Venus Game Converter_
  * _Front Fareast Magicard_ (adds 8 KiB PRG-"ROM" banking mode)
  * _Bung Super Game Doctor_ (in 256 KiB and 512 KiB versions; adds 8 KiB PRG-"ROM" banking mode and cycle-based IRQ counter)
  * _Venus Turbo Game Doctor 4M_ (512 KiB PRG-RAM, adds 8 KiB PRG-ROM banking mode, cycle-based IRQ counter, and real-time saving capability)



## Second generation

Later unlicensed RAM cartridges increase the PRG-RAM size to 512 KiB, and the CHR-RAM size to 256 KiB bankable in amounts smaller than 8 KiB. Games still had to be modified to run on these devices. 

  * _Venus Turbo Game Doctor 6M/6+_
  * _Front Fareast[Super Magic Card](Super_Magic_Card.xhtml "Super Magic Card")_. Adds PA12-based IRQ counter. Can also load games from a 3.5 inch MS-DOS-format floppy disk drive (720 KiB or 1.44 MiB), or from a parallel interface connected to a PC with CD-ROM drive.



## Third generation

The _Bung Game Master_ has a Xilinx XC3042 FPGA to mimic most ASIC-based mappers, no longer requiring modifications to the games themselves. The mapper-specific fusemap is loaded from floppy disk (called "preboot disk"), just like the games themselves. It can load games from 2.8 inch QuickDisks like earlier models, or from a 3.5 inch MS-DOS-format floppy disk drive. 

## Fourth generation

Modern RAM cartridges such as the [PowerPak](PowerPak.xhtml "PowerPak") or [Everdrive N8](Everdrive_N8.xhtml "Everdrive N8") are similar to the _Bung Game Master_ in using an FPGA to mimic almost any mapper hardware, but load games and fusemaps from solid state flash cards exclusively. For this reason, they are called "flashcarts". 

# See also

  * [Info on various Famicom "copiers"](http://www.famicomdisksystem.com/game-doctor-copiers/)



  1. ↑ The contemporary term in their East Asian target markets was "RAM disk", while actual copiers were called "wild card".


