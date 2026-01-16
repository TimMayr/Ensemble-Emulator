# NES 2.0 Mapper 521

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_521) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_521)

**NES 2.0 Mapper 521** is used for _장두진 바둑교실: 입문편_ , commonly known as _Korean Igo_. Its UNIF board name is **DREAMTECH01** , without prefix. 

# Banks

  * CPU $8000-$BFFF: 16 KiB switchable PRG-ROM bank
  * CPU $C000-$FFFF: 16 KiB fixed PRG-ROM bank (last bank in ROM file)
  * PPU $0000-$1FFF: 8 KiB of unbanked CHR-RAM.



Mirroring is hard-wired. 

Since the fixed PRG-ROM bank comes from a separate PRG-ROM chip, the ROM file can have a non-power-of-two size. 

# Bank Register ($5020, write)
    
    
    Mask: $F030
    D~[.... BBBB]
            ++++- PRG A18..14 when CPU A14=0
    

Note that writes to $5030 are ignored. 

# See also

[PCB images and circuit diagram](https://forums.nesdev.org/viewtopic.php?p=296603)

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml)
