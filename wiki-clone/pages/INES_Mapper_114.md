# INES Mapper 114

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_114) | View [other pages](Special_AllPages.xhtml#INES_Mapper_114)

iNES Mapper 114 is used for an [MMC3](MMC3.xhtml "MMC3")-clone-bearing board with scrambled register addresses and indices, a 256 KiB outer CHR-ROM bank register, and an NROM-like PRG-ROM register that can override the MMC3 clone's PRG-ROM bank. 

  * **Submapper 0** : 
    * _Aladdin_ (SuperGame, not _Super Aladdin_ , and not to be confused with _Aladdin_ from J.Y. Company)
    * _The Lion King_ (SuperGame)
    * _Pocohontos_ (Hosenkan, not to be confused with _Pocahontas Part 2_ from SuperGame)
    * _Super Donkey Kong_ (Hosenkan)
  * **Submapper 1** (different scrambling pattern): 
    * _Boogerman_ (SuperGame, not to be confused with _Boogerman II_ by Rex Soft)



## Contents

  * 1 Registers
    * 1.1 NROM Override Register ($6000)
    * 1.2 Outer CHR-ROM Bank Register ($6001)
    * 1.3 MMC3-compatible registers ($8000-$FFFF)
      * 1.3.1 Submapper 0
      * 1.3.2 Submapper 1
  * 2 Notes
  * 3 Similar Mappers



# Registers

## NROM Override Register ($6000)
    
    
    Mask: Probably $E001
    
    D~7654 3210
      ---------
      M.S. BBBb
      | |  ++++- Select 16 KiB PRG-ROM bank at CPU
      | |        $8000-$BFFF and $C000-$FFFF
      | +------- 0: Do not replace bit 0 (NROM-128)
      |          1: Replace bit 0 with CPU A14 (NROM-256)
      +--------- 0: Use PRG bank from MMC3; ignore $6000 bits 0-3/5
                 1: Ignore PRG bank from MMC3; apply $6000 bits 0-3/5
    

## Outer CHR-ROM Bank Register ($6001)
    
    
    Mask: Probably $E001
    
    D~7654 3210
      ---------
      .... ...+- Select 256 KiB CHR-ROM bank at PPU $0000-$1FFF
    

## MMC3-compatible registers ($8000-$FFFF)

### Submapper 0
    
    
    Address written   MMC3 register meant
    ---------------   -------------------
    $8000             $A001
    $8001             $A000
    $A000             $8000
    $A001             $C000
    $C000             $8001
    $C001             $C001
    $E000             $E000
    $E001             $E001
    
    
    
    $8000 index written   MMC3 register meant
    -------------------   -------------------
    0                     0
    1                     3
    2                     1
    3                     5
    4                     6
    5                     7
    6                     2
    7                     4
    

### Submapper 1
    
    
    Address written   MMC3 register meant
    ---------------   -------------------
    $8000             $A001
    $8001             $8001
    $A000             $8000
    $A001             $C001
    $C000             $A000
    $C001             $C000
    $E000             $E000
    $E001             $E001
    
    
    
    $8000 index written   MMC3 register meant
    -------------------   -------------------
    0                     0
    1                     2
    2                     5
    3                     3
    4                     6
    5                     1
    6                     7
    7                     4
    

Bits 6 and 7 of the value written to $8000 are unchanged. See [MMC3](MMC3.xhtml "MMC3") for further details. 

# Notes

  * Unlike many similar mappers, the $600x registers are not connected to the MMC3 clone's WRAM interface and thus function regardless of whether WRAM is enabled or not.
  * IRQ behavior resembles that of the MMC3A, i.e. a latch value of zero disables the IRQ, on which _Aladdin_ depends.
  * Previous descriptions of Mapper 114/182 claim that a write to address $C001 is equivalent to writing both to an MMC3's $C000 and $C001 registers, and that writing to $A001 has no effect. A comparison of _Aladdin_ (Mapper 114) and the _Super Aladdin_ (Mapper 4, by the same developer) however indicates a one-to-one mapping, provided that MMC3A-like IRQ behavior is enforced.



# Similar Mappers

  * [INES Mapper 115](INES_Mapper_115.xhtml "INES Mapper 115") has the same $6000 and $6001 registers, but does not scramble MMC3 register addresses and indices. Also, Mapper 115's IRQ behavior resembles that of the MMC3C, i.e. a latch value of zero causes an IRQ on every scanline, on which the Chinese version of _幽☆遊☆白書 Final_ (Yuu Yuu Hakusho Final) depends.
  * [INES Mapper 123](INES_Mapper_123.xhtml "INES Mapper 123") moves the $6000 register to $5800 while rearranging its bits and drops the outer CHR-ROM bank register at $6001 as well as register address scrambling while retaining index register scrambling (using the same pattern).
  * [INES Mapper 182](INES_Mapper_114.xhtml "INES Mapper 182") is a duplicate of Mapper 114. In fact, FCEUX explicitly reassigns Hosenkan's Mapper 182 ROMs to Mapper 114. Because Hosenkan's games make no use of the $6000-$6001 registers, other than writing zero to both of them on startup, some implementations (such as the EverDrive N8's) do not emulate the $600x registers.
  * [INES Mapper 215](INES_Mapper_215.xhtml "INES Mapper 215") extends and moves $6000/$6001 to $5000/$5001, and makes the MMC3 register address and index scrambling pattern selectable via $5007.



Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
