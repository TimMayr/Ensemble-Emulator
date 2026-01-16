# NES 2.0 Mapper 321

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_321) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_321)

**NES 2.0 Mapper 321** denotes the **820310** [MMC3](MMC3.xhtml "MMC3")-based multicart circuit board, [UNIF](UNIF.xhtml "UNIF") MAPR **BMC-820310-C**. It is basically [NES 2.0 Mapper 287](NES_2_0_Mapper_287.xhtml "NES 2.0 Mapper 287") looking at differently-ordered address bits. 

## Outer Bank Register ($6000-$7FFF, write)
    
    
    A~[011. .... ..PP MB..], A001.7=1, A001.6=0
                   || |+---- PRG/CHR A17
                   || +----- 0=PRG A16..A13 from MMC3
                   ||        1=PRG A16..A15 from PP,
                   ||          PRG A14..A13=CPU A14..A13
                   ||          (NROM-256-like PRG banking)
                   ++------- PRG A16..A15 is M=1
    

Categories: [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
