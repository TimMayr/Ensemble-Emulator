# NES 2.0 Mapper 350

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_350) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_350)

NES 2.0 mapper 350 is used for a _Super 15-in-1 Game Card_ multicart. Its UNIF board name is **BMC-891227**. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Outer Bank Register ($8000-$BFFF)
    * 2.2 Inner Bank Register ($C000-$FFFF)
  * 3 Similar Mappers



# Banks

  * CPU $6000-$7FFF: 8 KiB fixed PRG-ROM bank #1
  * CPU $8000-$BFFF: 16 KiB switchable PRG-ROM bank
  * CPU $C000-$FFFF: 16 KiB switchable PRG-ROM bank
  * PPU $0000-$1FFF: 8 KiB of unbanked CHR-RAM



# Registers

## Outer Bank Register ($8000-$BFFF)
    
    
    Mask: $C000
    
    D~7654 3210
      ---------
      MPPB B...
      |||+-+---- Select 128 KiB outer PRG-ROM bank
      ||+------- Chip select
      |||         0: First 512 KiB
      |||         1: Second 512 KiB (selectable only in UNROM mode)
      |++------- Select PRG-ROM banking mode
      |           0: NROM-128
      |           1: NROM-256 (inner bank bit 0 replaced with CPU A14)
      |           2: UNROM (inner bank fixed to #7 at CPU $C000-$FFFF)
      |           3: same as 2
      +--------- Select nametable mirroring type
                  0: Vertical
                  1: Horizontal
                  
    

CHR-RAM is write-protected in the NROM modes. 

## Inner Bank Register ($C000-$FFFF)
    
    
    Mask: $C000
    
    D~7654 3210
      ---------
      .... .BBB
            +++- Select 16 KiB inner PRG-ROM bank at CPU $8000-$BFFF/$C000-$FFFF
    

# Similar Mappers

[NES 2.0 Mapper 337](NES_2_0_Mapper_337.xhtml "NES 2.0 Mapper 337") is a variant for smaller ROM sizes of this mapper. 

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
