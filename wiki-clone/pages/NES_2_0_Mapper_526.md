# NES 2.0 Mapper 526

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_526) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_526)

NES 2.0 Mapper 526 is used for a bootleg version of Namco's 三国志: 中原の覇者 (_Sangokushi: Chūgen no Hasha_). Its [UNIF](UNIF.xhtml "UNIF") board name is **UNL-BJ-56**. Mirroring seems to be hard-wired (to vertical). 

## Contents

  * 1 Registers
    * 1.1 CHR-ROM bank select ($8000-$8007)
    * 1.2 PRG-ROM bank select ($8008-$800B)
    * 1.3 IRQ counter ($800D/$800F)
  * 2 IRQ Counter Operation



# Registers

## CHR-ROM bank select ($8000-$8007)
    
    
    Mask: unknown, probably $800F
    $8000: Set 1 KiB CHR-ROM bank at PPU $0000-$03FF
    $8001: Set 1 KiB CHR-ROM bank at PPU $0400-$07FF
    $8002: Set 1 KiB CHR-ROM bank at PPU $0800-$0BFF
    $8003: Set 1 KiB CHR-ROM bank at PPU $0C00-$0FFF
    $8004: Set 1 KiB CHR-ROM bank at PPU $1000-$13FF
    $8005: Set 1 KiB CHR-ROM bank at PPU $1400-$17FF
    $8006: Set 1 KiB CHR-ROM bank at PPU $1800-$1BFF
    $8007: Set 1 KiB CHR-ROM bank at PPU $1C00-$1FFF
    

## PRG-ROM bank select ($8008-$800B)
    
    
    Mask: unknown, probably $800F
    $8008: Set 8 KiB PRG-ROM bank at CPU $8000-$9FFF
    $8009: Set 8 KiB PRG-ROM bank at CPU $A000-$BFFF
    $800A: Set 8 KiB PRG-ROM bank at CPU $C000-$DFFF
    $800B: Set 8 KiB PRG-ROM bank at CPU $E000-$FFFF
    

## IRQ counter ($800D/$800F)

Mask: unknown, probably $800F. One of these two acknowledges a pending IRQ, and the other resets to IRQ counter to zero. Because they are always written to one after the other, it's not clear which one does which. 

# IRQ Counter Operation

The IRQ counter is increased on every M2 cycle. The IRQ line is asserted when bit 12 is set and released when bit 12 is cleared. In other words, an IRQ is generated after 4096 M2 cycles and self-acknowledged after further 4096 M2 cycles. 

Categories: [Mappers with fixed-timing cycle IRQs](Category_Mappers_with_fixed_timing_cycle_IRQs.xhtml)
