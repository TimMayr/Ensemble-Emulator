# NES 2.0 Mapper 454

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_454) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_454)

**NES 2.0 Mapper 454** denotes the PCB (with unknown ID code) of a _110-in-1_ multicart consisting of NROM games in the first 512 KiB and UNROM games in the second 512 KiB, and where PRG A19 selects between NROM and UNROM banking modes. It can be considered a custom implementation of [INES Mapper 227](INES_Mapper_227.xhtml "INES Mapper 227"). 

## Contents

  * 1 Registers
    * 1.1 Address latch ($8000-$FFFF)
    * 1.2 Data latch ($8000-$FFFF)
  * 2 Effective banks



# Registers

## Address latch ($8000-$FFFF)
    
    
    A~[1... ...L OQQP PpMN]
               | |||| |||+- 0: PRG A14=p (NROM-128)
               | |||| |||   1: PRG A14=CPU A14 (NROM-256)
               | |||| ||+-- 0: Vertical mirroring
               | |||| ||    1: Horizontal mirroring
               | |||+-+++-- PRG A16..14 if L=0
               | |++------- PRG A18..17
               | +--------- 0: PRG A18..14=0 if CPU A14=1 ("inverse UNROM")
               |            1: PRG A18..14=QQPPp if CPU A14=1 (NROM)
               +----------- PRG A19,
                            0: NROM mode, address latch active, data latch disabled
                            1: UNROM Mode, address latch disabled, data latch active
    

## Data latch ($8000-$FFFF)
    
    
    D~[.... .PPP]
             +++- PRG A16..A14 if CPU A14==0
    

# Effective banks
    
    
     Bit 8   Bit 7   Bit 0   Meaning
     $100s   $080s   $001s
      (L)     (O)     (S)
       0       0       0     Switchable inner 16 KiB bank PPp at CPU $8000-$BFFF, fixed inner+outer bank #0 at CPU $C000-$FFFF (UNROM-like with fixed bank 0)
       0       0       1     Switchable inner 16 KIB bank PP0 at CPU $8000-$BFFF, fixed inner+outer bank #0 at CPU $C000-$FFFF (UNROM-like with only even banks reachable, pointless)
       0       1       0     Switchable 16 KiB inner bank PPp at CPU $8000-$BFFF, mirrored at CPU $C000-$FFFF (NROM-128)
       0       1       1     Switchable 32 KiB inner bank PP at CPU $8000-$FFFF (NROM-256)
       1       ?       0     Switchable inner 16 KiB bank PPp at CPU $8000-$BFFF, fixed inner bank #7 at CPU $C000-$FFFF (UNROM)
       1       ?       1     Switchable inner 16 KIB bank PP0 at CPU $8000-$BFFF, fixed inner bank #7 at CPU $C000-$FFFF (UNROM with only even banks reachable, pointless)
    

Categories: [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
