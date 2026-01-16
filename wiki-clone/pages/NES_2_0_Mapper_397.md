# NES 2.0 Mapper 397

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_397) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_397)

**NES 2.0 Mapper 295** is used for a single multicart from J.Y. Company: 

  * _1996 Soccer 6-in-1_ (JY-082)



The same bits in register $D003 select both PRG and CHR outer banks. It is almost identical to [NES 2.0 Mapper 295](NES_2_0_Mapper_295.xhtml "NES 2.0 Mapper 295"), except that PRG A17 selected via $8000-$8FFF is not masked off. 

## Outer Bank Register ($D003)
    
    
    Mask: $F003
    
    7654 3210
    ---- ----
    .... .BBC
          +++- Select 128 KiB outer CHR-ROM bank (CHR A19..17)
          ++-- Select 256 KiB outer PRG-ROM bank (PRG A19..18)
    

PRG ($8000-$8FFF) and CHR ($9000-$AFFF) bank select register bits that select 256/128 KiB banks are masked off. 

For a description of all other registers, see [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"). 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using J.Y. Company ASIC](Category_Mappers_using_J_Y__Company_ASIC.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with ROM nametables](Category_Mappers_with_ROM_nametables.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
