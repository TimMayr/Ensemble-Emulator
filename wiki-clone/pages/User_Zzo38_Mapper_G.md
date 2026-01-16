# User:Zzo38/Mapper G

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/User%3AZzo38/Mapper_G) | View [other pages](Special_AllPages.xhtml#User_Zzo38_Mapper_G)

  
This is a generic [GNROM](GxROM.xhtml "GNROM")-like mapper. The NES 2.0 header is required for its use. 

There must be at most one of CHR-ROM, CHR-non-battery-RAM, CHR-battery-RAM. There also must be at most one of PRG-non-battery-RAM and PRG-battery-RAM. If no CHR ROM/RAM is present, then CIRAM is used, similar to [iNES Mapper 218](INES_Mapper_218.xhtml "INES Mapper 218"), but without the one-screen modes. 

PRG RAM can be mapped at $6000-$7FFF if it exists. If there is more than 8K, the CHR bank numbers are used to select PRG RAM banks (in addition to CHR banks, if applicable; it doesn't matter if one is more than the other or less or equal). 

The high bit of the submapper number (bit3) is used to designate whether PRG or CHR is shifted (PRG if set, CHR if cleared); the other is starting the low bit on the right side. 

The low three bits of a submapper number tells how much it is shifted by, from zero to seven. It is possible to overlap. If it overflows on the left then it wraps around. 

If PRG or CHR has no bankswitching but is shifted anyways, then that bit position instead sets nametable mirroring mode: If horizontal mirroring is specified in header then it selects CIRAM A10 directly; if vertical mirroring is specified in the header then it specifies if connected to PPU A10 (if clear) or PPU A11 (if set). 

It is not allowed to shift unbankswitched registers if both four-screen mirroring and CHR ROM/RAM exists, since it would not do anything in such a case. 

Submapper 0 is reserved and should not be used. 

A second "G'" mapper is defined which uses 16K PRG bankswitching, with $C000-$FFFF fixed to the last bank. Everything else is the same as above. 

Example: 

  * [GNROM](GxROM.xhtml "GNROM") = G.12
  * [Color Dreams](Color_Dreams.xhtml "Color Dreams") = G.4
  * [iNES Mapper 107](INES_Mapper_107.xhtml "INES Mapper 107") = G.9
  * [BNROM](INES_Mapper_034.xhtml "BNROM") = G.8
  * [AxROM](AxROM.xhtml "AxROM") = G.4
  * [CNROM](CNROM.xhtml "CNROM") (no security diodes and audio) = G.8
  * [UxROM](UxROM.xhtml "UxROM") = G'.8
  * [iNES Mapper 070](INES_Mapper_070.xhtml "INES Mapper 070") = G'.12
  * [iNES Mapper 094](INES_Mapper_094.xhtml "INES Mapper 094") = G'.10


