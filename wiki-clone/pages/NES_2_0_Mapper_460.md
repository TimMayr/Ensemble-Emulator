# NES 2.0 Mapper 460

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_460) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_460)

**NES 2.0 Mapper 460** denotes the **FC-29-40** and **K-3101** [MMC3](MMC3.xhtml "MMC3")\--based multicart circuit boards, used by the _13-in-1 實實在在的好朋友 快打的超値感受_ and _Street Fighter III 40-in-1 - Games Screen Selectable_ multicarts. Mounting both 512 KiB of CHR ROM and 8 KiB of CHR RAM, they combine the functionalities of [INES Mapper 197](INES_Mapper_197.xhtml "INES Mapper 197") with NROM-based 32 KiB banking. 

## Extra Register ($6000-$7FFF, write)
    
    
    Mask: $E000
    
    A~[011. .... S.NM .CBB]
                 | ||  |++- PRG A18..A17
                 | ||  +--- 0: 8 KiB of unbanked CHR RAM
                 | ||       1: 512 KiB of CHR ROM
                 | ++------ PRG ROM banking mode
                 |          0x: MMC3
                 |          10: NROM-128
                 |          11: NROM-256
                 +--------- 1: Disable PRG ROM if solder pad connected
    
    Value on reset: $00
    

## Notes

  * WRAM must be enabled in $A001.7 before writing to the extra register.
  * NROM mode causes the MMC3's CPU A14 line to be held low, making the MMC3's two PRG registers apply to both $8000/$A000 and $C000/$E000.
  * NROM-256 mode additionally replaces PRG A14 with CPU A14.
  * 512 KiB of CHR ROM is realized by connecting the MMC3 clone according to [mapper 197.2](INES_Mapper_197.xhtml##Submapper_2 "INES Mapper 197").
  * Connecting the solder pad or not selects between two menus (20-in-1 and 40-in-1).



Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3 with CHR ROM and CHR RAM](Category_MMC3_with_CHR_ROM_and_CHR_RAM.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
