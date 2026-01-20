# NES 2.0 Mapper 358

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_358) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_358)

NES 2.0 Mapper 358 denotes a variation of [NES 2.0 Mapper 282](NES_2_0_Mapper_282.xhtml "NES 2.0 Mapper 282") with partially overlapping PRG/CHR outer banks: 

  * _1997 Super Game 7-in-1 (JY-016)_ (PCB YY860606C, 512 kiB PRG-ROM)
  * _1998 Super HiK 5-in-1 (JY-113)_ (PCB YY860606C, 512 kiB PRG-ROM)
  * _1998 Super HiK 6-in-1 (JY-117)_ (unmarked PCB, 1024 KiB PRG-ROM)



## Outer Bank Register ($D003)
    
    
    Mask: $F803
    
    7654 3210
    ---- ----
    ..M. .ppC
      |   ||+-- CHR-ROM A18 if M=0, ignored if M=1
      |   ++--- PRG-ROM A18-A19
      |   +---- CHR-ROM A19
      +-------- Outer CHR bank size and selection
                 0: Mask to 256 KiB, use C
                 1: Mask to 512 KiB, ignore C
    

PRG ($8000-$8FFF) bank select register bits that select 256 KiB banks are masked off. Dumping the CHR-ROM does not work reliably in 1 KiB CHR-ROM banking mode. 

For a description of all other registers, see [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"). 

Categories: [Mappers using J.Y. Company ASIC](Category_Mappers_using_J_Y__Company_ASIC.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with ROM nametables](Category_Mappers_with_ROM_nametables.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
