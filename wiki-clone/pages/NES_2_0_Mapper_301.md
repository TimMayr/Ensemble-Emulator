# NES 2.0 Mapper 301

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_301) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_301)

NES 2.0 Mapper 301 is used for _GG1_ multicarts. Its UNIF board name is **BMC-8157**. 

## Address latch ($8000-$FFFF)
    
    
    Mask: $8000
    
    A~FEDC BA98 7654 3210
      -------------------
      .... ..PD POOI IIM.
             || |||| ||+- Select nametable mirroring type
             || |||| ||    0: Vertical
             || |||| ||    1: Horizontal
             || |||+-++-- Select 16 KiB inner PRG-ROM bank at CPU $8000-$BFFF        
             || |++------ Select 128 KiB outer PRG-ROM bank
             +|-+-------- Select 16 KiB inner PRG-ROM bank at CPU $C000-$FFFF
              |           0: bank 0
              |           1: same as CPU $8000-$BFFF (NROM-128)
              |           2/3: bank 7 (UNROM)
              +---------- Select 512 KiB outer PRG-ROM bank
              
    

## Notes

  * On carts with 512 KiB PRG-ROM or less, a jumper or DIP switch selects whether setting bit 8 will result in wrapped-around PRG-ROM data or open bus. This switches between a short menu with no and a long menu with repeats.
  * CHR pattern data is stored in 8 KiB of unbanked CHR-RAM.



Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
