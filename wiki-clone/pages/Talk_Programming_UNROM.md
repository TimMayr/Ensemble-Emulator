# Talk:Programming UNROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AProgramming_UNROM) | View [other pages](Special_AllPages.xhtml#Talk_Programming_UNROM)

## NES 2.0 advocacy

Converting a ROM from original iNES to NES 2.0 isn't strictly necessary unless the mapper is ≥ 256 or the iNES header is ambiguous. For example: 

  * Several VRC mappers are ambiguous as to which low CPU address lines are connected to the mapper's port select.
  * Mapper 1 is ambiguous between SIROM (PRG A14 from MMC1 output) and other [SxROM](MMC1.xhtml "SxROM") boards using 32K PRG (PRG A14 from CPU A14).
  * Mappers 1, 4, and 5 are ambiguous among PRG RAM sizes. For example, SNROM vs. SOROM, MMC3 vs. MMC6.
  * A few games (notably _Low G Man_) require the PRG RAM to be absent.



What's ambiguous about mappers 2 (UxROM), 3 (CNROM), and 7 (AOROM), other than whether or not the ROM has a positive chip enable to avoid bus conflicts? --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 22:19, 26 February 2013 (MST) 

    Two things- 

  1. the comment about PRG-RAM: due to iNES's shortcomings one can't distinguish between 0="no ram" and 0="unspecified ram"; and
  2. the entire point behind NES2.0's backwards compatibility is the encouragement of its adoption without infringing on non-NES2.0-aware emulators. Creating mindshare makes it possible to say that it really is a solution.


    —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 01:50, 27 February 2013 (MST) 

    True, perhaps NES 2.0 might be warranted to keep programs from relying on an implied [PRG RAM circuit](PRG_RAM_circuit.xhtml "PRG RAM circuit") that iNES emulators emulate but which is not present on official mapper 2 boards. I'll see how I can work it in. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 11:45, 27 February 2013 (MST) 

    Leaving the ` .byte $00 ;UNROM has no PRG RAM` comment is explicitly misleading in light of the above.
    In any case, because of NES2.0's backwards compatibility, I don't see an argument for why we should recommend plain iNES headers. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 14:51, 27 February 2013 (MST) 

    If old iNES headers are used, the emulator ought to assume 8K PRG RAM for any mapper that doesn't already have PRG RAM and doesn't use $6000-$7FFF for anything else; this way it is compatible if something does add RAM there. If the header version bits specifies NES 2.0 then it ought to assume no RAM if the NES 2.0 header specifies that there is no RAM. (For example, I might want to make some UNROM game that includes PRG RAM and lacks CHR RAM; even the old iNES header probably supports this, isn't it? Some emulator might not support it so how do you deal with such things like this?) --[Zzo38](User_Zzo38.xhtml "User:Zzo38") ([talk](User_talk_Zzo38.xhtml "User talk:Zzo38")) 00:30, 24 June 2013 (MDT) 

    Do you mean UNROM with 8 KiB CHR ROM, sort of like what _[Morita Shogi](http://bootgod.dyndns.org:7777/profile.php?id=3479)_ does on the SNROM board? Technically that should be supported but I don't see the point. For one thing, it's two custom mask ROMs, one for PRG and one for CHR, instead of one and a cheap 6264. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 00:39, 24 June 2013 (MDT)
