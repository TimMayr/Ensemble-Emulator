# NES 2.0 Mapper 299

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_299) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_299)

NES 2.0 Mapper 299 is used for the TXC 6-in-1 multicart (MGC-023). Its UNIF board name is **BMC-11160**. 

## Data latch
    
    
    Mask: $8000
    
    Bit 7654 3210
        ---------
        M.PP ..CC
        | ||   ++- Select 8 KiB (inner) CHR-ROM bank at PPU $0000-$1FFF
        | ++------ Select 32 KiB PRG-ROM bank at CPU $8000-$FFFF, and 32 KiB (outer) CHR-ROM bank
        +--------- Select nametable mirroring, 0=horizontal, 1=vertical
    

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
