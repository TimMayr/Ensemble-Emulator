# INES Mapper 219

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_219) | View [other pages](Special_AllPages.xhtml#INES_Mapper_219)

iNES Mapper 219 is used for the 卡聖 (Kǎshèng) A9461 MMC3-clone-bearing board, used by: 

  * _Toy Story_
  * _Super 1997 4-in-1 (NT-8029)_



## Contents

  * 1 Mode Register ($8002)
  * 2 MMC3 mode registers ($8000-$FFFF)
  * 3 MMC3 Bank Register Index ($8000)/Data ($8001)
  * 4 Extended PRG-ROM Bank Index ($8000, $23-$26)/Data ($8001)
  * 5 CHR-ROM Bank Number Latch Index ($8000, $08-$1E, even)/Data ($8001)
  * 6 CHR-ROM Pseudo 2 KiB switch Index ($8000, $09-$0F, odd)/Data ($8001)
  * 7 CHR-ROM 1 KiB switch Index ($8000, $11-$1F, odd)/Data ($8001)
  * 8 Outer Bank Bit 0 ($5002)
  * 9 Outer Bank Bit 1 ($5003)
  * 10 Note
  * 11 Sources



## Mode Register ($8002)
    
    
    Mask: $E003
    
    7654 3210
    ---------
    CPMR RRRR
    ||++-++++-*Select bank register number (0-7 in MMC3 mode, $08-$26 in Extended mode)
    ||+------- Select mode
    ||          0: MMC3 mode
    ||          1: Extended mode
    |+--------*1: PRG A14 inversion in MMC3 mode
    +---------*1: CHR A12 inversion in MMC3 mode
    
    * Same as $8000
    

## MMC3 mode registers ($8000-$FFFF)
    
    
    Mask: $E001
    
    $A000, $A001, $C000, $C001, $E000, $E001: As on regular [MMC3](MMC3.xhtml "MMC3").
    

## MMC3 Bank Register Index ($8000)/Data ($8001)

Mask: $E001. Used only if $8002 bit 5=0, see [MMC3](MMC3.xhtml "MMC3"). Note that PRG-ROM and CHR-ROM banks are masked to 128 KiB. 

## Extended PRG-ROM Bank Index ($8000, $23-$26)/Data ($8001)

Mask: $E001. Used only if $8002 bit 5=1. 

$8000/$8002 value: 
    
    
    D~7654 3210
      ---------
      ..10 .RRR
            +++- 6: Select 8 KiB PRG-ROM bank at CPU $8000-$9FFF
                 5: Select 8 KiB PRG-ROM bank at CPU $A000-$BFFF
                 4: Select 8 KiB PRG-ROM bank at CPU $C000-$DFFF 
                 3: Select 8 KiB PRG-ROM bank at CPU $E000-$FFFF
    

$8001 value: 
    
    
    D~7654 3210
      ---------
      ..BB BB..
        ++-++--- Bits 0-3 (reversed order) of 8 KiB PRG-ROM bank number
    

## CHR-ROM Bank Number Latch Index ($8000, $08-$1E, even)/Data ($8001)

Mask: $E001. Used only if $8002 bit 5=1. 

$8000 value: 
    
    
    D~7654 3210
      ---------
      ..00 1..0
    or
    D~7654 3210
      ---------
      ..01 ...0
    
    

$8001 value: 
    
    
    D~7654 3210
      ---------
      .... .654
            +++- Bits 4-6 OR'd with next CHR-ROM switch's data
    

## CHR-ROM Pseudo 2 KiB switch Index ($8000, $09-$0F, odd)/Data ($8001)

Mask: $E001. Used only if $8002 bit 5=1. 

$8000 value: 
    
    
    D~7654 3210
      ---------
      ..00 1RR1
            ++-- Select CHR-ROM address range to switch
             |    0: PPU $0000-$03FF
             |    1: PPU $0400-$07FF
             |    2: PPU $0800-$0BFF
             |    3: PPU $0C00-$0FFF
             +-- OR'd with Bit 0 of CHR-ROM bank data
    

$8001 value: 
    
    
    D~7654 3210
      ---------
      BBBB BBB.
      ++++-+++-- Bits 0-6 of 1 KiB CHR-ROM bank number at PPU address selected via $8000/$8002.
    

The value is OR'd with bit 1 SHR 1 of the value written to $8000, then OR'd with bits 0-2 SHL 4 of the value written to the CHR-ROM latch. 

## CHR-ROM 1 KiB switch Index ($8000, $11-$1F, odd)/Data ($8001)

Mask: $E001. Used only if $8002 bit 5=1. 

$8000 value: 
    
    
    D~7654 3210
      ---------
      ..01 RR.1
           ++--- Select CHR-ROM address range to switch
                  0: PPU $1000-$13FF
                  1: PPU $1400-$17FF
                  2: PPU $1800-$1BFF
                  3: PPU $1C00-$1FFF
    

$8001 value: 
    
    
    D~7654 3210
      ---------
      BBBB BBB.
      ++++-+++-- Bits 0-6 of 1 KiB CHR-ROM bank number at PPU address selected via $8000/$8002.
    

The value is OR'd with bits 0-2 SHL 4 of the value written to the CHR-ROM latch. 

## Outer Bank Bit 0 ($5002)
    
    
    Mask: $5003
    
    D~7654 3210
      ---------
      .... ...B
              +- Bit 0 of 128 KiB outer PRG-ROM and CHR-ROM bank
    

## Outer Bank Bit 1 ($5003)
    
    
    Mask: $5003
    
    D~7654 3210
      ---------
      ..B. ....
        +------- Bit 1 of 128 KiB outer PRG-ROM and CHR-ROM bank
    

The Outer Bank register must be initialized to $03 for the menu to appear. 

## Note

This mapper gets hot and breaks down easily when using dumping devices. 

## Sources

  * [FCEUX implementation](https://sourceforge.net/p/fceultra/code/3320/tree/fceu/trunk/src/boards/a9746.cpp)
  * [Nestopia's implementation](https://github.com/rdanbrook/nestopia/blob/88d130fd083b9662ef49e8d5ef95513f4bb8759e/source/core/board/NstBoardUnlA9746.cpp)



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml)
