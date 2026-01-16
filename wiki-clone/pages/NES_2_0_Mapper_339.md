# NES 2.0 Mapper 339

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_339) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_339)

NES 2.0 mapper 339 is used for a _21-in-1_ multicart. Its UNIF board name is **BMC-K-3006**. 

## Outer Bank and Mode Register ($6000-$7FFF, write)
    
    
    Mask: $E000
    
    A~FEDC BA98 7654 3210
      -------------------
      0110 .... ..PO OIIi
                  || |+++- Select 16 KiB inner PRG-ROM bank at CPU $8000-$BFFF and
                  || |     $C000-$FFFF (NROM-128) if P=0.
                  || |     If P=0 and II=6, i is replaced with CPU A14 (NROM-256).
                  |+-+---- Select 128 KiB outer PRG-ROM and CHR-ROM bank
                  +------- Select PRG banking mode
                            0: NROM, use OO and IIi bits
                            1: MMC3, use only OO bits
    

As it uses the MMC3 clones's WRAM interface, writing to the Outer Bank register requires enabling and not write-protecting WRAM in the MMC's $A001 register. 

## MMC3-compatible registers
    
    
    Mask: $E001
    
    See [MMC3](MMC3.xhtml "MMC3").
    

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
