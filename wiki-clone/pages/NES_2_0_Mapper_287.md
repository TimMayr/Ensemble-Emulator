# NES 2.0 Mapper 287

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_287) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_287)

NES 2.0 Mapper 287 is used for at least two 4-in-1 multicarts (411120-C, 811120-C). Its UNIF board names are **BMC-411120-C** and **BMC-K-3088**. The former has a DIP switch or jumper with a 0/1 setting. 

## Outer Bank Register ($6000-$7FFF, write)
    
    
    Mask: $E000
    
    D~7654 3210
      ---------
      ..II MmOO
        || |+++- Select 128 KiB Outer PRG-ROM/CHR-ROM bank regardless of banking mode
        || |+--- If DIP Switch ==1: Same function as Bit 3
        || ++--- Select PRG Banking Mode
        ||        0: MMC3 Inner Bank, 128 KiB Outer Bank
        ||        1: 32 KiB Inner Bank (Bits 0-1) and 128 KiB Outer Bank
        ++------ Select 32 KiB inner PRG-ROM bank at CPU $8000-$FFFF if Bit 3==0
    

## MMC3-compatible registers ($8000-$FFFF, write)
    
    
    Mask: $E001
    
    $8000, $8001, $A000, $A001, $C000, $C001, $E000, $E001: As normal [MMC3](MMC3.xhtml "MMC3").
    

# Notes

  * Because the outer bank register is connected to where WRAM would normally be, WRAM needs to be enabled via bit 7 of MMC3 register $A001 before accessing the outer bank register.



Categories: [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
