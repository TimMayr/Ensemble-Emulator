# NES 2.0 Mapper 344

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_344) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_344)

**NES 2.0 Mapper 344** denotes the **GN-26** [MMC3](MMC3.xhtml "MMC3")-based multicart circuit board. Its UNIF board name is **BMC-GN-26** (with incorrect PRG bank order). 

  * _快打金卡终极挑战 (Kuàidǎ Jīnkǎ Zhōngjí Tiǎozhàn) 3/6-in-1_



## Outer Bank and Mode Register ($6000-$7FFF, write)
    
    
    Mask: $E00F
    
    A~FEDC BA98 7654 3210
      -------------------
      011. .... .... SMBA
                     ||++- PRG/CHR A18..17
                     ||+-- CHR A17 mode
                     ||     0: CHR A17=MMC3 A17
                     ||     1: CHR A17=$6000.0
                     |+--- Select PRG-ROM mode
                     |      0: MMC3 PRG mode
                     |      1: NROM PRG mode
                     +---- Solder pad test
    

  * WRAM must be enabled in $A001.7 before writing to this register.
  * The inner PRG bank is restricted to 128 KiB.
  * NROM mode PRG forces MMC3's CPU A13 and A14 inputs to GND and replaces MMC3's PRG A13 output with CPU A13. This means that MMC3 bank register 6 bits 1-3 provide PRG A14..A16 for the entire CPU $8000-$FFFF range.
  * The common dump of _快打金卡终极挑战 (Kuàidǎ Jīnkǎ Zhōngjí Tiǎozhàn) 3/6-in-1_ has the 128 KiB PRG-ROM banks mixed-up, correct would be in the order 0, 3, 1, 2.
  * Depending on whether a solder pad is connected, setting the S bit may disable PRG-ROM. The menu sets this bit to select between two different game counts.



## MMC3-compatible registers

Mask: $E001 

See [MMC3](MMC3.xhtml "MMC3"). 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
