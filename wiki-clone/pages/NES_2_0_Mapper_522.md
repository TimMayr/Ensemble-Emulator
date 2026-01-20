# NES 2.0 Mapper 522

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_522) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_522)

NES 2.0 Mapper 522 is used for Whirlwind Manu's cartridge conversion of the Famicom Disk System game 風雲 少林拳 (Fūun Shōrinken). Its UNIF board name is **UNL-LH10**. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 $8000-$9FFE (even)
    * 2.2 $8001-$9FFF (odd)
  * 3 Notes



# Banks

  * CPU $6000-$7FFF: 8 KiB PRG-ROM, hard-wired to second-to-last bank
  * CPU $8000-$9FFF: 8 KiB PRG-ROM, switchable
  * CPU $A000-$BFFF: 8 KiB PRG-ROM, switchable
  * CPU $C000-$DFFF: 8 KiB PRG-RAM
  * CPU $E000-$FFFF: 8 KiB PRG-ROM, hard-wired to last bank
  * PPU $0000-$1FFF: 8 KiB of unbanked CHR-RAM



# Registers

## $8000-$9FFE (even)

Select bank register number 
    
    
    $0-$5: No function
    $6: Select 8 KiB PRG-ROM bank at CPU $8000-$9FFF
    $7: Select 8 KiB PRG-ROM bank at CPU $A000-$BFFF
    

## $8001-$9FFF (odd)

Data for bank register selected by $8000-$9FFE (even). 

# Notes

  * Mirroring is hard-wired (to Vertical in Fūun Shōrinken's case).
  * The register layout indicates that an MMC3 clone is being used, connected in a restricted manner.



Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml)
