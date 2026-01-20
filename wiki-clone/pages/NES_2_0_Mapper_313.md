# NES 2.0 Mapper 313

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_313) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_313)

**NES 2.0 Mapper 313** denotes reset-based multicarts around the [MMC3](MMC3.xhtml "MMC3") ASIC. Its UNIF board name is **BMC-RESET-TXROM**. When an interruption of the M2 signal is detected, the higher address lines are automatically incremented. The Submapper field denotes the sizes of the individual games between which the switching occurs: 

  * **Submapper 0** : 128 KiB PRG, 128 KiB CHR 
    * _1995 Super HiK 4-in-1 - 新系列熱血專輯組合卡 (JY-008)_
    * _1997 Super HiK 4-in-1 - 新系列獅子王組合卡 (JY-047)_
    * _1997 Super HiK 4-in-1 - 新系列米奇老鼠Ⅲ组合卡 (JY-024)_
    * _4-in-1 (CK-015)_
    * _Mario Series 4-in-1 (JY-013)_
    * _Sheng Tian Gao K 4-in-1 Huang Jin Zu He_
  * **Submapper 1** : 256 KiB PRG, 128 KiB CHR 
    * _1995 Super HiK 4-in-1 (JY-044)_
  * **Submapper 2** : 128 KiB PRG, 256 KiB CHR 
    * _1995 Super HiK 4-in-1 - 新系列蝙蝠俠Ⅲ组合强卡 (JY-040)_
  * **Submapper 3** : 256 KiB PRG, 256 KiB CHR
  * **Submapper 4** : 256 KiB PRG on first game, 128 KiB PRG on all others, 128 KiB CHR 
    * _4-in-1 (A01-1)_



Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml), [NES 2.0 mappers with submappers](Category_NES_2_0_mappers_with_submappers.xhtml)
