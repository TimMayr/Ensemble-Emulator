# NES 2.0 Mapper 323

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_323) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_323)

NES 2.0 Mapper 323 is used for Farid's homebrew 8-in-1 SLROM multicart board. It consists of an MMC1 (clone) with WRAM replaced by an outer bank register. Its UNIF board name is **FARID_SLROM_8-IN-1** (note underscores). 

## Outer Bank Register ($6000-$7FFF, write)
    
    
    Mask: $E000
    
    D~7654 3210
      ---------
      .OOO L...
       ||| +---- 1=Disable decoder (lock Outer Bank)
       +++------ Select 128 KiB outer PRG-ROM and CHR-ROM bank
    

WRAM must be enabled in the MMC1's registers for writes to be effective. 

This register is cleared on both warm and cold boot. 

## MMC1 registers ($8000-$FFFF)

See [INES Mapper 001](MMC1.xhtml "INES Mapper 001"). 

## See also

[NesDev discussion](https://forums.nesdev.org/viewtopic.php?t=10632)

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
