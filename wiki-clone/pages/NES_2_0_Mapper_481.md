# NES 2.0 Mapper 481

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_481) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_481)

**NES 2.0 Mapper 481** denotes the Subor **045N** PCB, used on at least one educational computer cartridge with two games on it. It's basically 2x128 KiB of [UNROM](UxROM.xhtml "UNROM"), switched by an outer bank bit. 

  * _小霸王 2合1꞉ 仓库世家 & 动脑筋_



## Banks

  * CPU 8000-BFFF: Switchable 16 KiB inner bank
  * CPU C000-FFFF: Fixed 16 KiB inner bank 7
  * PPU 0000-1FFF: 8 KiB unbanked CHR-RAM
  * Mirroring: hard-wired to horizontal or vertical



## Bank register ($8000-$FFFF, write)
    
    
    D~[O... .BBB], Address mask: $8000
       |     +++- PRG A16..A14 if CPU A14=0
       |          7 if CPU A14=1
       +--------- PRG A17 regardless of CPU A14
    

## Notes

The common dump of _小霸王 2合1꞉ 仓库世家 & 动脑筋_ is an overdump for FCEUX' idiosyncratic implementation of [INES Mapper 241](INES_Mapper_241.xhtml "INES Mapper 241"). 

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
