# Talk:INES Mapper 047

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_047) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_047)

## Oversize definition

It would be nice to define this as "the latch controls the MSB of both PRG and CHR addresses" instead of always A17. This would make it trivial to get a RAMless oversize MMC3 with 1MiB PRG and 512KiB CHR. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 02:18, 15 September 2013 (MDT) 

    A competing oversize definition would be to keep each segment 128K and add more bits to the latch. For example 4 bits would give 16 banks, or 2 MiB PRG and 2 MiB CHR. --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 06:41, 15 September 2013 (MDT)
