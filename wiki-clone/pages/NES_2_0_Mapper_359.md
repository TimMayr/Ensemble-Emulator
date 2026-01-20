# NES 2.0 Mapper 359

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_359) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_359)

NES 2.0 Mapper 359 is used for at least three multicarts using a PCB marked alternatively as _SB-5013_ , _GCL8050_ or _841242C_. Its UNIF board name is **BMC-SB-5013**. 

  * _Super Boy 6-in-1_ (CHR-RAM)
  * _Super Boy Top 5-in-1 特輯_ (CHR-RAM)
  * _96 卡聖 Supreme 3-in-1 (NT-646)_ (CHR-ROM)



## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 PRG Bank Select ($8000-$8003, write)
    * 2.2 PRG Base Select ($9000, write)
    * 2.3 Outer Bank Size Select ($9001, write)
    * 2.4 Nametable Mirroring Type Select ($9002, write)
    * 2.5 CHR Base Select ($9003, write)
    * 2.6 CHR Bank Select ($A000-$B003, write)
    * 2.7 IRQ Counter Low/Reload ($C000, write)
    * 2.8 IRQ Counter High/Load Latch ($C001, write)
    * 2.9 IRQ Mode ($C002, write)
    * 2.10 IRQ Disable/Enable ($C003, write)
  * 3 IRQ Operation



# Banks

  * CPU $6000-$7FFF: 8 KiB switchable inner PRG-ROM bank
  * CPU $8000-$9FFF: 8 KiB switchable inner PRG-ROM bank
  * CPU $A000-$BFFF: 8 KiB switchable inner PRG-ROM bank
  * CPU $C000-$DFFF: 8 KiB switchable inner PRG-ROM bank
  * CPU $E000-$FFFF: 8 KiB inner PRG-ROM bank, fixed to last bank (within outer bank)
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM or 8x1 KiB banked CHR-ROM



# Registers

## PRG Bank Select ($8000-$8003, write)

Mask: $F003 

  * $8000: Select 8 KiB PRG-ROM bank at CPU $8000-$9FFF
  * $8001: Select 8 KiB PRG-ROM bank at CPU $A000-$BFFF
  * $8002: Select 8 KiB PRG-ROM bank at CPU $C000-$DFFF
  * $8003: Select 8 KiB PRG-ROM bank at CPU $6000-$7FFF



## PRG Base Select ($9000, write)
    
    
    Mask: $F003
    
    D~7654 3210
      ---------
      ..PP P...
        ++-+---- Select 128 KiB outer PRG-ROM bank
    

## Outer Bank Size Select ($9001, write)
    
    
    Mask: $F003
    
    D~7654 3210
      ---------
      .C.. ..SS
       |     ++- Select size of outer PRG-ROM bank
       |          0: 512 KiB (8 KiB inner PRG-ROM bank ANDed with $3F)
       |          1: 256 KiB (8 KiB inner PRG-ROM bank ANDed with $1F)
       |          2: Unused (8 KiB inner PRG-ROM bank ANDed with $2F?)
       |          3: 128 KiB (8 KiB inner PRG-ROM bank ANDed with $0F)
       +-------- Select size of outer CHR-ROM bank
                  0: 128 KiB (1 KiB inner CHR-ROM bank ANDed with $7F)
                  1: 256 KiB (1 KiB inner CHR-ROM bank ANDed with $FF)
    

## Nametable Mirroring Type Select ($9002, write)
    
    
    Mask: $F003
    
    D~7654 3210
      ---------
      .... ..MM
             ++- Select nametable mirroring type
                  0: Vertical
                  1: Horizontal
                  2: One-screen, page 0
                  3: One-screen, page 1
    

## CHR Base Select ($9003, write)
    
    
    Mask: $F003
    
    D~7654 3210
      ---------
      .... ..CC
             ++- Select 128 KiB outer CHR-ROM bank
    

This register has no effect if CHR-RAM is used instead of CHR-ROM. 

## CHR Bank Select ($A000-$B003, write)

Mask: $F003 

  * $A000: Select 1 KiB CHR-ROM bank at PPU $0000-$03FF
  * $A001: Select 1 KiB CHR-ROM bank at PPU $0400-$07FF
  * $A002: Select 1 KiB CHR-ROM bank at PPU $0800-$0BFF
  * $A003: Select 1 KiB CHR-ROM bank at PPU $0C00-$0FFF
  * $B000: Select 1 KiB CHR-ROM bank at PPU $1000-$13FF
  * $B001: Select 1 KiB CHR-ROM bank at PPU $1400-$17FF
  * $B002: Select 1 KiB CHR-ROM bank at PPU $1800-$1BFF
  * $B003: Select 1 KiB CHR-ROM bank at PPU $1C00-$1FFF



These registers have no effect if CHR-RAM is used instead of CHR-ROM, as the CHR-RAM is unbanked. 

## IRQ Counter Low/Reload ($C000, write)

Mask: $F003 

  * In M2 mode, directly sets the low eight bits of the 16-bit M2 cycle counter.
  * In PA12 mode, writing sets the Reload flag, just as writing to register $C001 does on the [MMC3](MMC3.xhtml "MMC3").
  * In either mode, if the Auto-Enable/Disable Flag is set, disables counting (M2) or just IRQ generation (PA12).



## IRQ Counter High/Load Latch ($C001, write)

Mask: $F003 

  * In M2 mode, directly sets the high eight bits of the 16-bit M2 cycle counter.
  * In PA12 mode, reloads the latch, just as writing to register $C000 does on the [MMC3](MMC3.xhtml "MMC3").
  * In either mode, if the Auto-Enable/Disable Flag is set, enables counting (M2) or just IRQ generation (PA12).



## IRQ Mode ($C002, write)
    
    
    Mask: $F003
    
    D~7654 3210
      ---------
      .... .AME
            ||+- IRQ Disable/Enable
            ||    0=M2 Mode: Disable counting, PA12 Mode: Disable IRQ generation
            ||    1=M2 Mode: Enable counting, PA12 Mode: Enable IRQ generation
            |+-- Select IRQ source
            |     0=CPU Cycles (M2)
            |     1=Filtered PPU A12 rises (PA12)
            +--- Auto-Enable/Disable Flag
                  0=IRQ is not disabled/enabled by writing $C000/$C001, only by writing to $C002/$C003
                  1=IRQ is disabled by writing to $C000 and enabled by writing to $C001, $C003 has no effect
    

## IRQ Disable/Enable ($C003, write)
    
    
    Mask: $F003
    
    D~7654 3210
      ---------
      .... ...E
              +- IRQ Disable/Enable
                  0=M2 Mode: Disable counting, PA12 Mode: Disable IRQ generation
                  1=M2 Mode: Enable counting, PA12 Mode: Enable IRQ generation
    

Writing to $C003 has no effect if Auto-Enable/Disable flag is set. 

# IRQ Operation

  * In M2 mode: If IRQ is enabled, counts down on every M2 cycle until counter is zero, then generates an IRQ.
  * In PA12 mode: Just like the [MMC3](MMC3.xhtml "MMC3")'s scanline counter.



Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml), [Mappers with scanline IRQs](Category_Mappers_with_scanline_IRQs.xhtml), [Multicart mappers](Category_Multicart_mappers.xhtml)
