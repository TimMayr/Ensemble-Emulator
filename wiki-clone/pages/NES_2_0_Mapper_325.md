# NES 2.0 Mapper 325

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_325) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_325)

NES 2.0 Mapper 325 is used for _Super Mali Splash Bomb_ , a bootleg hack of _Keroppi to Keroriinu no Splash Bomb_. Its UNIF board name is **UNL-MALISB**. It's an MMC3 clone with the address input and output lines connected in an unusual manner: The MMC3 clones' ... 

  * CPU A0 input is connected to CPU A3 and additionally to CPU A2 if CPU A14==1;
  * PRG A15 (PRG bank number bit 2) is swapped with PRG A16 (PRG bank number bit 3);
  * CHR A11 (CHR bank number bit 1) is swapped with CHR A15 (CHR bank number bit 5).



Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
