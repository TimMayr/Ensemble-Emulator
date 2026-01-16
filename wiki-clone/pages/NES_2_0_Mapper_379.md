# NES 2.0 Mapper 379

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_379) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_379)

**NES 2.0 Mapper 379** denotes a circuit board with unknown ID, used for a simple _35-in-1_ NROM multicart. It's basically identical to [INES Mapper 038](INES_Mapper_038.xhtml "INES Mapper 038") except that the bank register is in the $8000-$FFFF rather than the $7000-$7FFF range. 

## Outer Bank register ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    D~7654 3210
      ---------
      .... CCPP
           ||++- Select 32 KiB PRG-ROM bank
           ++--- Select 8 KiB CHR-ROM bank
    

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
