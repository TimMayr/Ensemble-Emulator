# NES 2.0 Mapper 295

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_295) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_295)

NES 2.0 Mapper 295 is used for several multicarts from J.Y. Company: 

  * _1996 Soccer 7-in-1 (JY-014B)_
  * _1997 Super HIK 4-in-1 (JY-095)_
  * _1997 Super HIK 7-in-1 (JY-096)_
  * _1997 Super HIK 4-in-1 (JY-099)_
  * _1997 Super 9-in-1 (JY-109)_
  * _1997 Super 13-in-1 (JY-110)_



Its UNIF board name is **BMC-13in1JY110** , although FCEUX' support is non-working. It uses the [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"). 

The same bits in register $D003 select both PRG and CHR outer banks. It is almost identical to [NES 2.0 Mapper 281](NES_2_0_Mapper_281.xhtml "NES 2.0 Mapper 281"), except that the outer bank size is 128 KiB rather than 256 KiB. 

## Outer Bank Register ($D003)
    
    
    Mask: $F003
    
    7654 3210
    ---- ----
    .... .CCC
          +++- Select 128 KiB outer PRG-ROM bank
          +++- Select 128 KiB outer CHR-ROM bank
    

PRG ($8000-$8FFF) and CHR ($9000-$AFFF) bank select register bits that select 128/256 KiB banks are masked off. 

For a description of all other registers, see [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"). 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using J.Y. Company ASIC](Category_Mappers_using_J_Y__Company_ASIC.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with ROM nametables](Category_Mappers_with_ROM_nametables.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
