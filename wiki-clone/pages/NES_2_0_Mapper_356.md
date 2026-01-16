# NES 2.0 Mapper 356

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_356) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_356)

NES 2.0 Mapper 356 is used for the J.Y. Company's _7-in-1 Rockman_ (JY-208) multicart. It is basically [INES Mapper 045](INES_Mapper_045.xhtml "INES Mapper 045") but with added functionality to switch between CHR-ROM and CHR-RAM, and between normal two-screen and four-screen mirroring. 

All registers work as [INES Mapper 045](INES_Mapper_045.xhtml "INES Mapper 045"), except $6000 sequential **register 2** (third write): 
    
    
    D~7654 3210
      ---------
      .4RC SSSS
       ||| ++++ CHR-AND block size (see Mapper 45 description)
       ||+----- High bit of CHR-OR (CHR A18, see Mapper 45 description)
       |+------ CHR memory type select
       |         0: CHR-RAM (8 KiB)
       |         1: CHR-ROM
       +------- Nametable memory select
                 0: CIRAM (two nametables, mirrored according to MMC3 setting)
                 1: Cartridge VRAM (four nametables)
    

Because unlike the original Mapper 45, there is only one high CHR-OR bit, only 512 KiB of CHR-ROM are supported. 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3 with CHR ROM and CHR RAM](Category_MMC3_with_CHR_ROM_and_CHR_RAM.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
