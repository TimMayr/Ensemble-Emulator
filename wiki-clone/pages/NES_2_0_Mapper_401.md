# NES 2.0 Mapper 401

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_401) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_401)

**NES 2.0 Mapper 401** denotes the **KC885** multicart circuit board, used for the _Super 19-in-1_ (VIP19) multicart. It is basically [mapper 45](INES_Mapper_045.xhtml "INES Mapper 045") with the higher address lines connected weirdly. 

  * PRG A13-A17: from $6000 #1 bits 0-4, same as mapper 45
  * PRG A18: either from $6000 #2 bit 5 or from $6000 #1 bit 6, depending on solder pad setting
  * PRG A19: either from $6000 #2 bit 6 or from $6000 #1 bit 5, depending on solder pad setting
  * PRG /CE: $6000 #1 bit 7, if solder pad connected



The menu code tries the two ways of selecting PRG A18 and A19 and whether $6000 #1 bit 7 disables PRG-ROM, and selects one of eight different menus based on what it finds. Other multicarts using mapper 45's chipset do the same thing; KC885 is unique in that the standard mapper 45 way of connecting PRG-ROM will not work at all. 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
