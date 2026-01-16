# NES 2.0 Mapper 443

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_443) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_443)

**NES 2.0 Mapper 443** denotes the **NC3000M** multicart circuit board. 

Known cartridges: 

  * _3000-in-1_



## Outer Bank and Mode Register ($6000-$7FFF, write)
    
    
    Mask: $E000
    
    A~FEDC BA98 7654 3210
      -------------------
      011. .... ...P MMpC
                   | |||+- CHR A18
                   | ||+-- PRG A18
                   | ++--- Select PRG-ROM mode
                   |        0: MMC3 mode
                   |        1: NROM-256 mode (PRG A13..14=CPU A13..14, PRG A15..16=MMC3 A15..16)
                   |        2: Read solder pads' value in D0/D1 (leftmost pads in picture)
                   |        3: NROM-128 mode (PRG A13=CPU A13, PRG A14..16=MMC3 A14..16)
                   +------ PRG A17
    

  * WRAM must be enabled in $A001.7 before writing to this register.
  * The inner PRG bank is restricted to 128 KiB.
  * The inner CHR bank is unrestricted to 256 KiB.
  * NROM mode forces MMC3's CPU A13 and A14 inputs to GND to make MMC3 bank register 6 apply to the entire CPU $8000-$FFFF range.



## MMC3-compatible registers

Mask: $E001 

See [MMC3](MMC3.xhtml "MMC3"). 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
