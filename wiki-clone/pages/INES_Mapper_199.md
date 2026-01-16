# INES Mapper 199

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_199) | View [other pages](Special_AllPages.xhtml#INES_Mapper_199)

iNES Mapper 199 is used for two kinds of Waixing boards: 

  * A duplicate of [INES Mapper 176](INES_Mapper_176.xhtml "INES Mapper 176"). Such games can be detected by them writing $02 to $5FF3 to enable Mapper 176's Extended MMC3 Mode.
  * The Waixing **FS309** circuit board, a [TNROM](TxROM.xhtml "TNROM")-compatible board with 32 KiB of battery-backed PRG RAM, of which 12 KiB are mapped into CPU address space from $5000-$7FFF. Used by the following games: 
    * 成吉思汗 (Chéngjísīhán), Waixing's Chinese translation of Koei's _Genghis Khan_
    * 風色幻想 (Fēngsè Huànxiǎng) / 湯姆歷險記中文版 (Tāngmǔ Lìxiǎn jì Zhōngwén bǎn) / 汤姆寻宝记 (Tāngmǔ Xúnbǎo jì); Waixing's Chinese translation of _Square no Tom Sawyer_
    * 吞食天地: 三国外传 (Tūnshí Tiāndì - Sānguó Wàizhuàn): Waixing's re-release of Capcom's 天地を喰らう II: 諸葛孔明伝 (Destiny of an Emperor II). Unlike the [original 1994 Hong Kong release](INES_Mapper_198.xhtml "INES Mapper 198") by an unknown publisher, this release uses one conventional 1 MiB of PRG-ROM.
  * The MMC3 clone used by this board supports more than 512 KiB PRG-ROM, necessary for that last game in the list.
  * The 8 KiB of CHR RAM are unbanked, and games do not initialize the MMC3's CHR bank registers to a contiguous 8 KiB block!



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
