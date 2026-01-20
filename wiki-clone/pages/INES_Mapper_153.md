# INES Mapper 153

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/INES_Mapper_153) | View [other pages](Special_AllPages.xhtml#INES_Mapper_153)

iNES Mapper 153 is used for a [Bandai FCG](Bandai_FCG_board.xhtml "Bandai FCG") board with an [LZ93D50 ASIC](Bandai_LZ93D50_pinout.xhtml "Bandai LZ93D50 pinout") and 8192 bytes of battery-backed WRAM. These 8192 bytes must be denoted as PRG-NVRAM in the [NES 2.0](NES_2_0.xhtml "NES 2.0") header using byte value $70. Only one game, _Famicom Jump II: Saikyou no 7-nin_ , uses this mapper. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 Outer PRG-ROM Bank Select ($8000-$8003 write)
    * 2.2 Inner PRG-ROM Bank Select ($8008 write)
    * 2.3 Nametable Mirroring Type Select ($8009 write)
    * 2.4 IRQ Control ($800A write)
    * 2.5 IRQ Latch ($800B-$800C write)
    * 2.6 PRG-RAM Control ($800D write)
  * 3 Note



# Banks

  * CPU $6000-$7FFF: 8 KiB battery-backed WRAM
  * CPU $8000-$BFFF: 16 KiB switchable PRG-ROM bank
  * CPU $C000-$FFFF: 16 KiB PRG ROM bank, fixed to the last bank
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM



# Registers

## Outer PRG-ROM Bank Select ($8000-$8003 write)
    
    
    Mask: $800F
    
    7  bit  0
    ---- ----
    xxxx xxxP
            |
            +-- Select 256 KiB outer PRG-ROM bank at CPU $8000-$FFFF
    

  * The same value must be written to all four registers, or the outer PRG-ROM bank would be switched as the PPU is rendering.
  * Because the ASIC's PA12 and PA13 inputs are grounded, only registers $8000-$8003 instead of $8000-$8007 are relevant.
  * No CHR banking is available.



## Inner PRG-ROM Bank Select ($8008 write)

## Nametable Mirroring Type Select ($8009 write)

## IRQ Control ($800A write)

## IRQ Latch ($800B-$800C write)

These four registers function the same way as on [INES Mapper 016](INES_Mapper_016.xhtml "INES Mapper 016"), submapper 5. 

## PRG-RAM Control ($800D write)
    
    
    Mask: $800F
    
    7  bit  0
    ---- ----
    ..E. ....
      |
      +-------- PRG-RAM Chip Enable (1=Enable)
    

# Note

When booting with WRAM filled with zeroes, _Famicom Jump II_ will freeze with a black screen. Simply soft-resetting the console will then always run the game properly unless WRAM is zeroed out again. 

Categories: [INES Mappers](Category_INES_Mappers.xhtml), [Mappers with CHR RAM](Category_Mappers_with_CHR_RAM.xhtml), [Mappers with cycle IRQs](Category_Mappers_with_cycle_IRQs.xhtml)
