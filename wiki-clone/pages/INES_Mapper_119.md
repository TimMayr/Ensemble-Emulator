# INES Mapper 119

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_119) | View [other pages](Special_AllPages.xhtml#INES_Mapper_119)

iNES Mapper 119 is used to designate the **TQROM** board, which uses the [Nintendo MMC3](MMC3.xhtml "Nintendo MMC3") like other [TxROM](TxROM.xhtml "TxROM") boards but uses the CHR bank number in such a way so as to allow both CHR ROM and CHR RAM to be used simultaneously. 

Example games: 

  * _High Speed_
  * _Pin-Bot_



## Contents

  * 1 Registers
    * 1.1 Bank data ($8001-$9FFF, odd)
  * 2 Hardware
  * 3 Variants
  * 4 References



## Registers

Other than the CHR bank register described below, see [MMC3](MMC3.xhtml "MMC3") for a full description. 

### Bank data ($8001-$9FFF, odd)

Bit 6 of the CHR bank number is used to switch between the CHR ROM chip and the CHR RAM chip. 
    
    
    7  bit  0
    ---- ----
    xCDD DDDD
     ||| ||||
     |++-++++- New bank value, based on last value written to Bank select register
     |         0: Select 2 KB CHR bank at PPU $0000-$07FF (or $1000-$17FF);
     |         1: Select 2 KB CHR bank at PPU $0800-$0FFF (or $1800-$1FFF);
     |         2: Select 1 KB CHR bank at PPU $1000-$13FF (or $0000-$03FF);
     |         3: Select 1 KB CHR bank at PPU $1400-$17FF (or $0400-$07FF);
     |         4: Select 1 KB CHR bank at PPU $1800-$1BFF (or $0800-$0BFF);
     |         5: Select 1 KB CHR bank at PPU $1C00-$1FFF (or $0C00-$0FFF);
     |         6, 7: As standard MMC3
     +-------- Chip select (for CHR banks)
               0: Select CHR ROM; 1: Select CHR RAM
    

Nintendo _could_ have used bit 7 instead of bit 6, allowing 128 KiB of CHR ROM instead of 64 KiB. However, using bit 7 would have precluded a hypothetical "TQSROM" board combining TQROM-style ROM/RAM mixing with [TLSROM](TLSROM.xhtml "TLSROM")-style single-screen mirroring. [Rare](Rare.xhtml "Rare") developed both games on this board, and also frequently used single-screen mirroring on the [AxROM](AxROM.xhtml "AxROM") boards. 

## Hardware

A [74HC32](7432.xhtml "7432") IC controls the CHR ROM's enable line. PPU A13 is ORed with CHR A16 in order to create a /CE signal for the CHR ROM. CHR A16 also directly connects the positive CE pin on the [6264](6264_static_RAM.xhtml "6264") SRAM used as CHR RAM. 

For some totally unknown reason (board space?) official TQROM boards doesn't allow PRG ROMs greater than 128 KB. However, a third party board could expand this mapper up to 512 KB of PRG ROM, like any other MMC3 boards, and split a 128 KiB CHR ROM between banks 0-63 and banks 128-191. 

Both TQROM games used an 8 KiB CHR RAM. 

## Variants

iNES mappers [74](INES_Mapper_074.xhtml "INES Mapper 074"), [191](INES_Mapper_191.xhtml "INES Mapper 191"), [192](INES_Mapper_192.xhtml "INES Mapper 192"), [194](INES_Mapper_194.xhtml "INES Mapper 194"), and [195](INES_Mapper_195.xhtml "INES Mapper 195") are similar to mapper 119 except they replace fewer CHR ROM banks with RAM. 

## References

  * [NES-TQROM](http://kevtris.org/mappers/mmc3/NES_TQROM.html) by Kevin Horton
  * [Disch's Mapper Notes](http://www.romhacking.net/documents/362/)



Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3 with CHR ROM and CHR RAM](Category_MMC3_with_CHR_ROM_and_CHR_RAM.xhtml), [Nintendo licensed mappers](Category_Nintendo_licensed_mappers.xhtml)
