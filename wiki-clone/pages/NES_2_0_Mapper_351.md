# NES 2.0 Mapper 351

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_351) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_351)

NES 2.0 Mapper 351 is used for Techline XB-xxx multicarts. 

## Contents

  * 1 Set CHR-ROM Base ($5000, write)
  * 2 Set PRG-ROM Base ($5001, write)
  * 3 Set Banking Mode ($5002, write)
  * 4 NROM Mirroring ($4025, write)



## Set CHR-ROM Base ($5000, write)
    
    
    Mask: $F003
    
    D~7654 3210
      ---------
      BBBB BBMM
      |||| ||++- ASIC compatibility mode for inner bank selection
      |||| ||     0/1: [MMC3](MMC3.xhtml "MMC3")
      |||| ||     2: [MMC1](MMC1.xhtml "MMC1")
      |||| ||     3: [VRC4](VRC2_and_VRC4.xhtml "VRC4")
      ++++-++--- Select 8 KiB CHR-ROM base
    

All bits are cleared on reset, so that MMC3 mode is active. VRC4 mode uses bits 2 and 3 ($004s and $008s) as VRC A0/A1 if address bit 11 ($800s) is clear, and as VRC A1/A0 if bit 11 is address set during a write. 

## Set PRG-ROM Base ($5001, write)
    
    
    Mask: $F003
    
    D~7654 3210
      ---------
      BBBB BB..
      ++++-++--- Select 16 KiB PRG-ROM base
    

All bits are cleared on reset. 

## Set Banking Mode ($5002, write)
    
    
    Mask: $F003
    
    D~7654 3210
      ---------
      .CCp PP.R
       ||| || +- 0: CHR-ROM mode
       ||| ||    1: CHR-RAM mode
       ||| ++--- Inner PRG-ROM bank size
       |||        00: 256 KiB (p=0), 32 KiB (p=1)
       |||        01: 128 KiB (p=0), 16 KiB (p=1)
       |||        1x: 8 KiB (p=1)
       ||+------ PRG-ROM banking mode
       |||        0: ASIC (selecte by $5000.0-1)
       |||        1: NROM
       +++------ Inner CHR-ROM bank size
                  1xx: 8 KiB
                  01x: 128 KiB
                  001: 32 KiB
                  000: 256 KiB
    

All bits are cleared on reset. Unusually, in CHR-RAM mode (R=1), CHR-ROM becomes the second half of an enlarged PRG address space that becomes addressable via register $5001. At least one multicart containing both TLROM and UNROM games makes use of this feature and puts the UNROM game's PRG data into CHR-ROM. This seems to be possible as the mapper ASIC, PRG and CHR-ROM are under a single glob. 

## NROM Mirroring ($4025, write)
    
    
    Mask: $FFFF
    
    D~7654 3210
      ---------
      .... M...
           +---- 0: Vertical Mirroring
                 1: Horizontal Mirroring
    

In NROM mode ($5002.4=1), mirroring can be changed via this [FDS](Family_Computer_Disk_System.xhtml "FDS")-inspired register as well, apparently in an attempt to become compatible with FDS conversions. 

Categories: [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [MMC3-like mappers](Category_MMC3_like_mappers.xhtml), [Multi-ASIC mappers](Category_Multi_ASIC_mappers.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
