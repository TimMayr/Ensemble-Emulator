# NES 2.0 Mapper 320

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_320) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_320)

NES 2.0 Mapper 320 is used for the _Super HiK 6-in-1 A-030_ multicart. Basically [UxROM](UxROM.xhtml "UxROM") with an address-latch-based outer bank register, its UNIF board name is **BMC-830425C-4391T**. Mirroring is hard-wired. 

# Banks

  * CPU $8000-$BFFF: 16 KiB switchable inner and outer PRG-ROM bank
  * CPU $C000-$FFFF: 16 KiB fixed inner and switchable outer PRG-ROM bank
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM



## Outer Bank Register ($F0E0-$F0FF, write)
    
    
    Mask: $FFE0
    
    A~FEDC BA98 7654 3210
      -------------------
      1111 0000 111M BBBB
                   | ++++- Select 128 KiB Outer Bank at CPU $8000-$FFFF
                   +------ Select Outer Bank Size
                            0: 256 KiB (UOROM), fixed inner bank #15 at CPU $C000-$FFFF
                            1: 128 KiB (UNROM), fixed inner bank #7 at CPU $C000-$FFFF
    

## Inner Bank Register ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    D~7654 3210
      ---------
      .... pPPP
           |+++- Select 16 KiB Inner PRG-ROM bank at CPU $8000-$BFFF
           +---- Select 128 KiB Inner PRG-ROM bank at CPU $8000-$BFFF if Outer Bank bit 4=0
    

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
