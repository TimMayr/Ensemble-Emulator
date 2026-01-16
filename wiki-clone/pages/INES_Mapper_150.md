# INES Mapper 150

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_150) | View [other pages](Special_AllPages.xhtml#INES_Mapper_150)

**iNES Mapper 150** denotes the Sachen **SA-015** circuit board and its chip-on-board variant, **SA-630**. Using an eight-register ASIC with a fake "[74LS374N](Sachen_74LS374N_pinout.xhtml "Sachen 74LS374N pinout")" marking, it supports up to 128 KiB PRG-ROM, and 64 KiB of CHR-ROM. Its UNIF board name is **UNL-Sachen-74LS374N**. Used for the following games: 

  * _영어, 피라미드_ (Yeong-eo, Pyramid, K-001)
  * _将棋学園 - Chess Academy_ (Shōgi Gakuen, SA-015)
  * _Tasac_ (SA-020)
  * _2-in-1: Cosmo Cop/Cyber Monster_ (SA-023)
  * _2-in-1: Tough Cop/Super Tough Cop_ (SA-024)
  * _臺灣 16 花牌麻将_ (Táiwān 16 Huāpái Májiàng, SA-025)
  * _Poker II_ (TC-013)
  * _Strategist_ (TC-014)
  * _Olympic I.Q._ (TC-015)
  * _Happy Pairs_ (TC-016)
  * _Auto-Upturn_ (TC-017)
  * _Magic Cube_ (TC-018)
  * _Chinese Checkers_ (TC-019)
  * _Poker III 5-in-1_ (TC-020)



## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Register Index ($4100, write)
    * 2.2 Register Data ($4101, read/write)
  * 3 Solder Pad
  * 4 Errata
  * 5 See also



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
      .... ...C  R4: CHR A15
      .... ..PP  R5: PRG A16..A15
      .... ..BA  R6: CHR A14..A13
      ...  .MM.  R7: Nametable mirroring
                  0: S0-S0-S0-S1 (lower right unique, or vertically-flipped L)
                  1: Horizontal
                  2: Vertical
                  3: Single-screen, page 1
    

Registers 0-3 are completely unused. _Shōgi Gakuen_ checks that the registers are actually readable (in addition to checking the solder pad setting), presumably as a protection measure. All three bits in all eight registers are fully implemented and can be read from and written to. 

# Solder Pad

The circuit board has a solder pad that selects whether ASIC pin 14 is connected to CPU D2 or to Vcc. In the latter setting, the ASIC sees all writes as being OR'd with $04, while on reads, D2 is open bus. The pad setting only makes a difference for the game _Shōgi Gakuen_ , which writes to R2 and reads from R6 to check whether it now holds that written value, as would be the case if the solder pad were in the Vcc position; depending on the setting, a different copyright tag ("Sachen" vs. "Sachen and Hacker International") is displayed. 

# Errata

The **SA-020A** PCB, used for _美女拳 - Honey Peach_ (SA-006) and denoted by [INES Mapper 243](INES_Mapper_243.xhtml "INES Mapper 243"), connects the same ASIC differently to support 128 KiB of CHR-ROM. The game is commonly found with an incorrect CHR-ROM bank order set to mapper 150; if that misordered ROM file is to be emulated, R2 bit 0 must additionally provide CHR A16. 

# See also

  * [Cart and PCB picture of _Taiwan Mahjong II_ (SA-015 PCB)](https://www.flickr.com/photos/153392699@N08/sets/72157678810248784)
  * [Box, cart, and PCB picture (chip-on-board variant) of _Taiwan Mahjong II_ (SA-630 PCB)](https://www.flickr.com/photos/153392699@N08/sets/72157678801553324)



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml)
