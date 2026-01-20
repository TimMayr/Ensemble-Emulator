# INES Mapper 242

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_242) | View [other pages](Special_AllPages.xhtml#INES_Mapper_242)

**iNES Mapper 242** denotes an address-latch-based multicart circuit board mounting 512 KiB of PRG-ROM plus 8 KiB of unbanked CHR-RAM. A variant used by Chinese RPGs from Waixing adds 8 KiB battery-backed WRAM and does not implement the UNROM-like bankswitching modes. Its UNIF MAPR is **UNL-43272**. 

A variant of this PCB (PCB code **ET-113**) adds provisions for a second PRG-ROM chip that is always 128 KiB in size and that contains a replacement menu plus one or two replacement games for the first 512 KiB PRG-ROM chip that contained a menu of its own, resulting in a non-power-of-two 640 KiB of PRG-ROM. The chip select logic is implemented such that the replacement chip is banked-in on power-on/reset. 

## Address Latch ($8000-$FFFF, write)
    
    
    [A~1... .CLm OQQP PpMS]
             ||| |||| |||+-0: PRG A14=p
             ||| |||| |||  1: PRG A14=CPU A14
             ||| |||| ||+- 0: Vertical mirroring
             ||| |||| ||   1: Horizontal mirroring
             ||| |||+-++-- PRG A16..A14 (inner bank)
             ||| |++------ PRG A18..A17 (outer bank)
             ||| +-------- 0: When CPU A14=1: PRG A16..14=LLL
             |||           1: When CPU A14=1: PRG A16..14=PPp
             ||+---------- 0: PRG A4..A0=CPU A4..A0
             ||            1: PRG A4..A0=Solder pad 4-0
             |+----------- Value for PRG A16..14 when CPU A14=1 and O=0
             +------------ 0: Select 128 KiB PRG-ROM chip (appearing second in .NES file)
                           1: Select 512 KiB PRG-ROM chip (appearing first in .NES file)                         
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
    

  * On multicart PCBs (no PRG-RAM/battery), CHR-RAM is write-protected when O=1 and write-enabled when O=0, i.e. write-protected in the NROM-128 and NROM-256 modes.
  * When the _m_ bit is set, PRG A4-A0 are replaced with the values of five solder pads, which when the menu code reads particular ROM locations effectively selects one of up to 32 menus with different game counts.
  * Because all bits are cleared on reset, both CPU $8000-$BFFF and $C000-$FFFF are set to 16 KiB bank #0 on reset.



## See also

  * [Thread with PCB photos](https://forums.nesdev.org/viewtopic.php?t=24115)



## Similar mappers

  * [INES Mapper 227](INES_Mapper_227.xhtml "INES Mapper 227") is a variant with 1 MIB PRG-ROM that moves the _m_ bit to a different bit location.
  * [NES 2.0 Mapper 380](NES_2_0_Mapper_380.xhtml "NES 2.0 Mapper 380") is an incompatible variant with similar functionality.
  * [NES 2.0 Mapper 449](NES_2_0_Mapper_449.xhtml "NES 2.0 Mapper 449") is an incompatible variant that adds 32 KiB CHR-RAM bankswitching functionality.



Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
