# INES Mapper 148

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_148) | View [other pages](Special_AllPages.xhtml#INES_Mapper_148)

**iNES Mapper 148** denotes the **Sachen SA-008-A** and **Tengen 800008** circuit boards, which switch up to 64 KiB of PRG-ROM in 32 KiB amounts and up to 64 KiB of CHR-ROM in 8 KiB amounts using a data latch. The bit assignment of the data latch is the same as [INES Mapper 079](NINA_003_006.xhtml "INES Mapper 079")'s, but unlike mapper 79, the latch register is in the CPU $8000-$FFFF range instead of $4100-$5FFF, introducing bus conflicts. 

## Data Latch
    
    
     Mask: $8000
     Bus conflicts:
     $8000: [.... PCCC] - Select 32 KiB PRG bank and 8 KiB CHR bank
    

Sachen's SA-008-A board uses a mask of $E000 but is otherwise the same. 

## References

  * <http://cah4e3.shedevr.org.ru/%5Blst%5D-sachen-mappers.txt>
  * [Kev's analysis of Tengen 800008](http://kevtris.org/mappers/tengen/800008.html)
  * [Box, cart, and PCB pictures](https://www.flickr.com/photos/153392699@N08/sets/72157680729618601)



Categories: [GNROM-like mappers](Category_GNROM_like_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
