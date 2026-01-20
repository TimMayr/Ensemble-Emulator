# INES Mapper 088

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_088) | View [other pages](Special_AllPages.xhtml#INES_Mapper_088)

iNES Mapper 088 is the same as [mapper 206](INES_Mapper_206.xhtml "INES Mapper 206") with the following exception: 

  * CHR support is increased to 128KB by connecting PPU's A12 line to the CHR ROM's A16 line.



Consequently, CHR is split into two halves. $0xxx can only have CHR from the first 64K, $1xxx can only have CHR from the second 64K. 

A possible way to implement this would be to mask the CHR ROM 1K bank output from the mapper by ANDing with $3F, and then OR it with $40 for N108 registers 2, 3, 4, and 5. 

If the CHR ROM is 64K or smaller, it is identical to mapper 206. 

There are three games known to use this mapper: 

  * [Quinty (J)](https://nescartdb.com/profile/view/1576/quinty)
  * [Namcot Mahjong 3 - Mahjong Tengoku](https://nescartdb.com/profile/view/1828/namco-mahjong-iii-mahjong-tengoku)
  * [Dragon Spirit - Aratanaru Densetsu](https://nescartdb.com/profile/view/2307/dragon-spirit-aratanaru-densetsu)



Categories: [In NesCartDB](Category_In_NesCartDB.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml)
