# NES 2.0 Mapper 455

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_455) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_455)

**NES 2.0 Mapper 455** denotes the NTDEC **N625836** [MMC3](MMC3.xhtml "MMC3")-based multicart circuit PCB, used for a _9000000-in-1_ multicart. 

## Extra Register ($4100, write)
    
    
    A~[010. .... ..Q. .qDC] D~[...P PPBA]
                   |   |||        | |||+- 0: MMC3 mode, 1: NROM mode
                   |   |||        | ||+-- 0: NROM-128 mode, 1: NROM-256 mode (if A=1)
                   |   |||        +-++--- PRG A16..A14 in NROM mode
                   |   ||+--------------- 0: 128 KiB MMC3 inner PRG bank, 1: 256 KiB MMC3 inner PRG bank
                   |   |+---------------- 0: 128 KiB MMC3 inner CHR bank, 1: 256 KiB MMC3 inner CHR bank
                   +---+----------------- PRG/CHR A18..A17
    

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
