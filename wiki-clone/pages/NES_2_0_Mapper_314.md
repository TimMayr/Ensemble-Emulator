# NES 2.0 Mapper 314

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_314) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_314)

NES 2.0 Mapper 314 is used for the _32-in-1 Y2K_ , _64-in-1 Y2K_ and _76-in-1 Y2K_ multicarts. Its UNIF board name is **BMC-64in1NoRepeat**. It exists in both CHR-ROM and unbanked 8 KiB CHR-RAM variants. 

## Contents

  * 1 Miscellaneous register ($5000, write)
  * 2 PRG-ROM register ($5001, write)
  * 3 CHR-ROM mode ($5002, write)
  * 4 UNROM Latch ($8000-$FFFF, write)



## Miscellaneous register ($5000, write)
    
    
    Mask: Unknown, probably $F003
    
    D~7654 3210
      ---------
      U.M. .CC.
      | |   ++- Bits 0-1 of 8 KiB CHR-ROM bank at PPU $0000-$1FFF
      | +------ Select nametable mirroring type
      |          0: Vertical
      |          1: Horizontal
      +-------- 0: UNROM
                1: NROM
    
    Power-on value: $80
    

## PRG-ROM register ($5001, write)
    
    
    Mask: Unknown, probably $F003
    
    D~7654 3210
      ---------
      MpPP PPPP
      ||++-++++- Select 32 KiB PRG-ROM bank at CPU $8000-$FFFF
      |+-------- Select 16 KiB PRG-ROM bank at CPU $8000-$BFFF and
      |          CPU $C000-$FFFF if M==0
      +--------- Select PRG banking mode
                  0: NROM-128
                  1: NROM-256
    
    Power-on value: $43
    

## CHR-ROM mode ($5002, write)
    
    
    Mask: Unknown, probably $F003
    
    D~7654 3210
      ---------
      .... CCCC
           ++++- Bits 2-5 of 8 KiB CHR-ROM bank at PPU $0000-$1FFF
    
    Power-on value: $00
    

## UNROM Latch ($8000-$FFFF, write)
    
    
    Mask: $8000
    
    D~7654 3210
      ---------
      .... .CCC
            +++- Select 16 KiB PRG-ROM bank at CPU $8000-$BFFF
    
    Power-on value: $00
    

Categories: [Multicart mappers](Category_Multicart_mappers.xhtml)
