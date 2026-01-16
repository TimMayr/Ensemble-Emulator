# NES 2.0 Mapper 372

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_372) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_372)

NES 2.0 Mapper 372 is used for a revision of the Rockman I-VI multicart (PCB ID **SFC-12**). It is [INES Mapper 045](INES_Mapper_045.xhtml "INES Mapper 045") but with one bit of outer bank register #2 working as a CHR-ROM/RAM switch. 

## CHR-AND, CHR-OR/PRG-OR MSB, CHR source ($6000 #2, write)
    
    
    Mask: $F001
    
    D~7654 3210
      ---------
      PPRC cccc
      |||| ++++- Select number of CHR bits from MMC3
      ||||       ($F: 256 KiB, $E: 128 KiB ... $7-$0: 1 KiB)
      |||+------ Select CHR A18
      ||+------- Select CHR-ROM (0)/CHR-RAM (1, unbanked)
      ++-------- Select CHR A20-A21 and PRG A21-22
    

Note that the CHR-RAM bit works in the opposite direction from [NES 2.0 Mapper 356](NES_2_0_Mapper_356.xhtml "NES 2.0 Mapper 356"). 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3 with CHR ROM and CHR RAM](Category_MMC3_with_CHR_ROM_and_CHR_RAM.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
