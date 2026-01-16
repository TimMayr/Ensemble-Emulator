# NES 2.0 Mapper 441

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_441) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_441)

**NES 2.0 Mapper 441** denotes the **841026C** and **850335C** multicart circuit boards. It is similar to [NES 2.0 Mapper 391](NES_2_0_Mapper_391.xhtml "NES 2.0 Mapper 391") in functionality, but the register bits arranged differently. 

Known cartridges: 

  * _1996 Power Rangers V 4-in-1_ (MK-412)



## Outer Bank and Mode Register ($6000-$7FFF, write)
    
    
    D~[LcCC pNPP]
       |||| ||++-- PRG A18..A17
       |||| |+---- 1=NROM-256 mode
       |||| +----- 0=PRG A17 from MMC1, 1=PRG A17 from PP
       ||++------- CHR A18..A17
       |+--------- 0=CHR A17 from MMC1, 1=CHR A17 from CC
       +---------- 1=Lock Outer Bank Register
    

  * WRAM must be enabled in $A001.7 before writing to this register.
  * NROM-256 mode forces MMC3's CPU A14 input to GND to make MMC3 bank registers 6 and 7 apply to the entire CPU $8000-$FFFF range, with PRG A13 and PRG A14 being replaced by CPU A13 and A14, respectively.



## MMC3-compatible registers

See [MMC3](MMC3.xhtml "MMC3"). 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
