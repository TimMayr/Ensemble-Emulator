# TxROM

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/TxROM) | View [other pages](Special_AllPages.xhtml#TxROM)

**NESCartDB**

[TxROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=-T%25ROM)  
---  
[TBROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TBROM)  
[TEROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TEROM)  
[TFROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TFROM)  
[TGROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TGROM)  
[TKROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TKROM)  
[TK1ROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TK1ROM)  
[TKSROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TKSROM)  
[TLROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TLROM)  
[TL1ROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TL1ROM)  
[TL2ROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TL2ROM)  
[TLSROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TLSROM)  
[TNROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TNROM)  
[TQROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TQROM)  
[TR1ROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TR1ROM)  
[TSROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TSROM)  
[TVROM](https://nescartdb.com/search/advanced?unif_op=LIKE+%60%25%40%25%60&unif=TVROM)  
  
The generic designation **TxROM** refers to cartridge boards made by Nintendo that use the [Nintendo MMC3](MMC3.xhtml "MMC3") mapper. 

## Contents

  * 1 Board Types
  * 2 Solder pad config
    * 2.1 Namco 108 backwards compatibility (TEROM and TFROM)
    * 2.2 Battery retention (TNROM, TKROM and TKSROM)
  * 3 Various notes



## Board Types

The following TxROM boards are known to exist: 

Board | PRG ROM | PRG RAM | CHR | Comments   
---|---|---|---|---  
TBROM | 64 KB |  | 16 / 32 / 64 KB ROM |   
TEROM | 32 KB |  | 16 / 32 / 64 KB ROM | Supports fixed mirroring   
TFROM | 128 / 256 / 512 KB |  | 16 / 32 / 64 KB ROM | Supports fixed mirroring   
TGROM | 128 / 256 / 512 KB |  | 8 KB RAM/ROM |   
TKROM | 128 / 256 / 512 KB | 8 KB | 128 / 256 KB ROM |   
[TK1ROM](http://forums.nesdev.org/viewtopic.php?f=9&t=9891) | 128 KB | 8KB | 128KB ROM | Uses [7432](7432.xhtml "7432") for 28-pin CHR ROM   
[TKSROM](TKSROM.xhtml "TKSROM") | 128 / 256 / 512 KB | 8 KB | 128 KB ROM | Alternate mirroring control, Famicom only   
TLROM | 128 / 256 / 512 KB |  | 128 / 256 KB ROM |   
TL1ROM | 128 KB |  | 128 KB | Uses [7432](7432.xhtml "7432") for 28-pin CHR ROM   
TL2ROM |  |  |  | Nonstandard pinout   
[TLBROM](https://forums.nesdev.org/viewtopic.php?p=228322#p228322) | 128 KB |  | 128 KB ROM | Uses 74541 to compensate for too-slow CHR ROM   
[TLSROM](TLSROM.xhtml "TLSROM") | 128 / 256 / 512 KB |  | 128 KB ROM | Alternate mirroring control   
TNROM | 128 / 256, 512 KB | 8 KB | 8 KB RAM/ROM | Famicom only   
[TQROM](INES_Mapper_119.xhtml "TQROM") | 128 KB |  | 16 / 32 / 64 KB ROM + 8 KB RAM |   
[TR1ROM](http://bootgod.dyndns.org:7777/profile.php?id=2890) | 128 / 256 / 512 KB |  | 64 KB ROM + 4 KB VRAM (4-screen [Mirroring](Mirroring.xhtml "Mirroring")) | NES only   
TSROM | 128 / 256 / 512 KB | 8 KB (no battery) | 128 / 256 KB ROM |   
[TVROM](http://bootgod.dyndns.org:7777/profile.php?id=137) | 64 KB |  | 16 / 32 / 64 KB ROM + 4 KB VRAM (4-screen [Mirroring](Mirroring.xhtml "Mirroring")) | NES only   
  
## Solder pad config

### [Namco 108](INES_Mapper_206.xhtml "INES Mapper 206") backwards compatibility (TEROM and TFROM)

  * Normal mode: 'CL1' connected, 'CL2' connected, 'H' disconnected, 'V' disconnected.
  * Backwards compatible with horizontal [mirroring](Mirroring.xhtml "Mirroring"): 'CL1' disconnected, 'CL2' disconnected, 'H' disconnected, 'V' connected
  * Backwards compatible with vertical mirroring: 'CL1' disconnected, 'CL2' disconnected, 'H' connected, 'V' disconnected



Connecting 'CL1' enables MMC3-controlled mirroring, while connecting 'CL2' enables IRQs. However, the additional bankswitching modes available by the MMC3 that weren't available with the Namco chip used on DEROM boards are still present and activated by bits 7-6 of port $8000. 

### Battery retention (TNROM, TKROM and TKSROM)

  * PRG RAM retaining data: 'SL' disconnected, Battery, D1, D2, R1 R2 and R3 present.
  * PRG RAM not retaining data: 'SL' connected, leave slots for Battery, D1, D2, R1, R2 and R3 free.



## Various notes

Boards with 4-screen mirroring uses a 8 KB SRAM chip, but only 4 KB is actually used. The 2 KB VRAM inside of the console is always disabled, and the CIRAM A10 pin of the MMC3 doesn't go to anything. 

TLSROM and TKSROM boards have different mirroring control than other MMC3 boards. The mirroring is controlled directly by MMC3's CHR A17 line, and MMC3's CIRAM A10 pin doesn't go to anything. Due to their incompatibility with other MMC3 boards on a software viewpoint, they are assigned to [INES Mapper 118](INES_Mapper_118.xhtml "INES Mapper 118") instead of mapper 4. 

TQROM board has both CHR ROM and RAM. Bit 6 of the bank number, which appears on MMC3's CHR A16 line, controls whenever CHR RAM or CHR-ROM is enabled. A [74HC32](7432.xhtml "74HC32") chip is used to combine this with other chip enable signals for the CHR-ROM and the CHR-RAM chips. Due to this incompatibility with the other MMC3 boards on a software viewpoint, this board is assigned to [INES Mapper 119](INES_Mapper_119.xhtml "INES Mapper 119") instead of mapper 4. 

Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
