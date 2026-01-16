# INES Mapper 154

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_154) | View [other pages](Special_AllPages.xhtml#INES_Mapper_154)

iNES Mapper 154 represents **NAMCOT-3453** , a board used only for the game [Devil Man](https://nescartdb.com/profile/view/3837/devil-man). 

It is identical to [Mapper 88](INES_Mapper_088.xhtml "INES Mapper 088"), but with the addition of a single bit allowing for mapper-controlled one-screen mirroring: 
    
    
     $8000-$FFFF: [.Mxx xxxx]
       x = See [mapper 206](INES_Mapper_206.xhtml "INES Mapper 206") documentation
       M = Mirroring
         0 = 1ScA
         1 = 1ScB
    

Note that this bit is present over the entire 32kB range; it is not present in only odd or even addresses unlike the associated Namcot 108. 

Like mapper 88, if the CHR ROM is larger than 65536 bytes, CHR ROM A16 is connected to PPU A12. This means content in the left pattern table ($0xxx) comes from the first 64kB of CHR-ROM and content in the right pattern table ($1xxx) comes from the last 64kB of CHR-ROM. 

Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
