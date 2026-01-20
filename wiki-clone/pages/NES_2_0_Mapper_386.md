# NES 2.0 Mapper 386

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_386) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_386)

**NES 2.0 Mapper 386** denotes J.Y. Company's **YY860729C** circuit board, used for only one known multicart: 

  * _1996 Super HiK 5-in-1 (JY-090)_



It uses the [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"), has 1 MiB of PRG-ROM and CHR-ROM each, and is similar to [NES 2.0 Mapper 282](NES_2_0_Mapper_282.xhtml "NES 2.0 Mapper 282") but has the Outer Bank register bits in a different order. [NES 2.0 Mapper 387](NES_2_0_Mapper_387.xhtml "NES 2.0 Mapper 387") is almost the same but changes the outer PRG bank size from 256 to 128 KiB. 

## Outer Bank Register ($D003)
    
    
    Mask: $F003
    
    7654 3210
    ---- ----
    ..L? PCpc
      || |||+- CHR A18, ignored if L=1
      || ||+-- PRG A18
      || |+--- CHR A19
      || +---- PRG A19
      |+------ Always 1
      +------- Select outer CHR-ROM bank size
                0: Mask $900x/$A00x to 256 KiB, use c
                1: Mask $900x/$A00x to 512 KiB, ignore c   
    

For a description of all other registers, see [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"). 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using J.Y. Company ASIC](Category_Mappers_using_J_Y__Company_ASIC.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with ROM nametables](Category_Mappers_with_ROM_nametables.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
