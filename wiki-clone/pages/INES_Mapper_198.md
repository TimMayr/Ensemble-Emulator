# INES Mapper 198

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_198) | View [other pages](Special_AllPages.xhtml#INES_Mapper_198)

iNES Mapper 198 is used for the original Hong Kong release, by an unknown publisher, of 吞食天地: 三国外传 (Tūnshí Tiāndì - Sānguó Wàizhuàn), a Chinese translation of Capcom's 天地を喰らう II: 諸葛孔明伝 (Destiny of an Emperor II). In addition to normal 8 KiB of CHR-RAM and 8 KiB of battery-backed WRAM at $6000-$7FFF, it has an additional non-backed 4 KiB of WRAM at $5000-$5FFF. 640 KiB of PRG-ROM data is spread over two chips, 512 KiB providing the data for 8 KiB PRG-ROM banks $00-$3F, and 128 KiB providing the data for 8 KiB PRG-ROM banks $40-$4F, for a total of 640 KiB of PRG-ROM. Special consideration is needed for the two fixed MMC3 banks, which are located in the second chip, meaning that they are bank numbers $4E and $4F, respectively. 

[INES Mapper 199](INES_Mapper_199.xhtml "INES Mapper 199") provides the same amount of RAM with more conventional (oversize) PRG-ROM banking. 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
