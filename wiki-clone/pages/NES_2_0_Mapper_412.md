# NES 2.0 Mapper 412

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_412) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_412)

**NES 2.0 Mapper 412** denotes an "MMC3 plus NROM" multicart mapper, used for the _Intellivision 10-in-1 Plug 'n Play 2nd Edition_ as well as assorted multicarts from Henggedianzi. It uses an [MMC3](MMC3.xhtml "MMC3") clone with additional outer bank registers at $6000-$7FFF that are accessible via the MMC3's WRAM interface, meaning that WRAM must be enabled via $A001 for them to be written to. 

## 8 KiB CHR Bank Register ($6000, write)
    
    
    D~[BBBB BB..]  Mask: $E003, reset value: 0
       ++++-++---- CHR A18..A13
    

Only applicable in NROM banking mode ($6002.1=1). 

## Mode Register ($6001, write)
    
    
    D~[CARQ DBPL]  Mask: $E003, reset value: 0
       |||| |||+-- Lock register $6001
       |||| ||+--- 0: PRG A18 from MMC3, 1: from B Bit
       |||| |+---- PRG A18 if Q=2
       |||| +----- CHR A18
       |||+------- 0: PRG A17 from MMC3, 1: from A Bit
       ||+-------- 0: CHR A17 from MMC3, 1: from C Bit
       |+--------- PRG A17 if P=1
       +---------- CHR A17 if R=1
    

## PRG Bank Register ($6002, write)
    
    
    D~[BBBB bQP.]  Mask: $E003, reset value: 0
       |||| ||+--- 0: MMC3 banking, 1: NROM PRG/CHR banking
       |||| |+---- 0: NROM-128, 1: NROM-256 banking, only if P=1
       |||| +----- PRG A14 if P=1 and Q=0
       ++++------- PRG A18..A15 if P=1
    

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
