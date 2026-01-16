# Talk:INES Mapper 087

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Talk%3AINES_Mapper_087) | View [other pages](Special_AllPages.xhtml#Talk_INES_Mapper_087)

## Mapper 87 vs 101

Just for the sake of argument, I'd like to outline my reasoning. Only one of the following three things can be true: 

  1. Mapper 87 describes only the 16 KiB CHR games, with the 2s bit controlling bank, and mapper 101 describes the 32 KiB CHR games, with the bits in order.
  2. Mapper 87 describes both, where the 32 KiB configuration has the bits out of order, and mapper 101 was erroneously assigned.
  3. There are two different 100% correct dumps of the exact same cartridge.



I object to #3. GoodNES and other sources have decided on #2. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 01:35, 25 April 2014 (MDT) 

    Any CNROM image will run when changed to GNROM, and any NROM image will work when changed to CNROM or GNROM. Heck, the only existing commercial BNROM game will work on Color Dreams. Whether to go with #1 or #2 depends on whether the community wants to deprecate 87/101 for new development in favor of CNROM (#2) or not (#1). --[Tepples](User_Tepples.xhtml "User:Tepples") ([talk](User_talk_Tepples.xhtml "User talk:Tepples")) 08:58, 25 April 2014 (MDT)

    

    But conceptually, a copy of (e.g.) SMB marked as GNROM isn't correct. Perhaps I should have said "canonical" instead of "100% correct". Anyway, as for the last point, there's no reason to pick either m87 or m101 anymore: they're inferior to CNROM because a bus-conflict prevention table is cheaper; or as INL pointed out, just connecting R/W to /WE (while tying /OE low) on most Flash will prevent bus conflicts; or even if it doesn't, a transistor or 74'1g04 is cheaper than a 74'20 or 74'138. —[Lidnariq](User_Lidnariq.xhtml "User:Lidnariq") ([talk](User_talk_Lidnariq.xhtml "User talk:Lidnariq")) 13:59, 25 April 2014 (MDT)

I also don't get the difference between 87 and 101, because who cares if the bits are in order or not - that's equivalent to exchange banks 1 and 2 in the CHR-dump (or exchange A13 and A14 when dumping the CHR-ROM). But how relevant is it that the banks in the iNES (or whathever other format) dump is in the same order as in the CHR-ROM itself physically ? If this is relevant then the 87 and 101 difference makes sense, but I don't think it's relevant, and therefore I don't think the difference makes sense.[Bregalad](https://www.nesdev.org/w/index.php?title=User:Bregalad&action=edit&redlink=1 "User:Bregalad \(page does not exist\)") ([talk](User_talk_Bregalad.xhtml "User talk:Bregalad")) 02:43, 15 May 2014 (MDT) 
