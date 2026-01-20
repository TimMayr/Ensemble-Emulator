# NES 2.0 Mapper 282

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_282) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_282)

NES 2.0 Mapper 282 is used by at least three J.Y. multicarts: 

  * _1997 Super HIK 18-in-1 (JY-101)_
  * _1997 Super HIK 21-in-1 (JY-105)_
  * _1998 Super HIK 5-in-1 (JY-114)_
  * _Super 25-in-1 Final Fight (SC-128)_



Each of them have 1 MiB each of PRG-ROM and CHR-ROM. They use the [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"). Register $D003 controls 256 KiB outer banks. 

## Outer Bank Register ($D003)
    
    
    Mask: $F003
    
    7654 3210
    ---- ----
    ..LM CPPc
      || |||+- Select 256 KiB outer CHR-ROM bank (CHR A18), ignored if L=1
      || |++-- Select 256/512 KiB outer PRG-ROM bank
      || +---- If bit 4==1: Select 512 KiB outer CHR-ROM bank
      |+------ 0: Ignore bit 3
      |        1: Obey bit 3
      +------- Select outer CHR-ROM bank size
                0: Mask $900x/$A00x to 256 KiB, use c
                1: Mask $900x/$A00x to 512 KiB, ignore c
    

PRG ($8000-$8FFF) and CHR ($9000-$AFFF) bank select register bits that select 256 KiB banks (L=0)/512 KiB banks (L=1) are masked off. 

For a description of all other registers, see [J.Y. Company ASIC](J_Y__Company_ASIC.xhtml "J.Y. Company ASIC"). 

# Trivia

Some of the multicarts show different version of menu according to what $5000 is returned from the mapper (the value that is read is masked with $C0, so only bits 7 & 6 matters.): 
    
    
    1997 Super HIK 18-in-1 (JY-101)
     $00   -> 18 in 1
     $40   -> 900 in 1
     $80   -> 1600 in 1
     $C0   -> 6000000 in 1
    
    
    
    1997 Super HIK 21-in-1 (JY-105)
     $00   -> 21 in 1
     $40   -> 350 in 1
     $80   -> 1200 in 1
     $C0   -> 7000000 in 1
    

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers using J.Y. Company ASIC](Category_Mappers_using_J_Y__Company_ASIC.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with ROM nametables](Category_Mappers_with_ROM_nametables.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
