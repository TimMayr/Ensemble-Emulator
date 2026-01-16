# INES Mapper 101

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_101) | View [other pages](Special_AllPages.xhtml#INES_Mapper_101)

This mapper was allocated for a naïve misdumping of the [JF-10](https://nescartdb.com/profile/view/1547) board that contains _Urusei Yatsura - Lum no Wedding Bell_ , which assumed the bank bits were reversed. Otherwise, it is entirely identical to [J87](INES_Mapper_087.xhtml "INES Mapper 087"). 

## Registers

### Bank Select ($6000-$7FFF)

This will select an 8 KB CHR ROM bank for PPU $0000-$1FFF. 
    
    
    7  bit  0
    ---- ----
    CCCC CCHL
    |||| ||||
    |||| |||+- (Normal) Low CHR bit
    |||| ||+-- (Normal) High CHR bit
    ++++-++--- (Oversize only) Extra CHR bits
    

  
This is the same as J87 but with the bits in order, and so has a better defined [oversize](Oversize.xhtml "Oversize") extension; or the same as [CNROM](CNROM.xhtml "CNROM") without bus conflicts. 

Categories: [Bad iNES Mappers](Category_Bad_iNES_Mappers.xhtml), [CNROM-like mappers](Category_CNROM_like_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml)
