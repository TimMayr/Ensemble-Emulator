# NES 2.0 Mapper 334

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_334) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_334)

NES 2.0 mapper 334 is used for the _5/20-in-1 1993 Copyright_ multicart. 

## Contents

  * 1 PRG-ROM Bank Register ($6000, write)
  * 2 Unknown Register ($6001, write)
  * 3 Jumper Register ($6002, read)
  * 4 MMC3-compatible registers
  * 5 Notes



## PRG-ROM Bank Register ($6000, write)
    
    
    Mask: probably $6003
    
    D~7654 3210
      ---------
      .... .PP.
            ++- Select 32 KiB PRG-ROM bank at CPU $8000-$FFFF
    

  


## Unknown Register ($6001, write)

Mask: probably $6003 

Unknown meaning; the menu writes $01 to it. 

## Jumper Register ($6002, read)
    
    
    Mask: probably $6003
    
    D~7654 3210
      ---------
      OOOO OOOJ
      |||| |||+- Jumper setting
      |||| |||    0: Show 20-in-1 menu (i.e. with repeats)
      |||| |||    1: Show 5-in-1 menu (i.e. without repeats)
      ++++-+++-- Open bus
      
    

The "hardware test" menu (seen when pressing SELECT+A+B during the menu) expects the jumper at zero. 

## MMC3-compatible registers

Mask: $E001 

See [MMC3](MMC3.xhtml "MMC3"). 

## Notes

The MMC3 clone is only used for Mirroring and CHR-ROM banking; the PRG-ROM bank comes entirely from an additional register. As they use the MMC3 clones's WRAM interface, accessing the $600x registers requires enabling and not write-protecting WRAM in the MMC's $A001 register. 

Categories: [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
