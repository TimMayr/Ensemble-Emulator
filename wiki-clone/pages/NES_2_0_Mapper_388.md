# NES 2.0 Mapper 388

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_388) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_388)

**NES 2.0 Mapper 388** denotes J.Y. Company's **YY841145C** and **YY850835C** circuit boards, used for the multicarts: 

  * _1996 Super HiK 4-in-1 (JY-056 earlier version)_
  * _1996 Super HiK 5-in-1 (JY-056 later version)_



It uses the [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"), has 1 MiB of PRG-ROM and CHR-ROM each, and is similar to [NES 2.0 Mapper 386](NES_2_0_Mapper_386.xhtml "NES 2.0 Mapper 386") but has the Outer Bank register bits in a different order. This particular PCB **inhibits** ROM nametables and Extended Mirroring that would otherwise be enabled via register, similar to [INES Mapper 090](J_Y__Company_ASIC.xhtml "INES Mapper 090"). 

## Outer Bank Register ($D003)
    
    
    Mask: $F003
    
    7654 3210
    ---- ----
    ..L. PpCc
      |  |||+- CHR A18, ignored if L=1
      |  ||+-- CHR A19
      |  |+--- PRG A18
      |  +---- PRG A19
      +------- Select outer CHR-ROM bank size
                0: Mask $900x/$A00x to 256 KiB, use c
                1: Mask $900x/$A00x to 512 KiB, ignore c   
    

For a description of all other registers, see [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"). 

Categories: [Mappers using J.Y. Company ASIC](Category_Mappers_using_J_Y__Company_ASIC.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with ROM nametables](Category_Mappers_with_ROM_nametables.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
