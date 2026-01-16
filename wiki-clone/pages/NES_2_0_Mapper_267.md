# NES 2.0 Mapper 267

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_267) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_267)

NES 2.0 Mapper 267 is used for the 8-in-1 JY-119 multicart. It consists of an [MMC3](MMC3.xhtml "MMC3") clone with an outer bank register at $6000. 

## MMC3-compatible registers

Mask: $E001 

  * $8000, $8001, $A000, $A001, $C000, $C001, $E000, $E001: As normal [MMC3](MMC3.xhtml "MMC3"). Inner PRG banks are limited to 256 KiB, Inner CHR banks to 128 KiB.



## Outer Bank ($6000)

Mask: Unknown 
    
    
    7654 3210
    ---- ----
    UUB. .BB.
    ||+---++-- Select 256 KiB outer PRG-ROM and 128 KiB outer CHR-ROM bank
    ++-------- Unknown, cleared when the menu is shown, both set once a game has been selected.
    

## Notes

  * The initial value of this register must be $00 for the multicart menu to be shown.
  * The cart seems to run just fine when ignoring the two unknown bits.



Categories: [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
