# NES 2.0 Mapper 285

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_285) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_285)

**NES 2.0 Mapper 285** is used for the _A65AS_ and _JY-066_ multicarts. The UNIF board name for both incompatible variants is **BMC-A65AS**. A second UNIF board name, **BMC-GF-401CD** refers to submapper 1. A later-discovered Russian bootleg cartridge of _Highway Star_ (Русские Гонки) is assigned to mapper 285 submapper 1 as well, as it uses UNROM-banking while changing mirroring via latch bit 3. 

## Contents

  * 1 Banks
  * 2 Data Latch (CPU $8000-$FFFF)
    * 2.1 A65AS (Submapper 0)
    * 2.2 JY-066 (Submapper 1)



# Banks

  * CPU $8000-$BFFF: In 16 KiB mode: 16 KiB switchable inner bank, 128 KiB switchable outer bank
  * CPU $C000-$FFFF: In 16 KiB mode: 16 KiB fixed inner bank 7, 128 KiB switchable outer bank
  * CPU $8000-$FFFF: In 32 KiB mode: 32 KiB switchable bank
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM



# Data Latch (CPU $8000-$FFFF)

## A65AS (Submapper 0)
    
    
    Bit 7654 3210
        ---------
        SPMO OBBB
        |||| |+++- Select 16 KiB inner PRG-ROM bank at CPU $8000-$BFFF in 16 KiB mode (bit 6=0)
        |||| |++-- Select 32 KiB PRG-ROM bank at CPU $8000-$FFFF in 32 KiB mode (bit 6=1)
        |||+-+---- Select 128 KiB outer PRG-ROM bank at CPU $8000-$FFFF in 16 KiB mode (bit 6=0)
        ||+------- Select nametable mirroring
        ||         0: Vertical (bit 7=0)/Single-screen, page 0 (bit 7=1)
        ||         1: Horizontal (bit 7=0)/Single-screen, page 1 (bit 7=1)
        |+-------- Select PRG banking mode
        |          0: 16 KiB
        |          1: 32 KiB
        +--------- Select nametable mirroring type
                   0: Horizontal/vertical (selected via bit 3)
                   1: Single-screen (selected via bit 5)
    

## JY-066 (Submapper 1)
    
    
    Bit 7654 3210
        ---------
        SPMO HBBB
        |||| |+++- Select 16 KiB inner PRG-ROM bank at CPU $8000-$BFFF in 16 KiB mode (bit 6=0)
        |||| |++-- Select 32 KiB PRG-ROM bank at CPU $8000-$FFFF in 32 KiB mode (bit 6=1)
        |||| +---- Select nametable mirroring if bit 7=0
        ||||       0: Vertical
        ||||       1: Horizontal
        ||++------ Select 128 KiB outer PRG-ROM bank at CPU $8000-$FFFF in 16 KiB mode (bit 6=0)
        ||+------- Select nametable mirroring if bit 7=1
        ||         0: Single-screen, page 0
        ||         1: Single-screen, page 1
        |+-------- Select PRG banking mode
        |          0: 16 KiB
        |          1: 32 KiB
        +--------- Select nametable mirroring type
                   0: Horizontal/vertical (selected via bit 3)
                   1: Single-screen (selected via bit 5)
    

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
