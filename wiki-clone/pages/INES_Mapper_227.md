# INES Mapper 227

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_227) | View [other pages](Special_AllPages.xhtml#INES_Mapper_227)

**iNES Mapper 227** denotes an address-latch-based multicart circuit board (PCB code **810449-C-A1** among others) mounting 1 MiB of PRG-ROM plus 8 KiB of unbanked CHR-RAM. 

A variant used by Chinese RPGs from Nanjing, Waixing (PCB code **FW-01**) and Yancheng adds 8 KiB battery-backed WRAM and does not implement the UNROM-like bankswitching modes. 

Other PCB that use this mapper : 

  * **N120-72** : _[1992 Contra 120 in 1](1992_Contra_120_in_1.xhtml "1992 Contra 120 in 1")_ multicart.



Nestopia Plus! has defined submappers 0 and 1: 

  * **submapper 0** denoting that CHR-RAM is _not_ write-protected in NROM modes (for use with Chinese RPGs),
  * **submapper 1** denoting that CHR-RAM _is_ write-protected in NROM modes (for use with multicarts), and a solder pad read mode is available.
  * **submapper 2** denoting that CHR-RAM _is_ write-protected in NROM modes (for use with multicarts), and "inner bank #0" also means "outer bank #0".



## Address Latch ($8000-$FFFF, write)
    
    
    [A~1... .mLQ OQQP PpMS]
             ||| |||| |||+-0: PRG A14=p
             ||| |||| |||  1: PRG A14=CPU A14
             ||| |||| ||+- 0: Vertical mirroring
             ||| |||| ||   1: Horizontal mirroring
             ||| |||+-++-- PRG A16..A14 (inner bank)
             ||+-|++------ PRG A19..A17 (outer bank)
             ||  +-------- 0: When CPU A14=1: PRG A16..14=LLL
             ||            1: When CPU A14=1: PRG A16..14=PPp
             |+----------- Value for PRG A16..14 when CPU A14=1 and O=0
             +------------ 0: PRG A3..A0=CPU A3..A0
                           1: PRG A3..A0=Solder pad 3-0 (submapper 1 only)
    Power-on value: 0
    

Effective meaning: 
    
    
    Bit 9   Bit 7   Bit 0   Meaning
    $200s   $080s   $001s
     (L)     (O)     (S)
      0       0       0     Switchable inner 16 KiB bank PPp at CPU $8000-$BFFF, fixed inner bank #0 at CPU $C000-$FFFF (UNROM-like with fixed bank 0)
      0       0       1     Switchable inner 16 KIB bank PP0 at CPU $8000-$BFFF, fixed inner bank #0 at CPU $C000-$FFFF (UNROM-like with only even banks reachable, pointless)
      1       0       0     Switchable inner 16 KiB bank PPp at CPU $8000-$BFFF, fixed inner bank #7 at CPU $C000-$FFFF (UNROM)
      1       0       1     Switchable inner 16 KIB bank PP0 at CPU $8000-$BFFF, fixed inner bank #7 at CPU $C000-$FFFF (UNROM with only even banks reachable, pointless)
      ?       1       0     Switchable 16 KiB inner bank PPp at CPU $8000-$BFFF, mirrored at CPU $C000-$FFFF (NROM-128)
      ?       1       1     Switchable 32 KiB inner bank PP at CPU $8000-$FFFF (NROM-256)
    

  * On multicart PCBs (no battery or **submapper 1**), CHR-RAM is write-protected when O=1 and write-enabled when O=0, i.e. write-protected in the NROM-128 and NROM-256 modes.
  * When the _m_ bit is set, PRG A3-A0 are replaced with the values of four solder pads, which when the menu code reads particular ROM locations effectively selects one of up to sixteen menus with different game counts.
  * Because all bits are cleared on reset, both CPU $8000-$BFFF and $C000-$FFFF are set to 16 KiB bank #0 on reset.



## Similar mappers

  * [INES Mapper 242](INES_Mapper_242.xhtml "INES Mapper 242") is a variant with only up to 512 KiB PRG-ROM that moves the _m_ bit to a different bit location. A variant with two PRG-ROM chips exists as well.
  * [NES 2.0 Mapper 375](NES_2_0_Mapper_375.xhtml "NES 2.0 Mapper 375") is a variant with up to 2 MiB PRG-ROM that allows the UNROM-like switchable bank to be changed via a data latch.
  * [NES 2.0 Mapper 380](NES_2_0_Mapper_380.xhtml "NES 2.0 Mapper 380") is an incompatible variant with similar functionality.
  * [NES 2.0 Mapper 449](NES_2_0_Mapper_449.xhtml "NES 2.0 Mapper 449") is an incompatible variant that adds 32 KiB CHR-RAM bankswitching functionality.



## See also

  * Dumping thread: [https://forums.nesdev.org/viewtopic.php?f=9&t=15271](https://forums.nesdev.org/viewtopic.php?f=9&t=15271)



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
