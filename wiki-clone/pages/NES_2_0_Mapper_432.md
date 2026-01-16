# NES 2.0 Mapper 432

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_432) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_432)

**NES 2.0 Mapper 432** denotes a series of MMC3-based Realtec multicart circuit boards. 

  * Submapper 0: Realtec 8043, 8086, 8090, GN-30C PCB
  * Submapper 1: Realtec 8023 PCB
  * Submapper 2: Realtec 8085 PCB
  * Submapper 3: Realtec 8145 PCB



## Solder Pad Enable Register ($6000-$7FFE, even)
    
    
    Mask: $E001
    
    D~7654 3210
      ---------
      .... ...S
              +- 1=Replace PRG-ROM with solder pad value
                   in the CPU $8000-$FFFF address range
    

## Outer Bank Register ($6001-$7FFF, odd)
    
    
    Mask: $E001
    
    D~7654 3210
      ---------
      pNQP CmMb
      |||| |||+- PRG/CHR A17 if M=1
      |||| ||+-- PRG A17 mode
      |||| ||     0: from MMC3 (256 KiB inner bank)
      |||| ||     1: from b (128 KiB inner bank)
      |||| |+--- CHR A17 mode
      |||| |      0: from MMC3 (256 KiB inner bank)
      |||| |      1: from b (128 KiB inner bank)
      |||| +---- CHR A18
      |||+------ PRG A18
      ||+------- Submapper-specific functionality
      ||         Submapper 0: PRG+CHR A19
      ||         Submapper 1: Read solder pad
      ||         Submapper 2: NROM-256 mode
      ||         Submapper 3: 512 KiB MMC3 CHR mode (like [INES Mapper 197](INES_Mapper_197.xhtml "INES Mapper 197") Submapper 2)
      |+-------- NROM-128 mode (1=enabled)
      +--------- Submapper 0/1: NROM-256 mode
                 Submapper 2/3: Lock Outer Bank register
    

As it uses the MMC3 clones's WRAM interface, writing to the outer bank register requires enabling and not write-protecting WRAM in the MMC's $A001 register. NROM-128 mode forces the MMC3 clone's CPU A14 input low, causing MMC3 register 6 to select both the bank at $8000 and $C000, and register 7 to select both the bank at $A000 and $E000. 

## MMC3-compatible registers ($8000-$FFFF)

Mask: $E001 

See [MMC3](MMC3.xhtml "MMC3"). 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
