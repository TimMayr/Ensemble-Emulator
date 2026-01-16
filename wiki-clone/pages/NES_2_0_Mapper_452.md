# NES 2.0 Mapper 452

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_452) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_452)

**NES 2.0 Mapper 452** denotes the **DS-9-27** multicart circuit board, used for the RCM _1992 蓋世 190-in-1 劃面選關_ and _1992 幸運星 350-in-1 金礸玉 劃面選關_ multicarts with 1-2 MiB PRG ROM, 8 KiB PRG RAM and 8 KiB unbanked CHR RAM. The 8 KiB of PRG RAM can be mapped anywhere between $8000-$FFFF and are used to run NROM games from partially-decompressed data, creating some rather bizarre memory layouts. 

## Address/Data Latch ($8000-$DFFF), write
    
    
    A~[1..B BBBB BBb.] D~[.UWW QLNM]
          | |||| |||       ||| |||+- Mirroring, 1=Horizontal
          | |||| |||       ||| ||+-- 1=NROM-128-like mode
          | |||| |||       ||| |+--- $E000-$FFFF bank bit 4s in NROM-256-like mode
          | |||| |||       ||| +---- 1=NROM-256-like mode
          | |||| |||       |++------ 8 KiB PRG RAM CPU address
          | |||| |||       +-------- $E000-$FFFF bank bit 8s in NROM-256-like mode
          +-++++-+++- PRG A20..A13
    

The latch seems to purposefully not respond in the $E000-$FFFF range to allow writing to PRG RAM mapped to that range without changing the mapper configuration. The following effective memory arrangements are possible: 
    
    
    Bit 3  Bit 1  PRG ROM banking
    $08s   $02s
      0      0    UNROM-like with fixed bank 0:
                  16 KiB PRG ROM bank BBBBBB at CPU $8000-$BFFF
                  16 KiB PRG ROM bank 0 at CPU $C000-$FFFF
      0      1    NROM-128-like:
                  8 KiB PRG ROM bank BBBBBBb at CPU $8000-$9FFF
                  8 KiB PRG ROM bank BBBBBBb at CPU $A000-$BFFF
                  8 KiB PRG ROM bank BBBBBBb at CPU $C000-$DFFF
                  8 KiB PRG ROM bank BBBBBBb at CPU $E000-$FFFF
      1      ?    NROM-256-like:
                  8 KiB PRG ROM bank BBBBBB0 OR 0 at CPU $8000-$9FFF
                  8 KiB PRG ROM bank BBBBBB0 OR 1 at CPU $A000-$BFFF
                  8 KiB PRG ROM bank BBBBBB0 OR 2 at CPU $C000-$DFFF
                  8 KiB PRG ROM bank BBBBBB0 OR 3 OR L*4 OR L*U*8 at CPU $E000-$FFFF
    

8 KiB of PRG RAM overlay PRG ROM in the address range specified by the WW bits: 
    
    
    WW   Address range
    00   $8000-$9FFF, also $C000-$DFFF in NROM-128-like mode only
    01   $A000-$BFFF, also $E000-$FFFF in NROM-128-like mode only
    10   $C000-$DFFF, also $8000-$9FFF in NROM-128-like mode only
    11   $E000-$FFFF, also $A000-$BFFF in NROM-128-like mode only
    

The "NROM-128-like" mode becomes such because two of the four 8 KiB banks are overlaid with PRG RAM. 

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
