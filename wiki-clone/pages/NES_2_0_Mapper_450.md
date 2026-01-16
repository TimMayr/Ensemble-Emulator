# NES 2.0 Mapper 450

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_450) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_450)

**NES 2.0 Mapper 450** denotes the [VRC2](VRC2_and_VRC4.xhtml "VRC2")-based 晶太 (Jingtai) **YY841157C** multicart circuit board used for J.Y. Company's _1996 Super HiK 4-in-1 - 新系列瑪莉13代組合卡_ multicart. The VRC2 clone's A0/A1 inputs are connected to CPU A0/A1; its µ-write interface is used to form an additional outer bank register at CPU $6000-$6FFF. It can be seen as the oversize VRC2 equivalent of [INES Mapper 047](INES_Mapper_047.xhtml "INES Mapper 047"). 

## Outer Bank Register ($6000-$6FFF, write)
    
    
    D~[.... ..BB]
              ++- PRG/CHR A18..A17
    

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
