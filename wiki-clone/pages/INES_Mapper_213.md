# INES Mapper 213

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_213) | View [other pages](Special_AllPages.xhtml#INES_Mapper_213)

**iNES Mapper 213** is a duplicate of [INES Mapper 058](INES_Mapper_058.xhtml "INES Mapper 058"), assigned to by: 

  * _9999999-in-1_ (_Can You Feel the Love Tonight_ menu music)
  * _168-in-1_ (_My Way_ menu music)



[Pre-2018 versions of FCEUX](https://sourceforge.net/p/fceumm/code/186/tree//src/boards/addrlatch.c) as well as [current versions of Mesen](https://github.com/SourMesen/Mesen/blob/master/Core/Mapper213.h) emulate only the NROM-256 setting with hard-wired mirroring, while [current versions of FCEUX](https://github.com/TASVideos/fceux/blob/master/src/boards/addrlatch.cpp) implement a duplicate of mapper 58 with some logic for determining nametable mirroring that only works for one ROM (said _168-in-1_). Both ROM files run well as mapper 58. 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
