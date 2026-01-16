# TNES

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/TNES) | View [other pages](Special_AllPages.xhtml#TNES)

The **TNES** file format was discovered in Nintendo 3DS Virtual Console games. It fulfills much the same role as [iNES](INES.xhtml "INES") format, but it can also encapsulate Quick Disk images for FDS and avoids [allegations of having misappropriated a format from the emulation scene](https://forums.nesdev.org/viewtopic.php?f=5&t=15403). 

Offset | Contents   
---|---  
0-3 | "TNES" (54 4E 45 53)   
4 | Mapper (TNES codes, not iNES codes)   
5 | PRG ROM size in 8192 byte units   
6 | CHR ROM size in 8192 byte units   
7 | WRAM (0 for none, 1 for 8192 bytes)   
8 | Mirroring (0 mapper-controlled; 1 horizontal; 2 vertical)   
9 | Battery (0 none; 1 battery present)   
10-11 | Used only for FDS images; purpose unknown, possibly side count   
12-15 | Zero   
  
[Mapper](Mapper.xhtml "Mapper") numbers differ, and Nintendo's own ASIC mappers have a contiguous block: 

TNES mapper | iNES mapper | Meaning | Notes   
---|---|---|---  
0 | [0](NROM.xhtml "INES Mapper 000") | [NROM](NROM.xhtml "NROM") |   
1 | [1](MMC1.xhtml "INES Mapper 001") | SxROM ([MMC1](MMC1.xhtml "MMC1")) |   
2 | [9](MMC2.xhtml "INES Mapper 009") | PNROM ([MMC2](MMC2.xhtml "MMC2")) |   
3 | [4](MMC3.xhtml "INES Mapper 004") | TxROM ([MMC3](MMC3.xhtml "MMC3")) | And presumably HKROM (MMC6)   
4 | [10](MMC4.xhtml "INES Mapper 010") | FxROM ([MMC4](MMC4.xhtml "MMC4")) |   
5 | [5](MMC5.xhtml "INES Mapper 005") | ExROM ([MMC5](MMC5.xhtml "MMC5")) |   
6 | [2](UxROM.xhtml "INES Mapper 002") | [UxROM](UxROM.xhtml "UxROM") |   
7 | [3](CNROM.xhtml "INES Mapper 003") | [CNROM](CNROM.xhtml "CNROM") |   
9 | [7](AxROM.xhtml "INES Mapper 007") | [AxROM](AxROM.xhtml "AxROM") |   
31 | [86](INES_Mapper_086.xhtml "INES Mapper 086") | Jaleco JF-13 |   
100 |  | [FDS](Family_Computer_Disk_System.xhtml "FDS") | ROM image consists of 8192 byte BIOS followed by .qd sides, which are 65536-byte FDS sides with CRCs included but no 16-byte header. Size bytes 5-9 are zero, but 10-11 are used.   
  
## References

  * Based on [TNES format](https://pastebin.com/KLeWt2W3) by einstein95, via [TCRF forum post](http://jul.rustedlogic.net/thread.php?id=17316)
  * [FDS .qd format](https://forums.nesdev.org/viewtopic.php?f=2&t=15792)



Categories: [File formats](Category_File_formats.xhtml)
