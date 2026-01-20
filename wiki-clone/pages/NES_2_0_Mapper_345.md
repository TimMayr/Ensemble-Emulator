# NES 2.0 Mapper 345

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_345) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_345)

NES 2.0 Mapper 345 is used for the _New Star 6-in-1 Game Cartridge_ multicart. Unusually, it combines [MMC3](MMC3.xhtml "MMC3") and [AxROM](AxROM.xhtml "AxROM") games. Its UNIF board name is **BMC-L6IN1**. CHR data comes from 8 KiB of MMC3-banked CHR-RAM. 

## Outer Bank and Mode Register ($6000-$7FFF, write)
    
    
    Mask: $E000
    
    D~7654 3210
      ---------
      OOMS mmPP
      |||| ||++- Select 32 KiB inner PRG-ROM bank in AxROM mode
      |||| ++--- Select PRG-ROM banking mode (0=AxROM, 1-3=MMC3)
      |||+------ Select 1 KiB CIRAM bank in one-screen mirroring mode
      ||+------- Select nametable mirroring type
      ||          0: from MMC3
      ||          1: one-screen mirroring
      ++-------- Select 128 KiB outer PRG-ROM bank in either mode
    

As it uses the MMC3 clones's WRAM interface, writing to the Outer Bank register requires enabling and not write-protecting WRAM in the MMC's $A001 register. 

## MMC3-compatible registers
    
    
    Mask: $E001
    
    See [MMC3](MMC3.xhtml "MMC3").
    

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
