# NES 2.0 Mapper 548

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_548) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_548)

**NES 2.0 Mapper 548** denotes 科統實業股份有限公司 (Co Tung Co.)'s **CTC-15** circuit board, used for their cartridge conversion of the FDS game _Almanaの奇跡 (Almana no Kiseki)_. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Latch and IRQ control ($4800, write)
    * 2.2 Latch Apply ($5000, write)
  * 3 See also



# Banks

  * CPU $6000-$7FFF: 8 KiB unbanked PRG-RAM
  * CPU $8000-$BFFF: Switchable 16 KiB PRG-ROM bank
  * CPU $C000-$FFFF: Fixed 16 KiB PRG-ROM bank #3
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM



Mirroring is hard-wired to Horizontal. 

# Registers

## Latch and IRQ control ($4800, write)
    
    
    Mask: $F800
    
    A~FEDC BA98 7654 3210
      -------------------
      0100 1... ..A. BC..
                  +--++--- Latch value
                      +--- IRQ control
    
    Power-on value (ABC): $7
    

  * The latch value is not used until the Latch Apply register is written to.
  * IRQ control: 
    * 0: Enable M2 counting and IRQ generation
    * 1: Disable M2 counting and IRQ generation, acknowledge IRQ, counter is reset to 0.



If counting is enabled, the counter is clocked on every falling edge of M2. IRQ is asserted while `((counter÷640)&37)=37`, so it is asserted the first time when the counter reaches 23680 and self-acknowledges the first time when it reaches 24320. 

## Latch Apply ($5000, write)

Mask: $F800 

Upon writing to the $5000-$57FF address range, the latched value written to $4800 is XOR'd with $05 and applied as the 16 KiB PRG-ROM bank at CPU $8000-$BFFF. The power-on bank is $7. 

# See also

[PCB image and analysis](https://forums.nesdev.org/viewtopic.php?f=9&t=19359)

Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with fixed-timing cycle IRQs](Category_Mappers_with_fixed_timing_cycle_IRQs.xhtml)
