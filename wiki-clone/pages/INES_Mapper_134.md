# INES Mapper 134

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_134) | View [other pages](Special_AllPages.xhtml#INES_Mapper_134)

**iNES Mapper 134** denotes an [MMC3](MMC3.xhtml "MMC3")-clone-based multicart PCB referred to alternatively as **T4A54A** , **WX-KB4K** , or **BS-5652**. 

  * _2-in-1 Family Kid & Aladdin 4_
  * _Super Cool Boy 4-in-1 (CB-4034)_ (commonly-available ROM file has been hacked to run as [INES Mapper 044](INES_Mapper_044.xhtml "INES Mapper 044"))
  * _Super Cool Boy 4-in-1 (CB-4035)_
  * _Super 4-in-1 1998 (JH-274)_
  * _1998 HIGH 4-in-1 (YH-463)_



## Contents

  * 1 Mode Register ($6000, write)
  * 2 Outer Bank Register ($6001, write)
  * 3 CNROM register ($6002, write)
  * 4 Unknown Register ($6003, write)
  * 5 MMC3-compatible registers ($8000-$FFFF)



## Mode Register ($6000, write)
    
    
    Mask: $E003
    
    D~7654 3210
      ---------
      LDcp C...
      |||| +---- CHR-ROM banking mode
      ||||        0: MMC3 (inner banks from $8000.0-$8000.5)
      ||||        1: CNROM (8 KiB inner bank from $6002)
      |||+------ PRG A19
      ||+------- CHR A19
      |+-------- CPU $8000-$FFFF mode
      |           0: PRG-ROM
      |           1: Read solder pad setting
      +--------- $6xxx Register Lock (except $6002.0-1)
                  0: unlocked
                  1: locked, further writes ignored until reset
    

## Outer Bank Register ($6001, write)
    
    
    Mask: $E003
    
    D~7654 3210
      ---------
      NcCC npPP
      |||| ||++- PRG A18..17
      |||| |+--- Outer PRG Bank Size
      |||| |      0: 256 KiB, PRG A17 from MMC3
      |||| |      1: 128 KiB, PRG A17 from $6001 bit 0
      |||| +---- PRG A14 mode in NROM mode ($6001 bit 7=1)
      ||||        0: PRG A14=CPU A14 (NROM-256)
      ||||        1: PRG A14=MMC3 PRG A14 (NROM-128)
      ||++------ CHR A18..17
      |+-------- Outer CHR Bank Size
      |           0: 256 KiB, CHR A17 from MMC3
      |           1: 128 KiB, CHR A17 from $6001 bit 4
      +--------- PRG-ROM banking mode
                  0: MMC3
                  1: NROM
    

In NROM mode, the MMC3 clone's CPU A13 and A14 inputs are held low, so that MMC3 register 6 applies to the entire CPU $8000-$FFFF range, with PRG A13 substituted by CPU A13 and PRG A14 subject to $6001.3. 

## CNROM register ($6002, write)
    
    
    Mask: $E003
    

Contains the 8 KiB CHR-ROM bank if $6000.3=1, masked to 128 or 256 KiB according to $6000.6. Bits 0-1 respond even if the "Lock" bit is set, while changes to bits 2-7 will be ignored if the "Lock" bit has been set. 

## Unknown Register ($6003, write)
    
    
    Mask: $E003
    

## MMC3-compatible registers ($8000-$FFFF)
    
    
    Mask: $E001
    
    See [MMC3](MMC3.xhtml "MMC3").
    

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
