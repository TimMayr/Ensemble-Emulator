# NES 2.0 Mapper 375

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_375) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_375)

**iNES Mapper 375** denotes an address-latch-based multicart circuit board mounting 2 MiB of PRG-ROM plus 8 KiB of unbanked CHR-RAM. It is basically an enhancement of [INES Mapper 227](INES_Mapper_227.xhtml "INES Mapper 227") that increases the PRG-ROM size to 2 MiB and allows the UNROM-like switchable bank to be changed via a data latch like a normal [UNROM](UxROM.xhtml "UNROM") board. 

## Contents

  * 1 Address Latch ($8000-$FFFF, write)
  * 2 UNROM-like Data Latch ($8000-$FFFF, write)
  * 3 Effective banking modes
  * 4 Similar mappers



## Address Latch ($8000-$FFFF, write)
    
    
    [A~1... UQLQ OQQP PpMS]
            |||| |||| |||+-0: PRG A14=p
            |||| |||| |||  1: PRG A14=CPU A14
            |||| |||| ||+- 0: Vertical mirroring
            |||| |||| ||   1: Horizontal mirroring
            |||| |||+-++-- PRG A16..A14 (inner bank)
            |+|+-|++------ PRG A20..A17 (outer bank)
            | |  +-------- 0: When CPU A14=1: PRG A16..14=LLL
            | |            1: When CPU A14=1: PRG A16..14=PPp
            | +----------- Value for PRG A16..14 when CPU A14=1 and O=0
            +------------- 0: Address latch writable, PRG A16..A14 normal
                           1: Address latch locked, PRG A16..A14 from CBA
    Power-on value: 0
    

## UNROM-like Data Latch ($8000-$FFFF, write)
    
    
    [D~.... .CBA]
             +++- PRG A16..A14 when U=1 and CPU A14=0
    

## Effective banking modes
    
    
    Bit 11  Bit 9   Bit 7   Bit 0   Meaning
    $800s   $200s   $080s   $001s
     (U)     (L)     (O)     (S)
      0       0       0       0     Switchable inner 16 KiB bank PPp at CPU $8000-$BFFF, fixed inner bank #0 at CPU $C000-$FFFF (UNROM-like with fixed bank 0)
      0       0       0       1     Switchable inner 16 KIB bank PP0 at CPU $8000-$BFFF, fixed inner bank #0 at CPU $C000-$FFFF (UNROM-like with only even banks reachable, pointless)
      0       1       0       0     Switchable inner 16 KiB bank PPp at CPU $8000-$BFFF, fixed inner bank #7 at CPU $C000-$FFFF (UNROM via address latch)
      0       1       0       1     Switchable inner 16 KIB bank PP0 at CPU $8000-$BFFF, fixed inner bank #7 at CPU $C000-$FFFF (UNROM via address latch with only even banks reachable, pointless)
      0       ?       1       0     Switchable 16 KiB inner bank PPp at CPU $8000-$BFFF, mirrored at CPU $C000-$FFFF (NROM-128)
      0       ?       1       1     Switchable 32 KiB inner bank PP at CPU $8000-$FFFF (NROM-256)   
      1       0       0       0     Switchable inner 16 KiB bank CBA at CPU $8000-$BFFF, fixed inner bank #0 at CPU $C000-$FFFF (UNROM-like with fixed bank 0)
      1       0       0       1     Switchable inner 16 KIB bank CB0 at CPU $8000-$BFFF, fixed inner bank #0 at CPU $C000-$FFFF (UNROM-like with only even banks reachable, pointless)
      1       1       0       0     Switchable inner 16 KiB bank CBA at CPU $8000-$BFFF, fixed inner bank #7 at CPU $C000-$FFFF (UNROM via data latch)
      1       1       0       1     Switchable inner 16 KIB bank CB0 at CPU $8000-$BFFF, fixed inner bank #7 at CPU $C000-$FFFF (UNROM via data latch with only even banks reachable, pointless)
    

  * CHR-RAM is write-protected when O=1 and write-enabled when O=0, i.e. write-protected in the NROM-128 and NROM-256 modes.
  * Because all bits are cleared on reset, both CPU $8000-$BFFF and $C000-$FFFF are set to 16 KiB bank #0 on reset.



## Similar mappers

  * [INES Mapper 227](INES_Mapper_227.xhtml "INES Mapper 227") is a variant with only up to 1 MiB PRG-ROM without the data latch.
  * [INES Mapper 242](INES_Mapper_242.xhtml "INES Mapper 242") is a variant with only up to 512 KiB PRG-ROM without the data latch that moves the _m_ bit to a different bit location. A variant with two PRG-ROM chips exists as well.
  * [NES 2.0 Mapper 380](NES_2_0_Mapper_380.xhtml "NES 2.0 Mapper 380") is an incompatible variant without the data latch.
  * [NES 2.0 Mapper 449](NES_2_0_Mapper_449.xhtml "NES 2.0 Mapper 449") is an incompatible variant whose data latch is for 32 KiB CHR-RAM bankswitching functionality instead.



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
