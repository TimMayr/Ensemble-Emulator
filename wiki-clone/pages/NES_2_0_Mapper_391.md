# NES 2.0 Mapper 391

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_391) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_391)

**NES 2.0 Mapper 391** denotes the MMC3-based **BS-110** PCB. It is a variation of [INES Mapper 052](INES_Mapper_052.xhtml "INES Mapper 052") that adds support for GNROM-like PRG bankswitching. Note that the UNIF MAPR **BMC-BS-110** is incorrectly-named and refers to [NES 2.0 Mapper 444](NES_2_0_Mapper_444.xhtml "NES 2.0 Mapper 444") instead. 

## Outer Bank Register ($6000-$7FFF, write)
    
    
    Mask: $E000
    
    A~FEDC BA98 7654 3210  D~7654 3210
      -------------------    ---------
      011. ...C .... ....    LcGC pNPP
              |              |||| ||++- PRG A17..A18
              |              |||| |+--- NROM mode type if G=1
              |              |||| |      0=NROM-128
              |              |||| |      1=NROM-256
              |              |||| +---- Outer PRG bank size
              |              ||||        0=256 KiB (PRG A17 from MMC3)
              |              ||||        1=128 KiB (PRG A17 from outer bank register)
              |              |||+------ CHR A17
              |              ||+------- PRG banking mode
              |              ||          0=MMC3 PRG banking mode
              |              ||          1=GNROM-like PRG banking mode
              |              |+-------- Outer CHR bank size
              |              |           0=256 KiB (CHR A17 from MMC3)           |
              |              |           1=128 KiB (CHR A17 from outer bank register)
              |              +--------- 1=Lock outer bank register
              +------------------------ CHR A18
    

GNROM mode means that: 

  * the MMC3's CPU A14 input is held low, so MMC3 registers 6 and 7 apply to both $8000/$A000 and $C000/$E000,
  * in NROM-256 mode, PRG A14 is additionally replaced with CPU A14.



As it uses the MMC3 clones's WRAM interface, writing to the outer bank register requires enabling and not write-protecting WRAM in the MMC's $A001 register. 

## MMC3-compatible registers ($8000-$FFFF)
    
    
    Mask: $E001
    
    See [MMC3](MMC3.xhtml "MMC3").
    

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
