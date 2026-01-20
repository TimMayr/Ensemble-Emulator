# NES 2.0 Mapper 435

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_435) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_435)

**NES 2.0 Mapper 435** denotes the **F-1002** multicart circuit board mounting 2 MiB of PRG-ROM plus 8 KiB of unbanked CHR-RAM. It is basically a larger version of [NES 2.0 Mapper 380](NES_2_0_Mapper_380.xhtml "NES 2.0 Mapper 380"). 

## Address Latch ($8000-$FFFF, write)
    
    
    [A~1... CQOQ .QQP PpMS]
            ||||  ||| |||+-0: PRG A14=CPU A14
            ||||  ||| |||  1: PRG A14=p
            ||||  ||| ||+- 0: Vertical mirroring
            ||||  ||| ||   1: Horizontal mirroring
            ||||  ||+-++-- PRG A16..A14 (inner bank)
            |+|+--++------ PRG A20..A17 (outer bank)
            | +----------- 0: When CPU A14=1: PRG A16..14=111 (UNROM)
            |              1: When CPU A14=1: PRG A16..14=PPp (NROM)
            +------------- 0: CHR-RAM writeable
                           1: CHR-RAM write-protected
    Power-on value: 0
    

Effective meaning: 
    
    
    Bit 9   Bit 0   Meaning
    $200s   $001s
     (O)     (S)
      0       ?     Switchable inner 16 KiB bank PPp at CPU $8000-$BFFF, fixed inner bank #7 at CPU $C000-$FFFF (UNROM)
      1       0     Switchable 32 KiB inner bank PP at CPU $8000-$FFFF (NROM-256)
      1       1     Switchable 16 KiB inner bank PPp at CPU $8000-$BFFF, mirrored at CPU $C000-$FFFF (NROM-128)
    

Because all bits are cleared on reset, CPU $8000-$BFFF is set to 16 KiB bank #0, and $C000-$FFFF is set to 16 KiB bank #7 on reset. 

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
