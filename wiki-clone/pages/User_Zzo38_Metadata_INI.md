# User:Zzo38/Metadata INI

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Metadata_INI) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Metadata_INI)

You can have a INI file, with the .nes.ini (or .unif.ini or .fds.ini or .qdi.ini) extension, to describe preferred controllers and other information about a game other than what is in the ROM image file. 

Anything in the file not understood by whatever program is reading this file, should be ignored. **Everything is optional** (both for writing and for reading; different blocks may be meaningful for different programs that would use this information). The reason for this is to avoid complicating an implementation, while still allowing features that someone may use. 

If numbers are expected in a value, they are normally decimal, and can be hexadecimal by `0x` prefix. 

(Note: For QDI format disk images, the INI file must correspond to the file for the bootable side.) 

## Contents

  * 1 Blocks
    * 1.1 Main
      * 1.1.1 Title
      * 1.1.2 Author
      * 1.1.3 URL
      * 1.1.4 Version
      * 1.1.5 TV
      * 1.1.6 License
      * 1.1.7 CRC
      * 1.1.8 Bootgod
      * 1.1.9 Text
      * 1.1.10 PPUID
    * 1.2 Controller
      * 1.2.1 Players
      * 1.2.2 Player1
      * 1.2.3 Player2
      * 1.2.4 Player3
      * 1.2.5 Player4
      * 1.2.6 Expansion
      * 1.2.7 FourScore
    * 1.3 License
      * 1.3.1 Name
      * 1.3.2 FreeSoftware
      * 1.3.3 OpenSource
      * 1.3.4 PublicDomain
    * 1.4 Mapper
      * 1.4.1 Name
      * 1.4.2 Number
      * 1.4.3 NumberEx
      * 1.4.4 Feature
      * 1.4.5 Mirror
      * 1.4.6 RAM
      * 1.4.7 BusConflict
      * 1.4.8 Audio
      * 1.4.9 Verilog
    * 1.5 Switch
    * 1.6 Emulator
    * 1.7 Dump
    * 1.8 Hardware
    * 1.9 EmuTest
    * 1.10 Tag
    * 1.11 File
    * 1.12 Memory
    * 1.13 Enhance
      * 1.13.1 PopReduction
      * 1.13.2 CueAudio
      * 1.13.3 PointingDeviceRAM
      * 1.13.4 SpriteLimit
      * 1.13.5 HighScore
      * 1.13.6 ScreenMode
      * 1.13.7 Overscan
    * 1.14 GameGenie
  * 2 Example



# Blocks

## Main

### Title

The title of this game. This does not have to be the same as the title in a UNIF file or the title added to the end of some iNES files. 

### Author

The name of the author of this game. 

### URL

URL to access information of this game. Any URI scheme is allowed, but `file:///` and LAN addresses is not recommended for files distributed on the internet. 

### Version

Game version number. 

### TV

Specify the TV mode supported, separated by vertical bars if there is more than one. 

  * `NTSC`: NTSC Famicom or NTSC NES.
  * `RGB`: RGB Famicom or Vs System.
  * `PAL`: PAL NES.
  * `Dendy`: Dendy PAL Famiclone.



Usually, a subset of this information is specified in the ROM image header. 

### License

Specify if the game is officially licensed. 

  * `Official` means it is licensed by Nintendo.
  * `Unofficial` means that it is not licensed by Nintendo; for example, anything published by Tengen, Codemasters, or Color Dreams, or homebrew software.



### CRC

Specify the CRC32 checksum. If it doesn't match, it is error (alternatives are to make it a warning, or to just ignore the rest of the INI file if the checksum doesn't match). 

### Bootgod

The ID number for the game in bootgod database. Emulators should not use klugy hacks to work with specific games if the INI file exists and does not have either the `Bootgod` entry specified or the emulator's entry in the `Emulator` block set. 

### Text

Absolute address into ROM image file of a null-terminatd text. It is not required for UNIF, although it is still allowed. 

### PPUID

The low five bits of the value which should be read from $2002. Normally this is read from the [NES 2.0](NES_2_0.xhtml "NES 2.0") header, but in case that is not available, it is specified here. 

## Controller

### Players

Tells how many players. Can be 1, 2, 3, 4, or can be multiples separated by vertical bars. 

### Player1

Specify the device connected to the first NES controller port. You can have multiple values separated by vertical bars; it can also be blank entries meaning nothing is connected. 

  * `Standard`: The [standard controller](Standard_controller.xhtml "Standard controller") of NES/Famicom.
  * `DoubleFisted`: The standard NES controller, rotated 90 degrees with the Control Pad at the top and A at the bottom like a Wii Remote.
  * `Mouse`: Super NES [Mouse](Mouse.xhtml "Mouse").
  * `LightGun`: [Zapper](Zapper.xhtml "Zapper").
  * `Arkanoid`: [Arkanoid controller](Arkanoid_controller.xhtml "Arkanoid controller") (NES version).
  * `SuperNintendo`: Super Nintendo standard controller.
  * `SuperPakPak`: Super PakPak analog controller.
  * `VsSystem`: VS System. Player 1 VS system controller must be specified in the player 2 slot and vice versa.
  * `VsLightGun`: VS System serial light gun. Player 1 VS system controller must be specified in the player 2 slot and vice versa.
  * `PowerPad`, `PowerPadB`, `FamilyTrainer`, `FamilyTrainerB`: Power Pad B side.
  * `PowerPadA`, `FamilyTrainerA`: Power Pad A side.
  * `PowerPadLeft`, `FamilyTrainerLeft`: Power Pad B side, rotated by 90 degrees so that the encoder is at the left.
  * `StandardDDR`: Dance mat which is using the protocol of the standard controller and allows Left+Right and Up+Down presses. Used by some NOAC-based dance mats.



### Player2

Specify the device connected to the second NES controller port. They are the same as `Player1` devices, except for the following: 

  * `Standard`: Does not include SELECT and START buttons.
  * `StandardSelectStart`: Does include SELECT and START buttons.
  * `StandardMicrophone`: Uses the microphone.



### Player3

Used if `Expansion=FourPlayers` or `FourScore=Yes` is specified. Must be a controller that uses D0 or D1, not D3-D4. 

### Player4

Used if `Expansion=FourPlayers` or `FourScore=Yes` is specified. Must be a controller that uses D0 or D1, not D3-D4. 

### Expansion

Specify the device connected to the Famicom expansion port. (In case of some devices that work on Player2 or the expansion port, it can be specified either way.) 

  * `Keyboard`: Famicom keyboard.
  * `OekaKids`: Oeka Kids tablet.
  * `KeyboardTape`: Famicom keyboard and tape recorder.
  * `FourPlayers`: Four players game (parallel multiplayer adapter). The `Player3` and `Player4` fields specify the controller for player 3 and 4 whose D0 is connected to the Famicom's D1.
  * `LightGun`, `FamilyTrainer`, `FamilyTrainerB`, `FamilyTrainerA`, `FamilyTrainerLeft`, `StandardDDR`: As above (considered as player 2 port). CaH4e3 claims that _Dance 2000 12-in-1_ uses `StandardDDR`.
  * `Arkanoid`: Arkanoid controller (Famicom version).
  * `HoriTrack`: Hori trackball.
  * `ExciteBoxing`: Exciting boxing.
  * `Mahjong`: Mahjong.
  * `Stereovision`: Famicom 3D glasses.



### FourScore

Specifies if NES Four Score (serial multiplayer adapter) is used. Values can be `No` and `Yes`. You can have multiple values separated by vertical bars. 

If `Expansion=FourPlayers` or `FourScore=Yes` is specified, the `Player1`, `Player2`, `Player3`, and `Player4` indicate device connected to the adapter (such as `Standard`, `DoubleFisted`, or `StandardDDR`). Devices using D2-D4 won't work on either adapter, and devices using a long report, such as the Super NES Controller and Super NES Mouse, work only with `Expansion=FourPlayers`, not `FourScore=Yes`. If a game specifies both a four-player adapter and a peripheral incompatible with the adapter, the emulator MUST provide a way to switch between the adapter and the other peripheral. 

## License

### Name

Name of a software license. 

### FreeSoftware

The value can be `No` and `Yes`. Specify if it meets the Free Software definition ([from FSF](http://www.gnu.org/philosophy/free-sw.html)). 

### OpenSource

The value can be `No` and `Yes`. Specify if it meets the Open Source definition ([from OSI](http://opensource.org/docs/definition.php)). 

Usually this value will be the same as the `FreeSoftware` value, but there are some exceptions. 

### PublicDomain

The value can be `No` and `Yes`. Specify if it is effectively public domain (e.g.: public domain, WTFPL, Unlicense, CC0). 

## Mapper

Information of mapper. 

### Name

Name of the mapper. You can have multiple values separated by vertical bars. 

### Number

The iNES [mapper](Mapper.xhtml "Mapper") number (0-255 for basic plane, 256-4095 for [NES 2.0](NES_2_0.xhtml "NES 2.0") planes), optionally followed by a dot and the submapper number (0-15 or A-F). If no submapper is specified, submapper 0 is assumed. You can have multiple values separated by vertical bars. If used with NES 2.0 format ROM image files, at least one of the numbers listed here MUST be the same as in the ROM image file (and will normally be the first one), if this line is used at all. In old iNES files, it is probably going to be the same, although in some cases it might not be (these cases are fixed by the NES 2.0 format). 

### NumberEx

Specifies extensions to the mapper which are not necessarily relevant for emulation (or which are already disambiguated by the ROM and RAM sizes). You can have multiple numbers separated by vertical bars, but usually you won't, since you will only specify the features actually used on this mapper (some games might have been released with multiple mapper settings though, for some reason). 

This is a sixteen-bit number. Its meaning depends on the first iNES mapper number specified in the `Number`; the low four bits have the same meaning as submapper numbers. 

### Feature

Specifies features used by the mapper, separated by vertical bars. These specify that a game will run correctly in a subset of a mapper, such as one that fits in a smaller CPLD or FPGA than the full mapper. An emulator is not required to care about this, but a debugging emulator MAY raise a stink if a program uses an unlisted feature. Reproduction carts using new EEPROMs with the original ASIC mapper on a donor board need not care about this either. The possible features depend on which mapper is in use: 

  * [MMC5](MMC5.xhtml "MMC5"): `SquareWave`, `PCM_Write`, `PCM_Read`, `PCM_Zero_IRQ`, `PRGmode0`, `PRGmode1`, `PRGmode2`, `PRGmode3`, `CHRmode0`, `CHRmode1`, `CHRmode2`, `CHRmode3`, `RAM_Protect`, `ExNametable`, `ExAttribute`, `ExRAM`, `ExRAM_Protect`, `ExRAM_Write_Zero`, `FillMode`, `Vsplit`, `Scanline_IRQ`, `Multiply`.
  * [MMC3](MMC3.xhtml "MMC3"): `PRGmode0`, `PRGmode1`, `CHRmode0`, `CHRmode1`, `RAM_Protect`, `Scanline_IRQ`



### Mirror

Set the mirroring. You can have multiple values separated by vertical bars. 

  * `H`: Horizontal.
  * `V`: Vertical.
  * `1`: One screen.
  * `4`: Four screen.
  * `?`: Anything other than specified here (usually this means dynamically controlled mirroring).



### RAM

Specify the amount of PRG RAM used in the cartridge, in bytes. If the file is [NES 2.0](NES_2_0.xhtml "NES 2.0") format, the number specified here MUST NOT be greater than the PRG RAM amount in the NES 2.0 header (although it is allowed to be less, but usually shouldn't be). 

### BusConflict

Specify if bus conflict is used. You can have multiple values separated by vertical bars. 

  * `No`: Require not bus conflict.
  * `Yes`: Require bus conflict.



### Audio

Use of expansion audio. Can be `No` and `Yes`. 

### Verilog

Version number of Verilog and/or extension of Verilog in use. You can have multiple values separated by vertical bars. This is only for use with mapper 768 submapper 1. 

## Switch

This block specifies the name and default settings of VS Unisystem switches. The names are given by `Name0` to `Name7`, and the default value is given by `Default0` to `Default7` which have the values `0` and `1`. 

## Emulator

Specify klugy features of specific emulators. The key is the name of the emulator; the value is not described here. 

## Dump

Information about who dumped the ROM image from the cartridge, what hardware/software is used, etc. 

## Hardware

Specify what NES/Famicom hardware and hardware clone it is tested with. The values are: 

  * `Work`: It works.
  * `NotWork`: It doesn't works.
  * `Partial`: It works partially; probably it is good enough.
  * `Untested`: It is not tested.



The keys are (not limited to): 

  * `Nintendo`: The official hardware from Nintendo. (It ought to work!)



## EmuTest

Same as the `Hardware` block, but for emulators. You can also specify version numbers in this block, separated by vertical bars. 

## Tag

Miscellaneous metadata for use with search engines. (The format is not currently specified here.) 

## File

This block indicates what files are included, and what purpose of the files. The keys are the filename, and the values specify what they mean. You can have multiple values separated by vertical bars. 

  * `ROM`: ROM image file; usually implied (so is not needed).
  * `Documentation`: Documentation in HTML, XHTML, DocBook, DVI, DjVu, PDF, PostScript, or PCL format.
  * `DocumentationASCII`: Documentation in plain ASCII text. (This is separate from the above since plain ASCII has no "magic number" to detect, unlike all the above formats.)
  * `MetaFont`: METAFONT file for fonts used in DVI.
  * `SourceCode`: Source codes.
  * `Disk`: A disk image (in QDI format; not needed for FDS format).
  * `Icon`: Icons for use with iPhone, Android, etc.
  * `License`: License file (normally in plain ASCII text).
  * `ADPCM`: ADPCM waveform data for expansion audio.
  * `Label60`: Label for 60-pins cartridge.
  * `Label72`: Label for 72-pins cartridge.
  * `Boxart`: Box art.
  * `BIOS`: FDS BIOS ROM. (Normally there is no need to use this type even for FDS games, since the BIOS is not normally included with the disk image. However, it might be helpful in some cases.)
  * `Palette`: Recommended RGB palette data (for RGB mode only). It is not necessarily going to be used, but it can improve the color slightly.
  * `Enhance2xCHRROM`: Double-resolution CHR ROM for enhancement.
  * `EnhanceData`: Miscellaneous files for enhancement, refer by other files such as cue files.
  * `EnhanceAudioCue`: Cue file for telling what sounds to play when values are written to RAM. (TODO: Specify format of this file.)
  * `EnhanceTiles`: File containing full-color tiles mapping to CHR ROM bank/attribute pairs. (TODO: Specify format of this file.)
  * `InstrumentsFM`: File of exactly 120 bytes containing OPLL-format FM instruments numbered 1 to 15. (There is actually only one set of instruments, but unfortunately they may be unknown; this file may be provided if the author of a homebrew VRC7 game does not know the correct settings and/or believes it sounds best with specific settings.)
  * `SaveGame`: Default initial contents of battery RAM. (PRG RAM comes first if the cartridge uses battery RAM for both PRG and CHR)
  * `DatachEEPROM`: Shared EEPROM data for mapper 157; writable (and may not be distributed with the game). (If not specified, the emulator should create its own file.)



## Memory

Specify the memory mapping of the ROM and RAM for this game, for purpose of debugging, cheat codes, ROM hacks, etc. (TODO: need to specify details) 

## Enhance

This block lists enhancements that might be used in some emulators (the emulator should offer an option to turn them off, even if they are supported, though). Some may work on hardware too, but that does not usually affect the cartridge, so this block is probably not used when loading on hardware. 

### PopReduction

Values can be `No` or `Yes`. You can have multiple values separated by vertical bars. 

### CueAudio

Values can be `No` or `Yes`. You can have multiple values separated by vertical bars. 

### PointingDeviceRAM

This is used to write the coordinates of a pointing device into RAM. There are multiple values separated by commas, as follows: 

  * RAM address for X coordinate.
  * RAM address for Y coordinate.
  * Leftmost coordinate.
  * Rightmost coordinate.
  * Topmost coordinate.
  * Bottommost coordinate.



### SpriteLimit

Values can be `8`, `15`, or `NoLimit`. You can have multiple values separated by vertical bars. 

8
    Normal sprite evaluation. Clear secondary OAM in 1-64, [evaluate](PPU_sprite_evaluation.xhtml "PPU sprite evaluation") starting at 65, fetch patterns every 8 dots starting at 261.
15
    Optimized sprite evaluation. Clear secondary OAM in 1-30, evaluate starting at 31, fetch patterns every 4 dots starting at 261.
NoLimit
    No limit of sprites on one scanline. If evaluation or fetch overflows the allotted time, mapper side effects are disabled until it completes.

### HighScore

This is a list of RAM addresses separated by vertical bars, which store high score data. The emulator might choose to somehow make them persist, or to do other things with it. 

### ScreenMode

Values can be `Normal`, `2xCHRROM`, `Tiles`, `Overlay2xPPU`, `Super2xSaI`, `Scale2x`, or `hq2x`. You can have multiple values separated by vertical bars. 

### Overscan

Four numbers separated by commas. The first is the number of top lines that can be safely cut off, second is bottom lines, third is left side, fourth is right side. (It should not assume that they necessarily will be cut off; only that it can be.) 

## GameGenie

Contains unencoded [Game Genie](http://tuxnes.sourceforge.net/gamegenie.html) codes. The keys are the name of the code, and the value are one or more 32-bit numbers separated by vertical bars (normally written in hex format, with `0x` prefix). 

  * Bit14-bit0: Low 15 bits of address. High bit is always set.
  * Bit15: Set if this is a compare code.
  * Bit23-bit18: Value to read from that address.
  * Bit31-bit24: If it is a compare code, the value will not be changed unless it matches the value placed here.



# Example

(Note: This is a old example! The stuff above is correct; not necessarily as shown in this example!) 
    
    
    [Main]
    Title="Famicom Hangman"
    TV=NTSC|RGB ; NTSC and RGB both work (emphasis bits are not used)
    License=Unofficial ; Not officially license by Nintendo
    
    [Controller]
    Players=1 ; Single player game
    Player1= ; Player 1 controller is explicitly not used.
    Expansion=Keyboard ; Using Famicom keyboard
    Player2=|StandardMicrophone ; Player 2 controller is optional, and can use the microphone if available
    
    [License]
    Name="Public domain"
    Watermark=No ; Copies are not watermarked
    Copy=Allow
    Modify=Allow
    Commercial=Allow
    
    [Mapper]
    Mirror=H|V|1|4 ; Program doesn't care about nametablem mirroring
    RAM=0 ; PRG RAM is not used (this line is redundant if the ROM image is NES 2.0)
    BusConflict=No|Yes ; Program doesn't care if bus conflicts are used or not
    
