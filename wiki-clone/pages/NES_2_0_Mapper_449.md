# NES 2.0 Mapper 449

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_449) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_449)

**NES 2.0 Mapper 449** denotes the circuit board used for the _Super Games King_ multicart mounting 1 MiB KiB of PRG-ROM plus 32 KiB of 8-KiB-banked CHR-RAM. It is similar to [NES 2.0 Mapper 380](NES_2_0_Mapper_380.xhtml "NES 2.0 Mapper 380"), with the bit positions moved around somewhat, and an additional data latch to select the CHR-RAM bank. 

## Address/Data Latch ($8000-$FFFF, write)
    
    
    A~[1... ..mQ OQQP PpMS] D~[.... ..CC]
              || |||| ||||            ++- CHR A14..A13
              || |||| |||+-0: PRG A14=p
              || |||| |||  1: PRG A14=CPU A14
              || |||| ||+- 0: Vertical mirroring
              || |||| ||   1: Horizontal mirroring
              |+-|+++-++-- PRG A19..A14
              |  +-------- 0: When CPU A14=1: PRG A16..14=111 (UNROM)
              |            1: When CPU A14=1: PRG A16..14=PPp (NROM)
              +----------- 0: PRG A3..A0=CPU A3..A0
                           1: PRG A3..A0=Solder pad 3-0
    Power-on value: 0
    

Effective meaning: 
    
    
    Bit 7   Bit 0   Meaning
    $080s   $001s
     (O)     (S)
      0       ?     Switchable inner 16 KiB bank PPp at CPU $8000-$BFFF, fixed inner bank #7 at CPU $C000-$FFFF (UNROM)
      1       0     Switchable 16 KiB inner bank PPp at CPU $8000-$BFFF, mirrored at CPU $C000-$FFFF (NROM-128)
      1       1     Switchable 32 KiB inner bank PP at CPU $8000-$FFFF (NROM-256)
    

  * When the _m_ bit is set, PRG A3-A0 are replaced with the values of four solder pads, which when the menu code reads particular ROM locations effectively selects one of up to 16 menus with different game counts (submapper 0 only).
  * Because all bits are cleared on reset, CPU $8000-$BFFF is set to 16 KiB bank #0, and $C000-$FFFF is set to 16 KiB bank #7 on reset.



Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
