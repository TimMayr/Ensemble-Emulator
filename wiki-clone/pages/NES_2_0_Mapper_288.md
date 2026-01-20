# NES 2.0 Mapper 288

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_288) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_288)

NES 2.0 Mapper 288 is used for two GKCX1 21-in-1 multicarts. Oddly, GoodNES 3.23b has them set to [INES Mapper 133](INES_Mapper_133.xhtml "INES Mapper 133") even though it is entirely incompatible with that mapper. 

# Address latch
    
    
    A~1... .... ..MP PCCC
                  || |+++- Select 8 KiB CHR-ROM bank at PPU $0000-$1FFF
                  |+-+---- Select 32 KiB CHR-ROM bank at CPU $8000-$FFFF
                  -------- 0=V mirroring, 1=H mirroring
    

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
