# NES 2.0 Mapper 535

From [NESdev Wiki](Nesdev_Wiki.xhtml) | View [current version](https://www.nesdev.org/wiki/NES_2.0_Mapper_535) | View [other pages](Special_AllPages.xhtml#NES_2_0_Mapper_535)

NES 2.0 Mapper 535 is used for Whirlwind Manu's cartridge conversion (cartrige code LH53) of the Famicom Disk System Game _謎の村雨城_ (Nazo no Murasamejō). Its UNIF board name is **UNL-LH53**. 

## Contents

  * 1 Banks
  * 2 Registers
    * 2.1 IRQ register ($E000-$EFFF, write)
    * 2.2 PRG bank register ($F000-$FFFF, write)
  * 3 Notes



# Banks

  * CPU $6000-$7FFF: 8 KiB switchable PRG-ROM bank
  * CPU $8000-$FFFF: 32 KiB fixed PRG-ROM bank #3, except where overlaid by PRG-RAM
  * CPU $B800-$D7FF: 8 KiB fixed PRG-RAM bank
  * PPU $0000-$1FFF: 8 KiB unbanked CHR-RAM



# Registers

## IRQ register ($E000-$EFFF, write)
    
    
    Mask: probably $F000
    
    D~7654 3210
      ---------
      .... ..E.
             +-- 1= Enable counting
    

Writing to this register acknowledges a pending IRQ, and resets the counter to zero. When counting is enabled, the counter increases on every M2 cycle; upon reaching a value of 7560, an IRQ is generated (according to [FCEUX source code](https://github.com/TASVideos/fceux/blob/master/src/boards/lh53.cpp)). 

## PRG bank register ($F000-$FFFF, write)

Mask: probably $F000 

Sets the 8 KiB PRG-ROM bank at CPU $6000-$7FFF. 

# Notes

  * The conversion retains all writes to the FDS sound channel registers, so an emulator could provide the expansion sound channel even though the original Whirlwind Manu cartridge did not.
  * Game progress cannot be saved.



Categories: [FDS conversion mappers](Category_FDS_conversion_mappers.xhtml), [Mappers with fixed-timing cycle IRQs](Category_Mappers_with_fixed_timing_cycle_IRQs.xhtml)
