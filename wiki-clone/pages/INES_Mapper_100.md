# INES Mapper 100

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_100) | View [other pages](Special_AllPages.xhtml#INES_Mapper_100)

iNES Mapper 100 was assigned by fwNES[1] to denote [mapper hacks](Glossary.xhtml "Mapper hack") for the [MMC3](MMC3.xhtml "INES Mapper 004") emulation of Nesticle, an influential NES emulator for MS-DOS in the late 1990s. Nesticle did not directly support many of the custom ASICs by Irem, Konami and Namco, but did incorrectly emulate the MMC3 in a way that made it useful to run modified ROM images. Most of them use 512-byte trainers that must be loaded to CPU address $7000. 

Nestopia uses iNES mapper 100 as a magic number for UNIF-only boards. Nintendulator uses this as a general-purpose debugging mapper, allowing PRG/CHR banks to be selected arbitrarily using a control dialog. 

## Bankswitch Index Register ($8000)/Data Register ($8001)
    
    
    Mask: $E001 (nominally)
    
    $00: 1 KiB CHR-ROM bank at PPU $0000-$03FF := Value AND $FE
         1 KiB CHR-ROM bank at PPU $0400-$07FF := Value OR $01
    $01: 1 KiB CHR-ROM bank at PPU $0800-$0BFF := Value AND $FE
         1 KiB CHR-ROM bank at PPU $0C00-$0FFF := Value OR $01
    $02: 1 KiB CHR-ROM bank at PPU $1000-$13FF := Value
    $03: 1 KiB CHR-ROM bank at PPU $1400-$17FF := Value
    $04: 1 KiB CHR-ROM bank at PPU $1800-$1BFF := Value
    $05: 1 KiB CHR-ROM bank at PPU $1C00-$1FFF := Value
    $06: 8 KiB PRG-ROM bank at CPU $8000-$9FFF := Value
    $07: 8 KiB PRG-ROM bank at CPU $A000-$BFFF := Value
    $46: 8 KiB PRG-ROM bank at CPU $C000-$DFFF := Value
    $47: 8 KiB PRG-ROM bank at CPU $A000-$BFFF := Value
    $80: 1 KiB CHR-ROM bank at PPU $1000-$13FF := Value AND $FE
         1 KiB CHR-ROM bank at PPU $1400-$17FF := Value OR $01
    $81: 1 KiB CHR-ROM bank at PPU $1800-$1BFF := Value AND $FE
         1 KiB CHR-ROM bank at PPU $1C00-$1FFF := Value OR $01
    $82: 1 KiB CHR-ROM bank at PPU $0000-$03FF := Value
    $83: 1 KiB CHR-ROM bank at PPU $0400-$07FF := Value
    $84: 1 KiB CHR-ROM bank at PPU $0800-$0BFF := Value
    $85: 1 KiB CHR-ROM bank at PPU $0C00-$0FFF := Value
    

On an actual MMC3, changing bits 6 and 7 in the index register would immediately apply the current content of all seven bank registers to their flipped meaning. Nesticle applies these bits only on the next $8001 write, which makes eight independent 1 KiB CHR-ROM banks available: Registers $82-$85 to set four 1 KiB banks at CPU $0000-$0FFF, and registers $02-$05 to set four more 1 KiB banks at CPU $1000-$1FFF. 

  1. ↑ FWNES98E.TXT, fwNES 0.302b, 1998/11/16, by FanWen Yang



Categories: [Bad iNES Mappers](Category_Bad_iNES_Mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
