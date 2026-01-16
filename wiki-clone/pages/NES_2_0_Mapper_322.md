# NES 2.0 Mapper 322

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_322) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_322)

NES 2.0 Mapper 322 is used for the _35-in-1 (K-3033)_ multicart. Its UNIF board name is **BMC-K-3033**. 

## Outer Bank Register ($6000-$7FFF, write)
    
    
    Mask: $E000
    
    A~FEDC BA98 7654 3210
      011. .... SONO OIII
                |||| |+++- Select 16 KiB Inner PRG-ROM bank in NROM mode
                |+|+-+---- Select Outer Bank Number (size depends on mode)
                | +------- Select NROM/other mode
                |           0: NROM mode. Outer PRG-ROM/CHR-ROM bank size is 128 KiB. CHR banks still from MMC3.
                |              Bits 0-2 select inner PRG-ROM bank. 
                |              Inner banks 0 and 4: NROM-128.
                |              Inner banks 1-3, 5-7: NROM-256, bit 0 replaced with CPU A14.
                |           1: MMC3 PRG mode. Bit 7 selects outer bank size.
                +--------- Outer Bank size for MMC3 PRG mode
                            0: 128 KiB outer PRG-ROM and CHR-ROM bank size
                            1: 256 KiB outer PRG-ROM and CHR-ROM bank size
    

## MMC3-compatible registers ($8000-$FFFF, write)
    
    
    Mask: $E001
    
    $8000, $8001, $A000, $A001, $C000, $C001, $E000, $E001: As normal [MMC3](MMC3.xhtml "MMC3").
    

# Notes

  * Because the outer bank register is connected to where WRAM would normally be, WRAM needs to be enabled via bit 7 of MMC3 register $A001 before accessing the outer bank register.



Categories: [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
