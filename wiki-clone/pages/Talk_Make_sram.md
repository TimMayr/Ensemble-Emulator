# Talk:Make sram

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AMake_sram) | View [other pages](Special_AllPages.xhtml#Talk_Make_sram)

## Super NES support

Blargg and others [requested](http://forums.nesdev.org/viewtopic.php?p=59070#p59070) adding support for Super NES games to make_sram.py. In order to determine when to create a .sav or .srm file, follow these rules: 

  * filename.lower().endswith(('.sfc', '.smc', '.swc', '.fig'))
  * ROM image size is a multiple of 32768 bytes. If the size is 512 bytes more than this, a copier header is present, and the ROM image proper starts at $200 in the file. Presence of a copier header is not correlated with the file name suffix.
  * The internal header is 32 bytes at $7FC0 (plus copier header) for LoROM or $FFC0 (plus copier header) for HiROM.
  * To make sure the header is a valid Super NES ROM image, at least two out of three should match: 
    * internal_header[$15] is $20/$30 for LoROM or $21/$31 for HiROM.
    * TV system is internal_header[$19] and should be less than $10.
    * internal_header[$1e] = internal_header[$1c] EOR $FF, and internal_header[$1f] = internal_header[$1d] EOR $FF.
    * The header contains a checksum of the entire ROM (subject to mirroring if the ROM is not a power of 2 bytes in size). But ROM hacks often fail this checksum especially when stacked, and ROM hacks are part of PowerPak's reason for existence, so just ignore it.
  * internal_header[$16] tells what components are on the board. Value $02 indicates a battery-backed PRG RAM and no coprocessor.
  * Size of PRG RAM is 1024 << internal_header[$18] bytes; this byte must be in range $01 through $05.



\--[Tepples](User_Tepples.xhtml "User:Tepples") 14:37, 27 March 2010 (UTC) 
