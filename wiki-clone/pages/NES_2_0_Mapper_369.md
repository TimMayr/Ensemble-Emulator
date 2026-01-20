# NES 2.0 Mapper 369

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_369) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_369)

**NES 2.0 mapper 369** denotes the **N49C-300** circuit board, used for the _Super Mario Bros. Party_ multicart. Accordingly, it combines all the mappers used for the various Mario games (including SMB2J): [0](NROM.xhtml "INES Mapper 000"), [4](MMC3.xhtml "INES Mapper 004") and [40](INES_Mapper_040.xhtml "INES Mapper 040") with an outer bank register. Its UNIF board name is **BMC-N49C-300**. 

## Outer Bank register ($4100, write)

Mask: $C100 

The meaning of the individual bits is not understood yet. The following values are used: 
    
    
    $4100  PRG offset  CHR offset  Mapper
    Value  (8K bank#)  (8K bank#)
    -------------------------------------
    $00    $00         $00         [000: NROM](NROM.xhtml "INES Mapper 000") (Mario Bros.)
    $01    $01         $01         [000: NROM](NROM.xhtml "INES Mapper 000") (Super Mario Bros.)
    $13    $08         $03         [040: NTDEC 2722](INES_Mapper_040.xhtml "INES Mapper 040") (Super Mario Bros. 2J)
    $37    $10         $10         [004: TLROM](MMC3.xhtml "INES Mapper 004") (Super Mario Bros. 2), PRG-ROM bank limited to 128 KiB
    $FF    $20         $20         [004: TKROM](MMC3.xhtml "INES Mapper 004") (Super Mario Bros. 3), PRG-ROM bank limited to 256 KiB
    

Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [Mappers with fixed-timing cycle IRQs](Category_Mappers_with_fixed_timing_cycle_IRQs.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
