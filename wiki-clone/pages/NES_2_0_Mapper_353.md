# NES 2.0 Mapper 353

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_353) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_353)

NES 2.0 Mapper 353 is used for the _92 Super Mario Family_ multicart, consisting of an [MMC3](MMC3.xhtml "MMC3") clone ASIC together with a PAL. The PCB code is **81-03-05-C**. 

# Outer Bank Register ($8080-$E080)
    
    
    Mask: Unknown, but must be more than just $8080.
    
    A~FEDC BA98 7654 3210
      -------------------
      1OO. .... 1... ....
       ++----------------- Select 256 KiB outer PRG-ROM bank (PRG A18-19)
                           and 128 KiB outer CHR-ROM bank (CHR A17-18)
    
    Power-up value: $00
    

# Notes

  * In addition to the setting the outer PRG-ROM and CHR-ROM bank, the outer bank register also modifies other aspects: 
    * Outer bank 0: Uses CHR A17 instead of the MMC3's Mirroring register, similar to [INES Mapper 118](INES_Mapper_118.xhtml "INES Mapper 118").
    * Outer bank 2: The outer PRG-ROM bank size is just 128 KiB, with CHR A17 serving as PRG A17 instead. Additionally, if CHR A17 is high, 8 KiB of unbanked CHR-RAM are used instead of CHR-ROM.
    * Outer bank 3: If CHR A17 is low, PRG A17 is replaced with CPU A14, so that CPU $8000-$BFFF are taken from the first and CPU $C000-$FFF are taken from the second 128 KiB half.
  * The conversion of _Bio Miracle Bokutte Upa_ on that multicart retains all writes to the FDS sound channel registers, so an emulator could provide the expansion sound channel even though the original multicart did not.
  * The conversion of _Super Mario Bros. 2 (J)_ is incomplete and will fail to work after World 4-4.
  * Resetting the console always returns to the main menu (outer bank register reset to $0) due to a circuit detecting interruptions in the M2 signal.



# See also

[PCB image and analysis](http://forums.nesdev.org/viewtopic.php?f=9&t=17524)

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3 with CHR ROM and CHR RAM](Category_MMC3_with_CHR_ROM_and_CHR_RAM.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
