# NES 2.0 Mapper 281

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_281) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_281)

**NES 2.0 Mapper 281** denotes a number of compatible circuit boards from J.Y. Company: 

  * 512 KiB PRG-ROM, 512 KiB CHR-ROM: **YY841144C** , **YY850771C** , **YY851115C** , **YY860416C** , **YY860417C** , **YY860731C**
  * 1 MiB PRG-ROM, 1 MiB CHR-ROM: **YY850719C**



and is used for the following multicarts: 

  * _1996 Super HiK 4-in-1 (JY-052)_
  * _1997 Super HiK 4-in-1 (JY-052)_
  * _1996 Super HiK 4-in-1 (JY-053)_
  * _1996 Super HiK 4-in-1 (JY-054)_
  * _1997 Super HiK 4-in-1 (JY-054)_
  * _1996 Super HiK 4-in-1 (JY-055)_
  * _1996 Power Rangers HiK 4-in-1 (JY-066)_
  * _1997 Power Rangers HiK 4-in-1 (JY-066)_
  * _1996 Super HiK 3-in-1 (JY-068)_
  * _1996 Super HiK 3-in-1 (JY-080)_
  * _1996 Super HiK 5-in-1 (JY-088)_



It uses the [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"). 

## Outer Bank Register ($D003)

Mask: $F003 
    
    
    7654 3210
    ---- ----
    .... ..BB
           ++- PRG/CHR A18/A19 (256 KiB outer PRG/CHR-ROM bank)
    

PRG ($8000-$8FFF) and CHR ($9000-$AFFF) bank select register bits that select 256 KiB banks are masked off. 

For a description of all other registers, see [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"). 

Categories: [Mappers using J.Y. Company ASIC](Category_Mappers_using_J_Y__Company_ASIC.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with ROM nametables](Category_Mappers_with_ROM_nametables.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
