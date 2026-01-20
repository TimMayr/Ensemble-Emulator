# NES 2.0 Mapper 540

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_540) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_540)

NES 2.0 Mapper 540 is a variant of [NES 2.0 Mapper 359](NES_2_0_Mapper_359.xhtml "NES 2.0 Mapper 359") that rewires the PPU/CHR address lines to select 2 KiB instead of 1 KiB banks in order to address 512 KiB of CHR-RAM with 8 bit CHR banking registers. It is used on a "Super Boy" mapper hack of _Master Fighter VI'_ , originally for [NES 2.0 Mapper 264](NES_2_0_Mapper_264.xhtml "NES 2.0 Mapper 264"). Its UNIF board name is **UNL-82112C**. 
    
    
    Pin            Connected to
    PPU A10        CHR-ROM A10
    PPU A11        ASIC PA10
    PPU A12        ASIC PA11+PA12
    ASIC CHR A10   CHR-ROM A11
        :             :
    ASIC CHR A17   CHR-ROM A18
    

So that the ASIC sees the followings addresses: 
    
    
    Real PPU Addr  ASIC PPU Addr
    $0000          $0000 -> reg $A000
    $0400          $0000 -> reg $A000
    $0800          $0400 -> reg $A001
    $0C00          $0400 -> reg $A001
    $1000          $1800 -> reg $B002
    $1400          $1800 -> reg $B002
    $1800          $1C00 -> reg $B003
    $1C00          $1C00 -> reg $B003
    

## CHR Bank Select ($A000-$B003, write)

Mask: $F003 

  * $A000: Select 2 KiB CHR-ROM bank at PPU $0000-$07FF
  * $A001: Select 2 KiB CHR-ROM bank at PPU $0800-$0FFF
  * $A002: No function
  * $A003: No function
  * $B000: No function
  * $B001: No function
  * $B002: Select 2 KiB CHR-ROM bank at PPU $1000-$17FF
  * $B003: Select 2 KiB CHR-ROM bank at PPU $1800-$1FFF



Categories: [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml)
