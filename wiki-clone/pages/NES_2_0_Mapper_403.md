# NES 2.0 Mapper 403

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_403) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_403)

**NES 2.0 Mapper 403** denotes the **89433** circuit board with up to 1 MiB PRG-ROM and 32 KiB of CHR-RAM, bankable with 8 KiB granularity. 

  * _Tetris Family - 玩家 19-in-1 智瑟實典_ (NO-1683)
  * _Sachen Superpack_ (versions A-C)



## PRG bank register ($4100)
    
    
    Mask: $E103
    
    D~[.PPP PPp.]
        +++-+++--- PRG A19..A14
    
    Power-on value: $00
    

## CHR bank register ($4101, $8000-$FFFF)
    
    
    Mask: $E103, $8000
    
    D~[.... ..CC]
              ++-- CHR A14..A13
    
    Power-on value: $00
    

The latch at $8000-$FFFF is enabled/disabled via the Mode register's C bit. 

## Mode register ($4102)
    
    
    Mask: $E103
    
    D~[...H .C.N]
          |  | +- 0: PRG A14=CPU A14 (NROM-256)
          |  |    1: PRG A14=p (NROM-128)
          |  +--- 0: Latch at $8000-$FFFF disabled
          |       1: Latch at $8000-$FFFF enabled
          +------ 0: Vertical mirroring
                  1: Horizontal mirroring
    
    Power-on value: $00
    

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
