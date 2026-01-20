# INES Mapper 215

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_215) | View [other pages](Special_AllPages.xhtml#INES_Mapper_215)

iNES Mapper 215 is used for later single-game as well as multicart releases from Sugar Softec. They use an MMC3 clone with a selectable scrambling pattern. 

  * **Submapper 0** , UNIF board name **UNL-8237** : 
    * _Earthworm Jim 2_
    * _Mortal Kombat 3_ (SuperGame, not _Extra 60_ , not to be confused by similarly-named games from other developers)
    * _Mortal Kombat 3 Extra 60_ (both existing ROM images are just extracts of the 2-in-1 multicart containing this game)
    * _Pocahontas Part 2_
    * _2-in-1: Aladdin, EarthWorm Jim 2_ (Super 808)
    * _2-in-1: The Lion King, Bomber Boy_ (GD-103)
    * _2-in-1 Super Golden Card: EarthWorm Jim 2, Boogerman_ (king002)
    * _2-in-1: Mortal Kombat 3 Extra 60, The Super Shinobi_ (king005)
    * _3-in-1: Boogerman, Adventure Island 3, Double Dragon 3_ (Super 308)
    * _5-in-1 Golden Card: Aladdin, EarthWorm Jim 2, Garo Densetsu Special, Silkworm, Contra Force_ (SPC005)
    * _6-in-1 Golden Card: EarthWorm Jim 2, Mortal Kombat 3, Double Dragon 3, Contra 3, The Jungle Book, Turtles Tournament Fighters_ (SPC009)
  * **Submapper 1** , UNIF board name **UNL-8237A** : 
    * _9-in-1 High Standard Card: The Lion King, EarthWorm Jim 2, Aladdin, Boogerman, Somari, Turtles Tournament Fighters, Mortal Kombat 3, Captain Tsubasa 2, Taito Basketball_ (king001)



## Contents

  * 1 Registers
    * 1.1 NROM Override/Mode Register ($5000)
    * 1.2 Outer Bank Register ($5001) - Submapper 0 (UNL-8237)
    * 1.3 Outer Bank Register ($5001) - Submapper 1 (UNL-8237A)
    * 1.4 Scrambling Pattern Register ($5007)
    * 1.5 MMC3-compatible registers ($8000-$FFFF, write-only)
  * 2 Note
  * 3 Similar Mappers



# Registers

## NROM Override/Mode Register ($5000)
    
    
    Mask: $F007
    
    D~7654 3210
      ---------
      MCS. BBBb
      |||  ++++- Select 16 KiB PRG-ROM bank at CPU
      |||        $8000-$BFFF and $C000-$FFFF
      ||+------- 0: Do not replace bit 0 (NROM-128)
      ||         1: Replace bit 0 with CPU A14 (NROM-256)
      |+-------- 0: Do not replace PRG A17 and CHR A17 from the MMC3 (256 KiB outer bank)
      |          1: Replace PRG A17 and CHR A17 with $5001 bits 4 and 5, respectively (128 KiB outer bank)
      +--------- 0: Use PRG bank from MMC3; ignore bits 0-3/5
                 1: Ignore PRG bank from MMC3; apply bits 0-3/5
    
    Power-up value: $00
    

## Outer Bank Register ($5001) - Submapper 0 (UNL-8237)
    
    
    Mask: $F007
    
    D~7654 3210
      ---------
      ..cp CCPP
        || ||++- Select 256 KiB Outer PRG-ROM bank (PRG A18/A19)
        || ++--- Select 256 KiB Outer CHR-ROM bank (CHR A18/A19)
        |+------ Select 128 KiB Outer PRG-ROM bank (PRG A17) if $5000 bit 6=1
        +------- Select 128 KiB Outer CHR-ROM bank (CHR A17) if $5000 bit 6=1
    
    Power-up value: $xF
    

## Outer Bank Register ($5001) - Submapper 1 (UNL-8237A)
    
    
    Mask: $F007
    
    D~7654 3210
      ---------
      ..cp P.PP
           CCC.
        || +|++- Select 256 KiB Outer PRG-ROM bank (PRG A18-A20)
        || +++-- Select 256 KiB Outer CHR-ROM bank (CHR A18-A20)
        |+------ Select 128 KiB Outer PRG-ROM bank (PRG A17) if $5000 bit 6=1
        +------- Select 128 KiB Outer CHR-ROM bank (CHR A17) if $5000 bit 6=1
    
    Power-up value: $xF
    

## Scrambling Pattern Register ($5007)
    
    
    Mask: $F007
    
    D~7654 3210
      ---------
      .... .MMM
            +++- Select MMC3 register address and index scrambling mode (0-7)
    
    Power-up value: $00
    

## MMC3-compatible registers ($8000-$FFFF, write-only)

The scrambled addresses correspond to the real address as follows: 
    
    
    $5007	-------------address written-----------
    value	8000 8001 A000 A001 C000 C001 E000 E001
    -----------------------------------------------	
    0	8000 8001 A000 A001 C000 C001 E000 E001
    1	A001 A000 8000 C000 8001 C001 E000 E001
    2	8000 8001 A000 A001 C000 C001 E000 E001
    3	C001 8000 8001 A000 A001 E001 E000 C000
    4	A001 8001 8000 C000 A000 C001 E000 E001
    5	8000 8001 A000 A001 C000 C001 E000 E001
    6	8000 8001 A000 A001 C000 C001 E000 E001
    7	8000 8001 A000 A001 C000 C001 E000 E001
    

The data that is written to the low three bits of register $8000 corresponds to the real data as follows: 
    
    
    $5007	-value written-
    value	0 1 2 3 4 5 6 7
    -----------------------
    0	0 1 2 3 4 5 6 7
    1	0 2 6 1 7 3 4 5
    2	0 5 4 1 7 2 6 3
    3	0 6 3 7 5 2 4 1
    4	0 2 5 3 6 1 7 4
    5	0 1 2 3 4 5 6 7
    6	0 1 2 3 4 5 6 7
    7	0 1 2 3 4 5 6 7 
    

Bits 6 and 7 of register $8000 are kept as they are. The data written to $8001 and $A000-$FFFF is never scrambled. After unscrambling the address and the data written, these registers function the same as the MMC3's 

# Note

  * Several single-game cartridges that use this mapper are actually 2-in-1 multicarts that have PRG A18 forced to 0 or 1 via solder pad. A dump from these cartridges without changing the solder pad will therefore contain 256 KiB of PRG-ROM but 512 KiB of CHR-ROM, since CHR A18 is not forced in a similar fashion, with one 256 KiB half containing data from the other inaccessible game.
  * The Outer Bank Register is reset to its power-up value if a Reset has been detected via an interruption of the M2 signal. This makes it difficult to dump these cartridges using a Kazzo device.
  * _Boogerman_ ([INES Mapper 114](INES_Mapper_114.xhtml "INES Mapper 114") Submapper 1) was originally assigned by CaH4e3 to Mapper 215; he later reassigned **UNL-8237** to Mapper 215 instead.
  * Some multicarts may have alternative outer bank registers in the $6xxx range; since they all contain compatibility code that writes to the $5xxx range, they are assigned to Mapper 215 as well.



# Similar Mappers

  * [NES 2.0 Mapper 258](NES_2_0_Mapper_258.xhtml "NES 2.0 Mapper 258") adds readable protection registers in the $5000-$5007 range.



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using $4020-$5FFF](Category_Mappers_using__4020__5FFF.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
