# UNIF

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/UNIF) | View [other pages](Special_AllPages.xhtml#UNIF)

**UNIF** (Universal NES Image Format) is an alternative format for holding NES and Famicom ROM images. 

Its motivation was to offer more description of the mapper board than the popular [iNES 1.0](INES.xhtml "INES") format, but it suffered from other limiting constraints and a lack of popularity. The format is considered deprecated, replaced by the [NES 2.0](NES_2_0.xhtml "NES 2.0") revision of the iNES format, which better addresses the issues it had hoped to solve. 

There are a small number of game rips that currently [only exist as UNIF](UNIF_to_NES_2_0_Mapping.xhtml#So-far_unassigned_UNIF_board_names "UNIF to NES 2.0 Mapping"). 

## Contents

  * 1 History
  * 2 Format
    * 2.1 Types
  * 3 Shortcomings
  * 4 References
  * 5 See also



## History

The originator of the UNIF format, Tennessee Carmel-Veilleux, publically abandoned the format in December of 2008, and had its [website deleted](http://web.archive.org/web/20080827175203/http://www.parodius.com/~veilleux/). 

Since 2011 the UNIF standard has been maintained as part of the [libunif project](http://github.com/eteran/libunif), a library for loading and creating UNIF files. 

UNIF is currently considered a deprecated standard. Further updates to UNIF are unlikely, and it is recommended that new games or rips requiring special mapper specifications use the [NES 2.0](NES_2_0.xhtml "NES 2.0") format instead of attempting to extend UNIF. 

## Format

The suggested file extension for UNIF is `.unf` or `.unif`. 

("LE16" and "LE32" below denote little-endian 16/32-bit integers.) 

UNIF images start with a 32-byte header: 

Offset | Length (bytes) | Value   
---|---|---  
0 | 4 | "UNIF"  
4 | 4 | LE32, minimum version number required to parse all chunks in file   
8 | 24 | all nulls   
  
The header is followed by any number of Type+Length+Value blocks: 

Offset | Length (bytes) | Value   
---|---|---  
0 | 4 | Type, varies, defined below   
4 | 4 | LE32, length   
8 | length | content encoding varies by type   
  
### Types

The following Types are known: 

Type | Length | Minimum version required | Encoding | Content meaning   
---|---|---|---|---  
MAPR | variable | 1 | null-terminated UTF-8 | A unique human-readable identifier specifying the exact hardware used; **not** an iNES mapper number, and **not** a full text description of the mapper; required   
PRG _n_ | variable, usually power of two | 4 | raw | the contents of the _n_ th PRG ROM; at least PRG0 is required; _n_ is in hexadecimal   
CHR _n_ | variable, usually power of two | 4 | raw | the contents of the _n_ th CHR ROM   
PCK _n_ | 4 | 5 | LE32 | the CRC-32 of the _n_ th PRG ROM   
CCK _n_ | 4 | 5 | LE32 | the CRC-32 of the _n_ th CHR ROM   
NAME | variable | 1 | null-terminated UTF-8 | the name of the game   
WRTR | variable | unknown | null-terminated UTF-8 | unofficial, invalid. The name of the dumping software. Should be in a DINF chunk instead   
READ | variable | 1 | null-terminated UTF-8 | comments about the game, especially licensing information for homebrew   
DINF | 204 | 2 | special | Dumping information  | Offset | Length (bytes) | Value   
---|---|---  
0 | 100 | null-terminated UTF-8 dumper name   
100 | 1 | day-of-month of dump   
101 | 1 | month-of-year of dump   
102 | 2 | LE16, year of dump   
104 | 100 | null-terminated UTF-8 the name of the dumping software or mechanism   
TVCI | 1 | 6 | byte | TV standard compatibility information-  | 0. | NTSC only   
---|---  
1. | PAL only   
2. | compatible with both   
CTRL | 1 | 7 | byte | Controllers usable by this game (bitmask)  | 1. | [Standard controller](Standard_controller.xhtml "Standard controller")  
---|---  
2. | [Zapper](Zapper.xhtml "Zapper")  
4. | R.O.B.   
8. | [Arkanoid controller](Arkanoid_controller.xhtml "Arkanoid controller")  
16. | [Power Pad](Power_Pad.xhtml "Power Pad")  
32. | [Four Score](Four_player_adapters.xhtml "Four Score")  
64. | expansion (leave cleared)   
128. | expansion (leave cleared)   
BATR | 1 | 5 | byte | Boolean specifying whether the RAM is battery-backed.   
VROR | 1 | 5 | byte | "If this chunk is present, then the CHR-ROM area will be considered as RAM even if ROM is present."  
MIRR | 1 | 5 | byte | [What CIRAM A10 is connected to](Mirroring.xhtml "Mirroring"):  | 0. | PPU A11 (horizontal mirroring)   
---|---  
1. | PPU A10 (vertical mirroring)   
2. | Ground (one-screen A)   
3. | Vcc (one-screen B)   
4. | Extra memory has been added (four-screen)   
5. | Mapper-controlled   
  
## Shortcomings

Prior to 2013, no encoding was specified for any of the fields; 7-bit-clean ASCII was assumed, making NAME inadequate for the vast majority of non-US games. In the first quarter of 2013, [UTF-8 became the encoding](http://forums.nesdev.org/viewtopic.php?f=3&t=9883). 

Chunks can come in any order, so conventional patching tools cannot work without going through an "unpacked" intermediary stage. 

MAPR chunks are nominally supposed to use the text on the PCB, such as "NES-SNROM". However, some games have no identifying text on the PCB at all. Other games have only symbols in [Japanese](http://bootgod.dyndns.org:7777/profile.php?id=1762) or Chinese. Sometimes the [same PCB](UxROM.xhtml "UxROM") can have [different incompatible behavior](INES_Mapper_180.xhtml "INES Mapper 180"), depending on how things are populated or what things are jumpered. The workaround has been to add extra text in the MAPR chunk (in the _Crazy Climber_ case, ["HVC-UNROM+74HC08"](http://bootgod.dyndns.org:7777/profile.php?id=3869)). 

There is no ability to specify PRG RAM outside of the MAPR chunk. Two games using VRC4 ([Gradius II](http://bootgod.dyndns.org:7777/profile.php?id=1565) and [Bio Miracle Bokutte Upa](http://bootgod.dyndns.org:7777/profile.php?id=3988)) use the exact same PCB, but the former adds 2KiB PRG RAM and the latter adds none. 

For greater emulator compatibility, people sometimes use already-known-supported MAPR chunks to get something that's "close enough", rather than specifying a new MAPR for not-necessarily-identical behavior. 

BATR chunks do not disambiguate which RAM is battery-backed, if more than one is present. 

It's not clear exactly what VROR is supposed to mean—"Do not throw an error if this MAPR normally has CHR ROM but there are no CHR _n_ chunks, just give me 8KiB of CHR RAM"? "All the data I gave you for CHR-ROM, that was actually RAM, make it writeable"? As such, Nestopia, Nintendulator, and FCEUX just ignore it. 

CTRL chunks do not specify which controller should be plugged into which port, nor Famicom-only controllers, nor Super NES controllers plugged into a Famiclone or through an adapter (such as the [12-key controller](Standard_controller.xhtml "Standard controller") or the [mouse](Mouse.xhtml "Mouse")). 

There is no way to fully describe PlayChoice 10 or [Vs. System](Vs__System.xhtml "Vs. System") games. 

## References

  * [libunif project on Github](http://github.com/eteran/libunif)
  * [Last published version of the standard](http://raw.githubusercontent.com/eteran/libunif/master/UNIF_current.txt)



## See also

  * [UNIF to NES 2.0 Mapping](UNIF_to_NES_2_0_Mapping.xhtml "UNIF to NES 2.0 Mapping")



Categories: [File formats](Category_File_formats.xhtml)
