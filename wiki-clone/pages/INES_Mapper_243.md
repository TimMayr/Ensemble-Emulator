# INES Mapper 243

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_243) | View [other pages](Special_AllPages.xhtml#INES_Mapper_243)

**iNES Mapper 243** denotes the Sachen **SA-020A** circuit board. Using an eight-register ASIC with a fake "[74LS374N](Sachen_74LS374N_pinout.xhtml "Sachen 74LS374N pinout")" marking, it supports up to 128 KiB PRG-ROM, and 128 KiB of CHR-ROM. It is used for only one game, _美女拳 - Honey Peach_ (SA-006). 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Register Index ($4100, write)
    * 2.2 Register Data ($4101, read/write)
  * 3 Errata
  * 4 See also



# Banks

  * CPU $8000-$FFFF: switchable 32 KiB PRG-ROM bank
  * PPU $0000-$1FFF: switchable 8 KiB CHR-ROM bank
  * Nametable mirroring: switchable H/V/Vertically-flipped L/One-screen



# Registers

## Register Index ($4100, write)
    
    
    Mask: $C101
    
    D~7654 3210
      ---------
      .... .RRR
            +++- Select register number (Rx)
    

## Register Data ($4101, read/write)
    
    
    Mask: $C101
    
    D~7654 3210
      ---------
      .... .RRR
            +++- Register data
    
    D~7654 3210
      ---------
      .... ...A  R2: CHR A13
      .... ...B  R4: CHR A14
      .... ..PP  R5: PRG A16..A15
      .... ..DC  R6: CHR A16..A15
      ...  .MM.  R7: Nametable mirroring
                  0: S0-S0-S0-S1 (lower right unique, or vertically-flipped L)
                  1: Horizontal
                  2: Vertical
                  3: Single-screen, page 1
    

Registers 0, 1, and 3 have no external effect. All three bits in all eight registers are fully implemented and can be read from and written to. 

# Errata

The **SA-150** PCB, denoted by [INES Mapper 150](INES_Mapper_150.xhtml "INES Mapper 150"), connects the same ASIC differently, changing the meaning of the CHR-bank-related register bits. 

# See also

  * [Box, Cart, and PCB picture of _Honey Peach_](https://www.flickr.com/photos/153392699@N08/sets/72157682682439086)



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml)
