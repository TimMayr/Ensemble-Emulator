# INES Mapper 197

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_197) | View [other pages](Special_AllPages.xhtml#INES_Mapper_197)

iNES Mapper 197 denotes an [MMC3](MMC3.xhtml "MMC3") clone wired to support 512 KiB of CHR-ROM by increasing CHR banking granularity from 1/2 KiB to 2/4 KiB. All of its games are mapper hacks, usually by 卡聖 (Kǎshèng, NT-xxx cartridge codes), of games originally released on mappers [90](J_Y__Company_ASIC.xhtml "INES Mapper 090") and [91](INES_Mapper_091.xhtml "INES Mapper 091"). Submappers, assigned by Nestopia Plus!, denote three different wiring variants. PRG-ROM, nametable mirroring and IRQs function as in mapper [4](MMC3.xhtml "INES Mapper 004"). 

## Contents

  * 1 Submapper 0
  * 2 Submapper 1
  * 3 Submapper 2
  * 4 Submapper 3



## Submapper 0

  * _Super Fighter III_ (PRG-ROM CRC32 0xC333F621), originally for Mapper [91](INES_Mapper_091.xhtml "INES Mapper 091").1.


    
    
    Pin            Connected to
    PPU A10        CHR-ROM A10
    PPU A11        MMC3 PA10
    GND            MMC3 PA11
    PPU A12        MMC3 PA12
    MMC3 CHR A10   CHR-ROM A11
        :             :
    MMC3 CHR A17   CHR-ROM A18
    

So that the MMC3 sees the followings addresses: 
    
    
    Real PPU Addr  MMC3 PPU Addr
    $0000          $0000 -> reg 0 AND $FE
    $0400          $0000 -> reg 0 AND $FE
    $0800          $0400 -> reg 0 OR $01
    $0C00          $0400 -> reg 0 OR $01
    $1000          $1000 -> reg 2
    $1400          $1000 -> reg 2
    $1800          $1400 -> reg 3
    $1C00          $1400 -> reg 3
    

This results in the following CHR banks: 
    
    
    MMC3 reg#      Meaning
    0              4 KiB CHR-ROM bank at PPU $0000-$0FFF
    1              unused
    2              2 KiB CHR-ROM bank at PPU $1000-$17FF
    3              2 KiB CHR-ROM bank at PPU $1800-$1FFF
    4              unused
    5              unused
    

## Submapper 1

  * _Super Fighter III_ (PRG-ROM CRC32 0x2091BEB2), UNIF board name **UNL-FC-28-5027** , originally for Mapper [91](INES_Mapper_091.xhtml "INES Mapper 091").1.


    
    
    Pin            Connected to
    PPU A10        CHR-ROM A10
    PPU A11        MMC3 PA10
    Vcc            MMC3 PA11
    PPU A12        MMC3 PA12
    MMC3 CHR A10   CHR-ROM A11
        :             :
    MMC3 CHR A17   CHR-ROM A18
    

So that the MMC3 sees the followings addresses: 
    
    
    Real PPU Addr  MMC3 PPU Addr
    $0000          $0800 -> reg 1 AND $FE
    $0400          $0800 -> reg 1 AND $FE
    $0800          $0C00 -> reg 1 OR $01
    $0C00          $0C00 -> reg 1 OR $01
    $1000          $1800 -> reg 4
    $1400          $1800 -> reg 4
    $1800          $1C00 -> reg 5
    $1C00          $1C00 -> reg 5
    

This results in the following CHR banks: 
    
    
    MMC3 reg#      Meaning
    0              unused
    1              4 KiB CHR-ROM bank at PPU $0000-$0FFF
    2              unused
    3              unused
    4              2 KiB CHR-ROM bank at PPU $1000-$17FF
    5              2 KiB CHR-ROM bank at PPU $1800-$1FFF
    

## Submapper 2

  * _Mortal Kombat III Special (NT-328/YY-030)_ (PRG-ROM CRC32 0x60815A7D), originally for Mapper [90](J_Y__Company_ASIC.xhtml "INES Mapper 090").


    
    
    Pin            Connected to
    PPU A10        CHR-ROM A10
    PPU A11        MMC3 PA10 and MMC3 PA11
    PPU A12        MMC3 PA12
    MMC3 CHR A10   CHR-ROM A11
        :             :
    MMC3 CHR A17   CHR-ROM A18
    

So that the MMC3 sees the followings addresses: 
    
    
    Real PPU Addr  MMC3 PPU Addr
    $0000          $0000 -> reg 0 AND $FE
    $0400          $0000 -> reg 0 AND $FE
    $0800          $0C00 -> reg 1 OR $01
    $0C00          $0C00 -> reg 1 OR $01
    $1000          $1000 -> reg 2
    $1400          $1000 -> reg 2
    $1800          $1C00 -> reg 5
    $1C00          $1C00 -> reg 5
    

This results in the following CHR banks: 
    
    
    MMC3 reg#      Meaning
    0 (AND $FE)    2 KiB CHR-ROM bank at PPU $0000-$07FF
    1 (OR $01)     2 KiB CHR-ROM bank at PPU $0800-$0FFF
    2              2 KiB CHR-ROM bank at PPU $1000-$17FF
    3              unused
    4              unused
    5              2 KiB CHR-ROM bank at PPU $1800-$1FFF
    

## Submapper 3

  * _1995 Super 2-in-1 (2B20)_



Like Submapper 0, but with an additional outer bank register at $6000-$7FFF: 
    
    
    Mask: $E000
    
    D~7654 3210
      ---------
      .... S..P
           |  +- PRG A17 if S=1
           +---- PRG A17 mode
                  0: PRG A17=MMC3 PRG A17 (256 KiB)
                  1: PRG A17=P (128 KiB)
    

As the outer bank register uses the MMC3 clone's WRAM interface, writes only have an effect if WRAM is enabled and not write-protected via $A001. 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
