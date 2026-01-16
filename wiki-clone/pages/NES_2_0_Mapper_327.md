# NES 2.0 Mapper 327

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_327) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_327)

NES 2.0 mapper 327 is used for a _6-in-1_ multicart. Its UNIF board name is **BMC-10-24-C-A1**. 

## Outer Bank Register ($6000-$7FFF, write)
    
    
    Mask: $E000
    
    A~FEDC BA98 7654 3210
      -------------------
      0110 .... ..CT POOO
                  || |+++- Select 128 KiB PRG-ROM and CHR-ROM outer bank
                  || +---- Select MMC3 inner PRG-ROM bank address mask
                  ||        0: 128 KiB
                  ||        1: 256 KiB
                  |+------ Select CHR memory type
                  |         0: CHR-ROM
                  |         1: CHR-RAM
                  +------- Select MMC3 inner CHR-ROM bank address mask
                            0: 128 KiB
                            1: 256 KiB
    

As it uses the MMC3 clones's WRAM interface, writing to the Outer Bank register requires enabling and not write-protecting WRAM in the MMC's $A001 register. Note that there are also 8 KiB of WRAM in the same address range. The outer bank register becomes locked against further writes once an outer bank other than 0 is selected, and is only unlocked on power-up. 

## MMC3-compatible registers
    
    
    Mask: $E001
    
    See [MMC3](MMC3.xhtml "MMC3").
    

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3 with CHR ROM and CHR RAM](Category_MMC3_with_CHR_ROM_and_CHR_RAM.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
