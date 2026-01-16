# NES 2.0 Mapper 265

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_265) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_265)

NES 2.0 Mapper 265 is used for T-262 multicarts, all of which use 8 KiB of CHR-RAM. It latches both address and data bits. Its UNIF board name is **BMC-T-262**. 

## Address latch
    
    
    Mask: $8000
    
    $8000: A~[1.L. ..OO POO. ..MN]
                |    || |||    |+- 1=Replace PRG A14 with CPU A14 (NROM-256 when also P=1)
                |    || |||    +-- Select nametable mirroring. 0=Vertical, 1=Horizontal
                |    ++-|++------- 128 KiB Outer PRG-ROM bank at $8000-$FFFF
                |       +--------- PRG-ROM banking mode
                |                   0: Fixed Inner PRG-ROM bank 7 at $C000-$FFFF (UNROM)
                |                   1: Mirrored Inner PRG-ROM bank, selected by data latch(NROM-128)
                +----------------- Locking bit. 1=Prevent further writes from changing the latched
                                   address bits. The data bits, which select the inner PRG bank,
                                   are still changeable.
                                   
    

## Data latch
    
    
    Mask: $8000
    
    $8000: 7654 3210
           ---------
           .... .PPP
                 +++- Select 16 KiB Inner PRG-ROM bank at $8000-$BFFF,
                      also at $C000-$FFFF if address bit 7==1
    

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
