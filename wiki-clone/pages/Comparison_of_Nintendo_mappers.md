# Comparison of Nintendo mappers

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/Comparison_of_Nintendo_mappers) | View [other pages](Special_AllPages.xhtml#Comparison_of_Nintendo_mappers)

This article compares the capabilities of several Nintendo mappers. 

## Discrete logic

All of Nintendo's [discrete logic mappers](Category_Discrete_logic_mappers.xhtml "Category:Discrete logic mappers") have [bus conflicts](Bus_conflict.xhtml "Bus conflict") except some revisions of 7. 

iNES | Chips | Boards | Max PRG | PRG banks | Max CHR | CHR banks | Mirroring | PRG RAM? | IRQ   
---|---|---|---|---|---|---|---|---|---  
[0](NROM.xhtml "INES Mapper 000") | 0-1 | NROM | 32 |  | 8 |  | V/H hardwired | Japan only | None   
[2](UxROM.xhtml "INES Mapper 002") | 2 | [UNROM, UOROM](UxROM.xhtml "UxROM") | 256 | 16 + 16F | 8 |  | V/H hardwired | No | None   
[3](CNROM.xhtml "INES Mapper 003") | 1 | [CNROM](CNROM.xhtml "CNROM") | 32 |  | 32 | 8 | V/H hardwired | No | None   
[7](AxROM.xhtml "INES Mapper 007") | 1-2 | [AxROM](AxROM.xhtml "AxROM") | 256 | 32 | 8 |  | 1 | No | None   
[13](CPROM.xhtml "INES Mapper 013") | 3 | [CPROM](CPROM.xhtml "CPROM") (U) | 32 |  | 16 | 4 + 4F | V/H hardwired | No | None   
[34](INES_Mapper_034.xhtml "INES Mapper 034") | 1 | [BNROM](INES_Mapper_034.xhtml "BNROM") | 256 | 32 | 8 |  | V/H hardwired | No | None   
[66](GxROM.xhtml "INES Mapper 066") | 1 | [GNROM, MHROM](GxROM.xhtml "GxROM") | 128 | 32 | 32 | 8 | V/H hardwired | No | None   
[94](INES_Mapper_094.xhtml "INES Mapper 094") | 2 | [UN1ROM](UxROM.xhtml "UxROM") | 256 | 16 + 16F | 8 |  | V/H hardwired | No | None   
[180](INES_Mapper_180.xhtml "INES Mapper 180") | 2 | UNROM+74'08 | 256 | 16F + 16 | 8 |  | V/H hardwired | No | None   
  
## ASIC

None of these [ASIC mappers](Category_ASIC_mappers.xhtml "Category:ASIC mappers") has bus conflicts. 

iNES | Mapper | Boards | Max PRG ROM | PRG ROM banks | Max CHR | CHR banks | Mirroring | PRG RAM? | Assist   
---|---|---|---|---|---|---|---|---|---  
[1](MMC1.xhtml "INES Mapper 001") | [MMC1](MMC1.xhtml "MMC1") | [SGROM, SNROM, SUROM](MMC1.xhtml "SxROM") | 512 | 16 + 16F; 32 | 8 | 4 + 4 | V/H/1 switchable | Optional | None   
1 | MMC1 | SKROM, SLROM | 256 | 16 + 16F; 32 | 128 | 4 + 4 | V/H/1 switchable | Optional | None   
[4](MMC3.xhtml "INES Mapper 004") | [MMC3](MMC3.xhtml "MMC3") | [TxROM](TxROM.xhtml "TxROM") | 512 | 8 + 8 + 16F | 256 | 2 + 2 + 1 + 1 + 1 + 1 | V/H switchable | Optional | Scanline IRQ   
4 | [MMC3](MMC3.xhtml "MMC3") | TGROM, TNROM | 512 | 8 + 8 + 16F | 8 | 2 + 2 + 1 + 1 + 1 + 1 | V/H switchable | Japan only | Scanline IRQ   
[118](INES_Mapper_118.xhtml "INES Mapper 118") | MMC3 | [TKSROM](TKSROM.xhtml "TKSROM"), [TLSROM](TLSROM.xhtml "TLSROM") | 512 | 8 + 8 + 16F | 128 | 2 + 2 + 1 + 1 + 1 + 1 | Any switchable | Optional | Scanline IRQ   
[119](INES_Mapper_119.xhtml "INES Mapper 119") | MMC3 | TQROM | 128 | 8 + 8 + 16F | 64 + 8 | 2 + 2 + 1 + 1 + 1 + 1 | V/H switchable | No | Scanline IRQ   
4 | [MMC6](MMC6.xhtml "MMC6") | HKROM | 512 | 8 + 8 + 16F | 256 | 2 + 2 + 1 + 1 + 1 + 1 | V/H switchable | 1KiB built-in; US and Europe only | Scanline IRQ   
[5](MMC5.xhtml "INES Mapper 005") | [MMC5](MMC5.xhtml "MMC5") | [ExROM](ExROM.xhtml "ExROM") | 1024 | 32, 16 + 16, 16 + 8 + 8, 8 + 8 + 8 + 8 | 1024 | BG: 4, 2 + 2, or 1 + 1 + 1 + 1  
Sprites: 8, 4 + 4, 2 * 4, or 1 * 8  
Split window: 4 | Any | 1 KiB built in; more optional | Scanline IRQ  
Smaller attribute areas  
Separate CHR bank per background tile  
Separate sets of CHR banks for sprites  
Vertical split screen  
8x8 multiplier   
[9](MMC2.xhtml "INES Mapper 009") | [MMC2](MMC2.xhtml "MMC2") | PNROM | 128 | 8 + 24F | 128 | 4/4 + 4/4 | V/H switchable | No | Tile triggered CHR bank switching   
[10](MMC4.xhtml "INES Mapper 010") | [MMC4](MMC4.xhtml "MMC4") | [FJROM, FKROM](FxROM.xhtml "FxROM") (J) | 256 | 16 + 16F | 128 | 4/4 + 4/4 | V/H switchable | Yes | Tile triggered CHR bank switching   
  
## External links

  * [Mapper wizard](http://pineight.com/nes/mapperwizard.html): Web-based decision tree for choosing a mapper for your NES project


