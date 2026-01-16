# NES 2.0 Mapper 329

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_329) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_329)

NES 2.0 mapper 329 is used for the _Education Computer 2000_. Its UNIF board name is **UNL-EDU2000**. It's basically [BNROM](INES_Mapper_034.xhtml "BNROM") with 32 KiB WRAM and selectable mirroring. 

## Banks

  * CPU $6000-$7FFF: 8 KiB switchable PRG-RAM bank
  * CPU $8000-$FFFF: 32 KiB switchable PRG-ROM bank
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM



## Data Latch ($8000-$FFFF)
    
    
    Mask: $8000
    
    D~7654 3210
      ---------
      WWMP PPPP
      |||+-++++- Select 32 KiB PRG-ROM bank at CPU $8000-$FFFF
      ||+------- Select nametable mirroring type
      ||          0: Vertical
      ||          1: Horizontal
      ++-------- Select 8 KiB PRG-RAM bank at CPU $6000-$7FFF
    
