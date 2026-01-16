# NES 2.0 Mapper 373

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_373) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_373)

NES 2.0 Mapper 373 is used for a _Super 4-in-1_ multicart (PCB ID **SFC-13**). It is [INES Mapper 045](INES_Mapper_045.xhtml "INES Mapper 045") but with one bit of outer bank register #2 working as a GNROM switch. 

## CHR-AND, CHR-OR/PRG-OR MSB, CHR source ($6000 #2, write)
    
    
    Mask: $F001
    
    D~7654 3210
      ---------
      PPRC cccc
      |||| ++++- Select number of CHR bits from MMC3
      ||||       ($F: 256 KiB, $E: 128 KiB ... $7-$0: 1 KiB)
      |||+------ Select CHR A18
      ||+------- Select MMC3 (0)/GNROM-like (1) PRG mode
      ++-------- Select CHR A20-A21 and PRG A21-22
    

In GNROM-like mode, the MMC3 clone's CPU A14 input is held low, so that MMC3 registers #6 and #7 apply both to $8000/$C000 and $A000/$E000, with the bank number ANDed with NOT $02 in the lower range and ORed with $02 in the upper range. 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
