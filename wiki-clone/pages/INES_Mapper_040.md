# INES Mapper 040

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_040) | View [other pages](Special_AllPages.xhtml#INES_Mapper_040)

iNES Mapper 040 denotes the NTDEC **2722** PCB and imitations, used in several cartridge conversions of the Japanese version of _Super Mario Bros. 2_ : 

  * _Super Mario Bros. 2_ from Whirlwind Manu (cartridge code LF36)
  * _Super Mario Bros. II+_ from Hey Sung
  * _1990 Super Mario Bros. 4_



Several other SMB2J conversions running on other mappers reuse the cartridge label from Whirlwind Manu and therefore misleadingly also show an LF36 cartridge code. Hey Sung's version can be distinguished from Whirlwind Manu's by the fact that Whirlwind Manu's does not allow selecting the one of eight starting worlds by holding the D-Pad while pressing START. 

**Submapper 1** denotes the NTDEC **2752** PCB, which is a multicart PCB that adds an outer bank register. 

This description was produced from the source code of FCEUX and Nestopia. 
    
    
     Registers:
     ---------------------------
     Range,Mask:   $8000-FFFF, $E000
    
       $8000:  Disable and acknowledge IRQ
       $A000:  Enable IRQ
       $C000:  Outer bank register (Submapper 1 only)
       $E000:  8 KiB bank mapped at $C000
     
     PRG Setup:
     ---------------------------
     
           $6000   $8000   $A000   $C000   $E000  
         +-------+-------+-------+-------+-------+
         | { 6 } | { 4 } | { 5 } | $E000 | { 7 } |
         +-------+-------+-------+-------+-------+
    

Like [INES Mapper 050](INES_Mapper_050.xhtml "INES Mapper 050"), this hardware produces an interrupt 4096 M2 cycles after the IRQ is enabled. 

The corresponding hardware is a [CD4020 functioning as a 13-bit counter](https://forums.nesdev.org/viewtopic.php?p=217200#p217200), and if the software doesn't acknowledge the interrupt for another 4096 cycles it will self-acknowledge. 

## Outer bank register ($C000-$DFFF)
    
    
    A~[110. .... .ppN PCCM]
                  ||| |||+- 0: Vertical mirroring
                  ||| |||   1: Horizontal mirroring
                  ||| |++-- CHR A14..A13
                  ||| +- PRG A16 and mode
                  |||    0: SMB2J mode
                  |||    1: Regular NROM mode
                  ||+--- NROM mode if P=1
                  ||     0: NROM-128
                  ||     1: NROM-256
                  ++---- PRG A15..A14 if P=1
    

## See also

  * [SMB2j aka "The Lost Levels" Mapper #40 Info](http://nesdev.org/40.txt) by The Mad Dumper.



Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with fixed-timing cycle IRQs](Category_Mappers_with_fixed_timing_cycle_IRQs.xhtml)
